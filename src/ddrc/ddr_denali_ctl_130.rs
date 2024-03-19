#[doc = "Register `DDR_DENALI_CTL_130` reader"]
pub type R = crate::R<DdrDenaliCtl130Spec>;
#[doc = "Register `DDR_DENALI_CTL_130` writer"]
pub type W = crate::W<DdrDenaliCtl130Spec>;
#[doc = "Field `MRR_PROMOTE_THRESHOLD_F0` reader - MRR promotion number of long counts until the high priority request is asserted for frequency copy 0. Applies to SW MRR commands."]
pub type MrrPromoteThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `MRR_PROMOTE_THRESHOLD_F0` writer - MRR promotion number of long counts until the high priority request is asserted for frequency copy 0. Applies to SW MRR commands."]
pub type MrrPromoteThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MRR_PROMOTE_THRESHOLD_F1` reader - MRR promotion number of long counts until the high priority request is asserted for frequency copy 1. Applies to SW MRR commands."]
pub type MrrPromoteThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `MRR_PROMOTE_THRESHOLD_F1` writer - MRR promotion number of long counts until the high priority request is asserted for frequency copy 1. Applies to SW MRR commands."]
pub type MrrPromoteThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MRR promotion number of long counts until the high priority request is asserted for frequency copy 0. Applies to SW MRR commands."]
    #[inline(always)]
    pub fn mrr_promote_threshold_f0(&self) -> MrrPromoteThresholdF0R {
        MrrPromoteThresholdF0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - MRR promotion number of long counts until the high priority request is asserted for frequency copy 1. Applies to SW MRR commands."]
    #[inline(always)]
    pub fn mrr_promote_threshold_f1(&self) -> MrrPromoteThresholdF1R {
        MrrPromoteThresholdF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MRR promotion number of long counts until the high priority request is asserted for frequency copy 0. Applies to SW MRR commands."]
    #[inline(always)]
    #[must_use]
    pub fn mrr_promote_threshold_f0(&mut self) -> MrrPromoteThresholdF0W<DdrDenaliCtl130Spec> {
        MrrPromoteThresholdF0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - MRR promotion number of long counts until the high priority request is asserted for frequency copy 1. Applies to SW MRR commands."]
    #[inline(always)]
    #[must_use]
    pub fn mrr_promote_threshold_f1(&mut self) -> MrrPromoteThresholdF1W<DdrDenaliCtl130Spec> {
        MrrPromoteThresholdF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_130::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_130::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl130Spec;
impl crate::RegisterSpec for DdrDenaliCtl130Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_130::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl130Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_130::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl130Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_130 to value 0"]
impl crate::Resettable for DdrDenaliCtl130Spec {
    const RESET_VALUE: u32 = 0;
}
