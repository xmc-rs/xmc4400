#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Message Object Function Control Register"]
    pub mofcr: MOFCR,
    #[doc = "0x04 - Message Object FIFO/Gateway Pointer Register"]
    pub mofgpr: MOFGPR,
    #[doc = "0x08 - Message Object Interrupt Pointer Register"]
    pub moipr: MOIPR,
    #[doc = "0x0c - Message Object Acceptance Mask Register"]
    pub moamr: MOAMR,
    #[doc = "0x10 - Message Object Data Register Low"]
    pub modatal: MODATAL,
    #[doc = "0x14 - Message Object Data Register High"]
    pub modatah: MODATAH,
    #[doc = "0x18 - Message Object Arbitration Register"]
    pub moar: MOAR,
    #[doc = "0x1c - Message Object Control Register"]
    pub moctr: MOCTR,
}
#[doc = "Message Object Function Control Register"]
pub struct MOFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Object Function Control Register"]
pub mod mofcr;
#[doc = "Message Object FIFO/Gateway Pointer Register"]
pub struct MOFGPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Object FIFO/Gateway Pointer Register"]
pub mod mofgpr;
#[doc = "Message Object Interrupt Pointer Register"]
pub struct MOIPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Object Interrupt Pointer Register"]
pub mod moipr;
#[doc = "Message Object Acceptance Mask Register"]
pub struct MOAMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Object Acceptance Mask Register"]
pub mod moamr;
#[doc = "Message Object Data Register Low"]
pub struct MODATAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Object Data Register Low"]
pub mod modatal;
#[doc = "Message Object Data Register High"]
pub struct MODATAH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Object Data Register High"]
pub mod modatah;
#[doc = "Message Object Arbitration Register"]
pub struct MOAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Object Arbitration Register"]
pub mod moar;
#[doc = "Message Object Control Register"]
pub struct MOCTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Object Control Register"]
pub mod moctr;
#[doc = "Message Object Status Register"]
pub struct MOSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Message Object Status Register"]
pub mod mostat;
