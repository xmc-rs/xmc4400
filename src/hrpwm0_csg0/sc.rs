#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SC {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `PSRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSRMR {
    #[doc = "External start trigger is ignored"]
    VALUE1,
    #[doc = "Start prescaler"]
    VALUE2,
    #[doc = "Clear prescaler"]
    VALUE3,
    #[doc = "Clear & Start prescaler"]
    VALUE4,
}
impl PSRMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSRMR::VALUE1 => 0,
            PSRMR::VALUE2 => 1,
            PSRMR::VALUE3 => 2,
            PSRMR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSRMR {
        match value {
            0 => PSRMR::VALUE1,
            1 => PSRMR::VALUE2,
            2 => PSRMR::VALUE3,
            3 => PSRMR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PSRMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PSRMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PSRMR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PSRMR::VALUE4
    }
}
#[doc = "Possible values of the field `PSTM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSTMR {
    #[doc = "External stop trigger is ignored"]
    VALUE1,
    #[doc = "Stop prescaler"]
    VALUE2,
    #[doc = "Clear prescaler"]
    VALUE3,
    #[doc = "Clear & Stop prescaler"]
    VALUE4,
}
impl PSTMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSTMR::VALUE1 => 0,
            PSTMR::VALUE2 => 1,
            PSTMR::VALUE3 => 2,
            PSTMR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSTMR {
        match value {
            0 => PSTMR::VALUE1,
            1 => PSTMR::VALUE2,
            2 => PSTMR::VALUE3,
            3 => PSTMR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PSTMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PSTMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PSTMR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PSTMR::VALUE4
    }
}
#[doc = "Possible values of the field `FPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPDR {
    #[doc = "Division by 4 enabled"]
    VALUE1,
    #[doc = "Division by 4 disabled"]
    VALUE2,
}
impl FPDR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FPDR::VALUE1 => false,
            FPDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FPDR {
        match value {
            false => FPDR::VALUE1,
            true => FPDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FPDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FPDR::VALUE2
    }
}
#[doc = "Possible values of the field `PSV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSVR {
    #[doc = "division by 1"]
    VALUE1,
    #[doc = "division by 2"]
    VALUE2,
    #[doc = "division by 4"]
    VALUE3,
    #[doc = "division by 8"]
    VALUE4,
}
impl PSVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSVR::VALUE1 => 0,
            PSVR::VALUE2 => 1,
            PSVR::VALUE3 => 2,
            PSVR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSVR {
        match value {
            0 => PSVR::VALUE1,
            1 => PSVR::VALUE2,
            2 => PSVR::VALUE3,
            3 => PSVR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PSVR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PSVR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PSVR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == PSVR::VALUE4
    }
}
#[doc = "Possible values of the field `SCM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCMR {
    #[doc = "Slope generation disabled. Used when the switch between the two reference values, CSGyDSV1This register contains the actual value used for the DSV1 reference. and CSGyDSV2This register contains the actual value used for the DSV2 reference. is done via external signal."]
    VALUE1,
    #[doc = "Decrementing slope generation."]
    VALUE2,
    #[doc = "Incrementing slope generation."]
    VALUE3,
    #[doc = "Triangular slope generation."]
    VALUE4,
}
impl SCMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCMR::VALUE1 => 0,
            SCMR::VALUE2 => 1,
            SCMR::VALUE3 => 2,
            SCMR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCMR {
        match value {
            0 => SCMR::VALUE1,
            1 => SCMR::VALUE2,
            2 => SCMR::VALUE3,
            3 => SCMR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SCMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SCMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SCMR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == SCMR::VALUE4
    }
}
#[doc = "Possible values of the field `SSRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSRMR {
    #[doc = "External start trigger is ignored"]
    VALUE1,
    #[doc = "Start/restart slope generation"]
    VALUE2,
    #[doc = "Resumes slope"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SSRMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SSRMR::VALUE1 => 0,
            SSRMR::VALUE2 => 1,
            SSRMR::VALUE3 => 2,
            SSRMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SSRMR {
        match value {
            0 => SSRMR::VALUE1,
            1 => SSRMR::VALUE2,
            2 => SSRMR::VALUE3,
            i => SSRMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SSRMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SSRMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SSRMR::VALUE3
    }
}
#[doc = "Possible values of the field `SSTM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSTMR {
    #[doc = "External stop trigger is ignored"]
    VALUE1,
    #[doc = "Stops/Halts the slope generation"]
    VALUE2,
    #[doc = "Used in hybrid mode. It freezes the slope generation and feeds constantly the value programmed in CSGyDSV2This register contains the actual value used for the DSV2 reference. to the DAC."]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SSTMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SSTMR::VALUE1 => 0,
            SSTMR::VALUE2 => 1,
            SSTMR::VALUE3 => 2,
            SSTMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SSTMR {
        match value {
            0 => SSTMR::VALUE1,
            1 => SSTMR::VALUE2,
            2 => SSTMR::VALUE3,
            i => SSTMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SSTMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SSTMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SSTMR::VALUE3
    }
}
#[doc = "Possible values of the field `SVSC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVSCR {
    #[doc = "Only CSGyDSV1This register contains the actual value used for the DSV1 reference. value is used for the slope generation: if slope is incrementing, CSGyDSV1This register contains the actual value used for the DSV1 reference. is the bottom reference value from where the ramp starts; if decrementing, then CSGyDSV1This register contains the actual value used for the DSV1 reference. is the upper reference value from where the ramp starts."]
    VALUE1,
    #[doc = "The two reference values are being used: CSGyDSV1This register contains the actual value used for the DSV1 reference. is the low or high reference value from where the ramp starts (incrementing or decrementing respectively); CSGyDSV2This register contains the actual value used for the DSV2 reference. is used as a static value (this value is constantly fed to the DAC after a stop trigger as been detected)."]
    VALUE2,
    #[doc = "The two reference values are used: CSGyDSV1This register contains the actual value used for the DSV1 reference. is the low or high reference value from where the slope starts (incrementing or decrementing respectively); CSGyDSV2This register contains the actual value used for the DSV2 reference. is used as an internal re start condition for the slope."]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SVSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SVSCR::VALUE1 => 0,
            SVSCR::VALUE2 => 1,
            SVSCR::VALUE3 => 2,
            SVSCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SVSCR {
        match value {
            0 => SVSCR::VALUE1,
            1 => SVSCR::VALUE2,
            2 => SVSCR::VALUE3,
            i => SVSCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SVSCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SVSCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SVSCR::VALUE3
    }
}
#[doc = "Possible values of the field `SWSM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWSMR {
    #[doc = "CSGyDSV2This register contains the actual value used for the DSV2 reference. is fed to the DAC and initial conversion trigger is generated."]
    VALUE1,
    #[doc = "CSGyDSV1This register contains the actual value used for the DSV1 reference. is fed to the DAC and initial conversion trigger is generated."]
    VALUE2,
    #[doc = "CSGyDSV2This register contains the actual value used for the DSV2 reference. is fed to the DAC but initial conversion trigger is not generated."]
    VALUE3,
    #[doc = "CSGyDSV1This register contains the actual value used for the DSV1 reference. is fed to the DAC but initial conversion trigger is not generated."]
    VALUE4,
}
impl SWSMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SWSMR::VALUE1 => 0,
            SWSMR::VALUE2 => 1,
            SWSMR::VALUE3 => 2,
            SWSMR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SWSMR {
        match value {
            0 => SWSMR::VALUE1,
            1 => SWSMR::VALUE2,
            2 => SWSMR::VALUE3,
            3 => SWSMR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SWSMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SWSMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SWSMR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == SWSMR::VALUE4
    }
}
#[doc = "Possible values of the field `GCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCFGR {
    #[doc = "Each slope step has an increment/decrement of 1"]
    VALUE1,
    #[doc = "Each slope step has an increment/decrement of 2"]
    VALUE2,
    #[doc = "Each slope step has an increment/decrement of 4"]
    VALUE3,
    #[doc = "Each slope step has an increment/decrement of 8"]
    VALUE4,
}
impl GCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GCFGR::VALUE1 => 0,
            GCFGR::VALUE2 => 1,
            GCFGR::VALUE3 => 2,
            GCFGR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GCFGR {
        match value {
            0 => GCFGR::VALUE1,
            1 => GCFGR::VALUE2,
            2 => GCFGR::VALUE3,
            3 => GCFGR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == GCFGR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == GCFGR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == GCFGR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == GCFGR::VALUE4
    }
}
#[doc = r" Value of the field"]
pub struct ISTR {
    bits: bool,
}
impl ISTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `PSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSER {
    #[doc = "Pulse swallow disabled"]
    VALUE1,
    #[doc = "Pulse swallow enabled"]
    VALUE2,
}
impl PSER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PSER::VALUE1 => false,
            PSER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSER {
        match value {
            false => PSER::VALUE1,
            true => PSER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PSER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PSER::VALUE2
    }
}
#[doc = "Possible values of the field `PSWM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSWMR {
    #[doc = "16 clock cycle window"]
    VALUE1,
    #[doc = "32 clock cycle window"]
    VALUE2,
    #[doc = "64 clock cycle window"]
    VALUE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PSWMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSWMR::VALUE1 => 0,
            PSWMR::VALUE2 => 1,
            PSWMR::VALUE3 => 2,
            PSWMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSWMR {
        match value {
            0 => PSWMR::VALUE1,
            1 => PSWMR::VALUE2,
            2 => PSWMR::VALUE3,
            i => PSWMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PSWMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PSWMR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == PSWMR::VALUE3
    }
}
#[doc = "Values that can be written to the field `PSRM`"]
pub enum PSRMW {
    #[doc = "External start trigger is ignored"]
    VALUE1,
    #[doc = "Start prescaler"]
    VALUE2,
    #[doc = "Clear prescaler"]
    VALUE3,
    #[doc = "Clear & Start prescaler"]
    VALUE4,
}
impl PSRMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSRMW::VALUE1 => 0,
            PSRMW::VALUE2 => 1,
            PSRMW::VALUE3 => 2,
            PSRMW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSRMW<'a> {
    w: &'a mut W,
}
impl<'a> _PSRMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSRMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External start trigger is ignored"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSRMW::VALUE1)
    }
    #[doc = "Start prescaler"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSRMW::VALUE2)
    }
    #[doc = "Clear prescaler"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PSRMW::VALUE3)
    }
    #[doc = "Clear & Start prescaler"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PSRMW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSTM`"]
