#[doc = "Register `DDR_DENALI_CTL_272` reader"]
pub type R = crate::R<DdrDenaliCtl272Spec>;
#[doc = "Register `DDR_DENALI_CTL_272` writer"]
pub type W = crate::W<DdrDenaliCtl272Spec>;
#[doc = "Field `CALVL_TIMEOUT_F2` reader - CA training timeout number of long counts until the timeout is asserted."]
pub type CalvlTimeoutF2R = crate::FieldReader<u16>;
#[doc = "Field `CALVL_TIMEOUT_F2` writer - CA training timeout number of long counts until the timeout is asserted."]
pub type CalvlTimeoutF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CALVL_SW_PROMOTE_THRESHOLD_F2` reader - CA training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type CalvlSwPromoteThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `CALVL_SW_PROMOTE_THRESHOLD_F2` writer - CA training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type CalvlSwPromoteThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CA training timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    pub fn calvl_timeout_f2(&self) -> CalvlTimeoutF2R {
        CalvlTimeoutF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CA training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    pub fn calvl_sw_promote_threshold_f2(&self) -> CalvlSwPromoteThresholdF2R {
        CalvlSwPromoteThresholdF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CA training timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_timeout_f2(&mut self) -> CalvlTimeoutF2W<DdrDenaliCtl272Spec> {
        CalvlTimeoutF2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - CA training promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_sw_promote_threshold_f2(
        &mut self,
    ) -> CalvlSwPromoteThresholdF2W<DdrDenaliCtl272Spec> {
        CalvlSwPromoteThresholdF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_272::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_272::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl272Spec;
impl crate::RegisterSpec for DdrDenaliCtl272Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_272::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl272Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_272::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl272Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_272 to value 0"]
impl crate::Resettable for DdrDenaliCtl272Spec {
    const RESET_VALUE: u32 = 0;
}
