#[doc = "Register `SRMSK` reader"]
pub type R = crate::R<SrmskSpec>;
#[doc = "Register `SRMSK` writer"]
pub type W = crate::W<SrmskSpec>;
#[doc = "WDT pre-warning Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prwarn {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Prwarn> for bool {
    #[inline(always)]
    fn from(variant: Prwarn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRWARN` reader - WDT pre-warning Interrupt Mask"]
pub type PrwarnR = crate::BitReader<Prwarn>;
impl PrwarnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prwarn {
        match self.bits {
            false => Prwarn::Value1,
            true => Prwarn::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Prwarn::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Prwarn::Value2
    }
}
#[doc = "Field `PRWARN` writer - WDT pre-warning Interrupt Mask"]
pub type PrwarnW<'a, REG> = crate::BitWriter<'a, REG, Prwarn>;
impl<'a, REG> PrwarnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Prwarn::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Prwarn::Value2)
    }
}
#[doc = "RTC Periodic Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pi {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Pi> for bool {
    #[inline(always)]
    fn from(variant: Pi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PI` reader - RTC Periodic Interrupt Mask"]
pub type PiR = crate::BitReader<Pi>;
impl PiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pi {
        match self.bits {
            false => Pi::Value1,
            true => Pi::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pi::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pi::Value2
    }
}
#[doc = "Field `PI` writer - RTC Periodic Interrupt Mask"]
pub type PiW<'a, REG> = crate::BitWriter<'a, REG, Pi>;
impl<'a, REG> PiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pi::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pi::Value2)
    }
}
#[doc = "RTC Alarm Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ai {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Ai> for bool {
    #[inline(always)]
    fn from(variant: Ai) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AI` reader - RTC Alarm Interrupt Mask"]
pub type AiR = crate::BitReader<Ai>;
impl AiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ai {
        match self.bits {
            false => Ai::Value1,
            true => Ai::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ai::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ai::Value2
    }
}
#[doc = "Field `AI` writer - RTC Alarm Interrupt Mask"]
pub type AiW<'a, REG> = crate::BitWriter<'a, REG, Ai>;
impl<'a, REG> AiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ai::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ai::Value2)
    }
}
#[doc = "DLR Request Overrun Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlrovr {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Dlrovr> for bool {
    #[inline(always)]
    fn from(variant: Dlrovr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLROVR` reader - DLR Request Overrun Interrupt Mask"]
