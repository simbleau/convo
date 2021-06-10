// Enforce stricter documentation requirements
// TODO: Write documentation
/*
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::broken_intra_doc_links)]
*/

//! A modern dialogue executor and tree parser using YAML, focusing on ease-of-use
mod parser;

pub use convo_lib::tree::CTree;
pub use parser::parse;
