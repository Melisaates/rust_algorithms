fn main() {
    let fahrenheit:f32=77.0;

    let celsius=to_celsius(fahrenheit);

    println!("{fahrenheit}F = {celsius}C");
    
}

fn to_celsius(degree:f32) -> f32{
    (degree-32.0)/1.8

}

