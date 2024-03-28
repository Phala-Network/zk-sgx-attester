// Copyright 2024 RISC Zero, Inc.
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

contract DcapVerifier {
    /// @notice RISC Zero verifier contract address.
    IRiscZeroVerifier public immutable verifier;
    /// @notice Image ID of the only zkVM binary to accept verification from.
    bytes32 public constant imageId = ImageID.DCAP_VERIFIER_ID;

    /// @notice Public outputs of the DCAP, that is guaranteed, by the RISC Zero zkVM, to be valid.
    ///         It can be set by calling the `set` function.
    /// workerId => DCAPOutputs
    mapping (bytes32 => bytes) public outputs;

    /// @notice Initialize the contract, binding it to a specified RISC Zero verifier.
    constructor(IRiscZeroVerifier _verifier) {
        verifier = _verifier;
    }

    /// @notice Set the outputs of DCAP. Requires a RISC Zero proof that the DCAP is valid.
    function set(bytes calldata x, bytes32 postStateDigest, bytes calldata seal) public {
        // Construct the expected journal data. Verify will fail if journal does not match.
        bytes memory journal = x;
        require(verifier.verify(seal, imageId, postStateDigest, sha256(journal)));
        // Hardcode for debug
        outputs[bytes32(0)] = journal;
    }

    /// @notice Returns the number stored.
    function get() public view returns (bytes memory) {
        // Hardcode for debug
        return outputs[bytes32(0)];
    }
}
