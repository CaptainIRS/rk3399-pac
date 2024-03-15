#[doc = "Register `DENALI_CTL_249` reader"]
pub type R = crate::R<DenaliCtl249Spec>;
#[doc = "Register `DENALI_CTL_249` writer"]
pub type W = crate::W<DenaliCtl249Spec>;
#[doc = "Field `RDLVL_GATE_TIMEOUT_F1` reader - Gate training timeout number of long counts until the timeout is asserted."]
pub type RdlvlGateTimeoutF1R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_GATE_TIMEOUT_F1` writer - Gate training timeout number of long counts until the timeout is asserted."]
pub type RdlvlGateTimeoutF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RDLVL_GATE_SW_PROMOTE_THRESHOLD_F1` reader - Gate training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type RdlvlGateSwPromoteThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_GATE_SW_PROMOTE_THRESHOLD_F1` writer - Gate training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type RdlvlGateSwPromoteThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Gate training timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    pub fn rdlvl_gate_timeout_f1(&self) -> RdlvlGateTimeoutF1R {
        RdlvlGateTimeoutF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Gate training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    pub fn rdlvl_gate_sw_promote_threshold_f1(&self) -> RdlvlGateSwPromoteThresholdF1R {
        RdlvlGateSwPromoteThresholdF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Gate training timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_gate_timeout_f1(&mut self) -> RdlvlGateTimeoutF1W<DenaliCtl249Spec> {
        RdlvlGateTimeoutF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Gate training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_gate_sw_promote_threshold_f1(
        &mut self,
    ) -> RdlvlGateSwPromoteThresholdF1W<DenaliCtl249Spec> {
        RdlvlGateSwPromoteThresholdF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_249::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_249::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl249Spec;
impl crate::RegisterSpec for DenaliCtl249Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_249::R`](R) reader structure"]
impl crate::Readable for DenaliCtl249Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_249::W`](W) writer structure"]
impl crate::Writable for DenaliCtl249Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_249 to value 0"]
impl crate::Resettable for DenaliCtl249Spec {
    const RESET_VALUE: u32 = 0;
}
