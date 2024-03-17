#[doc = "Register `DENALI_CTL_165` reader"]
pub type R = crate::R<DenaliCtl165Spec>;
#[doc = "Register `DENALI_CTL_165` writer"]
pub type W = crate::W<DenaliCtl165Spec>;
#[doc = "Field `AREF_MAX_CREDIT` reader - AREF number of posted refreshes until the maximum number of refresh credits has been reached."]
pub type ArefMaxCreditR = crate::FieldReader;
#[doc = "Field `AREF_MAX_CREDIT` writer - AREF number of posted refreshes until the maximum number of refresh credits has been reached."]
pub type ArefMaxCreditW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ZQ_CALSTART_NORM_THRESHOLD_F0` reader - ZQ START number of long counts until the normal priority request is asserted for frequency copy 0. This value should be scaled based on the number of ranks (chip selects) the controller handles. The more chip selects there are, the more rotations there are to go through, and the smaller this threshold should be."]
pub type ZqCalstartNormThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CALSTART_NORM_THRESHOLD_F0` writer - ZQ START number of long counts until the normal priority request is asserted for frequency copy 0. This value should be scaled based on the number of ranks (chip selects) the controller handles. The more chip selects there are, the more rotations there are to go through, and the smaller this threshold should be."]
pub type ZqCalstartNormThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - AREF number of posted refreshes until the maximum number of refresh credits has been reached."]
    #[inline(always)]
    pub fn aref_max_credit(&self) -> ArefMaxCreditR {
        ArefMaxCreditR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:23 - ZQ START number of long counts until the normal priority request is asserted for frequency copy 0. This value should be scaled based on the number of ranks (chip selects) the controller handles. The more chip selects there are, the more rotations there are to go through, and the smaller this threshold should be."]
    #[inline(always)]
    pub fn zq_calstart_norm_threshold_f0(&self) -> ZqCalstartNormThresholdF0R {
        ZqCalstartNormThresholdF0R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - AREF number of posted refreshes until the maximum number of refresh credits has been reached."]
    #[inline(always)]
    #[must_use]
    pub fn aref_max_credit(&mut self) -> ArefMaxCreditW<DenaliCtl165Spec> {
        ArefMaxCreditW::new(self, 0)
    }
    #[doc = "Bits 8:23 - ZQ START number of long counts until the normal priority request is asserted for frequency copy 0. This value should be scaled based on the number of ranks (chip selects) the controller handles. The more chip selects there are, the more rotations there are to go through, and the smaller this threshold should be."]
    #[inline(always)]
    #[must_use]
    pub fn zq_calstart_norm_threshold_f0(
        &mut self,
    ) -> ZqCalstartNormThresholdF0W<DenaliCtl165Spec> {
        ZqCalstartNormThresholdF0W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_165::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_165::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl165Spec;
impl crate::RegisterSpec for DenaliCtl165Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_165::R`](R) reader structure"]
impl crate::Readable for DenaliCtl165Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_165::W`](W) writer structure"]
impl crate::Writable for DenaliCtl165Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_165 to value 0"]
impl crate::Resettable for DenaliCtl165Spec {
    const RESET_VALUE: u32 = 0;
}
