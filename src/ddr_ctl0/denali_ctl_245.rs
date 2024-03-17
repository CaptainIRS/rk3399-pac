#[doc = "Register `DENALI_CTL_245` reader"]
pub type R = crate::R<DenaliCtl245Spec>;
#[doc = "Register `DENALI_CTL_245` writer"]
pub type W = crate::W<DenaliCtl245Spec>;
#[doc = "Field `RDLVL_GATE_DFI_PROMOTE_THRESHOLD_F0` reader - Gate training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type RdlvlGateDfiPromoteThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_GATE_DFI_PROMOTE_THRESHOLD_F0` writer - Gate training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type RdlvlGateDfiPromoteThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RDLVL_NORM_THRESHOLD_F1` reader - Read leveling normal threshold number of long counts until the normal priority request is asserted."]
pub type RdlvlNormThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_NORM_THRESHOLD_F1` writer - Read leveling normal threshold number of long counts until the normal priority request is asserted."]
pub type RdlvlNormThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Gate training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    pub fn rdlvl_gate_dfi_promote_threshold_f0(&self) -> RdlvlGateDfiPromoteThresholdF0R {
        RdlvlGateDfiPromoteThresholdF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Read leveling normal threshold number of long counts until the normal priority request is asserted."]
    #[inline(always)]
    pub fn rdlvl_norm_threshold_f1(&self) -> RdlvlNormThresholdF1R {
        RdlvlNormThresholdF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Gate training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_gate_dfi_promote_threshold_f0(
        &mut self,
    ) -> RdlvlGateDfiPromoteThresholdF0W<DenaliCtl245Spec> {
        RdlvlGateDfiPromoteThresholdF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Read leveling normal threshold number of long counts until the normal priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_norm_threshold_f1(&mut self) -> RdlvlNormThresholdF1W<DenaliCtl245Spec> {
        RdlvlNormThresholdF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_245::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_245::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl245Spec;
impl crate::RegisterSpec for DenaliCtl245Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_245::R`](R) reader structure"]
impl crate::Readable for DenaliCtl245Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_245::W`](W) writer structure"]
impl crate::Writable for DenaliCtl245Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_245 to value 0"]
impl crate::Resettable for DenaliCtl245Spec {
    const RESET_VALUE: u32 = 0;
}
