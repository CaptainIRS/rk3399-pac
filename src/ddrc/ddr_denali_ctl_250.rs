#[doc = "Register `DDR_DENALI_CTL_250` reader"]
pub type R = crate::R<DdrDenaliCtl250Spec>;
#[doc = "Register `DDR_DENALI_CTL_250` writer"]
pub type W = crate::W<DdrDenaliCtl250Spec>;
#[doc = "Field `RDLVL_GATE_DFI_PROMOTE_THRESHOLD_F1` reader - Gate training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type RdlvlGateDfiPromoteThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_GATE_DFI_PROMOTE_THRESHOLD_F1` writer - Gate training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
pub type RdlvlGateDfiPromoteThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RDLVL_NORM_THRESHOLD_F2` reader - Read leveling normal threshold number of long counts until the normal priority request is asserted."]
pub type RdlvlNormThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_NORM_THRESHOLD_F2` writer - Read leveling normal threshold number of long counts until the normal priority request is asserted."]
pub type RdlvlNormThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Gate training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    pub fn rdlvl_gate_dfi_promote_threshold_f1(&self) -> RdlvlGateDfiPromoteThresholdF1R {
        RdlvlGateDfiPromoteThresholdF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Read leveling normal threshold number of long counts until the normal priority request is asserted."]
    #[inline(always)]
    pub fn rdlvl_norm_threshold_f2(&self) -> RdlvlNormThresholdF2R {
        RdlvlNormThresholdF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Gate training promotion number of long counts until the high priority request is asserted. Applies to DFI commands."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_gate_dfi_promote_threshold_f1(
        &mut self,
    ) -> RdlvlGateDfiPromoteThresholdF1W<DdrDenaliCtl250Spec> {
        RdlvlGateDfiPromoteThresholdF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Read leveling normal threshold number of long counts until the normal priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_norm_threshold_f2(&mut self) -> RdlvlNormThresholdF2W<DdrDenaliCtl250Spec> {
        RdlvlNormThresholdF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_250::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_250::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl250Spec;
impl crate::RegisterSpec for DdrDenaliCtl250Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_250::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl250Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_250::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl250Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_250 to value 0"]
impl crate::Resettable for DdrDenaliCtl250Spec {
    const RESET_VALUE: u32 = 0;
}
