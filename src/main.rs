mod browse;
mod error;

use browse::browse;
use std::env::current_dir;

use error::UnwrapOrError;

fn main() {
    let init_location = current_dir().unwrap_or_error(|e| e.to_string());
    let location_name = init_location
        .to_str()
        .unwrap_or_error(|_| String::from("Unable to convert the path of current directory into string"));
    println!("{:?}", browse(location_name).unwrap_or_error(|e| format!("Error: {}", e.to_string())));
}
