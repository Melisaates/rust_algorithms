fn main() {
    let my_dimensions:(u32,u32)=(10,50);


    println!("Area:{}",area(my_dimensions));



    fn area(dimensions:(u32,u32))->u32{
        dimensions.0*dimensions.1
    }
}
