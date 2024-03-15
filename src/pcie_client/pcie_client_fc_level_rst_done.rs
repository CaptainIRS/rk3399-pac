#[doc = "Register `PCIE_CLIENT_FC_LEVEL_RST_DONE` writer"]
pub type W = crate::W<PcieClientFcLevelRstDoneSpec>;
#[doc = "Field `FLR_DONE` writer - Physical function level reset done pulse generate The client must assert bit i of this bus when it has completed the reset operation of Function i. This causes the core to de-assert FLR_IN_PROGRESS for Function i and to re-enable configuration accesses to the Function. Write one to generate one high pulse."]
pub type FlrDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VF_FLR_DONE` writer - Virtual function level reset done pulse generate The client must assert bit i of this bus when it has completed the reset operation of Virtual Function i. This causes the core to de- assert FLR_IN_PROGRESS for VF i and to re-enable configuration accesses to the VF. Write one to generate one high pulse."]
pub type VfFlrDoneW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bit 0 - Physical function level reset done pulse generate The client must assert bit i of this bus when it has completed the reset operation of Function i. This causes the core to de-assert FLR_IN_PROGRESS for Function i and to re-enable configuration accesses to the Function. Write one to generate one high pulse."]
    #[inline(always)]
    #[must_use]
    pub fn flr_done(&mut self) -> FlrDoneW<PcieClientFcLevelRstDoneSpec> {
        FlrDoneW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Virtual function level reset done pulse generate The client must assert bit i of this bus when it has completed the reset operation of Virtual Function i. This causes the core to de- assert FLR_IN_PROGRESS for VF i and to re-enable configuration accesses to the VF. Write one to generate one high pulse."]
    #[inline(always)]
    #[must_use]
    pub fn vf_flr_done(&mut self) -> VfFlrDoneW<PcieClientFcLevelRstDoneSpec> {
        VfFlrDoneW::new(self, 8)
    }
}
#[doc = "Generate function level reset done pulse\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_fc_level_rst_done::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientFcLevelRstDoneSpec;
impl crate::RegisterSpec for PcieClientFcLevelRstDoneSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pcie_client_fc_level_rst_done::W`](W) writer structure"]
impl crate::Writable for PcieClientFcLevelRstDoneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_CLIENT_FC_LEVEL_RST_DONE to value 0"]
impl crate::Resettable for PcieClientFcLevelRstDoneSpec {
    const RESET_VALUE: u32 = 0;
}
