mod filters;
mod util;
mod proto {
    include!(concat!(env!("OUT_DIR"), "/proto/mod.rs"));
}

use base64::{engine::general_purpose::STANDARD, Engine};
use clap::{arg, command, value_parser, ArgAction, ArgGroup};
use filters::{execute_filter, Filter};
use std::{
    convert::identity,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};
use util::{edek_from_bytes, write_file, EdekFileEntry};

fn main() {
    let matches = command!()
        .arg(
            arg!(
                -i --id <VALUE> "Sets the KMS config ID we're searching for"
            )
            .value_parser(value_parser!(i32)),
        )
        .arg(arg!(
            -m --mismatched "Searches for mismatches between the KMS config ID in the EDEK header and the leased key used to encrypt the EDEK. Resulting EDEKs must be rekeyed with TSP 4.11.1+ to repair."
        ).action(ArgAction::SetTrue))
        .group(
            ArgGroup::new("search")
                .required(true)
                .args(&["id", "mismatched"])
                .multiple(true),
        )
        .arg(
            arg!(
                -f --file <FILE> r#"File with one `("identifier", "EDEK")` per line"#
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(-h --hex "Consume and output hex formatted EDEKs").action(ArgAction::SetTrue))
        .arg(
            arg!(-b --base64 "Consume and output base64 formatted EDEKs")
                .action(ArgAction::SetTrue),
        )
        .group(
            ArgGroup::new("format")
                .required(true)
                .args(&["hex", "base64"]),
        )
        .arg(arg!(-d --debug "Print extra debug information").action(ArgAction::SetTrue))
        .arg(arg!(-v --verbose "Output identifier and original EDEK (and error message if applicable). If not enabled, only identifiers will be output").action(ArgAction::SetTrue))
        .get_matches();

    // filter args
    let config_id_option = matches.get_one::<i32>("id").map(|c| Filter::ConfigId(*c));
    let mismatched = if matches.get_flag("mismatched") {
        Some(Filter::Mismatched)
    } else {
        None
    };

    // io args
    let edek_file_path = matches
        .get_one::<PathBuf>("file")
        .expect("file is required");
    let use_hex = matches.get_flag("hex");
    let use_base64 = matches.get_flag("base64");
    let debug: bool = matches.get_flag("debug");
    let verbose: bool = matches.get_flag("verbose");

    // open the edek tuples file handle, later to be read as a buffered streaming iterator
    let edek_file = File::open(edek_file_path)
        .map_err(|e| format!("Failed to open EDEK file: {e}"))
        .unwrap();
    // zip an index in so we can give line numbers
    let edek_lines = BufReader::new(edek_file).lines().zip(1..);
    // if we ever hit memory issues while writing the whole vec out of memory at once, we could write directly to shared write handlers
    let mut found_lines: Vec<EdekFileEntry> = vec![];
    let mut found_broken: Vec<(String, String, String)> = vec![];
    // a list of filters to attempt to match
    let active_filters: Vec<Filter> = vec![config_id_option, mismatched]
        .into_iter()
        .filter_map(identity)
        .collect();
    for line in edek_lines {
        if let (Ok(edek_entry), line_number) = line {
            if debug {
                println!("edek string: {edek_entry}");
            };
            let (identifier, edek) = ron::from_str::<EdekFileEntry>(&edek_entry)
                .map_err(|e| format!("Unexpected error processing line {line_number}: {e}"))
                .unwrap();
            // decode the edek string in the desired format
            // if we fail, log it, but keep going in case there are lines that match
            let decode_attempt = if use_base64 {
                STANDARD
                    .decode(edek.clone())
                    .map_err(|e| format!("EDEK was not base64: {e}"))
            } else if use_hex {
                let stripped = if edek.starts_with("0x") || edek.starts_with("0X") {
                    edek.chars().skip(2).collect()
                } else {
                    edek.clone()
                };
                hex::decode(stripped).map_err(|e| format!("EDEK was not hex: {e}"))
            } else {
                // this should've already been handled by clap, but writing again so Rust is happy
                panic!("Base64 or Hex format must be specified.");
            };
            match decode_attempt {
                Ok(decoded_edek) => {
                    // parse the proto and compare the kms config id with the one we're seeking
                    // if we fail, log it, but keep going in case there are lines that match
                    let parse_attempt = edek_from_bytes(&decoded_edek);
                    match parse_attempt {
                        Ok(parsed_edek) => {
                            if debug {
                                println!("parsed proto: {parsed_edek}, line {line_number}");
                            }
                            let (matched_filters, failures): (Vec<_>, Vec<_>) = active_filters
                                .iter()
                                .map(|filter| execute_filter(filter, &parsed_edek))
                                .partition(Result::is_ok);

                            if !failures.is_empty() {
                                let message = failures.into_iter().map(Result::unwrap_err).fold(
                                    format!("Failure on line {line_number}: "),
                                    |acc, failure| format!("{} {}", acc, failure),
                                );
                                if debug {
                                    println!("WARNING: {message}");
                                }
                                // there was at least one active filter that failed in some way, add this line to the broken ones
                                found_broken.push((identifier, edek, message))
                            } else {
                                if matched_filters
                                    .into_iter()
                                    .map(Result::unwrap)
                                    .all(identity)
                                {
                                    // the current line passed all filters, add it to found
                                    found_lines.push((identifier, edek));
                                }
                            }
                        }
                        Err(e) => {
                            if debug {
                                println!(
                                    "WARNING: Encountered an unparsable EDEK on line {line_number}: {e}"
                                );
                            }
                            found_broken.push((
                                identifier,
                                edek,
                                format!("Encountered an unparsable EDEK: {e}"),
                            ));
                        }
                    }
                }
                Err(e) => {
                    println!(
                        "Encountered an incorrectly formatted EDEK at line {line_number}: {e}"
                    );
                    found_broken.push((
                        identifier,
                        edek,
                        format!("Encountered an incorrectly formated EDEK: {e}"),
                    ));
                }
            }
        }
    }

    // write the edeks if we've found them
    if !found_lines.is_empty() {
        let path = Path::new("matching-edeks.txt").to_path_buf();
        write_file(&path, &found_lines, verbose).unwrap();
        println!(
            "Wrote {} matching EDEKs to {}",
            found_lines.len(),
            path.display()
        );
    } else {
        println!("Found no EDEKs matching the search parameters.");
    }
    if !found_broken.is_empty() {
        let path = Path::new("broken-edeks.txt").to_path_buf();
        write_file(&path, &found_broken, verbose).unwrap();
        println!(
            "Wrote {} broken EDEKs to {}.",
            found_broken.len(),
            path.display()
        );
    }
}
