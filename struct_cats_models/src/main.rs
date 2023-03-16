#[derive(Debug)]
struct Cat{
    name:String,
    year:u32,
    color:String,
}
fn main() {
    let cat1=Cat{name:String::from("Tarçın"),year:2,color:String::from("Brown"),};
    let cat2=Cat{name:String::from("Kahve"),..cat1};
    
    //println!("cat1:{:?}",cat1);
    println!("cat2:{:#?}",cat2);

}

