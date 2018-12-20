#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - External input selection"]
    pub dci: DCI,
    #[doc = "0x04 - External input selection"]
    pub ies: IES,
    #[doc = "0x08 - Slope generation control"]
    pub sc: SC,
    #[doc = "0x0c - Pulse swallow configuration"]
    pub pc: PC,
    #[doc = "0x10 - DAC reference value 1"]
    pub dsv1: DSV1,
    #[doc = "0x14 - DAC reference value 1"]
    pub dsv2: DSV2,
    #[doc = "0x18 - Shadow reference value 1"]
    pub sdsv1: SDSV1,
    #[doc = "0x1c - Shadow Pulse swallow value"]
    pub spc: SPC,
    #[doc = "0x20 - Comparator configuration"]
    pub cc: CC,
    #[doc = "0x24 - Passive level configuration"]
    pub plc: PLC,
    #[doc = "0x28 - Comparator blanking value"]
    pub blv: BLV,
    #[doc = "0x2c - Service request enable"]
    pub sre: SRE,
    #[doc = "0x30 - Service request line selector"]
    pub srs: SRS,
    #[doc = "0x34 - Service request SW set"]
    pub sws: SWS,
    #[doc = "0x38 - Service request SW clear"]
    pub swc: SWC,
    #[doc = "0x3c - Service request status"]
    pub istat: ISTAT,
}
#[doc = "External input selection"]
pub struct DCI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External input selection"]
pub mod dci;
#[doc = "External input selection"]
pub struct IES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External input selection"]
pub mod ies;
#[doc = "Slope generation control"]
pub struct SC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slope generation control"]
pub mod sc;
#[doc = "Pulse swallow configuration"]
pub struct PC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pulse swallow configuration"]
pub mod pc;
#[doc = "DAC reference value 1"]
pub struct DSV1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC reference value 1"]
pub mod dsv1;
#[doc = "DAC reference value 1"]
pub struct DSV2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DAC reference value 1"]
pub mod dsv2;
#[doc = "Shadow reference value 1"]
pub struct SDSV1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow reference value 1"]
pub mod sdsv1;
#[doc = "Shadow Pulse swallow value"]
pub struct SPC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Pulse swallow value"]
pub mod spc;
#[doc = "Comparator configuration"]
pub struct CC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Comparator configuration"]
pub mod cc;
#[doc = "Passive level configuration"]
pub struct PLC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Passive level configuration"]
pub mod plc;
#[doc = "Comparator blanking value"]
pub struct BLV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Comparator blanking value"]
pub mod blv;
#[doc = "Service request enable"]
pub struct SRE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Service request enable"]
pub mod sre;
#[doc = "Service request line selector"]
pub struct SRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Service request line selector"]
pub mod srs;
#[doc = "Service request SW set"]
pub struct SWS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Service request SW set"]
pub mod sws;
#[doc = "Service request SW clear"]
pub struct SWC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Service request SW clear"]
pub mod swc;
#[doc = "Service request status"]
pub struct ISTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Service request status"]
pub mod istat;
