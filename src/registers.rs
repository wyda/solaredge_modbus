pub struct Register {
    pub address : u16,
    pub size : u16
}

pub enum IStatus {
    IStatusOff = 1,
    IStatusSleep = 2,
    IStatusMPPT = 4,
}


#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct SolarEdgeCommonRegister {
    pub C_SunSpec_ID : Register,
    pub C_SunSpec_DID : Register,
    pub C_SunSpec_Lenght : Register,
    pub C_Hersteller : Register,
    pub C_Modell : Register,
    pub C_Version : Register,
    pub C_Seriennummer : Register,
    pub C_Device_adresse : Register,        
}
impl SolarEdgeCommonRegister {
    pub fn new() -> SolarEdgeCommonRegister {
        SolarEdgeCommonRegister {
            C_SunSpec_ID : Register {address : 40001, size: 2},
            C_SunSpec_DID : Register {address : 40003, size : 1},
            C_SunSpec_Lenght : Register {address : 40004, size : 1},
            C_Hersteller : Register {address : 40005, size : 16},
            C_Modell : Register {address : 40021, size : 16},
            C_Version : Register {address : 40045, size : 8},
            C_Seriennummer : Register {address : 40053, size : 16},
            C_Device_adresse : Register {address : 40069, size : 1},                
        }
    }    
}


#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct SolarEdgeInverterRegister {
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

impl SolarEdgeInverterRegister {
    pub fn new() -> SolarEdgeInverterRegister {
        SolarEdgeInverterRegister {
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

