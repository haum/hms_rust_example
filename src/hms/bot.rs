
use amqp::{Basic, Session, Channel, Table, protocol};
use std::default::Default;

pub struct HmsBot {
    exchange_name: String,

    session: Session,
    channel: Channel
}

impl HmsBot {

    /// Constructs a new `HMS_Bot`
    pub fn new() -> HmsBot {
        let exchange_name = "haum";
        let server_url = "amqp://localhost//";

        // Connect to the server and create a new channel on it
        let mut session = Session::open_url(server_url).unwrap();
        let mut channel = session.open_channel(1).unwrap();

        // Declare an exchanger and check if it worked
        let _ = channel.exchange_declare(
            exchange_name, // name of the exchanger
            "topic",       // type of the exchanger
            false,         // passive
            false,         // durable
            false,         // exclusive
            false,         // auto_delete
            false,         // nowait
            Table::new()   // arguments
        ).unwrap();

        // Create the object and return it
        HmsBot {
            exchange_name: exchange_name.to_string(),
            session: session,
            channel: channel,
        }
    }

    /// Send a IRC message on the IRC chan
    pub fn irc_debug(&mut self, message: &str) {
        let json = format!(r#"{{"privmsg": "{}"}}"#, message);
        
        self.send_json("irc_debug", json.as_str())
    }

    /// Send a JSON string over the exchanger
    pub fn send_json(&mut self, topic: &str, json: &str) {
        let content_type = Some("application/json".to_string());
        let json_bytes = json.as_bytes().to_vec();

        self.send_bytes(topic, json_bytes, content_type)
    }

    /// Send bytes over the exchanger
    ///
    /// This is a very low level send function that should not be accessible from the
    /// outside world for security reasons because we always want to send JSON at least
    /// and not raw bytes.
    fn send_bytes(&mut self, topic: &str, bytes: Vec<u8>, content_type: Option<String>) {
        // Set the correct content type
        let message_properties = protocol::basic::BasicProperties{
            content_type: content_type,
            ..Default::default()
        };

        // Send the message (and check it worked)
        self.channel.basic_publish(
            self.exchange_name.as_str(),
            topic,
            true,  // mandatory
            false, // immediate
            message_properties,
            bytes
        ).unwrap();
    }

    /// Close the AMQP channel and the connection to the AMQP server
    pub fn close(&mut self) {
        self.channel.close(200, "Bye").unwrap();
        self.session.close(200, "Good Bye");
    }
}
