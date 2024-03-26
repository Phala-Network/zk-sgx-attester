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

use alloy_primitives::FixedBytes;
use anyhow::Result;
use risc0_groth16::docker::stark_to_snark;
use risc0_zkvm::{
    get_prover_server, recursion::identity_p254, sha::Digestible, ExecutorEnv,
    ExecutorImpl, ProverOpts, VerifierContext,
};

/// An implementation of a Prover that runs on local machine.
pub struct LocalProver {}
impl LocalProver {
    /// Generates a snark proof as a triplet (`Vec<u8>`, `FixedBytes<32>`,
    /// `Vec<u8>) for the given elf and input.
    pub fn prove(elf: &[u8], input: &[u8]) -> Result<(Vec<u8>, FixedBytes<32>, Vec<u8>)> {
        println!("start local proving");
        let env = ExecutorEnv::builder()
            .write_slice(input)
            // .unwrap()
            .build()
            .unwrap();

        println!("execute");
        let mut exec = ExecutorImpl::from_elf(env, elf).unwrap();
        let session = exec.run().unwrap();

        println!("prove");
        let opts = ProverOpts::default();
        let ctx = VerifierContext::default();
        let prover = get_prover_server(&opts).unwrap();
        let receipt = prover.prove_session(&ctx, &session).unwrap();

        let claim = receipt.get_claim().unwrap();
        let composite_receipt = receipt.inner.composite().unwrap();
        let succinct_receipt = prover.compress(composite_receipt).unwrap();
        let journal: Vec<u8> = session.journal.unwrap().bytes;

        println!("identity_p254");
        let ident_receipt = identity_p254(&succinct_receipt).unwrap();
        let seal_bytes = ident_receipt.get_seal_bytes();

        println!("Start translate STARK to SNARK");
        let seal = stark_to_snark(&seal_bytes).unwrap().to_vec();
        println!(
            "Transform finish, proof size decrease from {:} bytes to {:} bytes, snark proof {:?}",
            seal_bytes.len(),
            seal.len(),
            hex::encode(&seal)
        );
        let post_state_digest: [u8; 32] = claim.post.digest().into();

        Ok((journal, post_state_digest.into(), seal))
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::U256;
    use alloy_sol_types::SolValue;
    use methods::IS_EVEN_ELF;

    // RISC0_DEV_MODE=false RUST_LOG=info cargo test --package apps --lib -- local_prover::tests --nocapture
    #[test]
    fn proves_even_number() {
        let even_number = U256::from(1304);

        let (journal, _post_state_digest, seal) =
            super::LocalProver::prove(IS_EVEN_ELF, &even_number.abi_encode()).unwrap();

        println!("Snark proof: {:?}", hex::encode(&seal));
        let x = U256::abi_decode(&journal, true).unwrap();
        assert_eq!(x, even_number);
    }
}
