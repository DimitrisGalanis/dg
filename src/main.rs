use dotenv::dotenv;
use std::process::Command;
use std::process;
use std::env;

fn main() {
    dotenv().ok();
    let chrome_path = env::var("chrome_path").expect("Chrome path not added in .env");
    let user_args: Vec<String> = env::args().collect();

    if user_args.len() > 1 {

        let search_query = user_args[1..].join(" ") + " site:stackoverflow.com OR site:medium.com"; 
        let search_url = format!("https://www.google.com/search?q={}", search_query.replace(" ", "+"));

    
        let status = Command::new(format!(r"{}", chrome_path))
            .arg("--new-tab")
            .arg(&search_url)
            .status()
            .expect("failed to execute process");

    if status.success() {
        println!("Arguments provided: {}",user_args.len());
        println!("Google Chrome opened successfully!");
    } else {
        println!("Failed to open Google Chrome.");
    }
    } else {
        println!("No arguments were provided.");
    }
}
