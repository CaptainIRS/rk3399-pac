#[doc = "Register `DDR_DENALI_CTL_72` reader"]
pub type R = crate::R<DdrDenaliCtl72Spec>;
#[doc = "Register `DDR_DENALI_CTL_72` writer"]
pub type W = crate::W<DdrDenaliCtl72Spec>;
#[doc = "Field `DFS_PROMOTE_THRESHOLD_F1` reader - DFS promotion number of long counts until the high priority request is asserted for frequency copy 1. Applies to SW and HW DFS commands."]
pub type DfsPromoteThresholdF1R = crate::FieldReader<u16>;
#[doc = "Field `DFS_PROMOTE_THRESHOLD_F1` writer - DFS promotion number of long counts until the high priority request is asserted for frequency copy 1. Applies to SW and HW DFS commands."]
pub type DfsPromoteThresholdF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DFS_PROMOTE_THRESHOLD_F2` reader - DFS promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and HW DFS commands."]
pub type DfsPromoteThresholdF2R = crate::FieldReader<u16>;
#[doc = "Field `DFS_PROMOTE_THRESHOLD_F2` writer - DFS promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and HW DFS commands."]
pub type DfsPromoteThresholdF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DFS promotion number of long counts until the high priority request is asserted for frequency copy 1. Applies to SW and HW DFS commands."]
    #[inline(always)]
    pub fn dfs_promote_threshold_f1(&self) -> DfsPromoteThresholdF1R {
        DfsPromoteThresholdF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - DFS promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and HW DFS commands."]
    #[inline(always)]
    pub fn dfs_promote_threshold_f2(&self) -> DfsPromoteThresholdF2R {
        DfsPromoteThresholdF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DFS promotion number of long counts until the high priority request is asserted for frequency copy 1. Applies to SW and HW DFS commands."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_promote_threshold_f1(&mut self) -> DfsPromoteThresholdF1W<DdrDenaliCtl72Spec> {
        DfsPromoteThresholdF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - DFS promotion number of long counts until the high priority request is asserted for frequency copy 2. Applies to SW and HW DFS commands."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_promote_threshold_f2(&mut self) -> DfsPromoteThresholdF2W<DdrDenaliCtl72Spec> {
        DfsPromoteThresholdF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_72::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_72::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl72Spec;
impl crate::RegisterSpec for DdrDenaliCtl72Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_72::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl72Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_72::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl72Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_72 to value 0"]
impl crate::Resettable for DdrDenaliCtl72Spec {
    const RESET_VALUE: u32 = 0;
}
