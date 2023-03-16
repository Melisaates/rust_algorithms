#[derive(Debug)]//Implememnt hatasından kurtulmak için kullanılabilir.

struct Rectangle{         
    width:u32,
    height:u32
}


fn main() {
    let my_rec=Rectangle{width:dbg!(30*3),height:50};
    println!("Area:{}",area(&my_rec));//Direkt kendisini gönderseydik fonksiyona ve fonksiyonu da böyle tanımlasaydık o zaman move yaptığımız için my_rec silinecek ve aşağıdaki gibi ona ulaşamayacaktık.
    
    println!("h={}",my_rec.height);
    println!("my_rec={:?}",my_rec);//{:#?}da kullanılabilir.
    dbg!(&my_rec);    


}


fn area(rec:&Rectangle)->u32{
    rec.width*rec.height
}
