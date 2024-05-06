mod QuickShotMessage;

use QuickShotMessage::{*, example_msgs::TEST_Deseriialize};

use crate::example_msgs::TEST_Seriialize;

fn main() {
    
    // TEST_Seriialize();

    println!("==========================================");

    TEST_Deseriialize();
}
