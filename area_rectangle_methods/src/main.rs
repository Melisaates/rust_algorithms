struct Rectangle{
    width:u32,
    height:u32,
}
impl Rectangle{
    fn area(&self)->32{
        self.width*self.height
    }

    fn can_hold(&self,rec:&Rectangle)->bool{
        if self.area()>rec.area(){
            true
        }else{
            false
        }
    }

}
    
fn main() {
    let rec1=Rectangle{width:10,height:30};
    let rec2=Rectangle{width:100,height:30};
    let rec3=Rectangle{width:40,height:30};

    println!("Can rec1 hold rec2? -{}",rec1.can_hold(&rec2));
    println!("Can rec1 hold rec3? -{}",rec1.can_hold(&rec3));
    println!("Can rec3 hold rec2? -{}",rec3.can_hold(&rec2));


    
}
