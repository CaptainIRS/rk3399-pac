#[doc = "Register `GMAC_REC_INT_WDT_TIMER` reader"]
pub type R = crate::R<GmacRecIntWdtTimerSpec>;
#[doc = "Register `GMAC_REC_INT_WDT_TIMER` writer"]
pub type W = crate::W<GmacRecIntWdtTimerSpec>;
#[doc = "Field `RIWT` reader - RI Watchdog Timer count Indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set. The watchdog timer gets triggered with the programmed value after the RxDMA completes the transfer of a frame for which the RI status bit is not set due to the setting in the corresponding descriptor RDES1\\[31\\]. When the watch-dog timer runs out, the RI bit is set and the timer is stopped. The watchdog timer is reset when RI bit is set high due to automatic setting of RI as per RDES1\\[31\\]
of any received frame."]
pub type RiwtR = crate::FieldReader;
#[doc = "Field `RIWT` writer - RI Watchdog Timer count Indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set. The watchdog timer gets triggered with the programmed value after the RxDMA completes the transfer of a frame for which the RI status bit is not set due to the setting in the corresponding descriptor RDES1\\[31\\]. When the watch-dog timer runs out, the RI bit is set and the timer is stopped. The watchdog timer is reset when RI bit is set high due to automatic setting of RI as per RDES1\\[31\\]
of any received frame."]
pub type RiwtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RI Watchdog Timer count Indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set. The watchdog timer gets triggered with the programmed value after the RxDMA completes the transfer of a frame for which the RI status bit is not set due to the setting in the corresponding descriptor RDES1\\[31\\]. When the watch-dog timer runs out, the RI bit is set and the timer is stopped. The watchdog timer is reset when RI bit is set high due to automatic setting of RI as per RDES1\\[31\\]
of any received frame."]
    #[inline(always)]
    pub fn riwt(&self) -> RiwtR {
        RiwtR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RI Watchdog Timer count Indicates the number of system clock cycles multiplied by 256 for which the watchdog timer is set. The watchdog timer gets triggered with the programmed value after the RxDMA completes the transfer of a frame for which the RI status bit is not set due to the setting in the corresponding descriptor RDES1\\[31\\]. When the watch-dog timer runs out, the RI bit is set and the timer is stopped. The watchdog timer is reset when RI bit is set high due to automatic setting of RI as per RDES1\\[31\\]
of any received frame."]
    #[inline(always)]
    #[must_use]
    pub fn riwt(&mut self) -> RiwtW<GmacRecIntWdtTimerSpec> {
        RiwtW::new(self, 0)
    }
}
#[doc = "Receive Interrupt Watchdog Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_rec_int_wdt_timer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_rec_int_wdt_timer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacRecIntWdtTimerSpec;
impl crate::RegisterSpec for GmacRecIntWdtTimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_rec_int_wdt_timer::R`](R) reader structure"]
impl crate::Readable for GmacRecIntWdtTimerSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_rec_int_wdt_timer::W`](W) writer structure"]
impl crate::Writable for GmacRecIntWdtTimerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_REC_INT_WDT_TIMER to value 0"]
impl crate::Resettable for GmacRecIntWdtTimerSpec {
    const RESET_VALUE: u32 = 0;
}
