#[doc = "Register `NIPR` reader"]
pub struct R(crate::R<NIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NIPR` writer"]
pub struct W(crate::W<NIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<NIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NIPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Alert Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ALINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "7: Interrupt output line INT_O7 is selected."]
    VALUE3 = 7,
}
impl From<ALINP_A> for u8 {
    #[inline(always)]
    fn from(variant: ALINP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ALINP` reader - Alert Interrupt Node Pointer"]
pub struct ALINP_R(crate::FieldReader<u8, ALINP_A>);
impl ALINP_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALINP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALINP_A> {
        match self.bits {
            0 => Some(ALINP_A::VALUE1),
            1 => Some(ALINP_A::VALUE2),
            7 => Some(ALINP_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ALINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ALINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == ALINP_A::VALUE3
    }
}
impl core::ops::Deref for ALINP_R {
    type Target = crate::FieldReader<u8, ALINP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALINP` writer - Alert Interrupt Node Pointer"]
pub struct ALINP_W<'a> {
    w: &'a mut W,
}
impl<'a> ALINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALINP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O7 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(ALINP_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Last Error Code Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LECINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "7: Interrupt output line INT_O7 is selected."]
    VALUE3 = 7,
}
impl From<LECINP_A> for u8 {
    #[inline(always)]
    fn from(variant: LECINP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LECINP` reader - Last Error Code Interrupt Node Pointer"]
pub struct LECINP_R(crate::FieldReader<u8, LECINP_A>);
impl LECINP_R {
    pub(crate) fn new(bits: u8) -> Self {
        LECINP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LECINP_A> {
        match self.bits {
            0 => Some(LECINP_A::VALUE1),
            1 => Some(LECINP_A::VALUE2),
            7 => Some(LECINP_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == LECINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LECINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == LECINP_A::VALUE3
    }
}
impl core::ops::Deref for LECINP_R {
    type Target = crate::FieldReader<u8, LECINP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LECINP` writer - Last Error Code Interrupt Node Pointer"]
pub struct LECINP_W<'a> {
    w: &'a mut W,
}
impl<'a> LECINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LECINP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LECINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LECINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O7 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(LECINP_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Transfer OK Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "7: Interrupt output line INT_O7 is selected."]
    VALUE3 = 7,
}
impl From<TRINP_A> for u8 {
    #[inline(always)]
    fn from(variant: TRINP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRINP` reader - Transfer OK Interrupt Node Pointer"]
pub struct TRINP_R(crate::FieldReader<u8, TRINP_A>);
impl TRINP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRINP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRINP_A> {
        match self.bits {
            0 => Some(TRINP_A::VALUE1),
            1 => Some(TRINP_A::VALUE2),
            7 => Some(TRINP_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TRINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TRINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == TRINP_A::VALUE3
    }
}
impl core::ops::Deref for TRINP_R {
    type Target = crate::FieldReader<u8, TRINP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRINP` writer - Transfer OK Interrupt Node Pointer"]
pub struct TRINP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRINP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O7 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TRINP_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Frame Counter Interrupt Node Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFCINP_A {
    #[doc = "0: Interrupt output line INT_O0 is selected."]
    VALUE1 = 0,
    #[doc = "1: Interrupt output line INT_O1 is selected."]
    VALUE2 = 1,
    #[doc = "7: Interrupt output line INT_O7 is selected."]
    VALUE3 = 7,
}
impl From<CFCINP_A> for u8 {
    #[inline(always)]
    fn from(variant: CFCINP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFCINP` reader - Frame Counter Interrupt Node Pointer"]
pub struct CFCINP_R(crate::FieldReader<u8, CFCINP_A>);
impl CFCINP_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFCINP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFCINP_A> {
        match self.bits {
            0 => Some(CFCINP_A::VALUE1),
            1 => Some(CFCINP_A::VALUE2),
            7 => Some(CFCINP_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CFCINP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CFCINP_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == CFCINP_A::VALUE3
    }
}
impl core::ops::Deref for CFCINP_R {
    type Target = crate::FieldReader<u8, CFCINP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFCINP` writer - Frame Counter Interrupt Node Pointer"]
pub struct CFCINP_W<'a> {
    w: &'a mut W,
}
impl<'a> CFCINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFCINP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt output line INT_O0 is selected."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CFCINP_A::VALUE1)
    }
    #[doc = "Interrupt output line INT_O1 is selected."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CFCINP_A::VALUE2)
    }
    #[doc = "Interrupt output line INT_O7 is selected."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CFCINP_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Alert Interrupt Node Pointer"]
    #[inline(always)]
    pub fn alinp(&self) -> ALINP_R {
        ALINP_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Last Error Code Interrupt Node Pointer"]
    #[inline(always)]
    pub fn lecinp(&self) -> LECINP_R {
        LECINP_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Transfer OK Interrupt Node Pointer"]
    #[inline(always)]
    pub fn trinp(&self) -> TRINP_R {
        TRINP_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Frame Counter Interrupt Node Pointer"]
    #[inline(always)]
    pub fn cfcinp(&self) -> CFCINP_R {
        CFCINP_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Alert Interrupt Node Pointer"]
    #[inline(always)]
    pub fn alinp(&mut self) -> ALINP_W {
        ALINP_W { w: self }
    }
    #[doc = "Bits 4:6 - Last Error Code Interrupt Node Pointer"]
    #[inline(always)]
    pub fn lecinp(&mut self) -> LECINP_W {
        LECINP_W { w: self }
    }
    #[doc = "Bits 8:10 - Transfer OK Interrupt Node Pointer"]
    #[inline(always)]
    pub fn trinp(&mut self) -> TRINP_W {
        TRINP_W { w: self }
    }
    #[doc = "Bits 12:14 - Frame Counter Interrupt Node Pointer"]
    #[inline(always)]
    pub fn cfcinp(&mut self) -> CFCINP_W {
        CFCINP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Node Interrupt Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nipr](index.html) module"]
pub struct NIPR_SPEC;
impl crate::RegisterSpec for NIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nipr::R](R) reader structure"]
impl crate::Readable for NIPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nipr::W](W) writer structure"]
impl crate::Writable for NIPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NIPR to value 0"]
impl crate::Resettable for NIPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
