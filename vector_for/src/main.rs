fn main() {
    let mut v:Vec<u32>=vec![10,9,8,7,6];
    for i in &mut v{

        *i -=3;
        println!("{i}");
    }
}
