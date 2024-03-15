#[doc = "Register `DENALI_CTL_269` reader"]
pub type R = crate::R<DenaliCtl269Spec>;
#[doc = "Register `DENALI_CTL_269` writer"]
pub type W = crate::W<DenaliCtl269Spec>;
#[doc = "Field `CALVL_HIGH_THRESHOLD_F1` reader - CA training high threshold number of long counts until the high priority request is asserted."]
pub type CalvlHighThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `CALVL_HIGH_THRESHOLD_F1` writer - CA training high threshold number of long counts until the high priority request is asserted."]
pub type CalvlHighThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CALVL_TIMEOUT_F1` reader - CA training timeout number of long counts until the timeout is asserted."]
pub type CalvlTimeoutF1R = crate::FieldReader<u16>;
#[doc = "Field `CALVL_TIMEOUT_F1` writer - CA training timeout number of long counts until the timeout is asserted."]
pub type CalvlTimeoutF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CA training high threshold number of long counts until the high priority request is asserted."]
    #[inline(always)]
    pub fn calvl_high_threshold_f1(&self) -> CalvlHighThresholdF1R {
        CalvlHighThresholdF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CA training timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    pub fn calvl_timeout_f1(&self) -> CalvlTimeoutF1R {
        CalvlTimeoutF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CA training high threshold number of long counts until the high priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_high_threshold_f1(&mut self) -> CalvlHighThresholdF1W<DenaliCtl269Spec> {
        CalvlHighThresholdF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - CA training timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_timeout_f1(&mut self) -> CalvlTimeoutF1W<DenaliCtl269Spec> {
        CalvlTimeoutF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_269::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_269::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl269Spec;
impl crate::RegisterSpec for DenaliCtl269Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_269::R`](R) reader structure"]
impl crate::Readable for DenaliCtl269Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_269::W`](W) writer structure"]
impl crate::Writable for DenaliCtl269Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_269 to value 0"]
impl crate::Resettable for DenaliCtl269Spec {
    const RESET_VALUE: u32 = 0;
}
