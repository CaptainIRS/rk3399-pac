#[doc = "Register `DENALI_CTL_71` reader"]
pub type R = crate::R<DenaliCtl71Spec>;
#[doc = "Register `DENALI_CTL_71` writer"]
pub type W = crate::W<DenaliCtl71Spec>;
#[doc = "Field `DFS_RDLVL_GATE_EN` reader - Enables read gate training during a DFS exit. Set to 1 to enable."]
pub type DfsRdlvlGateEnR = crate::BitReader;
#[doc = "Field `DFS_RDLVL_GATE_EN` writer - Enables read gate training during a DFS exit. Set to 1 to enable."]
pub type DfsRdlvlGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFS_PROMOTE_THRESHOLD_F0` reader - DFS promotion number of long counts until the high priority request is asserted for frequency copy 0. Applies to SW and HW DFS commands."]
pub type DfsPromoteThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `DFS_PROMOTE_THRESHOLD_F0` writer - DFS promotion number of long counts until the high priority request is asserted for frequency copy 0. Applies to SW and HW DFS commands."]
pub type DfsPromoteThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Enables read gate training during a DFS exit. Set to 1 to enable."]
    #[inline(always)]
    pub fn dfs_rdlvl_gate_en(&self) -> DfsRdlvlGateEnR {
        DfsRdlvlGateEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:23 - DFS promotion number of long counts until the high priority request is asserted for frequency copy 0. Applies to SW and HW DFS commands."]
    #[inline(always)]
    pub fn dfs_promote_threshold_f0(&self) -> DfsPromoteThresholdF0R {
        DfsPromoteThresholdF0R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enables read gate training during a DFS exit. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_rdlvl_gate_en(&mut self) -> DfsRdlvlGateEnW<DenaliCtl71Spec> {
        DfsRdlvlGateEnW::new(self, 0)
    }
    #[doc = "Bits 8:23 - DFS promotion number of long counts until the high priority request is asserted for frequency copy 0. Applies to SW and HW DFS commands."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_promote_threshold_f0(&mut self) -> DfsPromoteThresholdF0W<DenaliCtl71Spec> {
        DfsPromoteThresholdF0W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_71::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_71::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl71Spec;
impl crate::RegisterSpec for DenaliCtl71Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_71::R`](R) reader structure"]
impl crate::Readable for DenaliCtl71Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_71::W`](W) writer structure"]
impl crate::Writable for DenaliCtl71Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_71 to value 0"]
impl crate::Resettable for DenaliCtl71Spec {
    const RESET_VALUE: u32 = 0;
}
