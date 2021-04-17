use std::fs;

use postgres::{Client, NoTls};

use temperature_monitor::{read_temp, record_temp};

fn main() {
    let mut client = Client::connect("host=/var/run/postgresql/ user=pi dbname=temperature", NoTls).unwrap();

    loop {
        match fs::read_dir("/sys/bus/w1/devices") {
            Ok(d) => { 
                d.map(|e| e.unwrap().file_name())
                    .filter(|dev| dev != "w1_bus_master1")
                    .map(|dev| (read_temp(&dev).unwrap(), dev.into_string().unwrap()))
                    .for_each(|r| record_temp(&mut client, &r.1, r.0));
            },
            Err(e) => { eprintln!("Could not list /sys/bus/w1/devices: {}", e); }
        }
    }
}
