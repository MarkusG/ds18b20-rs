use std::fs;
use std::path::PathBuf;
use std::ffi::OsStr;

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

// the sensor only gives us 5 digits of precision, so an f32 should be fine
fn read_temp(device_name: &OsStr) -> Option<f32> {
    let mut path = PathBuf::from("/sys/bus/w1/devices");
    path.push(device_name);
    path.push("w1_slave");
    match fs::read_to_string(path) {
        Err(e) => {
            eprintln!("Error reading device {}: {}",
                      device_name.to_str().unwrap(),
                      e);
            return None;
        },
        Ok(raw) => { 
            // example reading:
            // a6 01 4b 46 7f ff 0c 10 5c : crc=5c YES
            // a6 01 4b 46 7f ff 0c 10 5c t=26375
            // divide by 1000 to get the real temperature
            return Some(raw.split('=').nth(2).unwrap().trim().parse::<f32>().unwrap() / 1000.0);
        }
    }
}
