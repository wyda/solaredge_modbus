use std::io::Error;
use std::time::Duration;
use tokio_serial::{Serial, SerialPortSettings};

use tokio_modbus::prelude::*;
use tokio_modbus::client::Context;

pub struct Register {
    address : u16,
    size : u16
}

#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct SolarEdgeRegister {
    pub C_SunSpec_DID : Register,
    pub C_SunSpec_Lenght : Register,
    pub I_AC_Strom : Register,
    pub I_AC_StromA : Register,
    pub I_AC_StromB : Register,
    pub I_AC_StromC : Register,
    pub I_AC_Strom_SF : Register,
    pub I_AC_SpannungAB : Register,
    pub I_AC_SpannungBC : Register,
    pub I_AC_SpannungCA : Register,
    pub I_AC_SpannungAN : Register,
    pub I_AC_SpannungBN : Register,
    pub I_AC_SpannungCN : Register,
    pub I_AC_Spannung_SF : Register,
    pub I_AC_Leistung : Register,
    pub I_AC_Leistung_SF : Register,
    pub I_AC_Frequenz : Register,
    pub I_AC_Frequenz_SF : Register,
    pub I_AC_VA : Register,
    pub I_AC_VA_SF : Register,
    pub I_AC_VAR : Register,
    pub I_AC_VAR_SF : Register,
    pub I_AC_PF : Register,
    pub I_AC_PF_SF : Register,
    pub I_AC_Energie_WH : Register,
    pub I_AC_Energie_WH_SF : Register,
    pub I_DC_Strom : Register,
    pub I_DC_Strom_SF : Register,
    pub I_DC_Spannung : Register,
    pub I_DC_Spannung_SF : Register,
    pub I_DC_Leistung : Register,
    pub I_DC_Leistung_SF : Register,
    pub I_Temp_Cooler : Register,
    pub I_Temp_SF : Register,
    pub I_Status : Register,
    pub I_Status_Anbieter : Register,
}

impl SolarEdgeRegister {
    pub fn new() -> SolarEdgeRegister {
        SolarEdgeRegister {
            C_SunSpec_DID : Register {address : 40070, size: 1},
            C_SunSpec_Lenght : Register {address : 40071, size : 1},
            I_AC_Strom : Register {address : 40072, size : 1},
            I_AC_StromA : Register {address : 40073, size : 1},
            I_AC_StromB : Register {address : 40074, size : 1},
            I_AC_StromC : Register {address : 40075, size : 1},
            I_AC_Strom_SF : Register {address : 40076, size : 1},
            I_AC_SpannungAB : Register {address : 40077, size : 1},
            I_AC_SpannungBC : Register {address : 40078, size : 1},
            I_AC_SpannungCA : Register {address : 40079, size : 1},
            I_AC_SpannungAN : Register {address : 40080, size : 1},
            I_AC_SpannungBN : Register {address : 40081, size : 1},
            I_AC_SpannungCN : Register {address : 40082, size : 1},
            I_AC_Spannung_SF : Register {address : 40083, size : 1},
            I_AC_Leistung : Register {address : 40084, size : 1},
            I_AC_Leistung_SF : Register {address : 40085, size : 1},
            I_AC_Frequenz : Register {address : 40086, size : 1},
            I_AC_Frequenz_SF : Register {address : 40087, size : 1},
            I_AC_VA : Register {address : 40088, size : 1},
            I_AC_VA_SF : Register {address : 40089, size : 1},
            I_AC_VAR : Register {address : 40090, size : 1},
            I_AC_VAR_SF : Register {address : 40091, size : 1},
            I_AC_PF :  Register {address : 40092, size : 1},
            I_AC_PF_SF : Register {address : 40093, size : 1},
            I_AC_Energie_WH :  Register {address : 40094, size : 2},
            I_AC_Energie_WH_SF : Register {address : 40096, size : 1},
            I_DC_Strom :  Register {address : 40097, size : 1},
            I_DC_Strom_SF : Register {address : 40098, size : 1},
            I_DC_Spannung : Register {address : 40099, size : 1},
            I_DC_Spannung_SF :  Register {address : 40100, size : 1},
            I_DC_Leistung : Register {address : 40101, size : 1},
            I_DC_Leistung_SF : Register {address : 40102, size : 1},
            I_Temp_Cooler : Register {address : 40104, size : 1},
            I_Temp_SF :  Register {address : 40107, size : 1},
            I_Status :  Register {address : 40108, size : 1},
            I_Status_Anbieter : Register {address : 40109, size : 1},
        }
    }    
}

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
        let sun_edge_register = SolarEdgeRegister::new();        
    }
}
