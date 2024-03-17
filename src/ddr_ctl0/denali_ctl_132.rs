#[doc = "Register `DENALI_CTL_132` reader"]
pub type R = crate::R<DenaliCtl132Spec>;
#[doc = "Register `DENALI_CTL_132` writer"]
pub type W = crate::W<DenaliCtl132Spec>;
#[doc = "Field `MRW_PROMOTE_THRESHOLD_F1` reader - MRW promotion number of long counts until the high priority request is asserted for frequency copy 1. Applies to SW MRW commands."]
pub type MrwPromoteThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `MRW_PROMOTE_THRESHOLD_F1` writer - MRW promotion number of long counts until the high priority request is asserted for frequency copy 1. Applies to SW MRW commands."]
pub type MrwPromoteThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MRW_PROMOTE_THRESHOLD_F2` reader - MRW promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW MRW commands."]
pub type MrwPromoteThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `MRW_PROMOTE_THRESHOLD_F2` writer - MRW promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW MRW commands."]
pub type MrwPromoteThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MRW promotion number of long counts until the high priority request is asserted for frequency copy 1. Applies to SW MRW commands."]
    #[inline(always)]
    pub fn mrw_promote_threshold_f1(&self) -> MrwPromoteThresholdF1R {
        MrwPromoteThresholdF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - MRW promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW MRW commands."]
    #[inline(always)]
    pub fn mrw_promote_threshold_f2(&self) -> MrwPromoteThresholdF2R {
        MrwPromoteThresholdF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MRW promotion number of long counts until the high priority request is asserted for frequency copy 1. Applies to SW MRW commands."]
    #[inline(always)]
    #[must_use]
    pub fn mrw_promote_threshold_f1(&mut self) -> MrwPromoteThresholdF1W<DenaliCtl132Spec> {
        MrwPromoteThresholdF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - MRW promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW MRW commands."]
    #[inline(always)]
    #[must_use]
    pub fn mrw_promote_threshold_f2(&mut self) -> MrwPromoteThresholdF2W<DenaliCtl132Spec> {
        MrwPromoteThresholdF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_132::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_132::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl132Spec;
impl crate::RegisterSpec for DenaliCtl132Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_132::R`](R) reader structure"]
impl crate::Readable for DenaliCtl132Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_132::W`](W) writer structure"]
impl crate::Writable for DenaliCtl132Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_132 to value 0"]
impl crate::Resettable for DenaliCtl132Spec {
    const RESET_VALUE: u32 = 0;
}
