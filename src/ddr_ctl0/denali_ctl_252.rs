#[doc = "Register `DENALI_CTL_252` reader"]
pub type R = crate::R<DenaliCtl252Spec>;
#[doc = "Register `DENALI_CTL_252` writer"]
pub type W = crate::W<DenaliCtl252Spec>;
#[doc = "Field `RDLVL_SW_PROMOTE_THRESHOLD_F2` reader - Read leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type RdlvlSwPromoteThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_SW_PROMOTE_THRESHOLD_F2` writer - Read leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type RdlvlSwPromoteThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RDLVL_DFI_PROMOTE_THRESHOLD_F2` reader - Read leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type RdlvlDfiPromoteThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_DFI_PROMOTE_THRESHOLD_F2` writer - Read leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type RdlvlDfiPromoteThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Read leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    pub fn rdlvl_sw_promote_threshold_f2(&self) -> RdlvlSwPromoteThresholdF2R {
        RdlvlSwPromoteThresholdF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Read leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    pub fn rdlvl_dfi_promote_threshold_f2(&self) -> RdlvlDfiPromoteThresholdF2R {
        RdlvlDfiPromoteThresholdF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Read leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_sw_promote_threshold_f2(
        &mut self,
    ) -> RdlvlSwPromoteThresholdF2W<DenaliCtl252Spec> {
        RdlvlSwPromoteThresholdF2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Read leveling promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_dfi_promote_threshold_f2(
        &mut self,
    ) -> RdlvlDfiPromoteThresholdF2W<DenaliCtl252Spec> {
        RdlvlDfiPromoteThresholdF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_252::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_252::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl252Spec;
impl crate::RegisterSpec for DenaliCtl252Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_252::R`](R) reader structure"]
impl crate::Readable for DenaliCtl252Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_252::W`](W) writer structure"]
impl crate::Writable for DenaliCtl252Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_252 to value 0"]
impl crate::Resettable for DenaliCtl252Spec {
    const RESET_VALUE: u32 = 0;
}
