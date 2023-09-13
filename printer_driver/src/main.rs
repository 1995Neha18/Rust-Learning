// use pdf_canvas::Pdf;
// use std::fs::File;

// mod printer;
// use printer::print_document;

// fn main() {
//  // Create a PDF document
//  let mut pdf = Pdf::create("output.pdf").unwrap();

//  // Add a page
//  let (page, _) = pdf.add_page(300.0, 300.0, "");

//  // Draw some text on the page
//  let font = pdf.load_font("Helvetica-Bold").unwrap();
//  let mut page = pdf.get_page(page);
//  page.set_font(&font, 24.0);
//  page.use_text("Hello, PDF!", 50.0, 200.0);

//  // Save the PDF to a file
//  pdf.save().unwrap();

//  println!("PDF created and saved as 'output.pdf'.");
// }

// -------------------------------------------------------------------

extern crate printpdf;
extern crate winit;

use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use winit::{
    event::{ Event, WindowEvent },
    event_loop::{ ControlFlow, EventLoop },
    window::WindowBuilder,
};
use std::process::Command;

fn main() {
    // Create an event loop for the window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // Initialize a PDF document
    // Initialize a PDF document
let (doc, page1, layer1) = PdfDocument::new("PDF Document", 210.0, 297.0, "Layer 1");


    // Add a page to the document
    let page = doc.get_page(page1);

    // Create a font and text context
    let font = doc.add_builtin_font(BuiltinFont::TimesRoman).unwrap();
    let mut page = doc.get_page(page1);
    let layer = doc.get_page(layer1);

    // Draw text on the page
    let text = "Hello, PDF!";
    let font_size = 48.0;
    page.use_text(text, 40.0, 200.0, &font, font_size);

    // Save the PDF to a file
    let pdf_filename = "output.pdf";
    let file = File::create(pdf_filename).expect("Failed to create PDF file");
    let buffer = BufWriter::new(file);

    doc.save_to(buffer).expect("Failed to save PDF");

    println!("PDF created and saved as 'output.pdf'.");

    // Print the PDF when the window is closed
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                print_pdf(pdf_filename);
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        }
    });
}

fn print_pdf(pdf_filename: &str) {
    // Use the `lpr` command to print the PDF on Windows.
    // Make sure `lpr` is installed and available in the system's PATH.
    let result = Command::new("lpr").arg(pdf_filename).status();

    match result {
        Ok(status) if status.success() => {
            println!("PDF successfully sent to the printer.");
        }
        Ok(status) => {
            println!("Error: Printing failed with status code {:?}", status.code());
        }
        Err(err) => {
            println!("Error: Failed to execute the 'lpr' command: {:?}", err);
        }
    }
}
