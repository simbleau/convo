// Enforce stricter documentation requirements
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::broken_intra_doc_links)]

//! A modern dialogue executor and tree parser using Extended Backus-Naur Form.
extern crate nom;

#[cfg(test)]
mod tests {
    #[test]
    fn first_test() {
        assert!(true);
    }
}
