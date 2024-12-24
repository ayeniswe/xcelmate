//! The module holds all logic to fully deserialize a .xlsx file and its contents
mod shared_string_table;
mod stylesheet;

use std::io::{Read, Seek};
use shared_string_table::SharedStringTable;
use stylesheet::Stylesheet;
use zip::ZipArchive;
use crate::errors::XcelmateError;

/// The `Xlsx` struct represents an Excel workbook stored in an OpenXML format (XLSX).
/// It encapsulates foundational pieces of a workbook
struct Xlsx<RS> {
    /// The zip archive containing all files of the XLSX workbook.
    zip: ZipArchive<RS>,
    /// The shared string table for efficient mapping of shared strings.
    shared_string_table: SharedStringTable,
    /// The stylesheet for formating cells.
    style: Stylesheet,
}

impl<RS: Read + Seek> Xlsx<RS> {
    fn read_shared_strings(&mut self) -> Result<(), XcelmateError> {
        self.shared_string_table.read_shared_strings(&mut self.zip)
    }
    fn read_stylesheet(&mut self) -> Result<(), XcelmateError> {
        self.style.read_stylesheet(&mut self.zip)
    }
}