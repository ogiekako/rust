mod phrases {
    pub mod english {
        pub mod farewells {
            pub fn goodbye() -> String {
                "Goodbye.".to_string()
            }
        }
        pub mod greetings {
            pub fn hello() -> String {
                "Hello!".to_string()
            }
        }
    }
    pub mod japanese {
        pub use self::greetings::hello;
        pub use self::farewells::goodbye;

        pub mod farewells {
            pub fn goodbye() -> String {
                "Sayounara".to_string()
            }
        }
        pub mod greetings {
            pub fn hello() -> String {
                "Konnnichiwa".to_string()
            }
        }
    }
}

use phrases::english::greetings;
use phrases::english::farewells;
use phrases::japanese;

fn main() {
    // println!("Hello in English: {}", greetings::hello());
    println!("Goodbye in English: {}", farewells::goodbye());

    // println!("Hello in Japanese: {}", japanese::hello());
    println!("Goodbye in Japanese: {}", japanese::goodbye());
}
