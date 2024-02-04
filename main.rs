// RUST CRATE FOR GETTING AN IP
use local_ip_address::local_ip;

fn main() {
    let roberta_ip = local_ip().unwrap();

    println!("This is Roberta's IP address: {:?}", roberta_ip);
}
