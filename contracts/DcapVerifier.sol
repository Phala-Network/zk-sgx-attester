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
//
// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.20;

import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {ImageID} from "./ImageID.sol"; // auto-generated contract after running `cargo build`.
import {Ownable} from "openzeppelin-contracts/contracts/access/Ownable.sol";

contract DcapVerifier is Ownable {
    /// @notice RISC Zero verifier contract address.
    IRiscZeroVerifier public verifier;
    /// @notice Image ID of the only zkVM binary to accept verification from.
    bytes32 public imageId = ImageID.DCAP_VERIFIER_ID;

    /// @notice Initialize the contract, binding it to a specified RISC Zero verifier.
    constructor(IRiscZeroVerifier _verifier) Ownable(msg.sender) {
        verifier = _verifier;
    }

    /// @notice Update RISC Zero verifier and corresponding imageId.
    function setVerifier(address _verifier, bytes32 _imageId) external onlyOwner {
        verifier = IRiscZeroVerifier(_verifier);
        imageId = _imageId;
    }

    /// @notice Check the proof of attestation verification and return the attestation output.
    function verifyAttestation(bytes calldata x, bytes32 postStateDigest, bytes calldata seal)
        external
        returns(bytes memory) {
        // Construct the expected journal data. Verify will fail if journal does not match.
        bytes memory journal = x;
        require(verifier.verify(seal, imageId, postStateDigest, sha256(journal)));

        return journal;
    }
}
