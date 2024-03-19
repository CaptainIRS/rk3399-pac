#[doc = "Register `DDR_DENALI_CTL_231` reader"]
pub type R = crate::R<DdrDenaliCtl231Spec>;
#[doc = "Register `DDR_DENALI_CTL_231` writer"]
pub type W = crate::W<DdrDenaliCtl231Spec>;
#[doc = "Field `WRLVL_NORM_THRESHOLD_F1` reader - Write leveling normal threshold number of long counts until the normal priority request is asserted."]
pub type WrlvlNormThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `WRLVL_NORM_THRESHOLD_F1` writer - Write leveling normal threshold number of long counts until the normal priority request is asserted."]
pub type WrlvlNormThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WRLVL_HIGH_THRESHOLD_F1` reader - Write leveling high threshold number of long counts until the high priority request is asserted."]
pub type WrlvlHighThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `WRLVL_HIGH_THRESHOLD_F1` writer - Write leveling high threshold number of long counts until the high priority request is asserted."]
pub type WrlvlHighThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Write leveling normal threshold number of long counts until the normal priority request is asserted."]
    #[inline(always)]
    pub fn wrlvl_norm_threshold_f1(&self) -> WrlvlNormThresholdF1R {
        WrlvlNormThresholdF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Write leveling high threshold number of long counts until the high priority request is asserted."]
    #[inline(always)]
    pub fn wrlvl_high_threshold_f1(&self) -> WrlvlHighThresholdF1R {
        WrlvlHighThresholdF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write leveling normal threshold number of long counts until the normal priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_norm_threshold_f1(&mut self) -> WrlvlNormThresholdF1W<DdrDenaliCtl231Spec> {
        WrlvlNormThresholdF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Write leveling high threshold number of long counts until the high priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn wrlvl_high_threshold_f1(&mut self) -> WrlvlHighThresholdF1W<DdrDenaliCtl231Spec> {
        WrlvlHighThresholdF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_231::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_231::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl231Spec;
impl crate::RegisterSpec for DdrDenaliCtl231Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_231::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl231Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_231::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl231Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_231 to value 0"]
impl crate::Resettable for DdrDenaliCtl231Spec {
    const RESET_VALUE: u32 = 0;
}
