#[doc = "Register `DENALI_CTL_69` reader"]
pub type R = crate::R<DenaliCtl69Spec>;
#[doc = "Register `DENALI_CTL_69` writer"]
pub type W = crate::W<DenaliCtl69Spec>;
#[doc = "Field `ENABLE_QUICK_SREFRESH` reader - Allow user to interrupt memory initialization to enter self-refresh mode. Set to 1 to allow interruption."]
pub type EnableQuickSrefreshR = crate::BitReader;
#[doc = "Field `ENABLE_QUICK_SREFRESH` writer - Allow user to interrupt memory initialization to enter self-refresh mode. Set to 1 to allow interruption."]
pub type EnableQuickSrefreshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKE_DELAY` reader - Additional cycles to delay CKE for status reporting."]
pub type CkeDelayR = crate::FieldReader;
#[doc = "Field `CKE_DELAY` writer - Additional cycles to delay CKE for status reporting."]
pub type CkeDelayW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DFS_STATUS` reader - Holds the error associated with the DFS interrupt. Bit (0) set indicates an illegal command and bit (1) set indicates that a shutdown occurred during DFS."]
pub type DfsStatusR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Allow user to interrupt memory initialization to enter self-refresh mode. Set to 1 to allow interruption."]
    #[inline(always)]
    pub fn enable_quick_srefresh(&self) -> EnableQuickSrefreshR {
        EnableQuickSrefreshR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - Additional cycles to delay CKE for status reporting."]
    #[inline(always)]
    pub fn cke_delay(&self) -> CkeDelayR {
        CkeDelayR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Holds the error associated with the DFS interrupt. Bit (0) set indicates an illegal command and bit (1) set indicates that a shutdown occurred during DFS."]
    #[inline(always)]
    pub fn dfs_status(&self) -> DfsStatusR {
        DfsStatusR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Allow user to interrupt memory initialization to enter self-refresh mode. Set to 1 to allow interruption."]
    #[inline(always)]
    #[must_use]
    pub fn enable_quick_srefresh(&mut self) -> EnableQuickSrefreshW<DenaliCtl69Spec> {
        EnableQuickSrefreshW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Additional cycles to delay CKE for status reporting."]
    #[inline(always)]
    #[must_use]
    pub fn cke_delay(&mut self) -> CkeDelayW<DenaliCtl69Spec> {
        CkeDelayW::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_69::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_69::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl69Spec;
impl crate::RegisterSpec for DenaliCtl69Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_69::R`](R) reader structure"]
impl crate::Readable for DenaliCtl69Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_69::W`](W) writer structure"]
impl crate::Writable for DenaliCtl69Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_69 to value 0"]
impl crate::Resettable for DenaliCtl69Spec {
    const RESET_VALUE: u32 = 0;
}
