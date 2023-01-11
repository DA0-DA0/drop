# DAO drop tool

This tool parses a Cosmos SDK chain export JSON file, to produce a CSV list of addresses and amounts. 

It can handle extremely large files and supports:
- Deduplication of addresses
- Whale cap
- Minimum staked amount

Using the CSV file produced, you can head to [Juno Tools](https://juno.tools) to create a merkle based drop!

## Getting the export file
Ask your favorite validator!

## Installation
[Install rust](https://rustup.rs/). Then:

``` sh
cargo run -- --file-path <path-to-export-json>
```

To see all supported commands and arguments:

``` sh
cargo run -- --help
```

## Future work
Would be great to support other types of drops:
- Specify eligible / ineligible validators
- NFT holders
- DAO members
- ???
