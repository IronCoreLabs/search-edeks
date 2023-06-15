use std::{fs::File, io::Write, path::PathBuf};

use crate::proto::transform::{EncryptedDek as EncryptedDekP, EncryptedDeks as EncryptedDeksP};
use protobuf::Message;
use serde::Serialize;

pub fn edek_from_bytes(encrypted_document_key: &[u8]) -> Result<EncryptedDekP, String> {
    let mut edeks = EncryptedDeksP::parse_from_bytes(encrypted_document_key)
        .map_err(|_| "Provided EDEK didn't contain IronCore EDEKs".to_string())?;

    // should change if we ever expect more than one dek for real
    edeks
        .encryptedDeks
        .pop()
        .ok_or_else(|| "No encrypted DEKs were provided".to_string())
}

pub fn write_file<T>(
    path: &PathBuf,
    to_write: &[T],
    full_information_output: bool,
) -> Result<(), String>
where
    T: Serialize + GetIdentifier,
{
    let output_str = to_write
        .into_iter()
        .map(|line| {
            if !full_information_output {
                format!("{}", line.identifier())
            } else {
                ron::to_string(&line).unwrap()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");
    let display = path.display();
    let mut file = File::create(&path)
        .map_err(|e| format!("Couldn't create output file {}: {}", display, e))?;

    file.write_all(output_str.as_bytes())
        .map_err(|e| format!("Couldn't write output to {}: {}", display, e))
}

pub trait GetIdentifier {
    fn identifier(&self) -> String;
}

// identifier, base64 || hex
pub type EdekFileEntry = (String, String);
impl GetIdentifier for EdekFileEntry {
    fn identifier(&self) -> String {
        self.0.clone()
    }
}
impl GetIdentifier for (String, String, String) {
    fn identifier(&self) -> String {
        self.0.clone()
    }
}
