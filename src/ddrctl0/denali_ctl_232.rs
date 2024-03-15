#[doc = "Register `DENALI_CTL_232` reader"]
pub type R = crate::R<DenaliCtl232Spec>;
#[doc = "Register `DENALI_CTL_232` writer"]
pub type W = crate::W<DenaliCtl232Spec>;
#[doc = "Field `WRLVL_TIMEOUT_F1` reader - Write leveling timeout number of long counts until the timeout is asserted."]
pub type WrlvlTimeoutF1R = crate::FieldReader<u16>;
#[doc = "Field `WRLVL_TIMEOUT_F1` writer - Write leveling timeout number of long counts until the timeout is asserted."]
pub type WrlvlTimeoutF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WRLVL_SW_PROMOTE_THRESHOLD_F1` reader - Write leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type WrlvlSwPromoteThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `WRLVL_SW_PROMOTE_THRESHOLD_F1` writer - Write leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
pub type WrlvlSwPromoteThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Write leveling timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    pub fn wrlvl_timeout_f1(&self) -> WrlvlTimeoutF1R {
        WrlvlTimeoutF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Write leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    pub fn wrlvl_sw_promote_threshold_f1(&self) -> WrlvlSwPromoteThresholdF1R {
        WrlvlSwPromoteThresholdF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write leveling timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_timeout_f1(&mut self) -> WrlvlTimeoutF1W<DenaliCtl232Spec> {
        WrlvlTimeoutF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Write leveling promotion number of long counts until the high priority request is asserted. Applies to SW commands."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_sw_promote_threshold_f1(
        &mut self,
    ) -> WrlvlSwPromoteThresholdF1W<DenaliCtl232Spec> {
        WrlvlSwPromoteThresholdF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_232::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_232::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl232Spec;
impl crate::RegisterSpec for DenaliCtl232Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_232::R`](R) reader structure"]
impl crate::Readable for DenaliCtl232Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_232::W`](W) writer structure"]
impl crate::Writable for DenaliCtl232Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_232 to value 0"]
impl crate::Resettable for DenaliCtl232Spec {
    const RESET_VALUE: u32 = 0;
}
