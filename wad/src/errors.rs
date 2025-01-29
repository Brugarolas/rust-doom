use anyhow::anyhow;
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("Corrupt WAD file")]
    CorruptWad(#[source] anyhow::Error),

    #[error("I/O WAD error")]
    Io(#[source] anyhow::Error),
}

impl ErrorKind {
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

    pub(crate) fn seeking_to_info_table_offset(offset: i32) -> Self {
        ErrorKind::Io(anyhow!("Seeking to `info_table_offset` at {offset} failed"))
    }

    pub(crate) fn seeking_to_lump(index: usize, name: &str) -> Self {
        ErrorKind::Io(anyhow!("Seeking to lump {index}, `{name}` failed"))
    }

    pub(crate) fn reading_lump(index: usize, name: &str) -> Self {
        ErrorKind::Io(anyhow!("Reading lump {index}, `{name}` failed"))
    }

    pub(crate) fn missing_required_lump<NameT: fmt::Debug>(name: &NameT) -> Self {
        ErrorKind::CorruptWad(anyhow!("Missing required lump {name:?}"))
    }
}
