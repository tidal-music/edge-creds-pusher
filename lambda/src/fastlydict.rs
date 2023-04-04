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

use fastly_api::apis::configuration::{ApiKey, Configuration};
use fastly_api::apis::Error;
use fastly_api::apis::dictionary_item_api::{upsert_dictionary_item, UpsertDictionaryItemParams, UpsertDictionaryItemError};

#[derive(Debug)]
pub struct FastlyDict {
	service_id: String,
	dictionary_id: String,
	api_key: String,
}

impl FastlyDict {
	pub async fn new(service_id: String, dictionary_id: String, api_key: String) -> Self {
		FastlyDict{
			service_id: service_id,
			dictionary_id: dictionary_id,
			api_key: api_key,
		}
	}

	pub async fn upsert(&self, key: &str, val: &str) -> Result<(), Error<UpsertDictionaryItemError>> {
		let params = UpsertDictionaryItemParams{
			dictionary_id: self.dictionary_id.clone(),
			dictionary_item_key: key.to_string(),
			service_id: self.service_id.clone(),
			item_value: Some(val.to_string()),
			..Default::default()
		};
		let mut cfg = Configuration{
			api_key: Some(ApiKey{
				prefix: None,
				key: self.api_key.clone(),
			}),
			..Default::default()
		};


		match upsert_dictionary_item(&mut cfg, params).await {
			Ok(_) => Ok(()),
			Err(e) => return Err(e),
		}
	}
}