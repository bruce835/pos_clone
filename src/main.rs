use std::io;
use std::io::Write;

fn get_user_input(current_window: String) {
    let current_window = current_window.trim();
    let current_window = current_window.to_string(); 
    print!("POS: ");
    io::stdout().flush().expect("Could not flush output buffer.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Could not read line.");
    let input = input;

    print!("{}", current_window);
    if input.trim() != "current_window" {
        change_window(input);
    }
    else {
        change_window(current_window);
    }
}

fn w_main() { 
    let current_window = String::from("Main");
    println!("At Main:");
    get_user_input(current_window);
}

fn w_other() {
    let current_window = String::from("Other");
    println!("At Other:");
    get_user_input(current_window);
}

fn change_window(window: String) {
    println!("Debug: change_window");
   if window.trim() == "Other" {
       w_other();
   } 

   else if window.trim() == "Main" {
       w_main();
   }
}

fn main() {
    w_main();
}
