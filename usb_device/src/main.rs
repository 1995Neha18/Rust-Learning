// fn main() {
//  for device in rusb::devices().unwrap().iter() {
//      let device_desc = device.device_descriptor().unwrap();

//      println!(
//          "Bus {:03} Device {:03} ID {:04x}:{:04x}",
//          device.bus_number(),
//          device.address(),
//          device_desc.vendor_id(),
//          device_desc.product_id()
//      );
//  }
// }

// -------------------------------------------------

// fn main() {
//     let devices = usb_enumeration::enumerate(None, None);

//     println!("{:#?}", devices);
// }

// fn main(){
//  use usb_enumeration::{Observer, Event};

// let sub = Observer::new()
//     .with_poll_interval(2)
//     .with_vendor_id(6127)
//     .with_product_id(24785)
//     .subscribe();

// // when sub is dropped, the background thread will close

// for event in sub.rx_event.iter() {
//     match event {
//         Event::Initial(d) => println!("Initial devices: {:?}", d),
//         Event::Connect(d) => println!("Connected device: {:?}", d),
//         Event::Disconnect(d) => println!("Disconnected device: {:?}", d),
//     }
// }
// }

// ------------------- Currently connected and history of connected devices -------------------

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
}
