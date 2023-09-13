// extern crate winapi;
// use std::ptr;
// // use std::os::windows::raw::HANDLE;
// use winapi::um::winspool::{
//     OpenPrinterW,
//     StartDocPrinterW,
//     StartPagePrinter,
//     WritePrinter,
//     EndPagePrinter,
//     EndDocPrinter,
//     ClosePrinter,
// };

// pub fn print_document(document_content: &[u8], printer_name: &str) -> Result<(), String> {
//     unsafe {
//         // Open the printer
//         let mut h_printer: winapi::HANDLE = ptr::null_mut();
//         if OpenPrinterW(printer_name.to_wide().as_ptr(), &mut h_printer, ptr::null_mut()) == 0 {
//             return Err("Failed to open printer".to_string());
//         }

//         // Start a new print job
//         if StartDocPrinterW(h_printer, 1, ptr::null_mut()) == 0 {
//             ClosePrinter(h_printer);
//             return Err("Failed to start print job".to_string());
//         }

//         // Start a new page
//         if StartPagePrinter(h_printer) == 0 {
//             EndDocPrinter(h_printer);
//             ClosePrinter(h_printer);
//             return Err("Failed to start page".to_string());
//         }

//         // Write the document content to the printer
//         if
//             WritePrinter(
//                 h_printer,
//                 document_content.as_ptr() as *mut _,
//                 document_content.len() as u32,
//                 ptr::null_mut()
//             ) == 0
//         {
//             EndPagePrinter(h_printer);
//             EndDocPrinter(h_printer);
//             ClosePrinter(h_printer);
//             return Err("Failed to write to the printer".to_string());
//         }

//         // End the page and the print job
//         if EndPagePrinter(h_printer) == 0 || EndDocPrinter(h_printer) == 0 {
//             ClosePrinter(h_printer);
//             return Err("Failed to end page or print job".to_string());
//         }

//         // Close the printer
//         if ClosePrinter(h_printer) == 0 {
//             return Err("Failed to close printer".to_string());
//         }
//     }

//     Ok(())
// }

// // Helper trait to convert &str to a wide string
// trait ToWide {
//     fn to_wide(&self) -> Vec<u16>;
// }

// impl ToWide for str {
//     fn to_wide(&self) -> Vec<u16> {
//         use std::ffi::OsStr;
//         use std::os::windows::ffi::OsStrExt;

//         OsStr::new(self).encode_wide().chain(std::iter::once(0)).collect()
//     }
// }

// -----------------------------------------------------------------------

