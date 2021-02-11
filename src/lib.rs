pub mod registers;
pub mod client;

#[cfg(test)]
mod tests {    
    use crate::registers::{SolarEdgeInverterRegister, SolarEdgeCommonRegister, SolarEdgeRegister, Register};
    //use crate::client::{SolarEdgeClient, SolarEdgeClientSync};
/*
    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }
*/
    #[test]    
    fn get_block_register_test() {                

        let solaredge_common_register = SolarEdgeCommonRegister::new();      
        let solaredge_inverter_register = SolarEdgeInverterRegister::new();      

        let solaredge_common_block = solaredge_common_register.get_block_register();      
        let solaredge_inverter_block = solaredge_inverter_register.get_block_register();                  

        let reg = Register{address:40001, size : 61};
        assert_eq!(solaredge_common_block.address, reg.address, "wrong register start address");
        assert_eq!(solaredge_common_block.size, reg.size, "wrong register size");

        let reg = Register{address:40070, size : 37};
        assert_eq!(solaredge_inverter_block.address, reg.address);
        assert_eq!(solaredge_inverter_block.size, reg.size);
    }

    #[test]    
    fn get_register_range_test() {                

        let solaredge_common_register = SolarEdgeCommonRegister::new();      
        let solaredge_inverter_register = SolarEdgeInverterRegister::new();                        

        let solaredge_common_register_range = solaredge_common_register.get_register_range(&solaredge_common_register.C_Device_adresse, &solaredge_common_register.C_Seriennummer);
        let reg =  Register{address:40053, size : 17};
        assert_eq!(solaredge_common_register_range.address, reg.address, "wrong register start address");
        assert_eq!(solaredge_common_register_range.size, reg.size, "wrong register size");        

        let solaredge_inverter_register_range = solaredge_inverter_register.get_register_range(&solaredge_inverter_register.I_AC_Frequenz, &solaredge_inverter_register.I_AC_Leistung);
        let reg =  Register{address:40084, size : 3};
        assert_eq!(solaredge_inverter_register_range.address, reg.address, "wrong register start address");
        assert_eq!(solaredge_inverter_register_range.size, reg.size, "wrong register size");        
               
    }

    #[test]    
    fn client_test() {       
        //ToDo create test server for client test  
        //let solaredge_common_register = SolarEdgeCommonRegister::new();  
        //let solaredge_common_register_range = solaredge_common_register.get_register_range(&solaredge_common_register.C_Device_adresse, &solaredge_common_register.C_Seriennummer);
        //let mut client = aw!(SolarEdgeClient::tcp_from("192.168.0.111", "5555")).unwrap(); 
        //let value = aw!(client.read_register(&solaredge_common_register_range));
           
        //let mut client = aw!(SolarEdgeClient::rtu_from("/dev/tty_USB0", 2, 19200)).unwrap();    
        //let value = aw!(client.read_register(&solaredge_common_register_range));
    }
}
