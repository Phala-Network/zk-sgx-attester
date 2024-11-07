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

// The following library provides utility functions to help generate SNARK proof
// of the execution of guest code, here by mean verify DCAP of Phala workers.

use anyhow::Result;
use risc0_ethereum_contracts::encode_seal;
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, VerifierContext};

/// An implementation of a Prover that runs on local machine.
pub struct LocalProver {}
impl LocalProver {
    /// Generates a snark proof as a triplet (`Vec<u8>`, `FixedBytes<32>`,
    /// `Vec<u8>) for the given elf and input.
    pub fn prove(elf: &[u8], input: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        let env = ExecutorEnv::builder().write_slice(input).build().unwrap();

        log::info!("Start local proving");
        let prover_info = default_prover().prove_with_ctx(
            env,
            &VerifierContext::default(),
            elf,
            &ProverOpts::groth16(),
        )?;
        log::info!(
            "Proving finished, receipt: {:?}, stats: {:?}",
            &prover_info.receipt,
            &prover_info.stats
        );

        let seal = encode_seal(&prover_info.receipt)?;
        let journal = prover_info.receipt.journal.bytes.clone();

        Ok((journal, seal))
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::U256;
    use alloy_sol_types::SolValue;
    use methods::DCAP_VERIFIER_ELF;

    // RISC0_DEV_MODE=false RUST_LOG=info cargo test --package apps --lib -- local_prover::tests --nocapture
    #[test]
    fn proves_even_number() {
        let even_number = U256::from(1304);

        let (journal, _post_state_digest, seal) =
            super::LocalProver::prove(DCAP_VERIFIER_ELF, &even_number.abi_encode()).unwrap();

        log::info!("Snark proof: {:?}", hex::encode(&seal));
        let x = U256::abi_decode(&journal, true).unwrap();
        assert_eq!(x, even_number);
    }
}
