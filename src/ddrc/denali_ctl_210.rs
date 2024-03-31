#[doc = "Register `DENALI_CTL_210` reader"]
pub type R = crate::R<DenaliCtl210Spec>;
#[doc = "Field `OUT_OF_RANGE_ADDR` reader - Address of command that caused an out-of-range interrupt."]
pub type OutOfRangeAddrR = crate::FieldReader;
#[doc = "Field `OUT_OF_RANGE_LENGTH` reader - Length of command that caused an out-of-range interrupt."]
pub type OutOfRangeLengthR = crate::FieldReader<u16>;
#[doc = "Field `OUT_OF_RANGE_TYPE` reader - Type of command that caused an out-of-range interrupt."]
pub type OutOfRangeTypeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Address of command that caused an out-of-range interrupt."]
    #[inline(always)]
    pub fn out_of_range_addr(&self) -> OutOfRangeAddrR {
        OutOfRangeAddrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:17 - Length of command that caused an out-of-range interrupt."]
    #[inline(always)]
    pub fn out_of_range_length(&self) -> OutOfRangeLengthR {
        OutOfRangeLengthR::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:30 - Type of command that caused an out-of-range interrupt."]
    #[inline(always)]
    pub fn out_of_range_type(&self) -> OutOfRangeTypeR {
        OutOfRangeTypeR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_210::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl210Spec;
impl crate::RegisterSpec for DenaliCtl210Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_210::R`](R) reader structure"]
impl crate::Readable for DenaliCtl210Spec {}
#[doc = "`reset()` method sets DENALI_CTL_210 to value 0"]
impl crate::Resettable for DenaliCtl210Spec {
    const RESET_VALUE: u32 = 0;
}
