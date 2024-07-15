# Aurora `SHA256` calculation Proof of Concept

This smartcontract on NEAR shows the results of the `sha256` calculation, which proves
the invulnerability and stability of the `sha256` function:

- for Aurora Engine smart contract SDK utility
- for NEAR protocol `sha256` host function called via `near-sdk`.
- for rust crate `sha2`

#### Calculation results

The hash calculation is done using functions:
➡️ `aurora_engine_sdk::sha256`
➡️ `near_sdk::sha256`
➡️ `sha2::Sha256::digest`

The results of all functions are compared for complete compliance.

After verification, the execution results are output as NEAR transaction logs.
➡️ Execution results as sample call for NEAR transaction with logs
for `testnet`: [As2hzF71k4wQDqGnJCMQogT2Mv46L96nGM4A4GAKm1ma](https://testnet.nearblocks.io/txns/As2hzF71k4wQDqGnJCMQogT2Mv46L96nGM4A4GAKm1ma)

### Useful commands

- `cargo make build` - build smartcontract
    - `cargo near build` - another way to build smartcontract
- `cargo make clippy` - run clippy (linter) check
- `cargo near create-dev-account` - optional step to create dev-account on `testnet`
- `near deploy <smart-contract-account-id> bin/contract.wasm` - after build - deploy smartcontract for specific account
  id
    - `cargo near deploy` - alternative wat to deploy smartcontract
- `near call <smart-contract-account-id> get_sha256 --useAccount=<your-signer-account-id>` - call for `get_sha256` for
  specific smartcontract
    - `near call dev-aurora-1.testnet get_sha256 --useAccount=<your-signer-account-id>` - call deployed smartcontract
      for `get_sha256`
- `near tx-status <tx-hash> <your-signer-account-id>` - get NEAR transaction status for `testnet`
    - `near tx-status As2hzF71k4wQDqGnJCMQogT2Mv46L96nGM4A4GAKm1ma <your-signer-account-id>` - get already existed
      transaction that **check sha256-correctness**

### ⚠️ Important notes

`aurora_engine_sdk` and `near_sdk` actually use the same host function of `nearcore`.
The purpose of this function (in term of WebAssembly - `host functions`) is to use the resources of
the NEAR node, thereby minimizing the consumption of smartcontract `Gas` for complex mathematical
calculations. For example, in EVM, `precompiles` are responsible for this.

The API host function is presented as:

```
extern "C" {
    fn sha256(value_len: u64, value_ptr: u64, register_id: u64);
}
```

This allows the WebAssembly to call external functions (also known in Rust lang as `FFI`). `FFI` is the only
possible secure mechanism for interacting with external functions. The Virtual Machine that runs the WebAssembly
takes responsibility for ensuring interaction with external (host) functions.

⚠️ There is special NOTE: it's `C` incompatible API.
➡️ It's Rust fully compatible API for FFI.

#### ⚠️ Empty input considerations

The `Sha256` hashing functions provided in the source code (`aurora_engine_sdk`, `near_sdk`) are completely
secure for the reasons:

- Its always correctly calculates the length of the input data array
    - this also applies to an empty input data array
- Passing an empty data array is expected and correct behavior
    - This is proven by the fact that the host function uses the `sha2` library, which correctly handles data for empty
      arrays and strings.
- The smartcontract example also includes an example of calculating an empty array with the `sha2` library. This
  shows the coincidence of the results of the host function and direct calculations in the smartcontract.
- It is important to note that the host function called in `nearcore` is implemented in Rust, and meets all the
  requirements for safe interaction with raw memory pointers.
- Additional notes about FFI call in Rust: by design it's always wrapped as `unsafe` as it calls uncontrolled external
  functions.

➡️ **As CONCLUSION**: The arguments given above and the specific implementation of the example smartcontract,
and the NEAR transactions show that the situation of uncontrolled interaction with memory, and as a consequence
the vulnerability of the smart contract, or unreliability of behavior, are impossible, which is ensured by the
guarantees of security of interaction with the memory of the Rust language.
