use crate::proto::transform::{EncryptedDek as EncryptedDekP, EncryptedDeks as EncryptedDeksP};
use protobuf::Message;

pub fn edek_from_bytes(encrypted_document_key: &[u8]) -> Result<EncryptedDekP, String> {
    let mut edeks = EncryptedDeksP::parse_from_bytes(encrypted_document_key)
        .map_err(|_| "Provided EDEK didn't contain IronCore EDEKs".to_string())?;

    // should change if we ever expect more than one dek for real
    edeks
        .encryptedDeks
        .pop()
        .ok_or_else(|| "No encrypted DEKs were provided".to_string())
}
