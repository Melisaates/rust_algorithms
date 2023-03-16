
fn main() {
    #[derive(Debug)]
    enum MyProperities{
        Year(i32),
        Height(f64),
        Name(String)
    }

    let row=vec![
        MyProperities::Year(21),
        MyProperities::Height(1.66),
        MyProperities::Name(String::from("Melisa"))    ];

    println!("{:?}", row);


}
