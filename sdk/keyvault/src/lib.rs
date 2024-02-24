use azure_core::{hello, version};

pub fn say_hello() {
    println!("{} from azure_core {}", hello(), version());
}
