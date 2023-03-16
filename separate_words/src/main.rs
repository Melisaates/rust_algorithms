fn main() {
    let mut s=String :: from("hello world");
    let my_first_word=first_word(&s);
    println!("m_f_w:{}",my_first_word);
    //s.clear();

}
fn first_word(s:&String)->usize{
    let bytes=s.as_bytes();
    for(i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    s.len()
}

