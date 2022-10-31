fn color(a: &str) -> (u8, u8, u8, u8) {
    match a
    .to_lowercase()
    .as_str() {
        "red" => (255, 0, 0, 1),
        "green" => (255, 0, 0, 1),
        "blue" => (0, 0, 255, 1),
        "black" => (0, 0, 0, 1),
        "white" => (255, 255, 255, 0),
        "pink" => (238, 130, 238, 1),
        "orange" => (255, 165, 0, 1),
        "purple" => (106, 90, 205, 1),
        "gray" => (120, 120, 120, 1),
        "yellow" => (255, 240, 0, 1),
        _ => (0, 0, 0, 0)
    }
}

fn main(){
    
    println!("RGBA: ({:?}, {:?}, {:?}, {:?})", color("Red").0, color("Red").1, color("Red").2, color("Red").3);
    
}