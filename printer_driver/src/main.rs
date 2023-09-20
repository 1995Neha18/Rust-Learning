extern crate winapi;

use std::os::windows::ffi::OsStrExt;
use winapi::um::winspool::{
    OpenPrinterW, StartDocPrinterW, StartPagePrinter, WritePrinter, EndPagePrinter, EndDocPrinter, ClosePrinter,
};
use std::ptr;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::raw::HANDLE;

// Define the print_document function here...


fn main() {
    // Specify the printer name you want to use
    let printer_name = "Everycom-58-series";

    // Prepare the document content (example: "Hello, world!")
    let document_content = b"Hello, world!";

    // Call the print_document function
    match print_document(document_content, printer_name) {
        Ok(_) => println!("Printing completed successfully."),
        Err(error) => eprintln!("Printing failed: {}", error),
    }
}
