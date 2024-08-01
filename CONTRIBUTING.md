# Contributing to Solana Serialization Benchmark

This is a quick guide to help you contribute to Solana Serialization Benchmark.

## Getting started

The root folder has a private `package.json` containing a few scripts and JavaScript dependencies that help generate IDLs; clients and start a local validator. First, [ensure you have pnpm installed](https://pnpm.io/installation) and run the following command to install the dependencies.

```sh
pnpm install
```

You will then have access to the following commands.

- `pnpm programs:build` - Build all programs and fetch all dependant programs.
- `pnpm programs:test` - Test all programs.
- `pnpm programs:debug` - Test all programs with logs enabled.
- `pnpm programs:clean` - Clean all built and fetched programs.
- `pnpm clients:js:test` - Run the JS client tests.
- `pnpm generate` - Shortcut for `pnpm generate:idls && pnpm generate:clients`.
- `pnpm generate:idls` - Generate IDLs for all programs, as configured in the `configs/shank.cjs` file.
- `pnpm generate:clients` - Generate clients using Kinobi, as configured in the `configs/kinobi.cjs` file.
- `pnpm validator` - Start a local validator using Amman, as configured in the `configs/validator.cjs` file.
- `pnpm validator:debug` - Start a local validator using Amman with logs enabled, as configured in the `configs/validator.cjs` file.
- `pnpm validator:stop` - Stop the local validator.
- `pnpm validator:logs` - Show the logs of the local validator.

## Managing clients

Each client has its own README with instructions on how to get started. You can find them in the `clients` folder.

- [JavaScript client](./clients/js/README.md)

In order to generate the clients, run the following command.

```sh
pnpm generate
```

You will need to run `pnpm generate` to re-generate the clients when something changes in the program(s).

## Adding a new serialization library and benchmark

Take the following steps to add a new serialization library and benchmark to the project.

### 1. Add the new serialization library to the `Cargo.toml` file

Add the new serialization library to the `Cargo.toml` file in the `programs/solana-serialization-benchmark` folder.

### 2. Create a new state file

Create a new state file in the `programs/solana-serialization-benchmark/src/state` folder by copying the `state.template.rs` file and renaming it to the name of the serialization library + `_state.rs`. Add the required derive traits or implementations to the new file on the BasicTypes and CollectionTypes structs.

### 3. Add new instructions

Add new instructions to the `programs/solana-serialization-benchmark/src/instruction.rs` file by copying the `CreateBasicNone`, `ReadBasicNone`, `UpdateBasicNone`, `CreateCollectionNone`, `ReadCollectionNone`, `UpdateCollectionNone` instructions and renaming them to the name of the serialization library + `CreateBasicSERDESLIB`, `ReadBasicSERDESLIB`, `UpdateBasicSERDESLIB`, `CreateCollectionSERDESLIB`, `ReadCollectionSERDESLIB`, `UpdateCollectionSERDESLIB`.

### 4. Add new processors

Add new processors to the `programs/solana-serialization-benchmark/src/processor` folder by copying the `processor.template` folder and renaming it to the name of the serialization library + `_processor`. Modify the `processor.template/mod.rs` file processors to serialize and deserialize the account data as is appropriate for the new serialization library.

### 5. Add the benchmark tests

Add the new serialization library to the `clients/js/bench` folder by copying the `_template.ts` file and renaming it to the name of the serialization library + `.ts`. Replace the "SERDESLIB" with the name of the serialization library. This should match up with the new instruction names in the `clients/js/src/generated/instructions` folder.

### 6. Run the benchmarks

Run the benchmarks by running `pnpm bench` in the `clients/js` folder. This will run the benchmarks for all the serialization libraries and output the results to the `docs/output.json` file.

### 7. Open a PR

Open a PR with the new serialization library and benchmark changes. Once the PR is merged the results will be published to https://metaplex-foundation.github.io/solana_serialization_benchmark/.
