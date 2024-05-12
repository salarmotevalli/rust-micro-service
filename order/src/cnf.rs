use std::env;

pub struct Cnf {
    pub port: u16,
    pub mongo_url: String,
}

impl Cnf {
    pub fn load() -> Self {
        Cnf {
            port: Self::get_port(),
            mongo_url: Self::get_mongo_url(),
        }
    }

    fn get_port() -> u16 {
        env::var("ORDER_SERVICE_PORT")
            .expect("The \"ORDER_SERVICE_PORT\" variable is not set.")
            .parse::<u16>()
            .expect("\"ORDER_SERVICE_PORT\" type should be a u16")
    }

    fn get_mongo_url() -> String {
        env::var("ORDER_SERVICE_PORT").expect("The \"ORDER_SERVICE_PORT\" variable is not set.")
    }
}
