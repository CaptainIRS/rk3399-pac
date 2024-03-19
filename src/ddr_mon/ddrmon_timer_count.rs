#[doc = "Register `DDRMON_TIMER_COUNT` reader"]
pub type R = crate::R<DdrmonTimerCountSpec>;
#[doc = "Field `TIMER_COUNT` reader - The DFI timer threshold, the statistics of DDR access only be done\n\nwhen timer counter is less then this threshold in hardware mode"]
pub type TimerCountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The DFI timer threshold, the statistics of DDR access only be done\n\nwhen timer counter is less then this threshold in hardware mode"]
    #[inline(always)]
    pub fn timer_count(&self) -> TimerCountR {
        TimerCountR::new(self.bits)
    }
}
#[doc = "The DFI Timer Threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_timer_count::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrmonTimerCountSpec;
impl crate::RegisterSpec for DdrmonTimerCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrmon_timer_count::R`](R) reader structure"]
impl crate::Readable for DdrmonTimerCountSpec {}
#[doc = "`reset()` method sets DDRMON_TIMER_COUNT to value 0"]
impl crate::Resettable for DdrmonTimerCountSpec {
    const RESET_VALUE: u32 = 0;
}
