use std::io;

fn input(txt: &str) -> String{
    println!("{}",txt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    let txt =input.to_string();
    return txt;
}

fn calc_sum_square(n: u32) -> u32{
    let mut result = 0;
    for x in 0..n{
        result+=x*x;
    }   
    return result;
}

fn calc_sum_simple(n: u32) -> u32{
    let mut result = 0;
    for x in 0..n{
        result+=x;
    }   
    return result;
}



fn main() {
    let m=input("Which value take n ?");
    let n=m.trim().parse().unwrap();
    let sum_square=calc_sum_square(n);
    println!("The result of the sum of the squares of all integers between 0 and {} is {}",n,sum_square);
    let sum_simple=calc_sum_simple(n);
    println!("The result of the sum of all integers between 0 and {} is {}",n,sum_simple);
    let final_result=sum_square-sum_simple;
    println!("The result of substraction of this two sums is {}", final_result);
}
