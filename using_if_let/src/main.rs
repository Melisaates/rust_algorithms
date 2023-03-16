fn main() {
    enum IpAdress{
        V5,
        V6,
    }/*
    
    let my_ip=IpAdress::V5;
    match my_ip{
        IpAdress::V6 => println!("V6"),
        _=>println!("Null!!!"),
    }*/
    let my_ip=IpAdress::V5;
    if let IpAdress::V6=my_ip{println!("v6");}
    else{
        println!("v5");
    }

    
}

