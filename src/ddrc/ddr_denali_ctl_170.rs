#[doc = "Register `DDR_DENALI_CTL_170` reader"]
pub type R = crate::R<DdrDenaliCtl170Spec>;
#[doc = "Register `DDR_DENALI_CTL_170` writer"]
pub type W = crate::W<DdrDenaliCtl170Spec>;
#[doc = "Field `ZQ_CALSTART_NORM_THRESHOLD_F1` reader - ZQ START number of long counts until the normal priority request is asserted for frequency copy 1. This value should be scaled based on the number of ranks (chip selects) the controller handles. The more chip selects there are, the more rotations there are to go through, and the smaller this threshold should be."]
pub type ZqCalstartNormThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CALSTART_NORM_THRESHOLD_F1` writer - ZQ START number of long counts until the normal priority request is asserted for frequency copy 1. This value should be scaled based on the number of ranks (chip selects) the controller handles. The more chip selects there are, the more rotations there are to go through, and the smaller this threshold should be."]
pub type ZqCalstartNormThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ZQ_CALSTART_HIGH_THRESHOLD_F1` reader - ZQ START number of long counts until the high priority request is asserted for frequency copy 1."]
pub type ZqCalstartHighThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CALSTART_HIGH_THRESHOLD_F1` writer - ZQ START number of long counts until the high priority request is asserted for frequency copy 1."]
pub type ZqCalstartHighThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ZQ START number of long counts until the normal priority request is asserted for frequency copy 1. This value should be scaled based on the number of ranks (chip selects) the controller handles. The more chip selects there are, the more rotations there are to go through, and the smaller this threshold should be."]
    #[inline(always)]
    pub fn zq_calstart_norm_threshold_f1(&self) -> ZqCalstartNormThresholdF1R {
        ZqCalstartNormThresholdF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ZQ START number of long counts until the high priority request is asserted for frequency copy 1."]
    #[inline(always)]
    pub fn zq_calstart_high_threshold_f1(&self) -> ZqCalstartHighThresholdF1R {
        ZqCalstartHighThresholdF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ZQ START number of long counts until the normal priority request is asserted for frequency copy 1. This value should be scaled based on the number of ranks (chip selects) the controller handles. The more chip selects there are, the more rotations there are to go through, and the smaller this threshold should be."]
    #[inline(always)]
    #[must_use]
    pub fn zq_calstart_norm_threshold_f1(
        &mut self,
    ) -> ZqCalstartNormThresholdF1W<DdrDenaliCtl170Spec> {
        ZqCalstartNormThresholdF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ZQ START number of long counts until the high priority request is asserted for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn zq_calstart_high_threshold_f1(
        &mut self,
    ) -> ZqCalstartHighThresholdF1W<DdrDenaliCtl170Spec> {
        ZqCalstartHighThresholdF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_170::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_170::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl170Spec;
impl crate::RegisterSpec for DdrDenaliCtl170Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_170::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl170Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_170::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl170Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_170 to value 0"]
impl crate::Resettable for DdrDenaliCtl170Spec {
    const RESET_VALUE: u32 = 0;
}
