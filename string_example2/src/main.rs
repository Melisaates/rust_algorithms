fn main() {

    let mut s=String::new();
    s.push_str("hello");
    s.push('!');
    println!("{s}");
    let m=String::from("world");
    let k=format!("{s}--{m}");
    let n=s+&m;
    
    println!("{}",k);

    println!("{n}");
    println!("{m}");

    for i in n.bytes(){
        println!("{i}");
    }

    println!("***************");

    for j in n.chars(){
        println!("{j}");
    }

}