pub enum PSTMW {
    #[doc = "External stop trigger is ignored"]
    VALUE1,
    #[doc = "Stop prescaler"]
    VALUE2,
    #[doc = "Clear prescaler"]
    VALUE3,
    #[doc = "Clear & Stop prescaler"]
    VALUE4,
}
impl PSTMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSTMW::VALUE1 => 0,
            PSTMW::VALUE2 => 1,
            PSTMW::VALUE3 => 2,
            PSTMW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSTMW<'a> {
    w: &'a mut W,
}
impl<'a> _PSTMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSTMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External stop trigger is ignored"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSTMW::VALUE1)
    }
    #[doc = "Stop prescaler"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSTMW::VALUE2)
    }
    #[doc = "Clear prescaler"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PSTMW::VALUE3)
    }
    #[doc = "Clear & Stop prescaler"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PSTMW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FPD`"]
pub enum FPDW {
    #[doc = "Division by 4 enabled"]
    VALUE1,
    #[doc = "Division by 4 disabled"]
    VALUE2,
}
impl FPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FPDW::VALUE1 => false,
            FPDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FPDW<'a> {
    w: &'a mut W,
}
impl<'a> _FPDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FPDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Division by 4 enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FPDW::VALUE1)
    }
    #[doc = "Division by 4 disabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FPDW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSV`"]
