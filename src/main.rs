use std::io;
use std::io::Write;

fn get_user_input(current_window: &str) -> String {
    let current_window = current_window.trim();
    let current_window = current_window.to_string(); 
    
    print!("POS: ");
    io::stdout().flush().expect("Could not flush output buffer.");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Could not read line.");
    let input = input.trim();
    let input = input.to_string();
    let is_window = input == String::from("Other") || input == String::from("Main");
    let input_clone = input.clone();
    
    if is_window == false || input == current_window {
        println!("Returning input clone");
        return input_clone;
    } 
        change_window(input);
        return String::from("changed_window");
}

fn w_main() { 
    let current_window = "Main";
    println!("At Main:");

    loop {
    let input = get_user_input(current_window);
println!("Debug: input received -> '{}'", input.trim());
        if input.trim() == "li" {
            println!("test");
        }

        else if input.trim() == "clear" {
            print!("\x1B[2J\x1B[1;1H");
        }

        else {
            println!("Invalid input");
            w_main();
        } 
    }
}

fn w_other() {
    let current_window = "Other";
    println!("At Other:");

    loop {
    let input = get_user_input(current_window);
println!("Debug: input received -> '{}'", input.trim());
        if input.trim() == "li" {
            println!("test");
        }

        else if input.trim() == "clear" {
            print!("\x1B[2J\x1B[1;1H");
        }

        else {
            println!("Invalid input");
            w_other();
        }
    }
}

fn change_window(window: String) {
   print!("\x1B[2J\x1B[1;1H"); 
   if window.trim() == "Other" {
       w_other();
   } 

   else if window.trim() == "Main" {
       w_main();
   }
   
   else {
       return;
   }
}

fn main() {
   print!("\x1B[2J\x1B[1;1H"); 
    w_main();
}
