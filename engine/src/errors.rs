use idcontain::Id;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("{0}")]
    CreateWindow(String),

    #[error("I/O error when accessing `{0}` for resource `{1}`.")]
    ResourceIo(&'static str, &'static str),

    #[error("Linking/compiling shader for `{needed_by}` failed with:\n{log}")]
    Shader { log: String, needed_by: String },

    #[error("Feature needed by `{needed_by}` is not supported on this platform.")]
    UnsupportedFeature { needed_by: String },

    #[error("Out of video memory when trying to allocate `{needed_by}`.")]
    OutOfVideoMemory { needed_by: String },

    #[error("No entity with id `{id:?}`, needed by `{needed_by:?}` when `{context}`")]
    NoSuchEntity {
        context: &'static str,
        needed_by: Option<&'static str>,
        id: Id<()>,
    },

    #[error("No component with id `{id:?}`, needed by `{needed_by:?}` when `{context}`")]
    NoSuchComponent {
        context: &'static str,
        needed_by: Option<&'static str>,
        id: Id<()>,
    },

    #[error("System {0} failed for `{1}`.")]
    System(&'static str, &'static str),
}
