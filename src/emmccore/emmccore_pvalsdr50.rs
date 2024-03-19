#[doc = "Register `EMMCCORE_PVALSDR50` reader"]
pub type R = crate::R<EmmccorePvalsdr50Spec>;
#[doc = "Field `SDCLKFREQUENCYSELECTVALUE` reader - 10-bit preset value to set SDCLK Frequency Select in the Clock\n\nControl Register is described by a host system."]
pub type SdclkfrequencyselectvalueR = crate::FieldReader<u16>;
#[doc = "This bit is effective when Host Controller supports programmable\n\nclockgenerator.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clockgeneratorselectvalue {
    #[doc = "1: Programmable Clock Generator"]
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
#[doc = "Field `CLOCKGENERATORSELECTVALUE` reader - This bit is effective when Host Controller supports programmable\n\nclockgenerator."]
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
    #[doc = "Programmable Clock Generator"]
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
impl R {
    #[doc = "Bits 0:9 - 10-bit preset value to set SDCLK Frequency Select in the Clock\n\nControl Register is described by a host system."]
    #[inline(always)]
    pub fn sdclkfrequencyselectvalue(&self) -> SdclkfrequencyselectvalueR {
        SdclkfrequencyselectvalueR::new(self.bits & 0x03ff)
    }
    #[doc = "Bit 10 - This bit is effective when Host Controller supports programmable\n\nclockgenerator."]
    #[inline(always)]
    pub fn clockgeneratorselectvalue(&self) -> ClockgeneratorselectvalueR {
        ClockgeneratorselectvalueR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Preset value register for SDR50\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_pvalsdr50::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccorePvalsdr50Spec;
impl crate::RegisterSpec for EmmccorePvalsdr50Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`emmccore_pvalsdr50::R`](R) reader structure"]
impl crate::Readable for EmmccorePvalsdr50Spec {}
#[doc = "`reset()` method sets EMMCCORE_PVALSDR50 to value 0"]
impl crate::Resettable for EmmccorePvalsdr50Spec {
    const RESET_VALUE: u16 = 0;
}
