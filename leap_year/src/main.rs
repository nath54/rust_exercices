use std::io;

fn input(txt: &str) -> String{
    println!("{}",txt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    let txt =input.to_string();
    return txt;
}

fn main(){
    let yt = input("\nWhat is your year ?");
    let yt=&yt;
    let year: u32 = yt.trim().parse().unwrap();
    //Pour savoir si une année est bissectile, il faut d'abord tester si elle est divisible par 4
    let t1 = year % 4;
    if t1==0 {
        //ensuite, il faut tester si elle est divisible par 100
        let t2=year%100;
        if t2==0 {
            let t3=year%400;
            if t3==0 {
                println!("The year {} is a leap year !", year);   
            }
            else{
                println!("The year {} is not a leap year !", year);
            }
        }
        else{
            println!("The year {} is a leap year !", year);    
        }
    }
    else{
        println!("The year {} is not a leap year !", year);
    }

}
