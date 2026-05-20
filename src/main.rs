use notify_rust::{Notification, Timeout};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let time: &String = &args[1];
        println!("Checked in at: {}", time);
    }

    /* Notification::new()
     .summary("Firefox News")
     .body("This will almost look like a real firefox notification.")
     .icon("firefox")
     .timeout(Timeout::Milliseconds(6000)) //milliseconds
     .show().unwrap();
    */
}
