#[doc = "Register `EMMCCORE_PVALHS400` reader"]
pub type R = crate::R<EmmccorePvalhs400Spec>;
#[doc = "Field `SDCLKFREQUENCYSELECTVALUE` reader - 10-bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system."]
pub type SdclkfrequencyselectvalueR = crate::FieldReader<u16>;
#[doc = "This bit is effective when Host Controller supports programmable clockgenerator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clockgeneratorselectvalue {
    #[doc = "1: Host Controller Ver2.00 Compatible Clock Generator"]
    B1 = 1,
    #[doc = "0: Host Controller Ver2.00 Compatible Clock Generator"]
    B0 = 0,
}
impl From<Clockgeneratorselectvalue> for bool {
    #[inline(always)]
    fn from(variant: Clockgeneratorselectvalue) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLOCKGENERATORSELECTVALUE` reader - This bit is effective when Host Controller supports programmable clockgenerator."]
pub type ClockgeneratorselectvalueR = crate::BitReader<Clockgeneratorselectvalue>;
impl ClockgeneratorselectvalueR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clockgeneratorselectvalue {
        match self.bits {
            true => Clockgeneratorselectvalue::B1,
            false => Clockgeneratorselectvalue::B0,
        }
    }
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Clockgeneratorselectvalue::B1
    }
    #[doc = "Host Controller Ver2.00 Compatible Clock Generator"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Clockgeneratorselectvalue::B0
    }
}
#[doc = "Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Driverstrengthselectvalue {
    #[doc = "3: Driver Type B is Selected"]
    B11 = 3,
    #[doc = "2: Driver Type B is Selected"]
    B10 = 2,
    #[doc = "1: Driver Type B is Selected"]
    B01 = 1,
    #[doc = "0: Driver Type B is Selected"]
    B00 = 0,
}
impl From<Driverstrengthselectvalue> for u8 {
    #[inline(always)]
    fn from(variant: Driverstrengthselectvalue) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Driverstrengthselectvalue {
    type Ux = u8;
}
#[doc = "Field `DRIVERSTRENGTHSELECTVALUE` reader - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
pub type DriverstrengthselectvalueR = crate::FieldReader<Driverstrengthselectvalue>;
impl DriverstrengthselectvalueR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Driverstrengthselectvalue {
        match self.bits {
            3 => Driverstrengthselectvalue::B11,
            2 => Driverstrengthselectvalue::B10,
            1 => Driverstrengthselectvalue::B01,
            0 => Driverstrengthselectvalue::B00,
            _ => unreachable!(),
        }
    }
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Driverstrengthselectvalue::B11
    }
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Driverstrengthselectvalue::B10
    }
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Driverstrengthselectvalue::B01
    }
    #[doc = "Driver Type B is Selected"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Driverstrengthselectvalue::B00
    }
}
impl R {
    #[doc = "Bits 0:9 - 10-bit preset value to set SDCLK Frequency Select in the Clock Control Register is described by a host system."]
    #[inline(always)]
    pub fn sdclkfrequencyselectvalue(&self) -> SdclkfrequencyselectvalueR {
        SdclkfrequencyselectvalueR::new(self.bits & 0x03ff)
    }
    #[doc = "Bit 10 - This bit is effective when Host Controller supports programmable clockgenerator."]
    #[inline(always)]
    pub fn clockgeneratorselectvalue(&self) -> ClockgeneratorselectvalueR {
        ClockgeneratorselectvalueR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Driver Strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
    #[inline(always)]
    pub fn driverstrengthselectvalue(&self) -> DriverstrengthselectvalueR {
        DriverstrengthselectvalueR::new(((self.bits >> 14) & 3) as u8)
    }
}
#[doc = "Preset value register for HS400\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_pvalhs400::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccorePvalhs400Spec;
impl crate::RegisterSpec for EmmccorePvalhs400Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`emmccore_pvalhs400::R`](R) reader structure"]
impl crate::Readable for EmmccorePvalhs400Spec {}
#[doc = "`reset()` method sets EMMCCORE_PVALHS400 to value 0"]
impl crate::Resettable for EmmccorePvalhs400Spec {
    const RESET_VALUE: u16 = 0;
}
