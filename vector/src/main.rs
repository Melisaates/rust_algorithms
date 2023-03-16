fn main() {

    let mut v:Vec<u32>=Vec::new();
    v=vec![0,1,2];

    v.push(3);
    let first:&u32=&v[0];

    let second:Option<&u32>=v.get(1);
    match second{
    Some(second)=>println!("vec_second : {second}"),   
    _=>println!("There isn't second vec"),
    }

    println!("vec_first :{first}");
    
}
