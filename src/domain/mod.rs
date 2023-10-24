//! src/domain/mod.rs

mod email_client;
mod new_subscriber;
mod subscriber_email;
mod subscriber_name;

pub use email_client::EmailClient;
pub use new_subscriber::NewSubscriber;
pub use subscriber_email::SubscriberEmail;
pub use subscriber_name::SubscriberName;
