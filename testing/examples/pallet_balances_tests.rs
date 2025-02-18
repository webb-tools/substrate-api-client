/*
	Copyright 2019 Supercomputing Systems AG
	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.
*/

//! Tests for the pallet balances interface functions.

use substrate_api_client::{
	ac_primitives::AssetRuntimeConfig, rpc::JsonrpseeClient, Api, GetBalance,
};

#[tokio::main]
async fn main() {
	// Setup
	let client = JsonrpseeClient::with_default_url().unwrap();
	let api = Api::<AssetRuntimeConfig, _>::new(client).unwrap();

	let _ed = api.get_existential_deposit().unwrap();
}
