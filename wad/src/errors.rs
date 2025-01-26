use anyhow::anyhow;
use std::fmt;
use std::result::Result as StdResult;
use thiserror::Error;

pub type Result<T> = StdResult<T, ErrorKind>;

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("Corrupt metadata file")]
    CorruptMetadata(#[source] anyhow::Error),

    #[error("Corrupt WAD file")]
    CorruptWad(#[source] anyhow::Error),

    #[error("I/O WAD error")]
    Io(#[source] anyhow::Error),
}

impl ErrorKind {
    pub(crate) fn invalid_byte_in_wad_name(byte: u8, bytes: &[u8]) -> Self {
        ErrorKind::CorruptWad(anyhow!(
            "Invalid character `{}` in wad name `{}`.",
            char::from(byte),
            String::from_utf8_lossy(bytes),
        ))
    }

    pub(crate) fn unfinished_image_column(
        i_column: usize,
        i_run: Option<usize>,
        width: usize,
        height: usize,
    ) -> Self {
        ErrorKind::CorruptWad(anyhow!(
            "Unfinished column {i_column} in run {i_run:?}, in image of size {width}x{height}",
        ))
    }

    pub(crate) fn image_too_large(width: usize, height: usize) -> Self {
        ErrorKind::CorruptWad(anyhow!("Image too large {width}x{height}."))
    }

    pub(crate) fn wad_name_too_long(bytes: &[u8]) -> Self {
        ErrorKind::CorruptWad(anyhow!(
            "Wad name too long `{}`.",
            String::from_utf8_lossy(bytes)
        ))
    }

    pub(crate) fn bad_wad_header_identifier(identifier: &[u8]) -> Self {
        ErrorKind::CorruptWad(anyhow!(
            "Invalid header identifier: {}",
            String::from_utf8_lossy(identifier)
        ))
    }

    pub(crate) fn seeking_to_info_table_offset(offset: i32) -> Self {
        ErrorKind::Io(anyhow!("Seeking to `info_table_offset` at {offset} failed"))
    }

    pub(crate) fn seeking_to_lump(index: usize, name: &str) -> Self {
        ErrorKind::Io(anyhow!("Seeking to lump {index}, `{name}` failed"))
    }

    pub(crate) fn reading_lump(index: usize, name: &str) -> Self {
        ErrorKind::Io(anyhow!("Reading lump {index}, `{name}` failed"))
    }

    pub(crate) fn bad_lump_size(
        index: usize,
        name: &str,
        total_size: usize,
        element_size: usize,
    ) -> Self {
        ErrorKind::CorruptWad(anyhow!(
            "Invalid lump size in `{name}` (index={index}): total={total_size}, element={element_size}, div={}, mod={}",
            total_size / element_size,
            total_size % element_size
        ))
    }

    pub(crate) fn missing_required_lump<NameT: fmt::Debug>(name: &NameT) -> Self {
        ErrorKind::CorruptWad(anyhow!("Missing required lump {name:?}"))
    }
}
