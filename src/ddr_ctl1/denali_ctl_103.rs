#[doc = "Register `DENALI_CTL_103` reader"]
pub type R = crate::R<DenaliCtl103Spec>;
#[doc = "Register `DENALI_CTL_103` writer"]
pub type W = crate::W<DenaliCtl103Spec>;
#[doc = "Field `LP_AUTO_SR_IDLE` reader - Number of long count sequences until the controller will place memory in self-refresh."]
pub type LpAutoSrIdleR = crate::FieldReader;
#[doc = "Field `LP_AUTO_SR_IDLE` writer - Number of long count sequences until the controller will place memory in self-refresh."]
pub type LpAutoSrIdleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_AUTO_SR_MC_GATE_IDLE` reader - Number of long count sequences until the controller will place memory in self-refresh with controller and memory clock gating."]
pub type LpAutoSrMcGateIdleR = crate::FieldReader;
#[doc = "Field `LP_AUTO_SR_MC_GATE_IDLE` writer - Number of long count sequences until the controller will place memory in self-refresh with controller and memory clock gating."]
pub type LpAutoSrMcGateIdleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HW_PROMOTE_THRESHOLD_F0` reader - HW interface promotion number of long counts until the high priority request is asserted for frequency copy 0."]
pub type HwPromoteThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `HW_PROMOTE_THRESHOLD_F0` writer - HW interface promotion number of long counts until the high priority request is asserted for frequency copy 0."]
pub type HwPromoteThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Number of long count sequences until the controller will place memory in self-refresh."]
    #[inline(always)]
    pub fn lp_auto_sr_idle(&self) -> LpAutoSrIdleR {
        LpAutoSrIdleR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of long count sequences until the controller will place memory in self-refresh with controller and memory clock gating."]
    #[inline(always)]
    pub fn lp_auto_sr_mc_gate_idle(&self) -> LpAutoSrMcGateIdleR {
        LpAutoSrMcGateIdleR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - HW interface promotion number of long counts until the high priority request is asserted for frequency copy 0."]
    #[inline(always)]
    pub fn hw_promote_threshold_f0(&self) -> HwPromoteThresholdF0R {
        HwPromoteThresholdF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of long count sequences until the controller will place memory in self-refresh."]
    #[inline(always)]
    #[must_use]
    pub fn lp_auto_sr_idle(&mut self) -> LpAutoSrIdleW<DenaliCtl103Spec> {
        LpAutoSrIdleW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Number of long count sequences until the controller will place memory in self-refresh with controller and memory clock gating."]
    #[inline(always)]
    #[must_use]
    pub fn lp_auto_sr_mc_gate_idle(&mut self) -> LpAutoSrMcGateIdleW<DenaliCtl103Spec> {
        LpAutoSrMcGateIdleW::new(self, 8)
    }
    #[doc = "Bits 16:31 - HW interface promotion number of long counts until the high priority request is asserted for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn hw_promote_threshold_f0(&mut self) -> HwPromoteThresholdF0W<DenaliCtl103Spec> {
        HwPromoteThresholdF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_103::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_103::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl103Spec;
impl crate::RegisterSpec for DenaliCtl103Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_103::R`](R) reader structure"]
impl crate::Readable for DenaliCtl103Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_103::W`](W) writer structure"]
impl crate::Writable for DenaliCtl103Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_103 to value 0"]
impl crate::Resettable for DenaliCtl103Spec {
    const RESET_VALUE: u32 = 0;
}
