fn main() {
    let term = std::env::args().nth(1).expect("no term is given");
    let user_string = std::env::args().nth(2).expect("no string is gizen");

    if user_string.contains(&term) {
        println!("Pattren {} is found on given String", term);
    } else {
        println!("Pattern Not found");
    }
}
