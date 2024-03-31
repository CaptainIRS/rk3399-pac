#[doc = "Register `DENALI_CTL_270` reader"]
pub type R = crate::R<DenaliCtl270Spec>;
#[doc = "Register `DENALI_CTL_270` writer"]
pub type W = crate::W<DenaliCtl270Spec>;
#[doc = "Field `CALVL_SW_PROMOTE_THRESHOLD_F1` reader - CA training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type CalvlSwPromoteThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `CALVL_SW_PROMOTE_THRESHOLD_F1` writer - CA training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type CalvlSwPromoteThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CALVL_DFI_PROMOTE_THRESHOLD_F1` reader - CA training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type CalvlDfiPromoteThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `CALVL_DFI_PROMOTE_THRESHOLD_F1` writer - CA training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type CalvlDfiPromoteThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CA training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    pub fn calvl_sw_promote_threshold_f1(&self) -> CalvlSwPromoteThresholdF1R {
        CalvlSwPromoteThresholdF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CA training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    pub fn calvl_dfi_promote_threshold_f1(&self) -> CalvlDfiPromoteThresholdF1R {
        CalvlDfiPromoteThresholdF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CA training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_sw_promote_threshold_f1(
        &mut self,
    ) -> CalvlSwPromoteThresholdF1W<DenaliCtl270Spec> {
        CalvlSwPromoteThresholdF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - CA training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_dfi_promote_threshold_f1(
        &mut self,
    ) -> CalvlDfiPromoteThresholdF1W<DenaliCtl270Spec> {
        CalvlDfiPromoteThresholdF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_270::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_270::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl270Spec;
impl crate::RegisterSpec for DenaliCtl270Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_270::R`](R) reader structure"]
impl crate::Readable for DenaliCtl270Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_270::W`](W) writer structure"]
impl crate::Writable for DenaliCtl270Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_270 to value 0"]
impl crate::Resettable for DenaliCtl270Spec {
    const RESET_VALUE: u32 = 0;
}
