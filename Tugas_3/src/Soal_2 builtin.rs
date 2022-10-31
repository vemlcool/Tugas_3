fn get_num(a: &Vec<i32>) -> Vec<bool> {
    return a.into_iter()
    .map(prime_num)
    .collect();
}

fn prime_num(number: &i32) -> bool {
    if *number == 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= *number {
        if *number % i == 0 {
            return false;
        }
        i = i + 1;
    }
    return true;
}

fn main(){
    let vec = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    println!("Is it Prime Number ? : {:?}", get_num(&vec));
}