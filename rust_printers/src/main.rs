use printers;

fn main() {


    // Vector of system printers
    let printers = printers::get_printers();

    // Print directly in all printers
    for printer in printers.clone() {

        // let job1 = printer.print("42".as_bytes());
        // let job2 = printer.print_file("/path/to/any.file");

        println!("{:?}", printer);
        // println!("{:?}", job1);
        // println!("{:?}", job2);
    }

    // Print with aux lib function (legacy)
    // printers::print(&printers[0], "42".as_bytes());
    // printers::print_file(&printers[1], "/path/to/any.file");

    // Try get printer by uuid
    let test_printer = printers::get_printer_by_id("4be0643f-1d98-573b-97cd-ca98a65347dd");
    println!("{:?}", test_printer);

    // // Try printer by name
    // let test_printer = printers::get_printer_by_name("test");
    // println!("{:?}", test_printer);

}