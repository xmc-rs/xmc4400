#[doc = "Reader of register SRMSK"]
pub type R = crate::R<u32, super::SRMSK>;
#[doc = "Writer for register SRMSK"]
pub type W = crate::W<u32, super::SRMSK>;
#[doc = "Register SRMSK `reset()`'s with value 0"]
impl crate::ResetValue for super::SRMSK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "WDT pre-warning Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRWARN_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PRWARN_A> for bool {
    #[inline(always)]
    fn from(variant: PRWARN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRWARN`"]
pub type PRWARN_R = crate::R<bool, PRWARN_A>;
impl PRWARN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRWARN_A {
        match self.bits {
            false => PRWARN_A::VALUE1,
            true => PRWARN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRWARN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRWARN_A::VALUE2
    }
}
#[doc = "Write proxy for field `PRWARN`"]
pub struct PRWARN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRWARN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRWARN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PRWARN_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PRWARN_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "RTC Periodic Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PI_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PI_A> for bool {
    #[inline(always)]
    fn from(variant: PI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PI`"]
pub type PI_R = crate::R<bool, PI_A>;
impl PI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PI_A {
        match self.bits {
            false => PI_A::VALUE1,
            true => PI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PI_A::VALUE2
    }
}
#[doc = "Write proxy for field `PI`"]
pub struct PI_W<'a> {
    w: &'a mut W,
}
impl<'a> PI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PI_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PI_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "RTC Alarm Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AI_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<AI_A> for bool {
    #[inline(always)]
    fn from(variant: AI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AI`"]
pub type AI_R = crate::R<bool, AI_A>;
impl AI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AI_A {
        match self.bits {
            false => AI_A::VALUE1,
            true => AI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AI_A::VALUE2
    }
}
#[doc = "Write proxy for field `AI`"]
pub struct AI_W<'a> {
    w: &'a mut W,
}
impl<'a> AI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AI_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AI_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "DLR Request Overrun Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLROVR_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<DLROVR_A> for bool {
    #[inline(always)]
    fn from(variant: DLROVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DLROVR`"]
pub type DLROVR_R = crate::R<bool, DLROVR_A>;
impl DLROVR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLROVR_A {
        match self.bits {
            false => DLROVR_A::VALUE1,
            true => DLROVR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DLROVR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DLROVR_A::VALUE2
    }
}
#[doc = "Write proxy for field `DLROVR`"]
pub struct DLROVR_W<'a> {
    w: &'a mut W,
}
impl<'a> DLROVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DLROVR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DLROVR_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DLROVR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "LPACLR Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPACCR_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<LPACCR_A> for bool {
    #[inline(always)]
    fn from(variant: LPACCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPACCR`"]
pub type LPACCR_R = crate::R<bool, LPACCR_A>;
impl LPACCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPACCR_A {
        match self.bits {
            false => LPACCR_A::VALUE1,
            true => LPACCR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPACCR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPACCR_A::VALUE2
    }
}
#[doc = "Write proxy for field `LPACCR`"]
pub struct LPACCR_W<'a> {
    w: &'a mut W,
}
impl<'a> LPACCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPACCR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPACCR_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPACCR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "LPACTH0 Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPACTH0_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<LPACTH0_A> for bool {
    #[inline(always)]
    fn from(variant: LPACTH0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPACTH0`"]
pub type LPACTH0_R = crate::R<bool, LPACTH0_A>;
impl LPACTH0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPACTH0_A {
        match self.bits {
            false => LPACTH0_A::VALUE1,
            true => LPACTH0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPACTH0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPACTH0_A::VALUE2
    }
}
#[doc = "Write proxy for field `LPACTH0`"]
pub struct LPACTH0_W<'a> {
    w: &'a mut W,
}
impl<'a> LPACTH0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPACTH0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPACTH0_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPACTH0_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "LPACTH1 Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPACTH1_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<LPACTH1_A> for bool {
    #[inline(always)]
    fn from(variant: LPACTH1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPACTH1`"]
pub type LPACTH1_R = crate::R<bool, LPACTH1_A>;
impl LPACTH1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPACTH1_A {
        match self.bits {
            false => LPACTH1_A::VALUE1,
            true => LPACTH1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPACTH1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPACTH1_A::VALUE2
    }
}
#[doc = "Write proxy for field `LPACTH1`"]
pub struct LPACTH1_W<'a> {
    w: &'a mut W,
}
impl<'a> LPACTH1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPACTH1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPACTH1_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPACTH1_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "LPACST Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPACST_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<LPACST_A> for bool {
    #[inline(always)]
    fn from(variant: LPACST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPACST`"]
pub type LPACST_R = crate::R<bool, LPACST_A>;
impl LPACST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPACST_A {
        match self.bits {
            false => LPACST_A::VALUE1,
            true => LPACST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPACST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPACST_A::VALUE2
    }
}
#[doc = "Write proxy for field `LPACST`"]
pub struct LPACST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPACST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPACST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPACST_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPACST_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "LPACCLR Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPACCLR_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<LPACCLR_A> for bool {
    #[inline(always)]
    fn from(variant: LPACCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPACCLR`"]
pub type LPACCLR_R = crate::R<bool, LPACCLR_A>;
impl LPACCLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPACCLR_A {
        match self.bits {
            false => LPACCLR_A::VALUE1,
            true => LPACCLR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPACCLR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPACCLR_A::VALUE2
    }
}
#[doc = "Write proxy for field `LPACCLR`"]
pub struct LPACCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> LPACCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPACCLR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPACCLR_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPACCLR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "LPACSET Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPACSET_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<LPACSET_A> for bool {
    #[inline(always)]
    fn from(variant: LPACSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPACSET`"]
pub type LPACSET_R = crate::R<bool, LPACSET_A>;
impl LPACSET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPACSET_A {
        match self.bits {
            false => LPACSET_A::VALUE1,
            true => LPACSET_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPACSET_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPACSET_A::VALUE2
    }
}
#[doc = "Write proxy for field `LPACSET`"]
pub struct LPACSET_W<'a> {
    w: &'a mut W,
}
impl<'a> LPACSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPACSET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LPACSET_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LPACSET_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "HINTST Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HINTST_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<HINTST_A> for bool {
    #[inline(always)]
    fn from(variant: HINTST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HINTST`"]
pub type HINTST_R = crate::R<bool, HINTST_A>;
impl HINTST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HINTST_A {
        match self.bits {
            false => HINTST_A::VALUE1,
            true => HINTST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HINTST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HINTST_A::VALUE2
    }
}
#[doc = "Write proxy for field `HINTST`"]
pub struct HINTST_W<'a> {
    w: &'a mut W,
}
impl<'a> HINTST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HINTST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HINTST_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HINTST_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "HINTCLR Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HINTCLR_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<HINTCLR_A> for bool {
    #[inline(always)]
    fn from(variant: HINTCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HINTCLR`"]
pub type HINTCLR_R = crate::R<bool, HINTCLR_A>;
impl HINTCLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HINTCLR_A {
        match self.bits {
            false => HINTCLR_A::VALUE1,
            true => HINTCLR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HINTCLR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HINTCLR_A::VALUE2
    }
}
#[doc = "Write proxy for field `HINTCLR`"]
pub struct HINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HINTCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HINTCLR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HINTCLR_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HINTCLR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "HINTSET Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HINTSET_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<HINTSET_A> for bool {
    #[inline(always)]
    fn from(variant: HINTSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HINTSET`"]
pub type HINTSET_R = crate::R<bool, HINTSET_A>;
impl HINTSET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HINTSET_A {
        match self.bits {
            false => HINTSET_A::VALUE1,
            true => HINTSET_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HINTSET_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HINTSET_A::VALUE2
    }
}
#[doc = "Write proxy for field `HINTSET`"]
pub struct HINTSET_W<'a> {
    w: &'a mut W,
}
impl<'a> HINTSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HINTSET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HINTSET_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HINTSET_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "HDCLR Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDCLR_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<HDCLR_A> for bool {
    #[inline(always)]
    fn from(variant: HDCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HDCLR`"]
pub type HDCLR_R = crate::R<bool, HDCLR_A>;
impl HDCLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDCLR_A {
        match self.bits {
            false => HDCLR_A::VALUE1,
            true => HDCLR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HDCLR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HDCLR_A::VALUE2
    }
}
#[doc = "Write proxy for field `HDCLR`"]
pub struct HDCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> HDCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDCLR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HDCLR_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HDCLR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "HDSET Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDSET_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<HDSET_A> for bool {
    #[inline(always)]
    fn from(variant: HDSET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HDSET`"]
pub type HDSET_R = crate::R<bool, HDSET_A>;
impl HDSET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDSET_A {
        match self.bits {
            false => HDSET_A::VALUE1,
            true => HDSET_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HDSET_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HDSET_A::VALUE2
    }
}
#[doc = "Write proxy for field `HDSET`"]
pub struct HDSET_W<'a> {
    w: &'a mut W,
}
impl<'a> HDSET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDSET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HDSET_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HDSET_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "HDCR Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDCR_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<HDCR_A> for bool {
    #[inline(always)]
    fn from(variant: HDCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HDCR`"]
pub type HDCR_R = crate::R<bool, HDCR_A>;
impl HDCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDCR_A {
        match self.bits {
            false => HDCR_A::VALUE1,
            true => HDCR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HDCR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HDCR_A::VALUE2
    }
}
#[doc = "Write proxy for field `HDCR`"]
pub struct HDCR_W<'a> {
    w: &'a mut W,
}
impl<'a> HDCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDCR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HDCR_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HDCR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "OSCSICTRL Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSICTRL_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<OSCSICTRL_A> for bool {
    #[inline(always)]
    fn from(variant: OSCSICTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OSCSICTRL`"]
pub type OSCSICTRL_R = crate::R<bool, OSCSICTRL_A>;
impl OSCSICTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCSICTRL_A {
        match self.bits {
            false => OSCSICTRL_A::VALUE1,
            true => OSCSICTRL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OSCSICTRL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OSCSICTRL_A::VALUE2
    }
}
#[doc = "Write proxy for field `OSCSICTRL`"]
pub struct OSCSICTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCSICTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCSICTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OSCSICTRL_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OSCSICTRL_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "OSCULCTRL Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCULCTRL_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<OSCULCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: OSCULCTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OSCULCTRL`"]
pub type OSCULCTRL_R = crate::R<bool, OSCULCTRL_A>;
impl OSCULCTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCULCTRL_A {
        match self.bits {
            false => OSCULCTRL_A::VALUE1,
            true => OSCULCTRL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OSCULCTRL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OSCULCTRL_A::VALUE2
    }
}
#[doc = "Write proxy for field `OSCULCTRL`"]
pub struct OSCULCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCULCTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCULCTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OSCULCTRL_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OSCULCTRL_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "RTC CTR Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_CTR_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<RTC_CTR_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_CTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTC_CTR`"]
pub type RTC_CTR_R = crate::R<bool, RTC_CTR_A>;
impl RTC_CTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_CTR_A {
        match self.bits {
            false => RTC_CTR_A::VALUE1,
            true => RTC_CTR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTC_CTR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTC_CTR_A::VALUE2
    }
}
#[doc = "Write proxy for field `RTC_CTR`"]
pub struct RTC_CTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_CTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTC_CTR_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTC_CTR_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "RTC ATIM0 Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_ATIM0_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<RTC_ATIM0_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_ATIM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTC_ATIM0`"]
pub type RTC_ATIM0_R = crate::R<bool, RTC_ATIM0_A>;
impl RTC_ATIM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_ATIM0_A {
        match self.bits {
            false => RTC_ATIM0_A::VALUE1,
            true => RTC_ATIM0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTC_ATIM0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTC_ATIM0_A::VALUE2
    }
}
#[doc = "Write proxy for field `RTC_ATIM0`"]
pub struct RTC_ATIM0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ATIM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_ATIM0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTC_ATIM0_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTC_ATIM0_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "RTC ATIM1 Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_ATIM1_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<RTC_ATIM1_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_ATIM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTC_ATIM1`"]
pub type RTC_ATIM1_R = crate::R<bool, RTC_ATIM1_A>;
impl RTC_ATIM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_ATIM1_A {
        match self.bits {
            false => RTC_ATIM1_A::VALUE1,
            true => RTC_ATIM1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTC_ATIM1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTC_ATIM1_A::VALUE2
    }
}
#[doc = "Write proxy for field `RTC_ATIM1`"]
pub struct RTC_ATIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ATIM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_ATIM1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTC_ATIM1_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTC_ATIM1_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "RTC TIM0 Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_TIM0_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<RTC_TIM0_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_TIM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTC_TIM0`"]
pub type RTC_TIM0_R = crate::R<bool, RTC_TIM0_A>;
impl RTC_TIM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_TIM0_A {
        match self.bits {
            false => RTC_TIM0_A::VALUE1,
            true => RTC_TIM0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTC_TIM0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTC_TIM0_A::VALUE2
    }
}
#[doc = "Write proxy for field `RTC_TIM0`"]
pub struct RTC_TIM0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TIM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_TIM0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTC_TIM0_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTC_TIM0_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "RTC TIM1 Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_TIM1_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<RTC_TIM1_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_TIM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTC_TIM1`"]
pub type RTC_TIM1_R = crate::R<bool, RTC_TIM1_A>;
impl RTC_TIM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_TIM1_A {
        match self.bits {
            false => RTC_TIM1_A::VALUE1,
            true => RTC_TIM1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTC_TIM1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTC_TIM1_A::VALUE2
    }
}
#[doc = "Write proxy for field `RTC_TIM1`"]
pub struct RTC_TIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TIM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTC_TIM1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RTC_TIM1_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RTC_TIM1_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Retention Memory Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMX_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<RMX_A> for bool {
    #[inline(always)]
    fn from(variant: RMX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RMX`"]
pub type RMX_R = crate::R<bool, RMX_A>;
impl RMX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMX_A {
        match self.bits {
            false => RMX_A::VALUE1,
            true => RMX_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RMX_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RMX_A::VALUE2
    }
}
#[doc = "Write proxy for field `RMX`"]
pub struct RMX_W<'a> {
    w: &'a mut W,
}
impl<'a> RMX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RMX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RMX_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(RMX_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - WDT pre-warning Interrupt Mask"]
    #[inline(always)]
    pub fn prwarn(&self) -> PRWARN_R {
        PRWARN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC Periodic Interrupt Mask"]
    #[inline(always)]
    pub fn pi(&self) -> PI_R {
        PI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTC Alarm Interrupt Mask"]
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn dlrovr(&self) -> DLROVR_R {
        DLROVR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LPACLR Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpaccr(&self) -> LPACCR_R {
        LPACCR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LPACTH0 Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpacth0(&self) -> LPACTH0_R {
        LPACTH0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LPACTH1 Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpacth1(&self) -> LPACTH1_R {
        LPACTH1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LPACST Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpacst(&self) -> LPACST_R {
        LPACST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LPACCLR Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpacclr(&self) -> LPACCLR_R {
        LPACCLR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LPACSET Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpacset(&self) -> LPACSET_R {
        LPACSET_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - HINTST Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn hintst(&self) -> HINTST_R {
        HINTST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - HINTCLR Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn hintclr(&self) -> HINTCLR_R {
        HINTCLR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - HINTSET Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn hintset(&self) -> HINTSET_R {
        HINTSET_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HDCLR Mirror Register Update Mask"]
    #[inline(always)]
    pub fn hdclr(&self) -> HDCLR_R {
        HDCLR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - HDSET Mirror Register Update Mask"]
    #[inline(always)]
    pub fn hdset(&self) -> HDSET_R {
        HDSET_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Mask"]
    #[inline(always)]
    pub fn hdcr(&self) -> HDCR_R {
        HDCR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Mask"]
    #[inline(always)]
    pub fn oscsictrl(&self) -> OSCSICTRL_R {
        OSCSICTRL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Mask"]
    #[inline(always)]
    pub fn osculctrl(&self) -> OSCULCTRL_R {
        OSCULCTRL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_ctr(&self) -> RTC_CTR_R {
        RTC_CTR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_atim0(&self) -> RTC_ATIM0_R {
        RTC_ATIM0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_atim1(&self) -> RTC_ATIM1_R {
        RTC_ATIM1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_tim0(&self) -> RTC_TIM0_R {
        RTC_TIM0_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_tim1(&self) -> RTC_TIM1_R {
        RTC_TIM1_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rmx(&self) -> RMX_R {
        RMX_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT pre-warning Interrupt Mask"]
    #[inline(always)]
    pub fn prwarn(&mut self) -> PRWARN_W {
        PRWARN_W { w: self }
    }
    #[doc = "Bit 1 - RTC Periodic Interrupt Mask"]
    #[inline(always)]
    pub fn pi(&mut self) -> PI_W {
        PI_W { w: self }
    }
    #[doc = "Bit 2 - RTC Alarm Interrupt Mask"]
    #[inline(always)]
    pub fn ai(&mut self) -> AI_W {
        AI_W { w: self }
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn dlrovr(&mut self) -> DLROVR_W {
        DLROVR_W { w: self }
    }
    #[doc = "Bit 6 - LPACLR Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpaccr(&mut self) -> LPACCR_W {
        LPACCR_W { w: self }
    }
    #[doc = "Bit 7 - LPACTH0 Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpacth0(&mut self) -> LPACTH0_W {
        LPACTH0_W { w: self }
    }
    #[doc = "Bit 8 - LPACTH1 Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpacth1(&mut self) -> LPACTH1_W {
        LPACTH1_W { w: self }
    }
    #[doc = "Bit 9 - LPACST Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpacst(&mut self) -> LPACST_W {
        LPACST_W { w: self }
    }
    #[doc = "Bit 10 - LPACCLR Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpacclr(&mut self) -> LPACCLR_W {
        LPACCLR_W { w: self }
    }
    #[doc = "Bit 11 - LPACSET Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpacset(&mut self) -> LPACSET_W {
        LPACSET_W { w: self }
    }
    #[doc = "Bit 12 - HINTST Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn hintst(&mut self) -> HINTST_W {
        HINTST_W { w: self }
    }
    #[doc = "Bit 13 - HINTCLR Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn hintclr(&mut self) -> HINTCLR_W {
        HINTCLR_W { w: self }
    }
    #[doc = "Bit 14 - HINTSET Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn hintset(&mut self) -> HINTSET_W {
        HINTSET_W { w: self }
    }
    #[doc = "Bit 17 - HDCLR Mirror Register Update Mask"]
    #[inline(always)]
    pub fn hdclr(&mut self) -> HDCLR_W {
        HDCLR_W { w: self }
    }
    #[doc = "Bit 18 - HDSET Mirror Register Update Mask"]
    #[inline(always)]
    pub fn hdset(&mut self) -> HDSET_W {
        HDSET_W { w: self }
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Mask"]
    #[inline(always)]
    pub fn hdcr(&mut self) -> HDCR_W {
        HDCR_W { w: self }
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Mask"]
    #[inline(always)]
    pub fn oscsictrl(&mut self) -> OSCSICTRL_W {
        OSCSICTRL_W { w: self }
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Mask"]
    #[inline(always)]
    pub fn osculctrl(&mut self) -> OSCULCTRL_W {
        OSCULCTRL_W { w: self }
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_ctr(&mut self) -> RTC_CTR_W {
        RTC_CTR_W { w: self }
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_atim0(&mut self) -> RTC_ATIM0_W {
        RTC_ATIM0_W { w: self }
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_atim1(&mut self) -> RTC_ATIM1_W {
        RTC_ATIM1_W { w: self }
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_tim0(&mut self) -> RTC_TIM0_W {
        RTC_TIM0_W { w: self }
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_tim1(&mut self) -> RTC_TIM1_W {
        RTC_TIM1_W { w: self }
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rmx(&mut self) -> RMX_W {
        RMX_W { w: self }
    }
}
