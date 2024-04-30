# Phala ZK-DCAP-Verifier

## [Optional] Build Docker image

```sh
$ docker build -t phala-zk-dcap-verifier .
```

## Submit DCAP verification ZK proof to blockchain with docker container

RISC0_DEV_MODE=false docker run -it --rm phala-zk-dcap-verifier \
          --chain-id=<chainId> \
          --rpc-url=<rpc endpoint> \
          --contract=<dcap verfier contract address> \
          --eth-wallet-private-key=<account key to submit proof>

Head to [deployment-guid](./deployment-guid.md) if you test with local testnet
