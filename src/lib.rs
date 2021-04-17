use std::fs;
use std::path::PathBuf;
use std::ffi::OsStr;

// the sensor only gives us 5 digits of precision, so an f32 should be fine
pub fn read_temp(device_name: &OsStr) -> Option<f32> {
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
