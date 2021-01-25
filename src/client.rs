use std::io::Error;
use std::time::Duration;
use tokio_serial::{Serial, SerialPortSettings};

use tokio_modbus::prelude::*;
use tokio_modbus::client::Context;

use crate::registers::Register;





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

pub struct SolarEdgeClientSync {    
    ctx : sync::Context,    
}

impl SolarEdgeClientSync {
    pub fn from<'a>(tty_path : &str, slave_address : u8, baud_rate : u32) -> Result<SolarEdgeClientSync, Error> {
        let mut settings = SerialPortSettings::default();
        settings.baud_rate = baud_rate;
        settings.timeout = Duration::from_secs(5);

        let slave = Slave::from(slave_address);        
        let ctx = sync::rtu::connect_slave(tty_path, &settings, slave)?;
        
        Ok(SolarEdgeClientSync{
            ctx,
        })
    }

    pub fn read_register(& mut self, register : Register) -> Result<Vec<u16>, Error> {        
        Ok(self.ctx.read_holding_registers(register.address, register.size)?)    
    }
}