fn main() {
    let married:bool=false;
    let surname:&str=if married {"!ateş"} else {"ateş"};  
    println!("My name is Melisa {surname}");

}
