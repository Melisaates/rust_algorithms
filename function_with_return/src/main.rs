fn main() {
    println!("Welcome to my number world.:)");
    let num=create_num();
    let new_num:u32=sum(num);
    println!("new_number -> {new_num}");
}

fn create_num()->u32{
    5
}

fn sum(number:u32)->u32{
    println!("last_number -> {number}");
    number*5
}
