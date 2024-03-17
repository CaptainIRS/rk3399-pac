#[doc = "Register `DENALI_CTL_234` reader"]
pub type R = crate::R<DenaliCtl234Spec>;
#[doc = "Register `DENALI_CTL_234` writer"]
pub type W = crate::W<DenaliCtl234Spec>;
#[doc = "Field `WRLVL_HIGH_THRESHOLD_F2` reader - Write leveling high threshold number of long counts until the high priority request is asserted."]
pub type WrlvlHighThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `WRLVL_HIGH_THRESHOLD_F2` writer - Write leveling high threshold number of long counts until the high priority request is asserted."]
pub type WrlvlHighThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WRLVL_TIMEOUT_F2` reader - Write leveling timeout number of long counts until the timeout is asserted."]
pub type WrlvlTimeoutF2R = crate::FieldReader<u16>;
#[doc = "Field `WRLVL_TIMEOUT_F2` writer - Write leveling timeout number of long counts until the timeout is asserted."]
pub type WrlvlTimeoutF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Write leveling high threshold number of long counts until the high priority request is asserted."]
    #[inline(always)]
    pub fn wrlvl_high_threshold_f2(&self) -> WrlvlHighThresholdF2R {
        WrlvlHighThresholdF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Write leveling timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    pub fn wrlvl_timeout_f2(&self) -> WrlvlTimeoutF2R {
        WrlvlTimeoutF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write leveling high threshold number of long counts until the high priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_high_threshold_f2(&mut self) -> WrlvlHighThresholdF2W<DenaliCtl234Spec> {
        WrlvlHighThresholdF2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Write leveling timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_timeout_f2(&mut self) -> WrlvlTimeoutF2W<DenaliCtl234Spec> {
        WrlvlTimeoutF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_234::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_234::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl234Spec;
impl crate::RegisterSpec for DenaliCtl234Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_234::R`](R) reader structure"]
impl crate::Readable for DenaliCtl234Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_234::W`](W) writer structure"]
impl crate::Writable for DenaliCtl234Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_234 to value 0"]
impl crate::Resettable for DenaliCtl234Spec {
    const RESET_VALUE: u32 = 0;
}
