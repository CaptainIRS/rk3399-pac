#[doc = "Register `DDR_DENALI_CTL_205` writer"]
pub type W = crate::W<DdrDenaliCtl205Spec>;
#[doc = "Field `INT_ACK` writer - Clear mask of the INT_STATUS parameter."]
pub type IntAckW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Clear mask of the INT_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn int_ack(&mut self) -> IntAckW<DdrDenaliCtl205Spec> {
        IntAckW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_205::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl205Spec;
impl crate::RegisterSpec for DdrDenaliCtl205Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_205::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl205Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_205 to value 0"]
impl crate::Resettable for DdrDenaliCtl205Spec {
    const RESET_VALUE: u32 = 0;
}
