fn main() {
    use std::collections::HashMap;

    let my_sentence="hello my world:))))";
    let mut my_map=HashMap::new();
    
    for word in my_sentence.split_whitespace(){
        let count=my_map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("{:?}",my_map);

    println!("Hello, world!");
}
