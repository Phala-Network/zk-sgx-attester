# RISC Zero Ethereum Deployment Guide

> **Note: This software is not production ready. Do not use in production.**

Welcome to the [RISC Zero] Ethereum Deployment guide!

Once you've written your [contracts] and your [methods], and [tested] your program, you're ready to deploy your contract.

You can either:

- [Deploy your project to a local network]
- [Deploy to a testnet]

## Deploy your project on a local network

You can deploy your contracts and run an end-to-end test or demo as follows:

1. Start a local testnet with `anvil` by running:

    ```bash
    anvil
    ```

    Once anvil is started, keep it running in the terminal, and switch to a new terminal.

2. Set your environment variables:
    > ***Note:*** *We don't use Bonsai prover, so no need to config Bonsai API key*

    ```bash
    # Anvil sets up a number of default wallets, and this private key is one of them.
    export ETH_WALLET_PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
    ```

3. Build your project:

    ```bash
    cargo build
    ```

4. Deploy your contract by running:

    ```bash
    forge script --rpc-url http://localhost:8545 --broadcast script/Deploy.s.sol
    ```

    This command should output something similar to:

    ```bash
    ...
    == Logs ==
    Deployed RiscZeroGroth16Verifier to 0x5FbDB2315678afecb367f032d93F642f64180aa3
    Deployed DcapVerifier to 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
    ...
    ```

    Save the `DcapVerifier` contract address to an env variable:

    ```bash
    export DCAP_VERIFIER_ADDRESS=0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512#COPY DCAP VERIFIER ADDRESS FROM DEPLOY LOGS
    ```

    > You can also use the following command to set the contract address if you have [`jq`][jq] installed:
    >
    > ```bash
    > export DCAP_VERIFIER_ADDRESS=$(jq -re '.transactions[] | select(.contractName == "DcapVerifier") | .contractAddress' ./broadcast/Deploy.s.sol/31337/run-latest.json)
    > ```

### Interact with your local deployment

1. Query the state:

    ```bash
    cast call --rpc-url http://localhost:8545 ${DCAP_VERIFIER_ADDRESS:?} 'get()(bytes memory)'
    ```

You will see the outputs like below, because no dcap data been submitted:

    ```bash
    0x0
    ```

2. Publish a new state (this will verify the hardcoded DCAP input and submit the proof and **serialized** output to DcapVerifier contract)

    ```bash
    RISC0_DEV_MODE=false cargo run --bin publisher -- \
        --chain-id=31337 \
        --rpc-url=http://localhost:8545 \
        --contract=${DCAP_VERIFIER_ADDRESS:?}
    ```

3. Query the state again to see the change:

    ```bash
    cast call --rpc-url http://localhost:8545 ${DCAP_VERIFIER_ADDRESS:?} 'get()(bytes memory)'
    ```

You will see the output like below, the submitted DCAP outputs returned:

    ```bash
    wwf@ubuntu-default:~/zk-dcap-verifier(dcap-guest)$ cast call --rpc-url http://localhost:8545 ${DCAP_VERIFIER_ADDRESS:?} 'get()(bytes memory)'
    0x400000000000000048656c6c6f2c20776f726c6421000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000200000000000000033d8736db756ed4997e04ba358d27833188f1932ff7b1d156904d3f560452fbb2000000000000000815f42f11cf64430c30bab7816ba596a1da0130c3b028b673133a66cf9a3e0e6000000002100000000000000436f6e66696775726174696f6e416e64535748617264656e696e674e656564656402000000000000000e00000000000000494e54454c2d53412d30303238390e00000000000000494e54454c2d53412d3030363135
    ```

## Deploy your project on a testnet

You can deploy your contracts on a testnet such as `Sepolia` and run an end-to-end test or demo as follows:

1. Get access to Ethereum node running on a given testnet, e.g., Sepolia (in this example, we will be using [Alchemy](https://www.alchemy.com/) as our Ethereum node provider) and export the following environment variables:

    ```bash
    export ALCHEMY_API_KEY="YOUR_ALCHEMY_API_KEY" # the API_KEY provided with an alchemy account
    export ETH_WALLET_PRIVATE_KEY="YOUR_WALLET_PRIVATE_KEY" # the private hex-encoded key of your Sepolia testnet wallet
    ```

2. Build your project:

    ```bash
    cargo build
    ```

3. Deploy your contract by running:

    ```bash
    forge script script/Deploy.s.sol --rpc-url https://eth-sepolia.g.alchemy.com/v2/${ALCHEMY_API_KEY:?} --broadcast
    ```

     This command should output something similar to:

    ```bash
    ...
    == Logs ==
    Deployed RiscZeroGroth16Verifier to 0x5FbDB2315678afecb367f032d93F642f64180aa3
    Deployed DcapVerifier to 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512
    ...
    ```

    Save the `DcapVerifier` contract address to an env variable:

    ```bash
    export DCAP_VERIFIER_ADDRESS=#COPY DCAP VERIFIER ADDRESS FROM DEPLOY LOGS
    ```

    > You can also use the following command to set the contract address if you have [`jq`][jq] installed:
    >
    > ```bash
    > export DCAP_VERIFIER_ADDRESS=$(jq -re '.transactions[] | select(.contractName == "DcapVerifier") | .contractAddress' ./broadcast/Deploy.s.sol/11155111/run-latest.json)
    > ```

### Interact with your testnet deployment

1. Query the state:

    ```bash
    cast call --rpc-url https://eth-sepolia.g.alchemy.com/v2/${ALCHEMY_API_KEY:?} ${DCAP_VERIFIER_ADDRESS:?} 'get()(uint256)'
    ```

2. Publish a new state

    ```bash
    cargo run --bin publisher -- \
        --chain-id=11155111 \
        --rpc-url=https://eth-sepolia.g.alchemy.com/v2/${ALCHEMY_API_KEY:?} \
        --contract=${DCAP_VERIFIER_ADDRESS:?} \
        --input=12345678
    ```

3. Query the state again to see the change:

    ```bash
    cast call --rpc-url https://eth-sepolia.g.alchemy.com/v2/${ALCHEMY_API_KEY:?} ${DCAP_VERIFIER_ADDRESS:?} 'get()(uint256)'
    ```

[Deploy to a testnet]: #deploy-your-project-on-a-testnet
[Deploy your project to a local network]: #deploy-your-project-on-a-local-network
[RISC Zero]: https://www.risczero.com/
[contracts]: ./contracts/
[jq]: https://jqlang.github.io/jq/
[methods]: ./methods/
[tested]: ./README.md#run-the-tests
