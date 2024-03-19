#[doc = "Register `DDR_DENALI_CTL_229` reader"]
pub type R = crate::R<DdrDenaliCtl229Spec>;
#[doc = "Register `DDR_DENALI_CTL_229` writer"]
pub type W = crate::W<DdrDenaliCtl229Spec>;
#[doc = "Field `WRLVL_HIGH_THRESHOLD_F0` reader - Write leveling high threshold number of long counts until the high priority request is asserted."]
pub type WrlvlHighThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `WRLVL_HIGH_THRESHOLD_F0` writer - Write leveling high threshold number of long counts until the high priority request is asserted."]
pub type WrlvlHighThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WRLVL_TIMEOUT_F0` reader - Write leveling timeout number of long counts until the timeout is asserted."]
pub type WrlvlTimeoutF0R = crate::FieldReader<u16>;
#[doc = "Field `WRLVL_TIMEOUT_F0` writer - Write leveling timeout number of long counts until the timeout is asserted."]
pub type WrlvlTimeoutF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Write leveling high threshold number of long counts until the high priority request is asserted."]
    #[inline(always)]
    pub fn wrlvl_high_threshold_f0(&self) -> WrlvlHighThresholdF0R {
        WrlvlHighThresholdF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Write leveling timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    pub fn wrlvl_timeout_f0(&self) -> WrlvlTimeoutF0R {
        WrlvlTimeoutF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write leveling high threshold number of long counts until the high priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_high_threshold_f0(&mut self) -> WrlvlHighThresholdF0W<DdrDenaliCtl229Spec> {
        WrlvlHighThresholdF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Write leveling timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_timeout_f0(&mut self) -> WrlvlTimeoutF0W<DdrDenaliCtl229Spec> {
        WrlvlTimeoutF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_229::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_229::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl229Spec;
impl crate::RegisterSpec for DdrDenaliCtl229Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_229::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl229Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_229::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl229Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_229 to value 0"]
impl crate::Resettable for DdrDenaliCtl229Spec {
    const RESET_VALUE: u32 = 0;
}
