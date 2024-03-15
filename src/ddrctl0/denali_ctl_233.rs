#[doc = "Register `DENALI_CTL_233` reader"]
pub type R = crate::R<DenaliCtl233Spec>;
#[doc = "Register `DENALI_CTL_233` writer"]
pub type W = crate::W<DenaliCtl233Spec>;
#[doc = "Field `WRLVL_DFI_PROMOTE_THRESHOLD_F1` reader - Write leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type WrlvlDfiPromoteThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `WRLVL_DFI_PROMOTE_THRESHOLD_F1` writer - Write leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type WrlvlDfiPromoteThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WRLVL_NORM_THRESHOLD_F2` reader - Write leveling normal threshold number of long counts until the normal priority request is asserted."]
pub type WrlvlNormThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `WRLVL_NORM_THRESHOLD_F2` writer - Write leveling normal threshold number of long counts until the normal priority request is asserted."]
pub type WrlvlNormThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Write leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    pub fn wrlvl_dfi_promote_threshold_f1(&self) -> WrlvlDfiPromoteThresholdF1R {
        WrlvlDfiPromoteThresholdF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Write leveling normal threshold number of long counts until the normal priority request is asserted."]
    #[inline(always)]
    pub fn wrlvl_norm_threshold_f2(&self) -> WrlvlNormThresholdF2R {
        WrlvlNormThresholdF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_dfi_promote_threshold_f1(
        &mut self,
    ) -> WrlvlDfiPromoteThresholdF1W<DenaliCtl233Spec> {
        WrlvlDfiPromoteThresholdF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Write leveling normal threshold number of long counts until the normal priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_norm_threshold_f2(&mut self) -> WrlvlNormThresholdF2W<DenaliCtl233Spec> {
        WrlvlNormThresholdF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_233::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_233::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl233Spec;
impl crate::RegisterSpec for DenaliCtl233Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_233::R`](R) reader structure"]
impl crate::Readable for DenaliCtl233Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_233::W`](W) writer structure"]
impl crate::Writable for DenaliCtl233Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_233 to value 0"]
impl crate::Resettable for DenaliCtl233Spec {
    const RESET_VALUE: u32 = 0;
}
