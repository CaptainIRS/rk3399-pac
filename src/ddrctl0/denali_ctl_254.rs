#[doc = "Register `DENALI_CTL_254` reader"]
pub type R = crate::R<DenaliCtl254Spec>;
#[doc = "Register `DENALI_CTL_254` writer"]
pub type W = crate::W<DenaliCtl254Spec>;
#[doc = "Field `RDLVL_GATE_TIMEOUT_F2` reader - Gate training timeout number of long counts until the timeout is asserted."]
pub type RdlvlGateTimeoutF2R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_GATE_TIMEOUT_F2` writer - Gate training timeout number of long counts until the timeout is asserted."]
pub type RdlvlGateTimeoutF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RDLVL_GATE_SW_PROMOTE_THRESHOLD_F2` reader - Gate training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type RdlvlGateSwPromoteThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_GATE_SW_PROMOTE_THRESHOLD_F2` writer - Gate training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type RdlvlGateSwPromoteThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Gate training timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    pub fn rdlvl_gate_timeout_f2(&self) -> RdlvlGateTimeoutF2R {
        RdlvlGateTimeoutF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Gate training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    pub fn rdlvl_gate_sw_promote_threshold_f2(&self) -> RdlvlGateSwPromoteThresholdF2R {
        RdlvlGateSwPromoteThresholdF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Gate training timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_gate_timeout_f2(&mut self) -> RdlvlGateTimeoutF2W<DenaliCtl254Spec> {
        RdlvlGateTimeoutF2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Gate training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_gate_sw_promote_threshold_f2(
        &mut self,
    ) -> RdlvlGateSwPromoteThresholdF2W<DenaliCtl254Spec> {
        RdlvlGateSwPromoteThresholdF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_254::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_254::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl254Spec;
impl crate::RegisterSpec for DenaliCtl254Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_254::R`](R) reader structure"]
impl crate::Readable for DenaliCtl254Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_254::W`](W) writer structure"]
impl crate::Writable for DenaliCtl254Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_254 to value 0"]
impl crate::Resettable for DenaliCtl254Spec {
    const RESET_VALUE: u32 = 0;
}
