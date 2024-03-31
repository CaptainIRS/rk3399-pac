#[doc = "Register `GBUSERRADDRLO` reader"]
pub type R = crate::R<GbuserraddrloSpec>;
#[doc = "Field `BUSERRADDR` reader - Bus Address - Low\n\nThis register contains the lower 32 bits of the first bus address\n\nthat encountered a SoC bus error. It is valid when the\n\nGSTS.BusErrAddrVld field is 1. It can only be cleared by resetting\n\nthe core.\n\nNote: Only supported in AHB and AXI configurations."]
pub type BuserraddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bus Address - Low\n\nThis register contains the lower 32 bits of the first bus address\n\nthat encountered a SoC bus error. It is valid when the\n\nGSTS.BusErrAddrVld field is 1. It can only be cleared by resetting\n\nthe core.\n\nNote: Only supported in AHB and AXI configurations."]
    #[inline(always)]
    pub fn buserraddr(&self) -> BuserraddrR {
        BuserraddrR::new(self.bits)
    }
}
#[doc = "Global SoC Bus Error Address Register - Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gbuserraddrlo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GbuserraddrloSpec;
impl crate::RegisterSpec for GbuserraddrloSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gbuserraddrlo::R`](R) reader structure"]
impl crate::Readable for GbuserraddrloSpec {}
#[doc = "`reset()` method sets GBUSERRADDRLO to value 0"]
impl crate::Resettable for GbuserraddrloSpec {
    const RESET_VALUE: u32 = 0;
}
