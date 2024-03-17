#[doc = "Register `DENALI_PHY_25` reader"]
pub type R = crate::R<DenaliPhy25Spec>;
#[doc = "Register `DENALI_PHY_25` writer"]
pub type W = crate::W<DenaliPhy25Spec>;
#[doc = "Field `PHY_WDQLVL_PATT_0` reader - Defines the training patterns to be used during the write data leveling sequence for slice 0. Bit (0) corresponds to the LFSR data training pattern. Bit (1) corresponds to the CLK data training pattern. Bit (2) corresponds to user-defined data pattern training. If multiple bits are set, the training for each of the chosen patterns will be executed and the settings that give the smallest data valid window eye will be chosen."]
pub type PhyWdqlvlPatt0R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_PATT_0` writer - Defines the training patterns to be used during the write data leveling sequence for slice 0. Bit (0) corresponds to the LFSR data training pattern. Bit (1) corresponds to the CLK data training pattern. Bit (2) corresponds to user-defined data pattern training. If multiple bits are set, the training for each of the chosen patterns will be executed and the settings that give the smallest data valid window eye will be chosen."]
pub type PhyWdqlvlPatt0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_WDQLVL_DQDM_SLV_DLY_JUMP_OFFSET_0` reader - Defines the write/read burst length in bytes during the write data leveling sequence for slice 0."]
pub type PhyWdqlvlDqdmSlvDlyJumpOffset0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WDQLVL_DQDM_SLV_DLY_JUMP_OFFSET_0` writer - Defines the write/read burst length in bytes during the write data leveling sequence for slice 0."]
pub type PhyWdqlvlDqdmSlvDlyJumpOffset0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_WDQLVL_UPDT_WAIT_CNT_0` reader - Number of cycles to wait after changing the DQ slave delay setting during write data leveling for slice 0."]
pub type PhyWdqlvlUpdtWaitCnt0R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_UPDT_WAIT_CNT_0` writer - Number of cycles to wait after changing the DQ slave delay setting during write data leveling for slice 0."]
pub type PhyWdqlvlUpdtWaitCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Defines the training patterns to be used during the write data leveling sequence for slice 0. Bit (0) corresponds to the LFSR data training pattern. Bit (1) corresponds to the CLK data training pattern. Bit (2) corresponds to user-defined data pattern training. If multiple bits are set, the training for each of the chosen patterns will be executed and the settings that give the smallest data valid window eye will be chosen."]
    #[inline(always)]
    pub fn phy_wdqlvl_patt_0(&self) -> PhyWdqlvlPatt0R {
        PhyWdqlvlPatt0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:18 - Defines the write/read burst length in bytes during the write data leveling sequence for slice 0."]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_slv_dly_jump_offset_0(&self) -> PhyWdqlvlDqdmSlvDlyJumpOffset0R {
        PhyWdqlvlDqdmSlvDlyJumpOffset0R::new(((self.bits >> 8) & 0x07ff) as u16)
    }
    #[doc = "Bits 24:27 - Number of cycles to wait after changing the DQ slave delay setting during write data leveling for slice 0."]
    #[inline(always)]
    pub fn phy_wdqlvl_updt_wait_cnt_0(&self) -> PhyWdqlvlUpdtWaitCnt0R {
        PhyWdqlvlUpdtWaitCnt0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Defines the training patterns to be used during the write data leveling sequence for slice 0. Bit (0) corresponds to the LFSR data training pattern. Bit (1) corresponds to the CLK data training pattern. Bit (2) corresponds to user-defined data pattern training. If multiple bits are set, the training for each of the chosen patterns will be executed and the settings that give the smallest data valid window eye will be chosen."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_patt_0(&mut self) -> PhyWdqlvlPatt0W<DenaliPhy25Spec> {
        PhyWdqlvlPatt0W::new(self, 0)
    }
    #[doc = "Bits 8:18 - Defines the write/read burst length in bytes during the write data leveling sequence for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dqdm_slv_dly_jump_offset_0(
        &mut self,
    ) -> PhyWdqlvlDqdmSlvDlyJumpOffset0W<DenaliPhy25Spec> {
        PhyWdqlvlDqdmSlvDlyJumpOffset0W::new(self, 8)
    }
    #[doc = "Bits 24:27 - Number of cycles to wait after changing the DQ slave delay setting during write data leveling for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_updt_wait_cnt_0(&mut self) -> PhyWdqlvlUpdtWaitCnt0W<DenaliPhy25Spec> {
        PhyWdqlvlUpdtWaitCnt0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_25::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_25::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy25Spec;
impl crate::RegisterSpec for DenaliPhy25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_25::R`](R) reader structure"]
impl crate::Readable for DenaliPhy25Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_25::W`](W) writer structure"]
impl crate::Writable for DenaliPhy25Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_25 to value 0"]
impl crate::Resettable for DenaliPhy25Spec {
    const RESET_VALUE: u32 = 0;
}