pub enum PSVW {
    #[doc = "division by 1"]
    VALUE1,
    #[doc = "division by 2"]
    VALUE2,
    #[doc = "division by 4"]
    VALUE3,
    #[doc = "division by 8"]
    VALUE4,
}
impl PSVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSVW::VALUE1 => 0,
            PSVW::VALUE2 => 1,
            PSVW::VALUE3 => 2,
            PSVW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSVW<'a> {
    w: &'a mut W,
}
impl<'a> _PSVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "division by 1"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSVW::VALUE1)
    }
    #[doc = "division by 2"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSVW::VALUE2)
    }
    #[doc = "division by 4"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PSVW::VALUE3)
    }
    #[doc = "division by 8"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(PSVW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCM`"]
pub enum SCMW {
    #[doc = "Slope generation disabled. Used when the switch between the two reference values, CSGyDSV1This register contains the actual value used for the DSV1 reference. and CSGyDSV2This register contains the actual value used for the DSV2 reference. is done via external signal."]
    VALUE1,
    #[doc = "Decrementing slope generation."]
    VALUE2,
    #[doc = "Incrementing slope generation."]
    VALUE3,
    #[doc = "Triangular slope generation."]
    VALUE4,
}
impl SCMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SCMW::VALUE1 => 0,
            SCMW::VALUE2 => 1,
            SCMW::VALUE3 => 2,
            SCMW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCMW<'a> {
    w: &'a mut W,
}
impl<'a> _SCMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Slope generation disabled. Used when the switch between the two reference values, CSGyDSV1This register contains the actual value used for the DSV1 reference. and CSGyDSV2This register contains the actual value used for the DSV2 reference. is done via external signal."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCMW::VALUE1)
    }
    #[doc = "Decrementing slope generation."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCMW::VALUE2)
    }
    #[doc = "Incrementing slope generation."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SCMW::VALUE3)
    }
    #[doc = "Triangular slope generation."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(SCMW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SSRM`"]
