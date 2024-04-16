use dashmap::Dashmap;
use lazy_static::lazy_static;
use crate::model::subscriber::Subscriber;

// Singleton of Database
lazy_static! {
    static ref SUBSCRIBERS: Dashmap<String, Dashmap<String, Subscriber>> = Dashmap::new();
}

pub struct SubscriberRepository;

impl SubscriberRepository {
     pub fn add(product_type: &str, subscriber:Subscriber) -> Subscriber {
        let subscriber_value = subscriber.clone();
        if SUBSCRIBERS.get(product_type).is_none(){
            SUBSCRIBERS.insert(String::from(product_type), DashMap::new());
        };

        SUBSCRIBERS.get(product_type).unwrap()
            .insert(subscriber_value.url.clone(), subscriber_value);
        return subscriber;
     }
     pub fn list_all(product_type: &str) -> Vec<Subscriber> {
         if SUBSCRIBERS.get(product_type).is_none() {
             SUBSCRIBERS.insert(String::from(product_type), DashMap::new());
         };

         return SUBSCRIBERS.get(product_type).unwrap().iter()
             .map(|f| f.value().clone()).collect();
     }
}