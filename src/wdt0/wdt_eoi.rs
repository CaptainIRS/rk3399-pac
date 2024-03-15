#[doc = "Register `WDT_EOI` reader"]
pub type R = crate::R<WdtEoiSpec>;
#[doc = "Field `WDT_INT_CLR` reader - Clears the watchdog interrupt. This can be used to clear the interrupt without restarting the watchdog counter.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type WdtIntClrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Clears the watchdog interrupt. This can be used to clear the interrupt without restarting the watchdog counter."]
    #[inline(always)]
    pub fn wdt_int_clr(&self) -> WdtIntClrR {
        WdtIntClrR::new((self.bits & 1) != 0)
    }
}
#[doc = "Interrupt clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt_eoi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtEoiSpec;
impl crate::RegisterSpec for WdtEoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt_eoi::R`](R) reader structure"]
impl crate::Readable for WdtEoiSpec {}
#[doc = "`reset()` method sets WDT_EOI to value 0"]
impl crate::Resettable for WdtEoiSpec {
    const RESET_VALUE: u32 = 0;
}