pub enum SSRMW {
    #[doc = "External start trigger is ignored"]
    VALUE1,
    #[doc = "Start/restart slope generation"]
    VALUE2,
    #[doc = "Resumes slope"]
    VALUE3,
}
impl SSRMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SSRMW::VALUE1 => 0,
            SSRMW::VALUE2 => 1,
            SSRMW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSRMW<'a> {
    w: &'a mut W,
}
impl<'a> _SSRMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSRMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "External start trigger is ignored"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SSRMW::VALUE1)
    }
    #[doc = "Start/restart slope generation"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SSRMW::VALUE2)
    }
    #[doc = "Resumes slope"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SSRMW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SSTM`"]
pub enum SSTMW {
    #[doc = "External stop trigger is ignored"]
    VALUE1,
    #[doc = "Stops/Halts the slope generation"]
    VALUE2,
    #[doc = "Used in hybrid mode. It freezes the slope generation and feeds constantly the value programmed in CSGyDSV2This register contains the actual value used for the DSV2 reference. to the DAC."]
    VALUE3,
}
impl SSTMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SSTMW::VALUE1 => 0,
            SSTMW::VALUE2 => 1,
            SSTMW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSTMW<'a> {
    w: &'a mut W,
}
impl<'a> _SSTMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSTMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "External stop trigger is ignored"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SSTMW::VALUE1)
    }
    #[doc = "Stops/Halts the slope generation"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SSTMW::VALUE2)
    }
    #[doc = "Used in hybrid mode. It freezes the slope generation and feeds constantly the value programmed in CSGyDSV2This register contains the actual value used for the DSV2 reference. to the DAC."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SSTMW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SVSC`"]
pub enum SVSCW {
    #[doc = "Only CSGyDSV1This register contains the actual value used for the DSV1 reference. value is used for the slope generation: if slope is incrementing, CSGyDSV1This register contains the actual value used for the DSV1 reference. is the bottom reference value from where the ramp starts; if decrementing, then CSGyDSV1This register contains the actual value used for the DSV1 reference. is the upper reference value from where the ramp starts."]
    VALUE1,
    #[doc = "The two reference values are being used: CSGyDSV1This register contains the actual value used for the DSV1 reference. is the low or high reference value from where the ramp starts (incrementing or decrementing respectively); CSGyDSV2This register contains the actual value used for the DSV2 reference. is used as a static value (this value is constantly fed to the DAC after a stop trigger as been detected)."]
    VALUE2,
    #[doc = "The two reference values are used: CSGyDSV1This register contains the actual value used for the DSV1 reference. is the low or high reference value from where the slope starts (incrementing or decrementing respectively); CSGyDSV2This register contains the actual value used for the DSV2 reference. is used as an internal re start condition for the slope."]
    VALUE3,
}
impl SVSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SVSCW::VALUE1 => 0,
            SVSCW::VALUE2 => 1,
            SVSCW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SVSCW<'a> {
    w: &'a mut W,
}
impl<'a> _SVSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SVSCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Only CSGyDSV1This register contains the actual value used for the DSV1 reference. value is used for the slope generation: if slope is incrementing, CSGyDSV1This register contains the actual value used for the DSV1 reference. is the bottom reference value from where the ramp starts; if decrementing, then CSGyDSV1This register contains the actual value used for the DSV1 reference. is the upper reference value from where the ramp starts."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SVSCW::VALUE1)
    }
    #[doc = "The two reference values are being used: CSGyDSV1This register contains the actual value used for the DSV1 reference. is the low or high reference value from where the ramp starts (incrementing or decrementing respectively); CSGyDSV2This register contains the actual value used for the DSV2 reference. is used as a static value (this value is constantly fed to the DAC after a stop trigger as been detected)."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SVSCW::VALUE2)
    }
    #[doc = "The two reference values are used: CSGyDSV1This register contains the actual value used for the DSV1 reference. is the low or high reference value from where the slope starts (incrementing or decrementing respectively); CSGyDSV2This register contains the actual value used for the DSV2 reference. is used as an internal re start condition for the slope."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SVSCW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SWSM`"]
