fn main() {
    let user_num=6;
    let mut count=1;
    let mut num=1;
    //let mut last_num=0;

    let mut temp=0;

    while count<user_num{
        let last_num=num;

        num+=temp;
        temp=last_num;
    
        count+=1;
    }

    println!("{user_num}. fibonacci sayısı = {num}");
}
