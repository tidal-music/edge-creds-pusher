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

use aws_sdk_sts::{Client};
use aws_config::{SdkConfig};
use aws_sdk_sts::types::{SdkError};
use aws_sdk_sts::error::{AssumeRoleError};
use std::fmt::{Debug, Display, Formatter};
use std::error::Error;

const DEFAULT_SESSION_TIME: i32 = 3600*4; // 4 hours

#[derive(Debug)]
pub struct STS {
	role_arn: String,
    sts_client: Client,
}

#[derive(Debug)]
pub struct Creds {
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub aws_session_token: String,
}

pub enum STSError {
    InvalidResponse,
	AWS(SdkError<AssumeRoleError>),
}

impl Debug for STSError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
            STSError::InvalidResponse => Display::fmt(self, f),
			STSError::AWS(error) => Display::fmt(error, f),
		}
	}
}

impl Error for STSError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            STSError::InvalidResponse => None,
            STSError::AWS(error) => Some(error)
        }
    }
}

impl Display for STSError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
            STSError::InvalidResponse => write!(f, "STS invalid response"),
			STSError::AWS(error) => Display::fmt(error, f),
		}
	}
}

impl From<SdkError<AssumeRoleError>> for STSError {
	fn from(error: SdkError<AssumeRoleError>) -> Self {
		STSError::AWS(error)
	}
}

impl STS {
	pub async fn new(role_arn: String, aws_config: &SdkConfig) -> Self {
        let client = Client::new(aws_config);

		STS{
			role_arn: role_arn,
            sts_client: client,
		}
	}

	pub async fn get(&self) -> Result<Creds, STSError> {
        println!("assuming role {}", self.role_arn);
        let response = self.sts_client.assume_role()
            .role_arn(&self.role_arn)
            .role_session_name("fastlycredspusher")
            .duration_seconds(DEFAULT_SESSION_TIME)
            .send()
            .await;

        match response {
            Ok(out) => {
                match out.credentials() {
                    None => return Err(STSError::InvalidResponse),
                    Some(sts_creds) => {
                        let key_id = sts_creds.access_key_id().unwrap();
                        let secret = sts_creds.secret_access_key().unwrap();
                        let session_token = sts_creds.session_token().unwrap();
                        let c = Creds { aws_access_key_id: key_id.to_string(), aws_secret_access_key: secret.to_string(), aws_session_token: session_token.to_string()}; 
                        return Ok(c);
                    }
                }
            }
            Err(e) => {
                return Err(STSError::AWS(e))
            }
        }
    }
}