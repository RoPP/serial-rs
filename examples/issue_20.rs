// imports
extern crate serial; //see: https://dcuddeback.github.io/serial-rs/serial/index.html
use std::env;
use std::io;
use std::time::Duration;
use std::process::exit;
use std::io::prelude::*;
use serial::prelude::*;
use serial::{BaudRate, CharSize, Parity, StopBits, FlowControl, SystemPort, PortSettings, Error};
use std::mem;
use std::fmt::Debug;
use std::fmt::Display;


fn main() {
    for arg in env::args_os().skip(1) {
        // open port
        let mut port = match serial::open(&arg) {
            Err(ref e) => {
                println!("Error opening serial port.");
                println!("Error:  {:?}", e);
                exit(0);
            }
            Ok(x) => x,
        };
        // push settings to stack
        let settings = PortSettings {
            baud_rate: BaudRate::Baud115200,
            char_size: CharSize::Bits8,
            parity: Parity::ParityNone,
            stop_bits: StopBits::Stop1,
            flow_control: FlowControl::FlowNone,
        };
        // configure port
        match port.configure(&settings) {
            Ok(_) => {}
            Err(ref e) => {
                println!("Error configuring serial port.");
                println!("Error:  {:?}", e);
                exit(0);
            }
        };
        // set rts
        match port.set_rts(true) {
            Ok(_) => {}
            Err(ref e) => {
                println!("Error setting RTS line");
                println!("Error:  {:?}", e);
                exit(0);
            }
        };
        // set DTR
        match port.set_dtr(true) {
            Ok(_) => {}
            Err(ref e) => {
                println!("Error setting DTR line");
                println!("Error:  {:?}", e);
                let os_e = ::std::io::Error::last_os_error();
                println!("OS Error: {:?}", os_e);
                exit(0);
            }
        };
        // allocate readbuffer on stack
        let mut rb: [u8; 1] = [0u8; 1];
        // allocate buffer to hold output
        let mut out: String = String::with_capacity(1024);
        // loop while reading
        loop {
            match port.read(&mut rb) {
                Ok(_) => {}
                Err(ref e) => {
                    println!("Error reading serial port.");
                    println!("Error:  {:?}", e);
                    exit(0);
                }
            };
            match rb[0] {
                // Linefeed
                10 => {
                    println!("{}<LF>", &out);
                    out = String::with_capacity(1024);
                }
                // carriage return
                13 => {
                    println!("{}<CR>", &out);
                    out = String::with_capacity(1024);
                }
                // normal chars
                32...126 => {
                    let temp: u32 = rb[0].clone() as u32;
                    let ch: char = unsafe { mem::transmute(temp) };
                    out.push(ch);
                }
                // everything else
                x => {
                    println!("Non standard character encountered");
                    println!("Value:  {:?}", x);
                    exit(0);
                }
            };
        }
    }
}
