#[doc = "Register `DDR_DENALI_CTL_244` reader"]
pub type R = crate::R<DdrDenaliCtl244Spec>;
#[doc = "Register `DDR_DENALI_CTL_244` writer"]
pub type W = crate::W<DdrDenaliCtl244Spec>;
#[doc = "Field `RDLVL_GATE_TIMEOUT_F0` reader - Gate training timeout number of long counts until the timeout is asserted."]
pub type RdlvlGateTimeoutF0R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_GATE_TIMEOUT_F0` writer - Gate training timeout number of long counts until the timeout is asserted."]
pub type RdlvlGateTimeoutF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RDLVL_GATE_SW_PROMOTE_THRESHOLD_F0` reader - Gate training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type RdlvlGateSwPromoteThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_GATE_SW_PROMOTE_THRESHOLD_F0` writer - Gate training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type RdlvlGateSwPromoteThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Gate training timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    pub fn rdlvl_gate_timeout_f0(&self) -> RdlvlGateTimeoutF0R {
        RdlvlGateTimeoutF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Gate training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    pub fn rdlvl_gate_sw_promote_threshold_f0(&self) -> RdlvlGateSwPromoteThresholdF0R {
        RdlvlGateSwPromoteThresholdF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Gate training timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_gate_timeout_f0(&mut self) -> RdlvlGateTimeoutF0W<DdrDenaliCtl244Spec> {
        RdlvlGateTimeoutF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Gate training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_gate_sw_promote_threshold_f0(
        &mut self,
    ) -> RdlvlGateSwPromoteThresholdF0W<DdrDenaliCtl244Spec> {
        RdlvlGateSwPromoteThresholdF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_244::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_244::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl244Spec;
impl crate::RegisterSpec for DdrDenaliCtl244Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_244::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl244Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_244::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl244Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_244 to value 0"]
impl crate::Resettable for DdrDenaliCtl244Spec {
    const RESET_VALUE: u32 = 0;
}
