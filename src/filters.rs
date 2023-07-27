use crate::{proto::transform::EncryptedDek, util::edek_from_bytes};

#[derive(Clone, Debug)]
pub(crate) enum Filter {
    ConfigId(i32),
    Mismatched,
}

fn execute_config_id_filter(
    parsed_edek: &EncryptedDek,
    config_id_to_match: i32,
) -> Result<bool, String> {
    Ok(parsed_edek.kmsConfigId == config_id_to_match)
}
fn execute_mismatched_filter(parsed_edek: &EncryptedDek) -> Result<bool, String> {
    if !parsed_edek.encryptedLeasedKeyData.is_empty() {
        match edek_from_bytes(&parsed_edek.encryptedLeasedKeyData) {
            Ok(lk_edek) => Ok(lk_edek.kmsConfigId != parsed_edek.kmsConfigId),
            Err(e) => Err(format!("Failed to parse leased key: {e}")),
        }
    } else {
        Ok(false)
    }
}
pub(crate) fn execute_filter(filter: &Filter, parsed_edek: &EncryptedDek) -> Result<bool, String> {
    match filter {
        Filter::ConfigId(config_id) => execute_config_id_filter(parsed_edek, *config_id),
        Filter::Mismatched => execute_mismatched_filter(parsed_edek),
    }
}
