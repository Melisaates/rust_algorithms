fn main() {
    enum AnimalKind{
        Cat,
        Dog,
        Bird(String)
    }
    impl AnimalKind{
        fn call(&self){
        }
    }

    let my_bird=AnimalKind::Bird(String::from("Birdy"));
    
    println!({},my_bird.call());

    //enum ve struct olarak tanımladığım variablelara nasıl erişicem?????????
    struct Animals{
        kind:AnimalKind,
        color:String
    }

    let animal1=Animals{kind:AnimalKind::Cat,color:String::from("black")};
    let animal2=Animals{kind:AnimalKind::Dog,color:String::from("white")};

    //println!("{:?}");
    //println!("{}",my_bird);


}
