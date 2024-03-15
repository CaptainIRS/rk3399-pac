#[doc = "Register `USB3_GBUSERRADDRLO` reader"]
pub type R = crate::R<Usb3GbuserraddrloSpec>;
#[doc = "Field `BUSERRADDR` reader - Bus Address - Low This register contains the lower 32 bits of the first bus address that encountered a SoC bus error. It is valid when the GSTS.BusErrAddrVld field is 1. It can only be cleared by resetting the core. Note: Only supported in AHB and AXI configurations."]
pub type BuserraddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bus Address - Low This register contains the lower 32 bits of the first bus address that encountered a SoC bus error. It is valid when the GSTS.BusErrAddrVld field is 1. It can only be cleared by resetting the core. Note: Only supported in AHB and AXI configurations."]
    #[inline(always)]
    pub fn buserraddr(&self) -> BuserraddrR {
        BuserraddrR::new(self.bits)
    }
}
#[doc = "Global SoC Bus Error Address Register - Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gbuserraddrlo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3GbuserraddrloSpec;
impl crate::RegisterSpec for Usb3GbuserraddrloSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_gbuserraddrlo::R`](R) reader structure"]
impl crate::Readable for Usb3GbuserraddrloSpec {}
#[doc = "`reset()` method sets USB3_GBUSERRADDRLO to value 0"]
impl crate::Resettable for Usb3GbuserraddrloSpec {
    const RESET_VALUE: u32 = 0;
}
