# Metadata Rust CLI

This CLI is used to interact with the metadata program. You can upload IDLs and metadata to Solana programs so that everyone can access them by just knowing the program id. It supports uploading from local files or URLs. You can also download the IDL and metadata from a program to a file on your system. The data is stored on-chain in a compressed format to minimize storage costs while maintaining accessibility.

The CLI provides a simple interface to manage both IDL (Interface Description Language) files, which describe a program's interface, and additional metadata that can provide more context about the program.

## Build the CLI

To build the CLI make sure to be inside the `rust_cli` directory.

Then build the CLI:

```bash
cargo build
```

## Usage

```bash
program-metadata [SUBCOMMAND]
```

### Commands

#### IDL Management

```bash
# Upload an IDL from local file
program-metadata idl upload <IDL_PATH> <PROGRAM_ID> [OPTIONS]

# Upload an IDL from URL
program-metadata idl upload-url <URL> <PROGRAM_ID> [OPTIONS]

# Download an IDL
program-metadata idl download <PROGRAM_ID> [OUTPUT_PATH]

Options:
  -k, --keypair <PATH>           Path to keypair file (optional, defaults to config)
  -p, --priority-fees <NUMBER>   Priority fees per compute unit (default: 0)
  -u, --rpc-url <URL>           RPC URL (optional, defaults to config)
  --help                         Print help information
```

#### Metadata Management

```bash
# Upload metadata from local file
program-metadata metadata upload <METADATA_PATH> <PROGRAM_ID> [OPTIONS]

# Upload metadata from URL
program-metadata metadata upload-url <URL> <PROGRAM_ID> [OPTIONS]

# Download metadata
program-metadata metadata download <PROGRAM_ID> [OUTPUT_PATH]

Options:
  -k, --keypair <PATH>           Path to keypair file (optional, defaults to config)
  -p, --priority-fees <NUMBER>   Priority fees per compute unit (default: 0)
  -u, --rpc-url <URL>           RPC URL (optional, defaults to config)
  --help                         Print help information
```

Examples:

```bash
# Upload IDL using default keypair and custom RPC URL
program-metadata idl upload ./target/idl/my_program.json GrAkz4CQ4zKm9KhZ9Q7PkCmqDP7JuSGbpwGY8dxKt6Kj -u https://api.devnet.solana.com

# Upload IDL from URL with custom keypair, priority fees, and RPC URL
program-metadata idl upload-url https://example.com/my_idl.json GrAkz4CQ4zKm9KhZ9Q7PkCmqDP7JuSGbpwGY8dxKt6Kj -k ./my-keypair.json -p 1000 -u https://api.mainnet-beta.solana.com

# Download IDL using custom RPC URL
program-metadata idl download GrAkz4CQ4zKm9KhZ9Q7PkCmqDP7JuSGbpwGY8dxKt6Kj -u https://api.testnet.solana.com

# Upload metadata using default keypair
program-metadata metadata upload ./metadata.json GrAkz4CQ4zKm9KhZ9Q7PkCmqDP7JuSGbpwGY8dxKt6Kj

# Upload metadata from URL
program-metadata metadata upload-url https://example.com/metadata.json GrAkz4CQ4zKm9KhZ9Q7PkCmqDP7JuSGbpwGY8dxKt6Kj

# Download metadata to custom path
program-metadata metadata download GrAkz4CQ4zKm9KhZ9Q7PkCmqDP7JuSGbpwGY8dxKt6Kj ./my-metadata.json
```

## Testing the Rust CLI against a local validator from within this repository

Start your local validator:

```bash
solana-test-validator --reset
```

Make sure you point your cluster to localnet and fix the program id in `Anchor.toml`.
Fix the program id in `programs/upload-idl-program/src/lib.rs` and `rust_cli/src/codama_sdk/programs.rs`.
Deploy the metadata program using anchor:

```bash
anchor build
anchor deploy
```

Airdrop SOL to your keypair - make sure to point your solana cli to localnet:

```bash
solana config set --url localhost
solana airdrop 20 -k tests/wallet.json  
```

Deploy any program using the keypair in `tests/wallet.json`:

```bash
 solana program deploy <path/to/any-program>.so -k tests/wallet.json
```

Run the CLI upload for the program you just deployed:

```bash
cargo run -- idl upload tests/testidl.json <program-id> -k tests/wallet.json -p 0
```

