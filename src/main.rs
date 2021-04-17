use std::fs;
use temperature_monitor::read_temp;

fn main() {
    loop {
        match fs::read_dir("/sys/bus/w1/devices") {
            Ok(d) => { 
                d.map(|e| e.unwrap().file_name())
                    .filter(|dev| dev != "w1_bus_master1")
                    .map(|dev| (read_temp(&dev).unwrap(), dev.into_string().unwrap()))
                    .for_each(|r| println!("{} from {}", r.0, r.1));
            },
            Err(e) => { eprintln!("Could not list /sys/bus/w1/devices: {}", e); }
        }
    }
}
