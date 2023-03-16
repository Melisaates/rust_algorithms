fn main() {

    enum Option<T>{
        None,
        Some(T),
    }

    let x=Some(5);
    let y:Option<u32>=Some(3);
    let z:Option<u32>=None;

    //println!("x:{x}");

}
