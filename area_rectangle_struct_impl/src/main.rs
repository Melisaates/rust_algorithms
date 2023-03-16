struct Rectangle{
    width:u32,
    height:u32,
}

impl Rectangle{
    fn area(&self)->u32{        
        self.width*self.heigt
    
    }
}
fn main() {
    
    let my_rec=Rectangle{width:10,height:50 };
    println!("Area:{}",my_rec.area());
}

