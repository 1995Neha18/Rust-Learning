use std::str;
use std::process::Command;

fn main() {
    let output = Command::new("wmic")
        .arg("printer get name")
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        let output_str = str::from_utf8(&output.stdout).expect("Invalid UTF-8");
        let lines: Vec<&str> = output_str.lines().collect();

        for line in lines.iter().skip(1) {
            let printer_name = line.trim();
            if !printer_name.is_empty() {
                println!("Printer Name: {}", printer_name);
            }
        }
    } else {
        let error_str = str::from_utf8(&output.stderr).expect("Invalid UTF-8");
        eprintln!("Error: {}", error_str);
    }
}
