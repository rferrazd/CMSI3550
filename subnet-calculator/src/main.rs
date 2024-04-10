use std::env::args;

fn main() {
    
    // read parameters
    let subnet_args: Vec<String> = args().collect();
    if subnet_args.len() >1 {
        let target_subnet = subnet_args[1].to_string();
        calculate_subnet(target_subnet);
    }
       
}
// cargo run -- 10.20.0.51/25 (1st)(2nd)(3rd)(4th)(cider)
fn calculate_subnet(target_subnet: String) {
    let [target_ip, cidr]: [&str; 2]  = target_subnet 
        .split("/")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap_or_default(); // try to unwrap capital s string into a str pointer
    
    println!("Target IP: {target_ip}");
    println!("CIDR: {cidr}");

    // parse target ip into octets
    let octets: [&str; 4] = target_ip
        .split(".")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap_or_default();
    
    for (i,octet) in octets.iter().enumerate() {
        let index = i + 1 ; 
        println!("octet {index}: {octet}");
    }
    // parse octet into integers
    for octet in octets {
        let octet_int : i32 = octet.parse().unwrap();// if parse fails find out with unwrap :)
        let octet_math : i32 = octet_int + 18;
        println!("value {octet_math}");
    }


    

    

        
}



