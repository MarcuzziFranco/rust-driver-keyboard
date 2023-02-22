use std::{
    io::{self},
    time::Duration,
};

use serialport::DataBits;

fn main() {
    println!("Initial driver");
    print_all_ports();
}

fn print_all_ports() {
    println!("List ports");
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }

    let com = String::from("COM6");
    connection_port(&com, 9600);
}

fn connection_port(com: &String, baut_rate: u32) {
    let connection = serialport::new(com, baut_rate)
        .data_bits(DataBits::Eight)
        .timeout(Duration::from_millis(10))
        .open();

    let mut serial_buf: Vec<u8> = vec![0; 4];
    let mut serial_data: Vec<u8> = vec![];
    match connection {
        Ok(mut connection) => {
            println!("Receiving data on {} at {}", com, baut_rate);
            loop {
                match connection.read_exact(&mut serial_buf) {
                    Ok(_t) => {
                        //println!("{:?}", serial_buf);
                        // serial_data.extend_from_slice(&serial_buf[..t]);
                        if serial_buf.len() > 0 {
                            // hacer algo con los datos leídos
                            // println!("Datos leídos: {:?}", serial_data);
                            let ascci_data = String::from_utf8(serial_buf.clone()).unwrap();
                            //println!("{}", ascci_data);
                            interpreter_command(ascci_data);
                            serial_data.clear();
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }

        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", com, e);
            ::std::process::exit(1);
        }
    }
}

fn interpreter_command(command_raw: String) {
    //println!("{}", command_raw);
    //let v: Vec<&str> = command_raw.split('@').collect();
    //println!("{:?}", v);

    if command_raw.contains("P-") {
        println!("Press key button");
    } else if command_raw.contains("H-") {
        println!("Hold key button");
    } else if command_raw.contains("R-") {
        println!("Relese key button");
    }
}
