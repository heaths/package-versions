use std::env;

pub fn say_hello() {
    let username = env::var("USER").or_else(|_| env::var("USERNAME")).ok();
    println!(
        "{} from azure_core {}",
        azure_core::hello(username.as_deref()),
        azure_core::version()
    );
}
