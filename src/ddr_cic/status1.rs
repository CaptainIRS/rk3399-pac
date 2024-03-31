#[doc = "Register `STATUS1` reader"]
pub type R = crate::R<Status1Spec>;
#[doc = "Field `STATE_CH0` reader - Channel 0 external self-refresh and standby mode state machine"]
pub type StateCh0R = crate::FieldReader<u16>;
#[doc = "Field `STATE_CH1` reader - Channel 1 external self-refresh and standby mode state machine"]
pub type StateCh1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - Channel 0 external self-refresh and standby mode state machine"]
    #[inline(always)]
    pub fn state_ch0(&self) -> StateCh0R {
        StateCh0R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Channel 1 external self-refresh and standby mode state machine"]
    #[inline(always)]
    pub fn state_ch1(&self) -> StateCh1R {
        StateCh1R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
#[doc = "DDR Controller LP Interface Status Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Status1Spec;
impl crate::RegisterSpec for Status1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status1::R`](R) reader structure"]
impl crate::Readable for Status1Spec {}
#[doc = "`reset()` method sets STATUS1 to value 0"]
impl crate::Resettable for Status1Spec {
    const RESET_VALUE: u32 = 0;
}
