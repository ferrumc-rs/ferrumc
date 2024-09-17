// Security or something like that
#![forbid(unsafe_code)]

use ferrumc_macros::event_handler;

fn main() {
    ferrumc_logging::init_logging();
    
    println!("good day to ya. enjoy your time with ferrumc!");
    
}

#[event_handler(priority = 0)]
async fn some_test_event() {
    println!("I'm a test event!");
}