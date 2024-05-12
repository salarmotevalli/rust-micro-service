use std::env;

pub struct Cnf {
    pub port: u16,
}

impl Cnf {
    pub fn load() -> Self {
        Cnf {
            port: Self::get_port(),
        }
    }

    fn get_port() -> u16 {
        env::var("BROKER_SERVICE_PORT")
            .expect("The \"BROKER_SERVICE_PORT\" variable is not set.")
            .parse::<u16>()
            .expect("\"BROKER_SERVICE_PORT\" type should be a u16")
    }
}
