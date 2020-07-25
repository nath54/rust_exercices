
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
    let n=0;
    let sum_square=calc_sum_square(n);
    let sum_simple=calc_sum_simple(n);
    let final_result=sum_square-sum_simple;
    println!("The result of the sum of the squares of all integers between 0 and {} and theirs addition is {}", n, final_result);
}
