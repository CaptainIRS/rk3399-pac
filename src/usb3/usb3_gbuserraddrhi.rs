#[doc = "Register `USB3_GBUSERRADDRHI` reader"]
pub type R = crate::R<Usb3GbuserraddrhiSpec>;
#[doc = "Field `BUSERRADDR` reader - Bus Address - High his register contains the higher 32 bits of the first bus address that encountered a SoC bus error. It is valid when the GSTS.BusErrAddrVld field is 1. It can only be cleared by resetting the core. Note: Only supported in AHB and AXI configurations."]
pub type BuserraddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bus Address - High his register contains the higher 32 bits of the first bus address that encountered a SoC bus error. It is valid when the GSTS.BusErrAddrVld field is 1. It can only be cleared by resetting the core. Note: Only supported in AHB and AXI configurations."]
    #[inline(always)]
    pub fn buserraddr(&self) -> BuserraddrR {
        BuserraddrR::new(self.bits)
    }
}
#[doc = "Global SoC Bus Error Address Register - High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gbuserraddrhi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3GbuserraddrhiSpec;
impl crate::RegisterSpec for Usb3GbuserraddrhiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_gbuserraddrhi::R`](R) reader structure"]
impl crate::Readable for Usb3GbuserraddrhiSpec {}
#[doc = "`reset()` method sets USB3_GBUSERRADDRHI to value 0"]
impl crate::Resettable for Usb3GbuserraddrhiSpec {
    const RESET_VALUE: u32 = 0;
}
