fn main() {
    use std::collections::HashMap;

    let mut hm=HashMap::new();
    
    hm.insert(String::from("Melisa"),21);
    hm.insert(String::from("Şevval"),16);
    let name=String::from("Melisa");
    let my_name=String::from("Yusuf");
    let values=hm.get(&name).copied().unwrap_or(0);
    let v=hm.get(&my_name).copied().unwrap_or(0);
    hm.insert(String::from("Melisa"),9);
    hm.insert(String::from("Yusuf"),21);
    
    println!("{:?}",hm);
    println!("{values}");
    println!("{v}");

    hm.entry(String::from("Seda")).or_insert(20);
    hm.entry(String::from("Melisa")).or_insert(21);

    for (key,value) in &hm{
        println!("{key} : {value}");
    }
}
