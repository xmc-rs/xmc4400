#[doc = "Register `GSTAT` reader"]
pub type R = crate::R<GSTAT_SPEC>;
#[doc = "CC80 IDLE status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0I_A {
    #[doc = "0: Running"]
    VALUE1 = 0,
    #[doc = "1: Idle"]
    VALUE2 = 1,
}
impl From<S0I_A> for bool {
    #[inline(always)]
    fn from(variant: S0I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0I` reader - CC80 IDLE status"]
pub type S0I_R = crate::BitReader<S0I_A>;
impl S0I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0I_A {
        match self.bits {
            false => S0I_A::VALUE1,
            true => S0I_A::VALUE2,
        }
    }
    #[doc = "Running"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0I_A::VALUE1
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0I_A::VALUE2
    }
}
#[doc = "CC81 IDLE status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1I_A {
    #[doc = "0: Running"]
    VALUE1 = 0,
    #[doc = "1: Idle"]
    VALUE2 = 1,
}
impl From<S1I_A> for bool {
    #[inline(always)]
    fn from(variant: S1I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1I` reader - CC81 IDLE status"]
pub type S1I_R = crate::BitReader<S1I_A>;
impl S1I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1I_A {
        match self.bits {
            false => S1I_A::VALUE1,
            true => S1I_A::VALUE2,
        }
    }
    #[doc = "Running"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1I_A::VALUE1
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1I_A::VALUE2
    }
}
#[doc = "CC82 IDLE status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2I_A {
    #[doc = "0: Running"]
    VALUE1 = 0,
    #[doc = "1: Idle"]
    VALUE2 = 1,
}
impl From<S2I_A> for bool {
    #[inline(always)]
    fn from(variant: S2I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2I` reader - CC82 IDLE status"]
pub type S2I_R = crate::BitReader<S2I_A>;
impl S2I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S2I_A {
        match self.bits {
            false => S2I_A::VALUE1,
            true => S2I_A::VALUE2,
        }
    }
    #[doc = "Running"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S2I_A::VALUE1
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S2I_A::VALUE2
    }
}
#[doc = "CC83 IDLE status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S3I_A {
    #[doc = "0: Running"]
    VALUE1 = 0,
    #[doc = "1: Idle"]
    VALUE2 = 1,
}
impl From<S3I_A> for bool {
    #[inline(always)]
    fn from(variant: S3I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S3I` reader - CC83 IDLE status"]
pub type S3I_R = crate::BitReader<S3I_A>;
impl S3I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S3I_A {
        match self.bits {
            false => S3I_A::VALUE1,
            true => S3I_A::VALUE2,
        }
    }
    #[doc = "Running"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S3I_A::VALUE1
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S3I_A::VALUE2
    }
}
#[doc = "Prescaler Run Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRB_A {
    #[doc = "0: Prescaler is stopped"]
    VALUE1 = 0,
    #[doc = "1: Prescaler is running"]
    VALUE2 = 1,
}
impl From<PRB_A> for bool {
    #[inline(always)]
    fn from(variant: PRB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRB` reader - Prescaler Run Bit"]
pub type PRB_R = crate::BitReader<PRB_A>;
impl PRB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRB_A {
        match self.bits {
            false => PRB_A::VALUE1,
            true => PRB_A::VALUE2,
        }
    }
    #[doc = "Prescaler is stopped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRB_A::VALUE1
    }
    #[doc = "Prescaler is running"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRB_A::VALUE2
    }
}
#[doc = "Parity Checker Run Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCRB_A {
    #[doc = "0: Parity Checker is stopped"]
    VALUE1 = 0,
    #[doc = "1: Parity Checker is running"]
    VALUE2 = 1,
}
impl From<PCRB_A> for bool {
    #[inline(always)]
    fn from(variant: PCRB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCRB` reader - Parity Checker Run Bit"]
pub type PCRB_R = crate::BitReader<PCRB_A>;
impl PCRB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCRB_A {
        match self.bits {
            false => PCRB_A::VALUE1,
            true => PCRB_A::VALUE2,
        }
    }
    #[doc = "Parity Checker is stopped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PCRB_A::VALUE1
    }
    #[doc = "Parity Checker is running"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PCRB_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - CC80 IDLE status"]
    #[inline(always)]
    pub fn s0i(&self) -> S0I_R {
        S0I_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC81 IDLE status"]
    #[inline(always)]
    pub fn s1i(&self) -> S1I_R {
        S1I_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC82 IDLE status"]
    #[inline(always)]
    pub fn s2i(&self) -> S2I_R {
        S2I_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CC83 IDLE status"]
    #[inline(always)]
    pub fn s3i(&self) -> S3I_R {
        S3I_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Prescaler Run Bit"]
    #[inline(always)]
    pub fn prb(&self) -> PRB_R {
        PRB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity Checker Run Bit"]
    #[inline(always)]
    pub fn pcrb(&self) -> PCRB_R {
        PCRB_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Global Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GSTAT_SPEC;
impl crate::RegisterSpec for GSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gstat::R`](R) reader structure"]
impl crate::Readable for GSTAT_SPEC {}
#[doc = "`reset()` method sets GSTAT to value 0x0f"]
impl crate::Resettable for GSTAT_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
