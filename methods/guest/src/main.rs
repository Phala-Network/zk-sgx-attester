// Copyright 2024 Phala Network.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
extern crate alloc;
extern crate core;

use std::io::Read;
use risc0_zkvm::guest::env;
use primitive_io::{Inputs, Outputs};
use scale_codec::Decode;

mod error;
use error::Error;
mod dcap;

fn main() {
    // Read the input data for this application.
    let mut input_bytes = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input_bytes).unwrap();

    // Deserialize input bytes
    let input: Inputs = bincode::deserialize(&input_bytes).unwrap();

    println!("Guest: check if dcap input is valid");
    let now = input.now;
    let raw_quote = input.quote;
    let raw_quote_collateral = input.quote_collateral;

    let quote_collateral =
        dcap::SgxV30QuoteCollateral::decode(&mut raw_quote_collateral.as_slice()).unwrap();
    let (report_data, mr_enclave, mr_signer, isv_prod_id, isv_svn, tcb_status, advisory_ids) =
        dcap::verify(&raw_quote, &quote_collateral, now).unwrap();

    let output = Outputs {
        report_data,
        mr_enclave,
        mr_signer,
        isv_prod_id,
        isv_svn,
        tcb_status,
        advisory_ids
    };

    println!("Guest: check passed, commit output to host");

    // write public output to the journal
    env::commit_slice(bincode::serialize(&output).unwrap().as_slice());
}
