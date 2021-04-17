use std::fs;
use ds18b20_parser::read_temp;

fn main() {
    let readings: Vec::<(f32, String)>;
    match fs::read_dir("/sys/bus/w1/devices") {
        Ok(d) => { 
            readings = d.map(|e| e.unwrap().file_name())
                .filter(|dev| dev != "w1_bus_master1")
                .map(|dev| (read_temp(&dev).unwrap(), dev.into_string().unwrap()))
                .collect::<Vec<_>>();

            readings.iter().for_each(|r| println!("{} from {}", r.0, r.1));
        },
        Err(e) => { eprintln!("Could not list /sys/bus/w1/devices: {}", e); }
    }
}
