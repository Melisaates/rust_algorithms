fn main() {
    let s= String::from("hello");
    takes_ownership(s);
    //println!("dnm---{s}");--error


    let x=5;
    makes_copy(x);
    println!("dnm--{x}");

}
fn takes_ownership(s:String){
    println!("{s}");
}

fn makes_copy(x:i32){
    println!("{x}");

}

