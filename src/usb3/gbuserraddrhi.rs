#[doc = "Register `GBUSERRADDRHI` reader"]
pub type R = crate::R<GbuserraddrhiSpec>;
#[doc = "Field `BUSERRADDR` reader - Bus Address - High\n\nhis register contains the higher 32 bits of the first bus address\n\nthat encountered a SoC bus error. It is valid when the\n\nGSTS.BusErrAddrVld field is 1. It can only be cleared by resetting\n\nthe core.\n\nNote: Only supported in AHB and AXI configurations."]
pub type BuserraddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bus Address - High\n\nhis register contains the higher 32 bits of the first bus address\n\nthat encountered a SoC bus error. It is valid when the\n\nGSTS.BusErrAddrVld field is 1. It can only be cleared by resetting\n\nthe core.\n\nNote: Only supported in AHB and AXI configurations."]
    #[inline(always)]
    pub fn buserraddr(&self) -> BuserraddrR {
        BuserraddrR::new(self.bits)
    }
}
#[doc = "Global SoC Bus Error Address Register - High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gbuserraddrhi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GbuserraddrhiSpec;
impl crate::RegisterSpec for GbuserraddrhiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gbuserraddrhi::R`](R) reader structure"]
impl crate::Readable for GbuserraddrhiSpec {}
#[doc = "`reset()` method sets GBUSERRADDRHI to value 0"]
impl crate::Resettable for GbuserraddrhiSpec {
    const RESET_VALUE: u32 = 0;
}
