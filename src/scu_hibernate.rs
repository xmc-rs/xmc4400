#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Hibernate Domain Status Register"]
    pub hdstat: HDSTAT,
    #[doc = "0x04 - Hibernate Domain Status Clear Register"]
    pub hdclr: HDCLR,
    #[doc = "0x08 - Hibernate Domain Status Set Register"]
    pub hdset: HDSET,
    #[doc = "0x0c - Hibernate Domain Control Register"]
    pub hdcr: HDCR,
    _reserved0: [u8; 4usize],
    #[doc = "0x14 - fOSI Control Register"]
    pub oscsictrl: OSCSICTRL,
    #[doc = "0x18 - OSC_ULP Status Register"]
    pub osculstat: OSCULSTAT,
    #[doc = "0x1c - OSC_ULP Control Register"]
    pub osculctrl: OSCULCTRL,
    #[doc = "0x20 - Analog Wake-up Configuration Register"]
    pub lpacconf: LPACCONF,
    #[doc = "0x24 - LPAC Threshold Register 0"]
    pub lpacth0: LPACTH0,
    #[doc = "0x28 - LPAC Threshold Register 1"]
    pub lpacth1: LPACTH1,
    #[doc = "0x2c - Hibernate Analog Control State Register"]
    pub lpacst: LPACST,
    #[doc = "0x30 - LPAC Control Clear Register"]
    pub lpacclr: LPACCLR,
    #[doc = "0x34 - LPAC Control Set Register"]
    pub lpacset: LPACSET,
    #[doc = "0x38 - Hibernate Internal Control State Register"]
    pub hintst: HINTST,
    #[doc = "0x3c - Hibernate Internal Control Clear Register"]
    pub hintclr: HINTCLR,
    #[doc = "0x40 - Hibernate Internal Control Set Register"]
    pub hintset: HINTSET,
}
#[doc = "Hibernate Domain Status Register"]
pub struct HDSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernate Domain Status Register"]
pub mod hdstat;
#[doc = "Hibernate Domain Status Clear Register"]
pub struct HDCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernate Domain Status Clear Register"]
pub mod hdclr;
#[doc = "Hibernate Domain Status Set Register"]
pub struct HDSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernate Domain Status Set Register"]
pub mod hdset;
#[doc = "Hibernate Domain Control Register"]
pub struct HDCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernate Domain Control Register"]
pub mod hdcr;
#[doc = "fOSI Control Register"]
pub struct OSCSICTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "fOSI Control Register"]
pub mod oscsictrl;
#[doc = "OSC_ULP Status Register"]
pub struct OSCULSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OSC_ULP Status Register"]
pub mod osculstat;
#[doc = "OSC_ULP Control Register"]
pub struct OSCULCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OSC_ULP Control Register"]
pub mod osculctrl;
#[doc = "Analog Wake-up Configuration Register"]
pub struct LPACCONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Wake-up Configuration Register"]
pub mod lpacconf;
#[doc = "LPAC Threshold Register 0"]
pub struct LPACTH0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LPAC Threshold Register 0"]
pub mod lpacth0;
#[doc = "LPAC Threshold Register 1"]
pub struct LPACTH1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LPAC Threshold Register 1"]
pub mod lpacth1;
#[doc = "Hibernate Analog Control State Register"]
pub struct LPACST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernate Analog Control State Register"]
pub mod lpacst;
#[doc = "LPAC Control Clear Register"]
pub struct LPACCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LPAC Control Clear Register"]
pub mod lpacclr;
#[doc = "LPAC Control Set Register"]
pub struct LPACSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LPAC Control Set Register"]
pub mod lpacset;
#[doc = "Hibernate Internal Control State Register"]
pub struct HINTST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernate Internal Control State Register"]
pub mod hintst;
#[doc = "Hibernate Internal Control Clear Register"]
pub struct HINTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernate Internal Control Clear Register"]
pub mod hintclr;
#[doc = "Hibernate Internal Control Set Register"]
pub struct HINTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hibernate Internal Control Set Register"]
pub mod hintset;
