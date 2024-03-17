#[doc = "Register `DENALI_CTL_176` reader"]
pub type R = crate::R<DenaliCtl176Spec>;
#[doc = "Register `DENALI_CTL_176` writer"]
pub type W = crate::W<DenaliCtl176Spec>;
#[doc = "Field `ZQ_CS_NORM_THRESHOLD_F2` reader - ZQ CS number of long counts until the normal priority request is asserted for frequency copy 2."]
pub type ZqCsNormThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CS_NORM_THRESHOLD_F2` writer - ZQ CS number of long counts until the normal priority request is asserted for frequency copy 2."]
pub type ZqCsNormThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ZQ_CS_HIGH_THRESHOLD_F2` reader - ZQ CS number of long counts until the high priority request is asserted for frequency copy 2."]
pub type ZqCsHighThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CS_HIGH_THRESHOLD_F2` writer - ZQ CS number of long counts until the high priority request is asserted for frequency copy 2."]
pub type ZqCsHighThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ZQ CS number of long counts until the normal priority request is asserted for frequency copy 2."]
    #[inline(always)]
    pub fn zq_cs_norm_threshold_f2(&self) -> ZqCsNormThresholdF2R {
        ZqCsNormThresholdF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ZQ CS number of long counts until the high priority request is asserted for frequency copy 2."]
    #[inline(always)]
    pub fn zq_cs_high_threshold_f2(&self) -> ZqCsHighThresholdF2R {
        ZqCsHighThresholdF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ZQ CS number of long counts until the normal priority request is asserted for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn zq_cs_norm_threshold_f2(&mut self) -> ZqCsNormThresholdF2W<DenaliCtl176Spec> {
        ZqCsNormThresholdF2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ZQ CS number of long counts until the high priority request is asserted for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn zq_cs_high_threshold_f2(&mut self) -> ZqCsHighThresholdF2W<DenaliCtl176Spec> {
        ZqCsHighThresholdF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_176::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_176::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl176Spec;
impl crate::RegisterSpec for DenaliCtl176Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_176::R`](R) reader structure"]
impl crate::Readable for DenaliCtl176Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_176::W`](W) writer structure"]
impl crate::Writable for DenaliCtl176Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_176 to value 0"]
impl crate::Resettable for DenaliCtl176Spec {
    const RESET_VALUE: u32 = 0;
}
