pub mod registers;
pub mod client;

#[cfg(test)]
mod tests {    
    use crate::registers::{SolarEdgeInverterRegister};
    use crate::client::SolarEdgeClient;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]    
    fn it_works() {        
        let client = aw!(SolarEdgeClient::from("/dev/tty_USB0", 2, 19200));
        let sun_edge_register = SolarEdgeInverterRegister::new();        
    }
}
