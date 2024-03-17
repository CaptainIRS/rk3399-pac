#[doc = "Register `DENALI_CTL_266` reader"]
pub type R = crate::R<DenaliCtl266Spec>;
#[doc = "Register `DENALI_CTL_266` writer"]
pub type W = crate::W<DenaliCtl266Spec>;
#[doc = "Field `CALVL_NORM_THRESHOLD_F0` reader - CA training normal threshold number of long counts until the normal priority request is asserted."]
pub type CalvlNormThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `CALVL_NORM_THRESHOLD_F0` writer - CA training normal threshold number of long counts until the normal priority request is asserted."]
pub type CalvlNormThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CALVL_HIGH_THRESHOLD_F0` reader - CA training high threshold number of long counts until the high priority request is asserted."]
pub type CalvlHighThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `CALVL_HIGH_THRESHOLD_F0` writer - CA training high threshold number of long counts until the high priority request is asserted."]
pub type CalvlHighThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CA training normal threshold number of long counts until the normal priority request is asserted."]
    #[inline(always)]
    pub fn calvl_norm_threshold_f0(&self) -> CalvlNormThresholdF0R {
        CalvlNormThresholdF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CA training high threshold number of long counts until the high priority request is asserted."]
    #[inline(always)]
    pub fn calvl_high_threshold_f0(&self) -> CalvlHighThresholdF0R {
        CalvlHighThresholdF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CA training normal threshold number of long counts until the normal priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_norm_threshold_f0(&mut self) -> CalvlNormThresholdF0W<DenaliCtl266Spec> {
        CalvlNormThresholdF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - CA training high threshold number of long counts until the high priority request is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_high_threshold_f0(&mut self) -> CalvlHighThresholdF0W<DenaliCtl266Spec> {
        CalvlHighThresholdF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_266::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_266::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl266Spec;
impl crate::RegisterSpec for DenaliCtl266Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_266::R`](R) reader structure"]
impl crate::Readable for DenaliCtl266Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_266::W`](W) writer structure"]
impl crate::Writable for DenaliCtl266Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_266 to value 0"]
impl crate::Resettable for DenaliCtl266Spec {
    const RESET_VALUE: u32 = 0;
}
