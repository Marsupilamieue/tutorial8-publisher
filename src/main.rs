use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, MessageHandler, HandleError};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        println!("Message received on handler 1: {:?}", message);
        Ok(())
    }
}

fn main() {
    let mut publisher = CrosstownBus::new_queue_publisher("amqp://guest:guest@localhost:5672".to_owned()).unwrap();
    
    let user1_message = UserCreatedEventMessage {
        user_id: "1".to_owned(),
        user_name: "2206081931y-Amir".to_owned(),
    };
    let user2_message = UserCreatedEventMessage {
        user_id: "2".to_owned(),
        user_name: "2206081931y-Budi".to_owned(),
    };
    let user3_message = UserCreatedEventMessage {
        user_id: "3".to_owned(),
        user_name: "2206081931y-Cica".to_owned(),
    };
    let user4_message = UserCreatedEventMessage {
        user_id: "4".to_owned(),
        user_name: "2206081931y-Dira".to_owned(),
    };
    let user5_message = UserCreatedEventMessage {
        user_id: "5".to_owned(),
        user_name: "2206081931y-Emir".to_owned(),
    };

    let _ = publisher.publish_event("user_created".to_owned(), user1_message);
    let _ = publisher.publish_event("user_created".to_owned(), user2_message);
    let _ = publisher.publish_event("user_created".to_owned(), user3_message);
    let _ = publisher.publish_event("user_created".to_owned(), user4_message);
    let _ = publisher.publish_event("user_created".to_owned(), user5_message);
}
