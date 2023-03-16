fn main(){
    println!("Hello world!");
    greeting();
    named_greeting("melisa");
    greeting_2('m',21);
}
fn greeting(){
    println!("Helloooo!");
}
fn named_greeting(name:&str){
    println!("Hello {name}");
}
fn greeting_2(name:char,year:u32){
    println!("Hello! My name is {name}. I'm {year} years old.");
}
