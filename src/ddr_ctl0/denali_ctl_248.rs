#[doc = "Register `DENALI_CTL_248` reader"]
pub type R = crate::R<DenaliCtl248Spec>;
#[doc = "Register `DENALI_CTL_248` writer"]
pub type W = crate::W<DenaliCtl248Spec>;
#[doc = "Field `RDLVL_GATE_NORM_THRESHOLD_F1` reader - Gate training normal threshold number of long counts until the normal priority request is asserted."]
pub type RdlvlGateNormThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_GATE_NORM_THRESHOLD_F1` writer - Gate training normal threshold number of long counts until the normal priority request is asserted."]
pub type RdlvlGateNormThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RDLVL_GATE_HIGH_THRESHOLD_F1` reader - Gate training high threshold number of long counts until the high priority request is asserted."]
pub type RdlvlGateHighThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_GATE_HIGH_THRESHOLD_F1` writer - Gate training high threshold number of long counts until the high priority request is asserted."]
pub type RdlvlGateHighThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Gate training normal threshold number of long counts until the normal priority request is asserted."]
    #[inline(always)]
    pub fn rdlvl_gate_norm_threshold_f1(&self) -> RdlvlGateNormThresholdF1R {
        RdlvlGateNormThresholdF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Gate training high threshold number of long counts until the high priority request is asserted."]
    #[inline(always)]
    pub fn rdlvl_gate_high_threshold_f1(&self) -> RdlvlGateHighThresholdF1R {
        RdlvlGateHighThresholdF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Gate training normal threshold number of long counts until the normal priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_gate_norm_threshold_f1(&mut self) -> RdlvlGateNormThresholdF1W<DenaliCtl248Spec> {
        RdlvlGateNormThresholdF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Gate training high threshold number of long counts until the high priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_gate_high_threshold_f1(&mut self) -> RdlvlGateHighThresholdF1W<DenaliCtl248Spec> {
        RdlvlGateHighThresholdF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_248::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_248::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl248Spec;
impl crate::RegisterSpec for DenaliCtl248Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_248::R`](R) reader structure"]
impl crate::Readable for DenaliCtl248Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_248::W`](W) writer structure"]
impl crate::Writable for DenaliCtl248Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_248 to value 0"]
impl crate::Resettable for DenaliCtl248Spec {
    const RESET_VALUE: u32 = 0;
}
