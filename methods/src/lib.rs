// Copyright 2023 RISC Zero, Inc.
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

//! Generated crate containing the image ID and ELF binary of the build guest.
include!(concat!(env!("OUT_DIR"), "/methods.rs"));

#[cfg(test)]
mod tests {
    use alloy_primitives::U256;
    use alloy_sol_types::SolValue;
    use primitive_io::Outputs;
    use risc0_zkvm::{default_executor, ExecutorEnv};

    #[test]
    fn proves_even_number() {
        let even_number = U256::from(1304);

        let env = ExecutorEnv::builder()
            .write_slice(&even_number.abi_encode())
            .build()
            .unwrap();

        // NOTE: Use the executor to run tests without proving.
        let session_info = default_executor()
            .execute(env, super::DCAP_VERIFIER_ELF)
            .unwrap();

        let x = U256::abi_decode(&session_info.journal.bytes, true).unwrap();
        assert_eq!(x, even_number);
    }

    #[test]
    #[should_panic(expected = "number is not even")]
    fn rejects_odd_number() {
        let odd_number = U256::from(75);

        let env = ExecutorEnv::builder()
            .write_slice(&odd_number.abi_encode())
            .build()
            .unwrap();

        // NOTE: Use the executor to run tests without proving.
        default_executor()
            .execute(env, super::DCAP_VERIFIER_ELF)
            .unwrap();
    }

    #[test]
    fn decode_output() {
        let output_bytes = hex::decode("b789ec190000000000000000000000000000000000000000000000000000000000000060964822424fe9ac792ac16f135b830ca549c8b22188703e72c5656f94af9b320a000000000000000000000000000000000000000000000000000000000000018000000000000000000000000000000000000000000000000000000000000000f9400000000000000048656c6c6f2c20776f726c6421000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000033d8736db756ed4997e04ba358d27833188f1932ff7b1d156904d3f560452fbb2000000000000000815f42f11cf64430c30bab7816ba596a1da0130c3b028b673133a66cf9a3e0e6000000002100000000000000436f6e66696775726174696f6e416e64535748617264656e696e674e656564656402000000000000000e00000000000000494e54454c2d53412d30303238390e00000000000000494e54454c2d53412d30303631350000000000000000000000000000000000000000000000000000000000000000000000000001001670712988d6d4c6b41304fc0741ea690d0ee3c2784f829eb32653ddef46983f137ebbd2e42da0f07fe517f06db64bb6901d5360a5c464b7ebc25f6c2fe8fc712a0c167bc18393f7c7d04bb0152cc9dac8b299cf092c7db35908732012a88c7908ef34b24386f09e386a67af9869de90d8b7ab753738294ecf057bc8b2b046982b2c933992f42951d12a8160a04ada681146ecda39a9483cedb2d04e161f44cc250e2e889577e6d41140cbc2055655343b58cb310ad4fea37f4c67c647eb485c06131bf967f3ff6c4b848588b1d996fbb4501ba0739be8b4dd783d5ceb59555a257109aae3ef3e6a63f48a774404e3e1bf6c43f9740cf9000feb6f390e1a509c").unwrap();
        let output: Outputs = bincode::deserialize(&output_bytes).unwrap();
        println!("The decoded output: {:?}", output);
    }
}
