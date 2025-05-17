use std::io;
use std::io::Write;

fn get_user_input(current_window: String) {
    let current_window = current_window.trim();
    let current_window = current_window.to_string(); 
    
    let is_window = current_window == "Main" || current_window == "Other";

    print!("POS: ");
    io::stdout().flush().expect("Could not flush output buffer.");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Could not read line.");
    let input = input.trim();
    let input = input.to_string();
    
    if input != "current_window" && is_window == true {
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
   print!("\x1B[2J\x1B[1;1H"); 
   if window.trim() == "Other" {
       w_other();
   } 

   else if window.trim() == "Main" {
       w_main();
   }
}

fn main() {
   print!("\x1B[2J\x1B[1;1H"); 
    w_main();
}
