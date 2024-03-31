#[doc = "Register `CQCAP` reader"]
pub type R = crate::R<CqcapSpec>;
#[doc = "Field `ITCFVAL` reader - Internal Timer Clock Frequency Value\n\nTCFMUL and ITCFVAL indicate the frequency of the clock used for\n\ninterrupt coalescing timer and for determining the polling period\n\nwhen using periodic SEND_QUEUE_ STATUS (CMD13) polling.\n\nThe clock frequency is calculated as ITCFVAL* ITCFMUL.\n\nFor example, to encode 19.2 MHz, ITCFVAL shall be C0h (= 192\n\ndecimal) and ITCFMUL shall be 2h (0.1 MHz)\n\n192 * 0.1 MHz=19.2 MHz"]
pub type ItcfvalR = crate::FieldReader<u16>;
#[doc = "nternal Timer Clock Frequency Multiplier\n\nITCFMUL and ITCFVAL indicate the frequency of the clock used\n\nfor interrupt coalescing timer and for determining the SQS polling\n\nperiod. See ITCFVAL definition for details.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Itcfmul {
    #[doc = "0: 0.001 MHz"]
    H0 = 0,
    #[doc = "1: 0.01 MHz"]
    H1 = 1,
    #[doc = "2: 0.1 MHz"]
    H2 = 2,
    #[doc = "3: 1 MHz"]
    H3 = 3,
    #[doc = "4: 10 MHz Other values are reserved"]
    H4 = 4,
}
impl From<Itcfmul> for u8 {
    #[inline(always)]
    fn from(variant: Itcfmul) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Itcfmul {
    type Ux = u8;
}
#[doc = "Field `ITCFMUL` reader - nternal Timer Clock Frequency Multiplier\n\nITCFMUL and ITCFVAL indicate the frequency of the clock used\n\nfor interrupt coalescing timer and for determining the SQS polling\n\nperiod. See ITCFVAL definition for details."]
pub type ItcfmulR = crate::FieldReader<Itcfmul>;
impl ItcfmulR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Itcfmul> {
        match self.bits {
            0 => Some(Itcfmul::H0),
            1 => Some(Itcfmul::H1),
            2 => Some(Itcfmul::H2),
            3 => Some(Itcfmul::H3),
            4 => Some(Itcfmul::H4),
            _ => None,
        }
    }
    #[doc = "0.001 MHz"]
    #[inline(always)]
    pub fn is_h0(&self) -> bool {
        *self == Itcfmul::H0
    }
    #[doc = "0.01 MHz"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == Itcfmul::H1
    }
    #[doc = "0.1 MHz"]
    #[inline(always)]
    pub fn is_h2(&self) -> bool {
        *self == Itcfmul::H2
    }
    #[doc = "1 MHz"]
    #[inline(always)]
    pub fn is_h3(&self) -> bool {
        *self == Itcfmul::H3
    }
    #[doc = "10 MHz Other values are reserved"]
    #[inline(always)]
    pub fn is_h4(&self) -> bool {
        *self == Itcfmul::H4
    }
}
impl R {
    #[doc = "Bits 0:9 - Internal Timer Clock Frequency Value\n\nTCFMUL and ITCFVAL indicate the frequency of the clock used for\n\ninterrupt coalescing timer and for determining the polling period\n\nwhen using periodic SEND_QUEUE_ STATUS (CMD13) polling.\n\nThe clock frequency is calculated as ITCFVAL* ITCFMUL.\n\nFor example, to encode 19.2 MHz, ITCFVAL shall be C0h (= 192\n\ndecimal) and ITCFMUL shall be 2h (0.1 MHz)\n\n192 * 0.1 MHz=19.2 MHz"]
    #[inline(always)]
    pub fn itcfval(&self) -> ItcfvalR {
        ItcfvalR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:15 - nternal Timer Clock Frequency Multiplier\n\nITCFMUL and ITCFVAL indicate the frequency of the clock used\n\nfor interrupt coalescing timer and for determining the SQS polling\n\nperiod. See ITCFVAL definition for details."]
    #[inline(always)]
    pub fn itcfmul(&self) -> ItcfmulR {
        ItcfmulR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "Command queueing capabilities register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cqcap::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqcapSpec;
impl crate::RegisterSpec for CqcapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqcap::R`](R) reader structure"]
impl crate::Readable for CqcapSpec {}
#[doc = "`reset()` method sets CQCAP to value 0"]
impl crate::Resettable for CqcapSpec {
    const RESET_VALUE: u32 = 0;
}
