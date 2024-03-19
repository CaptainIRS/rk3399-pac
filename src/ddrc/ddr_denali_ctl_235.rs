#[doc = "Register `DDR_DENALI_CTL_235` reader"]
pub type R = crate::R<DdrDenaliCtl235Spec>;
#[doc = "Register `DDR_DENALI_CTL_235` writer"]
pub type W = crate::W<DdrDenaliCtl235Spec>;
#[doc = "Field `WRLVL_SW_PROMOTE_THRESHOLD_F2` reader - Write leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type WrlvlSwPromoteThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `WRLVL_SW_PROMOTE_THRESHOLD_F2` writer - Write leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type WrlvlSwPromoteThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WRLVL_DFI_PROMOTE_THRESHOLD_F2` reader - Write leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type WrlvlDfiPromoteThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `WRLVL_DFI_PROMOTE_THRESHOLD_F2` writer - Write leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type WrlvlDfiPromoteThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Write leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    pub fn wrlvl_sw_promote_threshold_f2(&self) -> WrlvlSwPromoteThresholdF2R {
        WrlvlSwPromoteThresholdF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Write leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    pub fn wrlvl_dfi_promote_threshold_f2(&self) -> WrlvlDfiPromoteThresholdF2R {
        WrlvlDfiPromoteThresholdF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_sw_promote_threshold_f2(
        &mut self,
    ) -> WrlvlSwPromoteThresholdF2W<DdrDenaliCtl235Spec> {
        WrlvlSwPromoteThresholdF2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Write leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_dfi_promote_threshold_f2(
        &mut self,
    ) -> WrlvlDfiPromoteThresholdF2W<DdrDenaliCtl235Spec> {
        WrlvlDfiPromoteThresholdF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_235::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_235::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl235Spec;
impl crate::RegisterSpec for DdrDenaliCtl235Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_235::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl235Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_235::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl235Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_235 to value 0"]
impl crate::Resettable for DdrDenaliCtl235Spec {
    const RESET_VALUE: u32 = 0;
}
