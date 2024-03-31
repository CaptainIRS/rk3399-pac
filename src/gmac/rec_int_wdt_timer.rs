#[doc = "Register `REC_INT_WDT_TIMER` reader"]
pub type R = crate::R<RecIntWdtTimerSpec>;
#[doc = "Register `REC_INT_WDT_TIMER` writer"]
pub type W = crate::W<RecIntWdtTimerSpec>;
#[doc = "Field `RIWT` reader - RI Watchdog Timer count\n\nIndicates the number of system clock cycles multiplied by 256 for\n\nwhich the watchdog timer is set. The watchdog timer gets\n\ntriggered with the programmed value after the RxDMA completes\n\nthe transfer of a frame for which the RI status bit is not set due\n\nto the setting in the corresponding descriptor RDES1\\[31\\]. When\n\nthe watch-dog timer runs out, the RI bit is set and the timer is\n\nstopped. The watchdog timer is reset when RI bit is set high due\n\nto automatic setting of RI as per RDES1\\[31\\]
of any received\n\nframe."]
pub type RiwtR = crate::FieldReader;
#[doc = "Field `RIWT` writer - RI Watchdog Timer count\n\nIndicates the number of system clock cycles multiplied by 256 for\n\nwhich the watchdog timer is set. The watchdog timer gets\n\ntriggered with the programmed value after the RxDMA completes\n\nthe transfer of a frame for which the RI status bit is not set due\n\nto the setting in the corresponding descriptor RDES1\\[31\\]. When\n\nthe watch-dog timer runs out, the RI bit is set and the timer is\n\nstopped. The watchdog timer is reset when RI bit is set high due\n\nto automatic setting of RI as per RDES1\\[31\\]
of any received\n\nframe."]
pub type RiwtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - RI Watchdog Timer count\n\nIndicates the number of system clock cycles multiplied by 256 for\n\nwhich the watchdog timer is set. The watchdog timer gets\n\ntriggered with the programmed value after the RxDMA completes\n\nthe transfer of a frame for which the RI status bit is not set due\n\nto the setting in the corresponding descriptor RDES1\\[31\\]. When\n\nthe watch-dog timer runs out, the RI bit is set and the timer is\n\nstopped. The watchdog timer is reset when RI bit is set high due\n\nto automatic setting of RI as per RDES1\\[31\\]
of any received\n\nframe."]
    #[inline(always)]
    pub fn riwt(&self) -> RiwtR {
        RiwtR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RI Watchdog Timer count\n\nIndicates the number of system clock cycles multiplied by 256 for\n\nwhich the watchdog timer is set. The watchdog timer gets\n\ntriggered with the programmed value after the RxDMA completes\n\nthe transfer of a frame for which the RI status bit is not set due\n\nto the setting in the corresponding descriptor RDES1\\[31\\]. When\n\nthe watch-dog timer runs out, the RI bit is set and the timer is\n\nstopped. The watchdog timer is reset when RI bit is set high due\n\nto automatic setting of RI as per RDES1\\[31\\]
of any received\n\nframe."]
    #[inline(always)]
    #[must_use]
    pub fn riwt(&mut self) -> RiwtW<RecIntWdtTimerSpec> {
        RiwtW::new(self, 0)
    }
}
#[doc = "Receive Interrupt Watchdog Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rec_int_wdt_timer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rec_int_wdt_timer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RecIntWdtTimerSpec;
impl crate::RegisterSpec for RecIntWdtTimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rec_int_wdt_timer::R`](R) reader structure"]
impl crate::Readable for RecIntWdtTimerSpec {}
#[doc = "`write(|w| ..)` method takes [`rec_int_wdt_timer::W`](W) writer structure"]
impl crate::Writable for RecIntWdtTimerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REC_INT_WDT_TIMER to value 0"]
impl crate::Resettable for RecIntWdtTimerSpec {
    const RESET_VALUE: u32 = 0;
}
