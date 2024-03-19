#[doc = "Register `DDR_DENALI_CTL_131` reader"]
pub type R = crate::R<DdrDenaliCtl131Spec>;
#[doc = "Register `DDR_DENALI_CTL_131` writer"]
pub type W = crate::W<DdrDenaliCtl131Spec>;
#[doc = "Field `MRR_PROMOTE_THRESHOLD_F2` reader - MRR promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW MRR commands."]
pub type MrrPromoteThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `MRR_PROMOTE_THRESHOLD_F2` writer - MRR promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW MRR commands."]
pub type MrrPromoteThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MRW_PROMOTE_THRESHOLD_F0` reader - MRW promotion number of long counts until the high priority request is asserted for frequency copy 0. Applies to SW MRW commands."]
pub type MrwPromoteThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `MRW_PROMOTE_THRESHOLD_F0` writer - MRW promotion number of long counts until the high priority request is asserted for frequency copy 0. Applies to SW MRW commands."]
pub type MrwPromoteThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MRR promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW MRR commands."]
    #[inline(always)]
    pub fn mrr_promote_threshold_f2(&self) -> MrrPromoteThresholdF2R {
        MrrPromoteThresholdF2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - MRW promotion number of long counts until the high priority request is asserted for frequency copy 0. Applies to SW MRW commands."]
    #[inline(always)]
    pub fn mrw_promote_threshold_f0(&self) -> MrwPromoteThresholdF0R {
        MrwPromoteThresholdF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MRR promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW MRR commands."]
    #[inline(always)]
    #[must_use]
    pub fn mrr_promote_threshold_f2(&mut self) -> MrrPromoteThresholdF2W<DdrDenaliCtl131Spec> {
        MrrPromoteThresholdF2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - MRW promotion number of long counts until the high priority request is asserted for frequency copy 0. Applies to SW MRW commands."]
    #[inline(always)]
    #[must_use]
    pub fn mrw_promote_threshold_f0(&mut self) -> MrwPromoteThresholdF0W<DdrDenaliCtl131Spec> {
        MrwPromoteThresholdF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_131::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_131::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl131Spec;
impl crate::RegisterSpec for DdrDenaliCtl131Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_131::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl131Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_131::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl131Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_131 to value 0"]
impl crate::Resettable for DdrDenaliCtl131Spec {
    const RESET_VALUE: u32 = 0;
}
