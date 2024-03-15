#[doc = "Register `DENALI_CTL_247` reader"]
pub type R = crate::R<DenaliCtl247Spec>;
#[doc = "Register `DENALI_CTL_247` writer"]
pub type W = crate::W<DenaliCtl247Spec>;
#[doc = "Field `RDLVL_SW_PROMOTE_THRESHOLD_F1` reader - Read leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type RdlvlSwPromoteThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_SW_PROMOTE_THRESHOLD_F1` writer - Read leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type RdlvlSwPromoteThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RDLVL_DFI_PROMOTE_THRESHOLD_F1` reader - Read leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type RdlvlDfiPromoteThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_DFI_PROMOTE_THRESHOLD_F1` writer - Read leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type RdlvlDfiPromoteThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Read leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    pub fn rdlvl_sw_promote_threshold_f1(&self) -> RdlvlSwPromoteThresholdF1R {
        RdlvlSwPromoteThresholdF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Read leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    pub fn rdlvl_dfi_promote_threshold_f1(&self) -> RdlvlDfiPromoteThresholdF1R {
        RdlvlDfiPromoteThresholdF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Read leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_sw_promote_threshold_f1(
        &mut self,
    ) -> RdlvlSwPromoteThresholdF1W<DenaliCtl247Spec> {
        RdlvlSwPromoteThresholdF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Read leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_dfi_promote_threshold_f1(
        &mut self,
    ) -> RdlvlDfiPromoteThresholdF1W<DenaliCtl247Spec> {
        RdlvlDfiPromoteThresholdF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_247::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_247::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl247Spec;
impl crate::RegisterSpec for DenaliCtl247Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_247::R`](R) reader structure"]
impl crate::Readable for DenaliCtl247Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_247::W`](W) writer structure"]
impl crate::Writable for DenaliCtl247Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_247 to value 0"]
impl crate::Resettable for DenaliCtl247Spec {
    const RESET_VALUE: u32 = 0;
}
