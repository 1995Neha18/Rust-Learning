// -------------------------- Method 1 -------------------------

// use std::process::Command;
// use std::str;

// fn main() {
//     let output = Command::new(r"C:\Windows\System32\wbem\WMIC.exe")
//         .arg("printer")
//         .output()
//         .expect("Failed to execute process");

//     if output.status.success() {
//         let output_str = str::from_utf8(&output.stdout).expect("Invalid UTF-8");

//         let lines: Vec<&str> = output_str.lines().collect();

//         if let Some(header) = lines.first() {
//             println!("{}", header);
//         }

//         for line in lines.iter().skip(1) {
//             let printer_name = line.trim();
//             if !printer_name.is_empty() {
//                 println!("{}", printer_name);
//             }
//         }
//     } else {
//         let error_str = str::from_utf8(&output.stderr).expect("Invalid UTF-8");
//         eprintln!("Error executing WMIC: {}", error_str);
//     }
// }
// -------------------- x ------------------ x ------------------

// -------------------------- Method 2 --------------------------

use std::process::Command;

fn main() {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(
            "Get-WmiObject -Query 'SELECT * FROM Win32_Printer' | Select-Object -ExpandProperty Name"
        )
        .output()
        .expect("Failed to execute process");

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("{}", output_str);
    } else {
        let error_str = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error executing PowerShell: {}", error_str);
    }
}
