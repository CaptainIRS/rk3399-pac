#[doc = "Register `DDR_DENALI_CTL_230` reader"]
pub type R = crate::R<DdrDenaliCtl230Spec>;
#[doc = "Register `DDR_DENALI_CTL_230` writer"]
pub type W = crate::W<DdrDenaliCtl230Spec>;
#[doc = "Field `WRLVL_SW_PROMOTE_THRESHOLD_F0` reader - Write leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type WrlvlSwPromoteThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `WRLVL_SW_PROMOTE_THRESHOLD_F0` writer - Write leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type WrlvlSwPromoteThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WRLVL_DFI_PROMOTE_THRESHOLD_F0` reader - Write leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type WrlvlDfiPromoteThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `WRLVL_DFI_PROMOTE_THRESHOLD_F0` writer - Write leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type WrlvlDfiPromoteThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Write leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    pub fn wrlvl_sw_promote_threshold_f0(&self) -> WrlvlSwPromoteThresholdF0R {
        WrlvlSwPromoteThresholdF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Write leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    pub fn wrlvl_dfi_promote_threshold_f0(&self) -> WrlvlDfiPromoteThresholdF0R {
        WrlvlDfiPromoteThresholdF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_sw_promote_threshold_f0(
        &mut self,
    ) -> WrlvlSwPromoteThresholdF0W<DdrDenaliCtl230Spec> {
        WrlvlSwPromoteThresholdF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Write leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_dfi_promote_threshold_f0(
        &mut self,
    ) -> WrlvlDfiPromoteThresholdF0W<DdrDenaliCtl230Spec> {
        WrlvlDfiPromoteThresholdF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_230::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_230::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl230Spec;
impl crate::RegisterSpec for DdrDenaliCtl230Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_230::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl230Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_230::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl230Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_230 to value 0"]
impl crate::Resettable for DdrDenaliCtl230Spec {
    const RESET_VALUE: u32 = 0;
}