pub type DlrovrR = crate::BitReader<Dlrovr>;
impl DlrovrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dlrovr {
        match self.bits {
            false => Dlrovr::Value1,
            true => Dlrovr::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dlrovr::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dlrovr::Value2
    }
}
#[doc = "Field `DLROVR` writer - DLR Request Overrun Interrupt Mask"]
pub type DlrovrW<'a, REG> = crate::BitWriter<'a, REG, Dlrovr>;
impl<'a, REG> DlrovrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dlrovr::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dlrovr::Value2)
    }
}
#[doc = "LPACLR Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpaccr {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Lpaccr> for bool {
    #[inline(always)]
    fn from(variant: Lpaccr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACCR` reader - LPACLR Mirror Register Update Interrupt Mask"]
pub type LpaccrR = crate::BitReader<Lpaccr>;
impl LpaccrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpaccr {
        match self.bits {
            false => Lpaccr::Value1,
            true => Lpaccr::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lpaccr::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lpaccr::Value2
    }
}
#[doc = "Field `LPACCR` writer - LPACLR Mirror Register Update Interrupt Mask"]
pub type LpaccrW<'a, REG> = crate::BitWriter<'a, REG, Lpaccr>;
impl<'a, REG> LpaccrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpaccr::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpaccr::Value2)
    }
}
#[doc = "LPACTH0 Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpacth0 {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Lpacth0> for bool {
    #[inline(always)]
    fn from(variant: Lpacth0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACTH0` reader - LPACTH0 Mirror Register Update Interrupt Mask"]
pub type Lpacth0R = crate::BitReader<Lpacth0>;
impl Lpacth0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpacth0 {
        match self.bits {
            false => Lpacth0::Value1,
            true => Lpacth0::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lpacth0::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lpacth0::Value2
    }
}
#[doc = "Field `LPACTH0` writer - LPACTH0 Mirror Register Update Interrupt Mask"]
pub type Lpacth0W<'a, REG> = crate::BitWriter<'a, REG, Lpacth0>;
impl<'a, REG> Lpacth0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpacth0::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpacth0::Value2)
    }
}
#[doc = "LPACTH1 Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpacth1 {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Lpacth1> for bool {
    #[inline(always)]
    fn from(variant: Lpacth1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACTH1` reader - LPACTH1 Mirror Register Update Interrupt Mask"]
pub type Lpacth1R = crate::BitReader<Lpacth1>;
impl Lpacth1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpacth1 {
        match self.bits {
            false => Lpacth1::Value1,
            true => Lpacth1::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lpacth1::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lpacth1::Value2
    }
}
#[doc = "Field `LPACTH1` writer - LPACTH1 Mirror Register Update Interrupt Mask"]
pub type Lpacth1W<'a, REG> = crate::BitWriter<'a, REG, Lpacth1>;
impl<'a, REG> Lpacth1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpacth1::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpacth1::Value2)
    }
}
#[doc = "LPACST Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpacst {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Lpacst> for bool {
    #[inline(always)]
    fn from(variant: Lpacst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACST` reader - LPACST Mirror Register Update Interrupt Mask"]
pub type LpacstR = crate::BitReader<Lpacst>;
impl LpacstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpacst {
        match self.bits {
            false => Lpacst::Value1,
            true => Lpacst::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lpacst::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lpacst::Value2
    }
}
#[doc = "Field `LPACST` writer - LPACST Mirror Register Update Interrupt Mask"]
pub type LpacstW<'a, REG> = crate::BitWriter<'a, REG, Lpacst>;
impl<'a, REG> LpacstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpacst::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpacst::Value2)
    }
}
#[doc = "LPACCLR Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpacclr {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Lpacclr> for bool {
    #[inline(always)]
    fn from(variant: Lpacclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACCLR` reader - LPACCLR Mirror Register Update Interrupt Mask"]
pub type LpacclrR = crate::BitReader<Lpacclr>;
impl LpacclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpacclr {
        match self.bits {
            false => Lpacclr::Value1,
            true => Lpacclr::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lpacclr::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lpacclr::Value2
    }
}
#[doc = "Field `LPACCLR` writer - LPACCLR Mirror Register Update Interrupt Mask"]
pub type LpacclrW<'a, REG> = crate::BitWriter<'a, REG, Lpacclr>;
impl<'a, REG> LpacclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpacclr::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpacclr::Value2)
    }
}
#[doc = "LPACSET Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpacset {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Lpacset> for bool {
    #[inline(always)]
    fn from(variant: Lpacset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPACSET` reader - LPACSET Mirror Register Update Interrupt Mask"]
pub type LpacsetR = crate::BitReader<Lpacset>;
impl LpacsetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpacset {
        match self.bits {
            false => Lpacset::Value1,
            true => Lpacset::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Lpacset::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Lpacset::Value2
    }
}
#[doc = "Field `LPACSET` writer - LPACSET Mirror Register Update Interrupt Mask"]
pub type LpacsetW<'a, REG> = crate::BitWriter<'a, REG, Lpacset>;
impl<'a, REG> LpacsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpacset::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpacset::Value2)
    }
}
#[doc = "HINTST Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hintst {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Hintst> for bool {
    #[inline(always)]
    fn from(variant: Hintst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HINTST` reader - HINTST Mirror Register Update Interrupt Mask"]
pub type HintstR = crate::BitReader<Hintst>;
impl HintstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hintst {
        match self.bits {
            false => Hintst::Value1,
            true => Hintst::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hintst::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hintst::Value2
    }
}
#[doc = "Field `HINTST` writer - HINTST Mirror Register Update Interrupt Mask"]
pub type HintstW<'a, REG> = crate::BitWriter<'a, REG, Hintst>;
impl<'a, REG> HintstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hintst::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hintst::Value2)
    }
}
#[doc = "HINTCLR Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hintclr {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Hintclr> for bool {
    #[inline(always)]
    fn from(variant: Hintclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HINTCLR` reader - HINTCLR Mirror Register Update Interrupt Mask"]
pub type HintclrR = crate::BitReader<Hintclr>;
impl HintclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hintclr {
        match self.bits {
            false => Hintclr::Value1,
            true => Hintclr::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hintclr::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hintclr::Value2
    }
}
#[doc = "Field `HINTCLR` writer - HINTCLR Mirror Register Update Interrupt Mask"]
pub type HintclrW<'a, REG> = crate::BitWriter<'a, REG, Hintclr>;
impl<'a, REG> HintclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hintclr::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hintclr::Value2)
    }
}
#[doc = "HINTSET Mirror Register Update Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hintset {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Hintset> for bool {
    #[inline(always)]
    fn from(variant: Hintset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HINTSET` reader - HINTSET Mirror Register Update Interrupt Mask"]
pub type HintsetR = crate::BitReader<Hintset>;
impl HintsetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hintset {
        match self.bits {
            false => Hintset::Value1,
            true => Hintset::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hintset::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hintset::Value2
    }
}
#[doc = "Field `HINTSET` writer - HINTSET Mirror Register Update Interrupt Mask"]
pub type HintsetW<'a, REG> = crate::BitWriter<'a, REG, Hintset>;
impl<'a, REG> HintsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hintset::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hintset::Value2)
    }
}
#[doc = "HDCLR Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdclr {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Hdclr> for bool {
    #[inline(always)]
    fn from(variant: Hdclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCLR` reader - HDCLR Mirror Register Update Mask"]
pub type HdclrR = crate::BitReader<Hdclr>;
impl HdclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdclr {
        match self.bits {
            false => Hdclr::Value1,
            true => Hdclr::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hdclr::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hdclr::Value2
    }
}
#[doc = "Field `HDCLR` writer - HDCLR Mirror Register Update Mask"]
pub type HdclrW<'a, REG> = crate::BitWriter<'a, REG, Hdclr>;
impl<'a, REG> HdclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdclr::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hdclr::Value2)
    }
}
#[doc = "HDSET Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdset {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Hdset> for bool {
    #[inline(always)]
    fn from(variant: Hdset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDSET` reader - HDSET Mirror Register Update Mask"]
pub type HdsetR = crate::BitReader<Hdset>;
impl HdsetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdset {
        match self.bits {
            false => Hdset::Value1,
            true => Hdset::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hdset::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hdset::Value2
    }
}
#[doc = "Field `HDSET` writer - HDSET Mirror Register Update Mask"]
pub type HdsetW<'a, REG> = crate::BitWriter<'a, REG, Hdset>;
impl<'a, REG> HdsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdset::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hdset::Value2)
    }
}
#[doc = "HDCR Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdcr {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Hdcr> for bool {
    #[inline(always)]
    fn from(variant: Hdcr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCR` reader - HDCR Mirror Register Update Mask"]
pub type HdcrR = crate::BitReader<Hdcr>;
impl HdcrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdcr {
        match self.bits {
            false => Hdcr::Value1,
            true => Hdcr::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hdcr::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hdcr::Value2
    }
}
#[doc = "Field `HDCR` writer - HDCR Mirror Register Update Mask"]
pub type HdcrW<'a, REG> = crate::BitWriter<'a, REG, Hdcr>;
impl<'a, REG> HdcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcr::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hdcr::Value2)
    }
}
#[doc = "OSCSICTRL Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oscsictrl {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Oscsictrl> for bool {
    #[inline(always)]
    fn from(variant: Oscsictrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCSICTRL` reader - OSCSICTRL Mirror Register Update Mask"]
pub type OscsictrlR = crate::BitReader<Oscsictrl>;
impl OscsictrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oscsictrl {
        match self.bits {
            false => Oscsictrl::Value1,
            true => Oscsictrl::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Oscsictrl::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Oscsictrl::Value2
    }
}
#[doc = "Field `OSCSICTRL` writer - OSCSICTRL Mirror Register Update Mask"]
pub type OscsictrlW<'a, REG> = crate::BitWriter<'a, REG, Oscsictrl>;
impl<'a, REG> OscsictrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Oscsictrl::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Oscsictrl::Value2)
    }
}
#[doc = "OSCULCTRL Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Osculctrl {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Osculctrl> for bool {
    #[inline(always)]
    fn from(variant: Osculctrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCULCTRL` reader - OSCULCTRL Mirror Register Update Mask"]
pub type OsculctrlR = crate::BitReader<Osculctrl>;
impl OsculctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Osculctrl {
        match self.bits {
            false => Osculctrl::Value1,
            true => Osculctrl::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Osculctrl::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Osculctrl::Value2
    }
}
#[doc = "Field `OSCULCTRL` writer - OSCULCTRL Mirror Register Update Mask"]
pub type OsculctrlW<'a, REG> = crate::BitWriter<'a, REG, Osculctrl>;
impl<'a, REG> OsculctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Osculctrl::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Osculctrl::Value2)
    }
}
#[doc = "RTC CTR Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcCtr {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<RtcCtr> for bool {
    #[inline(always)]
    fn from(variant: RtcCtr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_CTR` reader - RTC CTR Mirror Register Update Mask"]
pub type RtcCtrR = crate::BitReader<RtcCtr>;
impl RtcCtrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcCtr {
        match self.bits {
            false => RtcCtr::Value1,
            true => RtcCtr::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RtcCtr::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RtcCtr::Value2
    }
}
#[doc = "Field `RTC_CTR` writer - RTC CTR Mirror Register Update Mask"]
pub type RtcCtrW<'a, REG> = crate::BitWriter<'a, REG, RtcCtr>;
impl<'a, REG> RtcCtrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCtr::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RtcCtr::Value2)
    }
}
#[doc = "RTC ATIM0 Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcAtim0 {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<RtcAtim0> for bool {
    #[inline(always)]
    fn from(variant: RtcAtim0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_ATIM0` reader - RTC ATIM0 Mirror Register Update Mask"]
pub type RtcAtim0R = crate::BitReader<RtcAtim0>;
impl RtcAtim0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcAtim0 {
        match self.bits {
            false => RtcAtim0::Value1,
            true => RtcAtim0::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RtcAtim0::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RtcAtim0::Value2
    }
}
#[doc = "Field `RTC_ATIM0` writer - RTC ATIM0 Mirror Register Update Mask"]
pub type RtcAtim0W<'a, REG> = crate::BitWriter<'a, REG, RtcAtim0>;
impl<'a, REG> RtcAtim0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcAtim0::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RtcAtim0::Value2)
    }
}
#[doc = "RTC ATIM1 Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcAtim1 {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<RtcAtim1> for bool {
    #[inline(always)]
    fn from(variant: RtcAtim1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_ATIM1` reader - RTC ATIM1 Mirror Register Update Mask"]
pub type RtcAtim1R = crate::BitReader<RtcAtim1>;
impl RtcAtim1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcAtim1 {
        match self.bits {
            false => RtcAtim1::Value1,
            true => RtcAtim1::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RtcAtim1::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RtcAtim1::Value2
    }
}
#[doc = "Field `RTC_ATIM1` writer - RTC ATIM1 Mirror Register Update Mask"]
pub type RtcAtim1W<'a, REG> = crate::BitWriter<'a, REG, RtcAtim1>;
impl<'a, REG> RtcAtim1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcAtim1::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RtcAtim1::Value2)
    }
}
#[doc = "RTC TIM0 Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcTim0 {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<RtcTim0> for bool {
    #[inline(always)]
    fn from(variant: RtcTim0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_TIM0` reader - RTC TIM0 Mirror Register Update Mask"]
pub type RtcTim0R = crate::BitReader<RtcTim0>;
impl RtcTim0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcTim0 {
        match self.bits {
            false => RtcTim0::Value1,
            true => RtcTim0::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RtcTim0::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RtcTim0::Value2
    }
}
#[doc = "Field `RTC_TIM0` writer - RTC TIM0 Mirror Register Update Mask"]
pub type RtcTim0W<'a, REG> = crate::BitWriter<'a, REG, RtcTim0>;
impl<'a, REG> RtcTim0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcTim0::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RtcTim0::Value2)
    }
}
#[doc = "RTC TIM1 Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcTim1 {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<RtcTim1> for bool {
    #[inline(always)]
    fn from(variant: RtcTim1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC_TIM1` reader - RTC TIM1 Mirror Register Update Mask"]
pub type RtcTim1R = crate::BitReader<RtcTim1>;
impl RtcTim1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcTim1 {
        match self.bits {
            false => RtcTim1::Value1,
            true => RtcTim1::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RtcTim1::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RtcTim1::Value2
    }
}
#[doc = "Field `RTC_TIM1` writer - RTC TIM1 Mirror Register Update Mask"]
pub type RtcTim1W<'a, REG> = crate::BitWriter<'a, REG, RtcTim1>;
impl<'a, REG> RtcTim1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RtcTim1::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RtcTim1::Value2)
    }
}
#[doc = "Retention Memory Mirror Register Update Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rmx {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Rmx> for bool {
    #[inline(always)]
    fn from(variant: Rmx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMX` reader - Retention Memory Mirror Register Update Mask"]
pub type RmxR = crate::BitReader<Rmx>;
impl RmxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rmx {
        match self.bits {
            false => Rmx::Value1,
            true => Rmx::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rmx::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rmx::Value2
    }
}
#[doc = "Field `RMX` writer - Retention Memory Mirror Register Update Mask"]
pub type RmxW<'a, REG> = crate::BitWriter<'a, REG, Rmx>;
impl<'a, REG> RmxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rmx::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rmx::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - WDT pre-warning Interrupt Mask"]
    #[inline(always)]
    pub fn prwarn(&self) -> PrwarnR {
        PrwarnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Periodic Interrupt Mask"]
    #[inline(always)]
    pub fn pi(&self) -> PiR {
        PiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Alarm Interrupt Mask"]
    #[inline(always)]
    pub fn ai(&self) -> AiR {
        AiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn dlrovr(&self) -> DlrovrR {
        DlrovrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - LPACLR Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpaccr(&self) -> LpaccrR {
        LpaccrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LPACTH0 Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpacth0(&self) -> Lpacth0R {
        Lpacth0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LPACTH1 Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpacth1(&self) -> Lpacth1R {
        Lpacth1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LPACST Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpacst(&self) -> LpacstR {
        LpacstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPACCLR Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpacclr(&self) -> LpacclrR {
        LpacclrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LPACSET Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn lpacset(&self) -> LpacsetR {
        LpacsetR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - HINTST Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn hintst(&self) -> HintstR {
        HintstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HINTCLR Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn hintclr(&self) -> HintclrR {
        HintclrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - HINTSET Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    pub fn hintset(&self) -> HintsetR {
        HintsetR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - HDCLR Mirror Register Update Mask"]
    #[inline(always)]
    pub fn hdclr(&self) -> HdclrR {
        HdclrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HDSET Mirror Register Update Mask"]
    #[inline(always)]
    pub fn hdset(&self) -> HdsetR {
        HdsetR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Mask"]
    #[inline(always)]
    pub fn hdcr(&self) -> HdcrR {
        HdcrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Mask"]
    #[inline(always)]
    pub fn oscsictrl(&self) -> OscsictrlR {
        OscsictrlR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Mask"]
    #[inline(always)]
    pub fn osculctrl(&self) -> OsculctrlR {
        OsculctrlR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_ctr(&self) -> RtcCtrR {
        RtcCtrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_atim0(&self) -> RtcAtim0R {
        RtcAtim0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_atim1(&self) -> RtcAtim1R {
        RtcAtim1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_tim0(&self) -> RtcTim0R {
        RtcTim0R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rtc_tim1(&self) -> RtcTim1R {
        RtcTim1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Mask"]
    #[inline(always)]
    pub fn rmx(&self) -> RmxR {
        RmxR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT pre-warning Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn prwarn(&mut self) -> PrwarnW<SrmskSpec> {
        PrwarnW::new(self, 0)
    }
    #[doc = "Bit 1 - RTC Periodic Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn pi(&mut self) -> PiW<SrmskSpec> {
        PiW::new(self, 1)
    }
    #[doc = "Bit 2 - RTC Alarm Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AiW<SrmskSpec> {
        AiW::new(self, 2)
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn dlrovr(&mut self) -> DlrovrW<SrmskSpec> {
        DlrovrW::new(self, 3)
    }
    #[doc = "Bit 6 - LPACLR Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn lpaccr(&mut self) -> LpaccrW<SrmskSpec> {
        LpaccrW::new(self, 6)
    }
    #[doc = "Bit 7 - LPACTH0 Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn lpacth0(&mut self) -> Lpacth0W<SrmskSpec> {
        Lpacth0W::new(self, 7)
    }
    #[doc = "Bit 8 - LPACTH1 Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn lpacth1(&mut self) -> Lpacth1W<SrmskSpec> {
        Lpacth1W::new(self, 8)
    }
    #[doc = "Bit 9 - LPACST Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn lpacst(&mut self) -> LpacstW<SrmskSpec> {
        LpacstW::new(self, 9)
    }
    #[doc = "Bit 10 - LPACCLR Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn lpacclr(&mut self) -> LpacclrW<SrmskSpec> {
        LpacclrW::new(self, 10)
    }
    #[doc = "Bit 11 - LPACSET Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn lpacset(&mut self) -> LpacsetW<SrmskSpec> {
        LpacsetW::new(self, 11)
    }
    #[doc = "Bit 12 - HINTST Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hintst(&mut self) -> HintstW<SrmskSpec> {
        HintstW::new(self, 12)
    }
    #[doc = "Bit 13 - HINTCLR Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hintclr(&mut self) -> HintclrW<SrmskSpec> {
        HintclrW::new(self, 13)
    }
    #[doc = "Bit 14 - HINTSET Mirror Register Update Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hintset(&mut self) -> HintsetW<SrmskSpec> {
        HintsetW::new(self, 14)
    }
    #[doc = "Bit 17 - HDCLR Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hdclr(&mut self) -> HdclrW<SrmskSpec> {
        HdclrW::new(self, 17)
    }
    #[doc = "Bit 18 - HDSET Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hdset(&mut self) -> HdsetW<SrmskSpec> {
        HdsetW::new(self, 18)
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hdcr(&mut self) -> HdcrW<SrmskSpec> {
        HdcrW::new(self, 19)
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn oscsictrl(&mut self) -> OscsictrlW<SrmskSpec> {
        OscsictrlW::new(self, 21)
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn osculctrl(&mut self) -> OsculctrlW<SrmskSpec> {
        OsculctrlW::new(self, 23)
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_ctr(&mut self) -> RtcCtrW<SrmskSpec> {
        RtcCtrW::new(self, 24)
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_atim0(&mut self) -> RtcAtim0W<SrmskSpec> {
        RtcAtim0W::new(self, 25)
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_atim1(&mut self) -> RtcAtim1W<SrmskSpec> {
        RtcAtim1W::new(self, 26)
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_tim0(&mut self) -> RtcTim0W<SrmskSpec> {
        RtcTim0W::new(self, 27)
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_tim1(&mut self) -> RtcTim1W<SrmskSpec> {
        RtcTim1W::new(self, 28)
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rmx(&mut self) -> RmxW<SrmskSpec> {
        RmxW::new(self, 29)
    }
}
#[doc = "SCU Service Request Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrmskSpec;
impl crate::RegisterSpec for SrmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srmsk::R`](R) reader structure"]
impl crate::Readable for SrmskSpec {}
#[doc = "`write(|w| ..)` method takes [`srmsk::W`](W) writer structure"]
impl crate::Writable for SrmskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRMSK to value 0"]
impl crate::Resettable for SrmskSpec {
    const RESET_VALUE: u32 = 0;
}
