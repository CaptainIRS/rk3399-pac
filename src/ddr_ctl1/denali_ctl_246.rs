#[doc = "Register `DENALI_CTL_246` reader"]
pub type R = crate::R<DenaliCtl246Spec>;
#[doc = "Register `DENALI_CTL_246` writer"]
pub type W = crate::W<DenaliCtl246Spec>;
#[doc = "Field `RDLVL_HIGH_THRESHOLD_F1` reader - Read leveling high threshold number of long counts until the high priority request is asserted."]
pub type RdlvlHighThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_HIGH_THRESHOLD_F1` writer - Read leveling high threshold number of long counts until the high priority request is asserted."]
pub type RdlvlHighThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RDLVL_TIMEOUT_F1` reader - Read leveling timeout number of long counts until the timeout is asserted."]
pub type RdlvlTimeoutF1R = crate::FieldReader<u16>;
#[doc = "Field `RDLVL_TIMEOUT_F1` writer - Read leveling timeout number of long counts until the timeout is asserted."]
pub type RdlvlTimeoutF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Read leveling high threshold number of long counts until the high priority request is asserted."]
    #[inline(always)]
    pub fn rdlvl_high_threshold_f1(&self) -> RdlvlHighThresholdF1R {
        RdlvlHighThresholdF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Read leveling timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    pub fn rdlvl_timeout_f1(&self) -> RdlvlTimeoutF1R {
        RdlvlTimeoutF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Read leveling high threshold number of long counts until the high priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_high_threshold_f1(&mut self) -> RdlvlHighThresholdF1W<DenaliCtl246Spec> {
        RdlvlHighThresholdF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Read leveling timeout number of long counts until the timeout is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_timeout_f1(&mut self) -> RdlvlTimeoutF1W<DenaliCtl246Spec> {
        RdlvlTimeoutF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_246::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_246::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl246Spec;
impl crate::RegisterSpec for DenaliCtl246Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_246::R`](R) reader structure"]
impl crate::Readable for DenaliCtl246Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_246::W`](W) writer structure"]
impl crate::Writable for DenaliCtl246Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_246 to value 0"]
impl crate::Resettable for DenaliCtl246Spec {
    const RESET_VALUE: u32 = 0;
}
