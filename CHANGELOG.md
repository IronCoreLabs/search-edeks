# Changelog

## 1.1.0

- Added `-v --verbose` flag that outputs full result tuples. Lack of the flag outputs only raw identifiers, one per line.
- Added a `broken-edeks.txt` output file for EDEKs that couldn't be parsed. Contains one line per broken EDEK of
  `("identifier", "edek_data", "error message")` if `--verbose` is enabled, only the raw identifiers otherwise.

## 1.0.0

- Initial release.
