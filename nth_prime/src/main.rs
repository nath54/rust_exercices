use std::io;

fn input(txt: &str) -> String{
    println!("{}",txt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    let txt =input.to_string();
    return txt;
}

fn main() {
    let log=false;
    let m = input("What is your n number ?");
    let n: u32=m.trim().parse().unwrap();
    let mut actual_prime=0;
    let mut number = 0;
    let mut last_prime: u32=0;
    let security=100000;
    while actual_prime<n && number<security {
        number+=1;        
        let mut is_prime=true;
        let mut divisible: u32 = 0;
        //On teste si le nombre est un nombre premier
        for x in 2..number{
            if number%x==0 {
                is_prime=false;
                divisible=x;
                break;
            }
        }
        //Si oui on incrémente actual_prime
        if is_prime {
            actual_prime+=1;
            last_prime=number;
        }

        if log{
            if is_prime {
                println!("Testing {n} : Is a prime number", n=number)
            }
            else{
                println!("Testing {n} : Is not a prime number, divisible by {}", divisible, n=number);
            }
        }
    }
    println!("Your {n}th prime number is : {prime}", n=actual_prime, prime=last_prime);
    
}
