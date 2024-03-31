#[doc = "Register `EOI` reader"]
pub type R = crate::R<EoiSpec>;
#[doc = "Field `WDT_INT_CLR` reader - Clears the watchdog interrupt.\n\nThis can be used to clear the interrupt without restarting the\n\nwatchdog counter.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type WdtIntClrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Clears the watchdog interrupt.\n\nThis can be used to clear the interrupt without restarting the\n\nwatchdog counter."]
    #[inline(always)]
    pub fn wdt_int_clr(&self) -> WdtIntClrR {
        WdtIntClrR::new((self.bits & 1) != 0)
    }
}
#[doc = "Interrupt clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eoi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EoiSpec;
impl crate::RegisterSpec for EoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eoi::R`](R) reader structure"]
impl crate::Readable for EoiSpec {}
#[doc = "`reset()` method sets EOI to value 0"]
impl crate::Resettable for EoiSpec {
    const RESET_VALUE: u32 = 0;
}
