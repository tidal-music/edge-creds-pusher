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

use aws_sdk_ssm::{Client};
use aws_sdk_ssm::types::{SdkError};
use aws_sdk_ssm::error::{GetParameterError};
use aws_config::{SdkConfig};
use std::fmt::{Debug, Display, Formatter};


#[derive(Debug)]
pub struct SSM {
	client: Client,
}

pub enum SSMError {
	EmptyParameter,
	ParameterNotExist,
	AWS(SdkError<GetParameterError>),
}

impl Debug for SSMError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			SSMError::EmptyParameter => Display::fmt(self, f),
			SSMError::ParameterNotExist => Display::fmt(self, f),
			SSMError::AWS(error) => Display::fmt(error, f),
		}
	}
}

impl Display for SSMError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			SSMError::EmptyParameter => write!(f, "SSM parameter was empty"),
			SSMError::ParameterNotExist=> write!(f, "SSM parameter doesn't exist"),
			SSMError::AWS(error) => Display::fmt(error, f),
		}
	}
}

impl From<SdkError<GetParameterError>> for SSMError {
	fn from(error: SdkError<GetParameterError>) -> Self {
		SSMError::AWS(error)
	}
}

impl SSM {
	pub async fn new(aws_config: &SdkConfig) -> SSM {
		let client = Client::new(aws_config);
		SSM{
			client: client,
		}
	}

	pub async fn get(&self, name: &String) -> Result<String, SSMError> {
		println!("fetching ssm parameter: {}", name);
		let res = self.client.get_parameter()
			.name(name)
			.with_decryption(true)
			.send()
			.await;
		match res {
			Ok(output) => {
				match output.parameter() {
					None => return Err(SSMError::ParameterNotExist),
					Some(p) => {
						match p.value() {
							None => return Err(SSMError::EmptyParameter),
							Some(v) => {
								return Ok(v.to_string())
							}
						}
					}
				}
			}
			Err(err) => { 
				return Err(SSMError::from(err));
			}
		}
	}
}
