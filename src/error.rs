//! A family of related errors when working with [`convo`][`crate`].

/// An [`ExportError`] is a category of errors returned by exporter functions that returns [`Result`]s.
#[derive(Debug)]
pub enum ExportError {
    /// An error caused when IO issues occur during exporting.
    IO(std::io::Error),
    /// An error caused when YAML is unable to be emitted.
    Emit(yaml_rust::EmitError),
    /// An error caused when a tree is not considered legal to export.
    /// See also: [validation rules](https://github.com/simbleau/convo/blob/dev/FORMATTING.md#validation-rules).
    Validation(TreeError),
}
impl From<std::io::Error> for ExportError {
    fn from(item: std::io::Error) -> Self {
        ExportError::IO(item)
    }
}
impl From<yaml_rust::EmitError> for ExportError {
    fn from(item: yaml_rust::EmitError) -> Self {
        ExportError::Emit(item)
    }
}
impl From<TreeError> for ExportError {
    fn from(item: TreeError) -> Self {
        ExportError::Validation(item)
    }
}

/// A [`ImportError`] is a category of errors returned by parser functions that returns [`Result`]s.
#[derive(Debug)]
pub enum ImportError {
    /// An error caused when IO issues occur during importing.
    IO(std::io::Error),
    /// An error caused when YAML is unable to be scanned in.
    Scan(yaml_rust::ScanError),
    /// An error caused when a tree is not considered legal when parsing.
    /// See also: [validation rules](https://github.com/simbleau/convo/blob/dev/FORMATTING.md#validation-rules).
    Validation(TreeError),
    /// An error caused when the target content contains multiple YAML documents.
    MultipleDocumentsProvided(),
}
impl From<std::io::Error> for ImportError {
    fn from(item: std::io::Error) -> Self {
        ImportError::IO(item)
    }
}
impl From<yaml_rust::ScanError> for ImportError {
    fn from(item: yaml_rust::ScanError) -> Self {
        ImportError::Scan(item)
    }
}
impl From<TreeError> for ImportError {
    fn from(item: TreeError) -> Self {
        ImportError::Validation(item)
    }
}

/// A [`TreeError`] is a category of validation errors returned when a tree is not considered legal.
/// See also: [validation rules](https://github.com/simbleau/convo/blob/dev/FORMATTING.md#validation-rules).
#[derive(Debug)]
pub enum TreeError {
    /// An error caused when a [`crate::Tree`] is missing a root [`crate::Node`].
    /// See also: [`crate::Tree#root`][`crate::Tree#structfield.root].
    RootNotSet(),
    /// An error caused when a [`crate::Tree`] is missing a current [`crate::Node`].
    /// See also: [`crate::Tree#current`][`crate::Tree#structfield.current].
    CurrentNotSet(),
    /// An error caused when a [`crate::Tree`] is missing a necessary [`crate::Node`].
    NodeDNE(String),
    /// An error caused when validating a family of rules a [`crate::Tree`] must obey.
    Validation(String),
}
