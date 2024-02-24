use azure_security_keyvault as keyvault;
use azure_security_keyvault_v1 as keyvault_v1;

fn main() {
    println!(
        "In azure_core {} all we could say was:",
        azure_core_v1::version()
    );
    keyvault_v1::say_hello();

    println!();
    println!(
        "Now that we're more evolved in azure_core {}, we can say:",
        azure_core::version()
    );
    keyvault::say_hello();
}
