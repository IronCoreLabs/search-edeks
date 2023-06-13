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
search-edeks 1.0.0
IronCore Labs <info@ironcorelabs.com>
Tool to find EDEKs that are encrypted with the given KMS config ID.

USAGE:
    search-edeks [OPTIONS] --id <VALUE> --file <FILE> <--hex|--base64>

OPTIONS:
    -b, --base64         Consume and output base64 formatted EDEKs
    -d, --debug          Print extra debug information
    -f, --file <FILE>    File with one `("identifier", "EDEK")` per line
    -h, --hex            Consume and output hex formatted EDEKs
        --help           Print help information
    -i, --id <VALUE>     Sets the KMS config ID we're searching for
    -V, --version        Print version information
```

For example `search-edeks -f edeks.txt -i 1201 -h` would search `edeks.txt` for any EDEKs that were created using KMS config ID `1201`.
