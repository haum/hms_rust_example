extern crate amqp;

mod hms;

use hms::bot::HmsBot;

fn main() {
    let mut bot = HmsBot::new();
    
    bot.irc_debug("Hello, from Rust!");
    bot.irc_debug("This langage is as verbose as Python, as fast as C and as secure as Haskell!");
    bot.close();
}
