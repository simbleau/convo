//! A family of related errors when working with [`crate`].

/// An [`ExportError`] is a category of errors returned by exporter functions that returns [`Result`]s.
#[derive(Debug)]
pub enum ExportError {
    /// An error caused when IO issues occur during exporting.
    IO(std::io::Error),
    /// An error caused when YAML is unable to be emitted.
    Emit(yaml_rust::EmitError),
    /// An error caused when a tree is not considered legal to export.
    /// See also: [format information here](https://github.com/simbleau/convo/tree/main/examples/dialogue_files/README.md).
    Tree(TreeError),
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
        ExportError::Tree(item)
    }
}

/// A [`ParseError`] is a category of errors returned by parser functions that returns [`Result`]s.
#[derive(Debug)]
pub enum ParseError {
    /// An error caused when IO issues occur during parsing.
    IO(std::io::Error),
    /// An error caused when YAML is unable to be scanned in.
    Scan(yaml_rust::ScanError),
    /// An error caused when a tree breaks validation rules.
    /// See also: [format information here](https://github.com/simbleau/convo/tree/main/examples/dialogue_files/README.md).
    Tree(TreeError),
    /// An error caused when the target parsing content contains multiple YAML documents.
    MultipleDocumentsProvided(),
}
impl From<std::io::Error> for ParseError {
    fn from(item: std::io::Error) -> Self {
        ParseError::IO(item)
    }
}
impl From<yaml_rust::ScanError> for ParseError {
    fn from(item: yaml_rust::ScanError) -> Self {
        ParseError::Scan(item)
    }
}
impl From<TreeError> for ParseError {
    fn from(item: TreeError) -> Self {
        ParseError::Tree(item)
    }
}

/// A [`TreeError`] is a category of validation errors returned when a tree is not considered legal per validation rules.
/// See also: [format information here](https://github.com/simbleau/convo/tree/main/examples/dialogue_files/README.md).
#[derive(Debug)]
pub enum TreeError {
    /// An error caused when a [`crate::CTree`] is missing a root [`crate::Node`].
    /// See also: [`crate::CTree#root`][`crate::CTree#structfield.root].
    RootNotSet(),
    /// An error caused when a [`crate::CTree`] is missing a current [`crate::Node`].
    /// See also: [`crate::CTree#current`][`crate::CTree#structfield.current].
    CurrentNotSet(),
    /// An error caused when a [`crate::CTree`] is missing a necessary [`crate::Node`].
    NodeDNE(String),
    /// An error caused when validating a family of rules a [`crate::CTree`] must obey.
    Validation(String),
}
