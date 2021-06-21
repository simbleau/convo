//! A family of related errors when working with [`crate`].

use yaml_rust::EmitError;

/// An [`ExportError`] is a category of errors returned by exporter functions that returns [`Result`]s.
#[derive(Debug)]
pub enum ExportError {
    /// An error caused when IO issues occur during exporting.
    IO(std::io::Error),
    /// An error caused when YAML is unable to be emitted.
    Emit(EmitError),
    /// An error caused when a tree is not considered legal to export.
    /// See also: [format information here](https://github.com/simbleau/convo/tree/main/examples/dialogue_files/README.md).
    Tree(TreeError),
}
impl From<std::io::Error> for ExportError {
    fn from(item: std::io::Error) -> Self {
        ExportError::IO(item)
    }
}

/// A [`TreeError`] is a category of errors returned by [`crate::CTree`] methods which returns [`Result`]s.
#[derive(Debug)]
pub enum TreeError {
    /// An error caused when a [`crate::CTree`] is missing a root [`crate::Node`].
    /// See also: [`crate::CTree#root`][`crate::CTree#structfield.root].
    RootNotSet(),
    /// An error caused when a [`crate::CTree`] is missing a current [`crate::Node`].
    /// See also: [`crate::CTree#current`][`crate::CTree#structfield.current].
    CurrentNotSet(),
    /// An error caused when a [`crate::CTree`] can not find a [`crate::Node`].
    NodeDNE(String),
    /// An error caused when validating a family of rules a [`crate::CTree`] must obey.
    Validation(String),
}

/// A [`ParseError`] is a category of errors returned by parser functions that returns [`Result`]s.
#[derive(Debug)]
pub enum ParseError {
    /// An error caused when IO issues occur during parsing.
    IO(std::io::Error),
    /// An error caused when YAML is unable to be scanned in.
    Scan(yaml_rust::ScanError),
    /// An error caused when a tree is not considered legal.
    /// See also: [format information here](https://github.com/simbleau/convo/tree/main/examples/dialogue_files/README.md).
    Tree(TreeError),
    /// An error caused when validating a file for parsing.
    Validation(String),
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
impl From<String> for ParseError {
    fn from(item: String) -> Self {
        ParseError::Validation(item)
    }
}
impl From<&str> for ParseError {
    fn from(item: &str) -> Self {
        ParseError::Validation(item.to_owned())
    }
}
