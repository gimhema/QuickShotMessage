mod QuickShotMessage;

use QuickShotMessage::{*, example_msgs::TEST_Deseriialize};
use QuickShotMessage::example_node::*;

use crate::example_msgs::TEST_Seriialize;

use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} -c|-s", args[0]);
        return;
    }

    match args[1].as_str() {
        "-s" => run_server().await,
        "-c" => run_client().await,
        _ => {
            eprintln!("Invalid option. Use -c for client or -s for server.");
        }
    }
}


// fn main() {
    
//     // TEST_Seriialize();

//     println!("==========================================");

//     TEST_Deseriialize();
// }

