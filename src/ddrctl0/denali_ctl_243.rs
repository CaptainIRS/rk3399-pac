#[doc = "Register `DENALI_CTL_243` reader"]
pub type R = crate::R<DenaliCtl243Spec>;
#[doc = "Register `DENALI_CTL_243` writer"]
pub type W = crate::W<DenaliCtl243Spec>;
#[doc = "Field `RDLVL_GATE_NORM_THRESHOLD_F0` reader - Gate training normal threshold number of long counts until the normal priority request is asserted."]
pub type RdlvlGateNormThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_GATE_NORM_THRESHOLD_F0` writer - Gate training normal threshold number of long counts until the normal priority request is asserted."]
pub type RdlvlGateNormThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RDLVL_GATE_HIGH_THRESHOLD_F0` reader - Gate training high threshold number of long counts until the high priority request is asserted."]
pub type RdlvlGateHighThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_GATE_HIGH_THRESHOLD_F0` writer - Gate training high threshold number of long counts until the high priority request is asserted."]
pub type RdlvlGateHighThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Gate training normal threshold number of long counts until the normal priority request is asserted."]
    #[inline(always)]
    pub fn rdlvl_gate_norm_threshold_f0(&self) -> RdlvlGateNormThresholdF0R {
        RdlvlGateNormThresholdF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Gate training high threshold number of long counts until the high priority request is asserted."]
    #[inline(always)]
    pub fn rdlvl_gate_high_threshold_f0(&self) -> RdlvlGateHighThresholdF0R {
        RdlvlGateHighThresholdF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Gate training normal threshold number of long counts until the normal priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_gate_norm_threshold_f0(&mut self) -> RdlvlGateNormThresholdF0W<DenaliCtl243Spec> {
        RdlvlGateNormThresholdF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Gate training high threshold number of long counts until the high priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_gate_high_threshold_f0(&mut self) -> RdlvlGateHighThresholdF0W<DenaliCtl243Spec> {
        RdlvlGateHighThresholdF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_243::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_243::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl243Spec;
impl crate::RegisterSpec for DenaliCtl243Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_243::R`](R) reader structure"]
impl crate::Readable for DenaliCtl243Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_243::W`](W) writer structure"]
impl crate::Writable for DenaliCtl243Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_243 to value 0"]
impl crate::Resettable for DenaliCtl243Spec {
    const RESET_VALUE: u32 = 0;
}
