#[doc = "Register `DDR_DENALI_CTL_242` reader"]
pub type R = crate::R<DdrDenaliCtl242Spec>;
#[doc = "Register `DDR_DENALI_CTL_242` writer"]
pub type W = crate::W<DdrDenaliCtl242Spec>;
#[doc = "Field `RDLVL_SW_PROMOTE_THRESHOLD_F0` reader - Read leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type RdlvlSwPromoteThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_SW_PROMOTE_THRESHOLD_F0` writer - Read leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type RdlvlSwPromoteThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RDLVL_DFI_PROMOTE_THRESHOLD_F0` reader - Read leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type RdlvlDfiPromoteThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_DFI_PROMOTE_THRESHOLD_F0` writer - Read leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type RdlvlDfiPromoteThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Read leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    pub fn rdlvl_sw_promote_threshold_f0(&self) -> RdlvlSwPromoteThresholdF0R {
        RdlvlSwPromoteThresholdF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Read leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    pub fn rdlvl_dfi_promote_threshold_f0(&self) -> RdlvlDfiPromoteThresholdF0R {
        RdlvlDfiPromoteThresholdF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Read leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_sw_promote_threshold_f0(
        &mut self,
    ) -> RdlvlSwPromoteThresholdF0W<DdrDenaliCtl242Spec> {
        RdlvlSwPromoteThresholdF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Read leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_dfi_promote_threshold_f0(
        &mut self,
    ) -> RdlvlDfiPromoteThresholdF0W<DdrDenaliCtl242Spec> {
        RdlvlDfiPromoteThresholdF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_242::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_242::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl242Spec;
impl crate::RegisterSpec for DdrDenaliCtl242Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_242::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl242Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_242::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl242Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_242 to value 0"]
impl crate::Resettable for DdrDenaliCtl242Spec {
    const RESET_VALUE: u32 = 0;
}