pub enum SWSMW {
    #[doc = "CSGyDSV2This register contains the actual value used for the DSV2 reference. is fed to the DAC and initial conversion trigger is generated."]
    VALUE1,
    #[doc = "CSGyDSV1This register contains the actual value used for the DSV1 reference. is fed to the DAC and initial conversion trigger is generated."]
    VALUE2,
    #[doc = "CSGyDSV2This register contains the actual value used for the DSV2 reference. is fed to the DAC but initial conversion trigger is not generated."]
    VALUE3,
    #[doc = "CSGyDSV1This register contains the actual value used for the DSV1 reference. is fed to the DAC but initial conversion trigger is not generated."]
    VALUE4,
}
impl SWSMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SWSMW::VALUE1 => 0,
            SWSMW::VALUE2 => 1,
            SWSMW::VALUE3 => 2,
            SWSMW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWSMW<'a> {
    w: &'a mut W,
}
impl<'a> _SWSMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWSMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CSGyDSV2This register contains the actual value used for the DSV2 reference. is fed to the DAC and initial conversion trigger is generated."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SWSMW::VALUE1)
    }
    #[doc = "CSGyDSV1This register contains the actual value used for the DSV1 reference. is fed to the DAC and initial conversion trigger is generated."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SWSMW::VALUE2)
    }
    #[doc = "CSGyDSV2This register contains the actual value used for the DSV2 reference. is fed to the DAC but initial conversion trigger is not generated."]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SWSMW::VALUE3)
    }
    #[doc = "CSGyDSV1This register contains the actual value used for the DSV1 reference. is fed to the DAC but initial conversion trigger is not generated."]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(SWSMW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GCFG`"]
pub enum GCFGW {
    #[doc = "Each slope step has an increment/decrement of 1"]
    VALUE1,
    #[doc = "Each slope step has an increment/decrement of 2"]
    VALUE2,
    #[doc = "Each slope step has an increment/decrement of 4"]
    VALUE3,
    #[doc = "Each slope step has an increment/decrement of 8"]
    VALUE4,
}
impl GCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GCFGW::VALUE1 => 0,
            GCFGW::VALUE2 => 1,
            GCFGW::VALUE3 => 2,
            GCFGW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _GCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Each slope step has an increment/decrement of 1"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(GCFGW::VALUE1)
    }
    #[doc = "Each slope step has an increment/decrement of 2"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(GCFGW::VALUE2)
    }
    #[doc = "Each slope step has an increment/decrement of 4"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(GCFGW::VALUE3)
    }
    #[doc = "Each slope step has an increment/decrement of 8"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(GCFGW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ISTW<'a> {
    w: &'a mut W,
}
impl<'a> _ISTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSE`"]
pub enum PSEW {
    #[doc = "Pulse swallow disabled"]
    VALUE1,
    #[doc = "Pulse swallow enabled"]
    VALUE2,
}
impl PSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PSEW::VALUE1 => false,
            PSEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSEW<'a> {
    w: &'a mut W,
}
impl<'a> _PSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pulse swallow disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSEW::VALUE1)
    }
    #[doc = "Pulse swallow enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSEW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSWM`"]
