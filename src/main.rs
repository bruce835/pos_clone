use std::io;
use std::io::Write;

fn w_main() { 
    println!("At Main:");
    print!("Window: ");
    io::stdout().flush().expect("Could not flush output buffer.");
    let mut window = String::new();
    io::stdin().read_line(&mut window).expect("Could not read line.");
    let window = window;
    change_window(window); 
}

fn w_other() {
    println!("At Other:");
}

fn main() {
    w_main();
}

fn change_window(window: String) {
   if window.trim() == "Other" {
       w_other();
   } 
}
