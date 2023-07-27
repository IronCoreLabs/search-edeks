# Changelog

## 1.2.0

- Added a `-m --mismatched` flag that filters the output to only EDEKs that need a rekey because the KMS config ID in their header and the leased key used to encrypt them is mismatched. Must be rekeyed with TSP 4.11.1+ to fix.

## 1.1.0

- Added a `-v --verbose` flag that outputs full result tuples. Lack of the flag outputs only raw identifiers, one per line.
- Added a `broken-edeks.txt` output file for EDEKs that couldn't be parsed. Contains one line per broken EDEK of
  `("identifier", "edek_data", "error message")` if `--verbose` is enabled, only the raw identifiers otherwise.

## 1.0.0

- Initial release.
