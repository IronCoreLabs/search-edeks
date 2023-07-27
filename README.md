# Search EDEKs

This is a small binary command line tool that can be used to search a file for IronCore EDEKs that match the provided KMS config ID.

## Installation

### Precompiled Binary

Download the appropriate binary for your system from the releases page.

### Compile Source

Check out this repo and run `cargo b --release`. The binary will be at `target/release/search-edeks`.

## Usage

```console
search-edeks --help
Tool to search IronCoreLabs Tenant Security Proxy EDEK's protobuf.

Usage: search-edeks [OPTIONS] --file <FILE> <--id <VALUE>|--mismatched> <--hex|--base64>

Options:
  -i, --id <VALUE>   Sets the KMS config ID we're searching for
  -m, --mismatched   Searches for mismatches between the KMS config ID in the EDEK header and the leased key used to encrypt the EDEK. Resulting EDEKs must be rekeyed with TSP 4.11.1+ to repair.
  -f, --file <FILE>  File with one `("identifier", "EDEK")` per line
  -h, --hex          Consume and output hex formatted EDEKs
  -b, --base64       Consume and output base64 formatted EDEKs
  -d, --debug        Print extra debug information
  -v, --verbose      Output identifier and original EDEK (and error message if applicable). If not enabled, only identifiers will be output
  -h, --help         Print help
  -V, --version      Print version
```

For example `search-edeks --file edeks.txt --id 1201 --hex` would search `edeks.txt` for any EDEKs that were created using KMS config ID `1201`. It would output `matching-edeks.txt` with the one identifier per line for each EDEK that matched. It would output `broken-edeks.txt` with one identifier per line for each EDEK that wasn't parsable as an EDEK. If `--verbose` was enabled, the output would be tuples of the required input form (with the broken EDEKs additonally containing an error message).

If multiple search filters are included, all must be present for an EDEK to match.

## Releasing

* update the version in Cargo.toml according to semver before tagging for release
* push a tag matching the version in Cargo.toml.
  * a release build will be run against it which will upload artifacts to a github release
  * the version in Cargo.toml will be uploaded to crates.io
