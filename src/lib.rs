pub mod registers;
pub use registers::{SolarEdgeCommonRegister, SolarEdgeInverterRegister, Register};

use std::io::Error;
use std::time::Duration;
use tokio_serial::{Serial, SerialPortSettings};

use tokio_modbus::prelude::*;
use tokio_modbus::client::Context;





pub struct SolarEdgeClient {    
    ctx : Context,    
}

impl SolarEdgeClient {
    pub async fn from<'a>(tty_path : &str, slave_address : u8, baud_rate : u32) -> Result<SolarEdgeClient, Error> {
        let mut settings = SerialPortSettings::default();
        settings.baud_rate = baud_rate;
        settings.timeout = Duration::from_secs(5);

        let port = Serial::from_path(tty_path, &settings)?;    
        let slave = Slave::from(slave_address);
        let ctx = rtu::connect_slave(port, slave).await?;  
        
        Ok(SolarEdgeClient{
            ctx,
        })
    }

    pub async fn read_register(& mut self, register : Register) -> Result<Vec<u16>, Error> {        
        Ok(self.ctx.read_holding_registers(register.address, register.size).await?)    
    }
}


#[cfg(test)]
mod tests {
    use super::*;

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
