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
    // vec_octet will store all the octets
    let octet_ints: [i32;4] = [0,0,0,0];
    
    // parse octet into integers
    for (i,octet) in octets.iter().enumerate() {
        let octet_int : i32 = octet.parse().unwrap();// if parse fails find out with unwrap :)
        let octet_math : i32 = octet_int + 18;

        // append the octets to the vec_octet
        octet_ints[i] = octet_int;

        println!("value {octet_math}");
    }

    // vector to save subnet calculator table/dictionary
                                            // /25, /26 /27 /28 /29 /30 /31 /32
    let subnet_cheatsheet: [(i32,i32); 8] = [(128,128),(64,192),(32,224),(16,240),(8,248),(4,252) ,(2,254), (1,255)];
    // let vec1 = vec![vec![1, 1,2,2], vec![3, 3,4,4]]
    
    // convert cidr to index in the subnet_cheatsheet array
    // convert cidr from string to integer
    let index: u32 = cidr.parse().unwrap(); 
    let subnet_tuple = subnet_cheatsheet[index];
    let subnet_size = subnet_tuple[0];
    let subnet_mask = subnet_tuple[1];
    
    println!("Subnet size: ");
    println!("Subnet mask: 255.255.255.{subnet_mask}");
    
    // increment octet 1 by the size until we get a number equal to or larger than 
    // number in the fourth octet







    

    

        
}



