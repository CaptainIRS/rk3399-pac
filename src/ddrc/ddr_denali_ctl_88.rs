#[doc = "Register `DDR_DENALI_CTL_88` reader"]
pub type R = crate::R<DdrDenaliCtl88Spec>;
#[doc = "Register `DDR_DENALI_CTL_88` writer"]
pub type W = crate::W<DdrDenaliCtl88Spec>;
#[doc = "Field `MRR_TEMPCHK_HIGH_THRESHOLD_F0` reader - MRR temp check number of long counts until the high priority request is asserted for frequency copy 0."]
pub type MrrTempchkHighThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `MRR_TEMPCHK_HIGH_THRESHOLD_F0` writer - MRR temp check number of long counts until the high priority request is asserted for frequency copy 0."]
pub type MrrTempchkHighThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MRR_TEMPCHK_TIMEOUT_F0` reader - MRR temp check number of long counts until the timeout is asserted for frequency copy 0."]
pub type MrrTempchkTimeoutF0R = crate::FieldReader<u16>;
#[doc = "Field `MRR_TEMPCHK_TIMEOUT_F0` writer - MRR temp check number of long counts until the timeout is asserted for frequency copy 0."]
pub type MrrTempchkTimeoutF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MRR temp check number of long counts until the high priority request is asserted for frequency copy 0."]
    #[inline(always)]
    pub fn mrr_tempchk_high_threshold_f0(&self) -> MrrTempchkHighThresholdF0R {
        MrrTempchkHighThresholdF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - MRR temp check number of long counts until the timeout is asserted for frequency copy 0."]
    #[inline(always)]
    pub fn mrr_tempchk_timeout_f0(&self) -> MrrTempchkTimeoutF0R {
        MrrTempchkTimeoutF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MRR temp check number of long counts until the high priority request is asserted for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn mrr_tempchk_high_threshold_f0(
        &mut self,
    ) -> MrrTempchkHighThresholdF0W<DdrDenaliCtl88Spec> {
        MrrTempchkHighThresholdF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - MRR temp check number of long counts until the timeout is asserted for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn mrr_tempchk_timeout_f0(&mut self) -> MrrTempchkTimeoutF0W<DdrDenaliCtl88Spec> {
        MrrTempchkTimeoutF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_88::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_88::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl88Spec;
impl crate::RegisterSpec for DdrDenaliCtl88Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_88::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl88Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_88::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl88Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_88 to value 0"]
impl crate::Resettable for DdrDenaliCtl88Spec {
    const RESET_VALUE: u32 = 0;
}
