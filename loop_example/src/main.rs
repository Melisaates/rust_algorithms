fn main() {
    let mut count=0;
    'counting_up: loop  {
        println!("count={}",count);
        let mut remaining = 10;
        //let mut remaining=loop{
        //    count+=1;
        //    if count==5{
        //        break count*5;
        //   }
        //};
        loop{
            println!("remaining={remaining}");
            if remaining==9{
                break;
            }
            if remaining==2{
                break 'counting_up;
            }
            remaining-=1;
        }
        count +=1;
    }
    println!("End count = {count}");


}
