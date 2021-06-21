// Documentation requirements
#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![deny(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::private_doc_tests)]
#![deny(rustdoc::broken_intra_doc_links)]
// Doc attributes
#![doc(issue_tracker_base_url = "https://github.com/simbleau/convo/issues/")]

//! A modern dialogue executor and tree parser using YAML, focusing on ease-of-use and speed.

pub mod error;
pub mod exporter;
pub mod parser;

mod link;
mod node;
mod tree;

pub use link::Link;
pub use node::Node;
pub use tree::CTree;
