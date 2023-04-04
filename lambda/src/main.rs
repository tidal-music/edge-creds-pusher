// Copyright 2023 Aspiro AB
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::env;

use aws_lambda_events::cloudwatch_events::CloudWatchEvent;
use lambda_runtime::{Error, LambdaEvent, run, service_fn};

use tracing::{Level, event};

use crate::sts::STS;
use crate::ssm::SSM;
use crate::fastlydict::FastlyDict;

mod config;
mod sts;
mod ssm;
mod fastlydict;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let log_level = match env::var("LOG_LEVEL") {
        Ok(val) => {
            match val.as_str() {
                "TRACE" => Level::TRACE,
                "DEBUG" => Level::DEBUG,
                "WARN" => Level::DEBUG,
                "ERROR" => Level::ERROR,
                _ => Level::INFO,
            }
        },
        Err(_) => Level::INFO,
    };
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .without_time()
        .init();

    let context = Context::new().await;
    let handler = |event: LambdaEvent<CloudWatchEvent>| handle(event.payload, &context);

    run(service_fn(handler)).await
}

async fn handle(_event: CloudWatchEvent, context: &Context) -> Result<(), Error> {
    event!(Level::INFO, "assuming role & pushing creds to fastly....");
    let creds = context.s.get().await;
    match creds {
        Ok(c) => {
            match context.f.upsert("aws_access_key_id", &c.aws_access_key_id).await {
                Ok(_) => event!(Level::INFO, "aws access key id pushed"),
                Err(e) => return Err(Error::from(e)),
            }
            match context.f.upsert("aws_secret_access_key", &c.aws_secret_access_key).await {
                Ok(_) => event!(Level::INFO, "aws secret key pushed"),
                Err(e) => return Err(Error::from(e)),
            }
            match context.f.upsert("aws_session_token", &c.aws_session_token).await {
                Ok(_) => event!(Level::INFO, "aws session token pushed"),
                Err(e) => return Err(Error::from(e)),
            }


        }
        Err(e) => {
            event!(Level::ERROR, "error fetching creds: {:?}", e);
            return Err(Error::from(e));
        }
    }
    Ok(())
}

#[derive(Debug)]
struct Context {
    s: STS,
    f: FastlyDict,
}

impl Context {
    async fn new() -> Self {
        let role_arn = env::var("ASSUME_ROLE_ARN")
            .expect("Env ASSUME_ROLE_ARN must be set");

        let fastly_api_key_ssm_name = env::var("FASTLY_API_TOKEN_SSM_NAME")
            .expect("Env FASTLY_API_TOKEN_SSM_NAME must be set");

        let fastly_service_id= env::var("FASTLY_SERVICE_ID")
            .expect("Env FASTLY_SERVICE_ID must be set");

        let fastly_dictionary_id= env::var("FASTLY_DICTIONARY_ID")
            .expect("Env FASTLY_DICTIONARY_ID must be set");



        let aws_config = config::fetch_aws_config().await;

        let sts = STS::new(role_arn.clone(), &aws_config).await;
        let ssm = SSM::new(&aws_config).await;

        let fastly_api_key = match ssm.get(&fastly_api_key_ssm_name).await {
            Ok(val) => val,
            Err(e) => panic!("Error fetching ssm param: {:?}", e),
        };

        let fastly_dict = FastlyDict::new(fastly_service_id, fastly_dictionary_id, fastly_api_key).await;

        Context {s: sts, f: fastly_dict}
    }
}