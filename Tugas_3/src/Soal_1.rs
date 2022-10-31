fn get_str(a: &str) -> String {
    return a.chars()
    .rev()
    .collect::<String>();
}


fn main(){
    let Word1 = get_str("Soal Nomor Satu");
    println!("Soal 1: {:?}", Word1);
}