pub enum PSWMW {
    #[doc = "16 clock cycle window"]
    VALUE1,
    #[doc = "32 clock cycle window"]
    VALUE2,
    #[doc = "64 clock cycle window"]
    VALUE3,
}
impl PSWMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSWMW::VALUE1 => 0,
            PSWMW::VALUE2 => 1,
            PSWMW::VALUE3 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSWMW<'a> {
    w: &'a mut W,
}
impl<'a> _PSWMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSWMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "16 clock cycle window"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSWMW::VALUE1)
    }
    #[doc = "32 clock cycle window"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSWMW::VALUE2)
    }
    #[doc = "64 clock cycle window"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(PSWMW::VALUE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Prescaler external start configuration"]
    #[inline]
    pub fn psrm(&self) -> PSRMR {
        PSRMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Prescaler external stop configuration"]
    #[inline]
    pub fn pstm(&self) -> PSTMR {
        PSTMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Fixed division disable"]
    #[inline]
    pub fn fpd(&self) -> FPDR {
        FPDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:6 - Prescaler division factor"]
    #[inline]
    pub fn psv(&self) -> PSVR {
        PSVR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Slope control mode"]
    #[inline]
    pub fn scm(&self) -> SCMR {
        SCMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Slope external start configuration"]
    #[inline]
    pub fn ssrm(&self) -> SSRMR {
        SSRMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Slope external stop configuration"]
    #[inline]
    pub fn sstm(&self) -> SSTMR {
        SSTMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Slope reference value mode"]
    #[inline]
    pub fn svsc(&self) -> SVSCR {
        SVSCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Initial DAC start mode"]
    #[inline]
    pub fn swsm(&self) -> SWSMR {
        SWSMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Slope step gain configuration"]
    #[inline]
    pub fn gcfg(&self) -> GCFGR {
        GCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Immediate shadow transfer"]
    #[inline]
    pub fn ist(&self) -> ISTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ISTR { bits }
    }
    #[doc = "Bit 21 - Pulse swallow enable"]
    #[inline]
    pub fn pse(&self) -> PSER {
        PSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:25 - Pulse swallow window mode"]
    #[inline]
    pub fn pswm(&self) -> PSWMR {
        PSWMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Prescaler external start configuration"]
    #[inline]
    pub fn psrm(&mut self) -> _PSRMW {
        _PSRMW { w: self }
    }
    #[doc = "Bits 2:3 - Prescaler external stop configuration"]
    #[inline]
    pub fn pstm(&mut self) -> _PSTMW {
        _PSTMW { w: self }
    }
    #[doc = "Bit 4 - Fixed division disable"]
    #[inline]
    pub fn fpd(&mut self) -> _FPDW {
        _FPDW { w: self }
    }
    #[doc = "Bits 5:6 - Prescaler division factor"]
    #[inline]
    pub fn psv(&mut self) -> _PSVW {
        _PSVW { w: self }
    }
    #[doc = "Bits 8:9 - Slope control mode"]
    #[inline]
    pub fn scm(&mut self) -> _SCMW {
        _SCMW { w: self }
    }
    #[doc = "Bits 10:11 - Slope external start configuration"]
    #[inline]
    pub fn ssrm(&mut self) -> _SSRMW {
        _SSRMW { w: self }
    }
    #[doc = "Bits 12:13 - Slope external stop configuration"]
    #[inline]
    pub fn sstm(&mut self) -> _SSTMW {
        _SSTMW { w: self }
    }
    #[doc = "Bits 14:15 - Slope reference value mode"]
    #[inline]
    pub fn svsc(&mut self) -> _SVSCW {
        _SVSCW { w: self }
    }
    #[doc = "Bits 16:17 - Initial DAC start mode"]
    #[inline]
    pub fn swsm(&mut self) -> _SWSMW {
        _SWSMW { w: self }
    }
    #[doc = "Bits 18:19 - Slope step gain configuration"]
    #[inline]
    pub fn gcfg(&mut self) -> _GCFGW {
        _GCFGW { w: self }
    }
    #[doc = "Bit 20 - Immediate shadow transfer"]
    #[inline]
    pub fn ist(&mut self) -> _ISTW {
        _ISTW { w: self }
    }
    #[doc = "Bit 21 - Pulse swallow enable"]
    #[inline]
    pub fn pse(&mut self) -> _PSEW {
        _PSEW { w: self }
    }
    #[doc = "Bits 24:25 - Pulse swallow window mode"]
    #[inline]
    pub fn pswm(&mut self) -> _PSWMW {
        _PSWMW { w: self }
    }
}
