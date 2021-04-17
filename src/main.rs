extern crate getopts;

use std::{env, fs, thread, time};

use getopts::Options;
use postgres::{Client, NoTls};

use temperature_monitor::{read_temp, record_temp};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("d", "delay", "set delay between measurements", "DELAY");
    let matches = opts.parse(&args[1..]).unwrap();
    
    let mut delay: u64 = 1000;
    if let Some(val) = matches.opt_str("d") {
        delay = val.parse::<u64>().unwrap_or(1000);
    }


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
        thread::sleep(time::Duration::from_millis(delay));
    }
}
