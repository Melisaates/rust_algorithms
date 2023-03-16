fn main() {
    let mut s=String::from("melisa");
    println!("{s}'nin boyutu {}",calculate_length(&s));
    dbg!(s.push_str("ateş"));
    println!("{s}'in boyutu {}",calculate_length(&s));

}

fn calculate_length(s:&String)->usize{
    s.len()
}

