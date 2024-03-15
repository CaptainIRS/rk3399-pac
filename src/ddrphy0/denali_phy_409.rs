#[doc = "Register `DENALI_PHY_409` reader"]
pub type R = crate::R<DenaliPhy409Spec>;
#[doc = "Register `DENALI_PHY_409` writer"]
pub type W = crate::W<DenaliPhy409Spec>;
#[doc = "Field `PHY_WDQLVL_PATT_3` reader - Defines the training patterns to be used during the write data leveling sequence for slice 3. Bit (0) corresponds to the LFSR data training pattern. Bit (1) corresponds to the CLK data training pattern. Bit (2) corresponds to user-defined data pattern training. If multiple bits are set, the training for each of the chosen patterns will be executed and the settings that give the smallest data valid window eye will be chosen."]
pub type PhyWdqlvlPatt3R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_PATT_3` writer - Defines the training patterns to be used during the write data leveling sequence for slice 3. Bit (0) corresponds to the LFSR data training pattern. Bit (1) corresponds to the CLK data training pattern. Bit (2) corresponds to user-defined data pattern training. If multiple bits are set, the training for each of the chosen patterns will be executed and the settings that give the smallest data valid window eye will be chosen."]
pub type PhyWdqlvlPatt3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHY_WDQLVL_DQDM_SLV_DLY_JUMP_OFFSET_3` reader - Defines the write/read burst length in bytes during the write data leveling sequence for slice 3."]
pub type PhyWdqlvlDqdmSlvDlyJumpOffset3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WDQLVL_DQDM_SLV_DLY_JUMP_OFFSET_3` writer - Defines the write/read burst length in bytes during the write data leveling sequence for slice 3."]
pub type PhyWdqlvlDqdmSlvDlyJumpOffset3W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_WDQLVL_UPDT_WAIT_CNT_3` reader - Number of cycles to wait after changing the DQ slave delay setting during write data leveling for slice 3."]
pub type PhyWdqlvlUpdtWaitCnt3R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_UPDT_WAIT_CNT_3` writer - Number of cycles to wait after changing the DQ slave delay setting during write data leveling for slice 3."]
pub type PhyWdqlvlUpdtWaitCnt3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Defines the training patterns to be used during the write data leveling sequence for slice 3. Bit (0) corresponds to the LFSR data training pattern. Bit (1) corresponds to the CLK data training pattern. Bit (2) corresponds to user-defined data pattern training. If multiple bits are set, the training for each of the chosen patterns will be executed and the settings that give the smallest data valid window eye will be chosen."]
    #[inline(always)]
    pub fn phy_wdqlvl_patt_3(&self) -> PhyWdqlvlPatt3R {
        PhyWdqlvlPatt3R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:18 - Defines the write/read burst length in bytes during the write data leveling sequence for slice 3."]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_slv_dly_jump_offset_3(&self) -> PhyWdqlvlDqdmSlvDlyJumpOffset3R {
        PhyWdqlvlDqdmSlvDlyJumpOffset3R::new(((self.bits >> 8) & 0x07ff) as u16)
    }
    #[doc = "Bits 24:27 - Number of cycles to wait after changing the DQ slave delay setting during write data leveling for slice 3."]
    #[inline(always)]
    pub fn phy_wdqlvl_updt_wait_cnt_3(&self) -> PhyWdqlvlUpdtWaitCnt3R {
        PhyWdqlvlUpdtWaitCnt3R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Defines the training patterns to be used during the write data leveling sequence for slice 3. Bit (0) corresponds to the LFSR data training pattern. Bit (1) corresponds to the CLK data training pattern. Bit (2) corresponds to user-defined data pattern training. If multiple bits are set, the training for each of the chosen patterns will be executed and the settings that give the smallest data valid window eye will be chosen."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_patt_3(&mut self) -> PhyWdqlvlPatt3W<DenaliPhy409Spec> {
        PhyWdqlvlPatt3W::new(self, 0)
    }
    #[doc = "Bits 8:18 - Defines the write/read burst length in bytes during the write data leveling sequence for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dqdm_slv_dly_jump_offset_3(
        &mut self,
    ) -> PhyWdqlvlDqdmSlvDlyJumpOffset3W<DenaliPhy409Spec> {
        PhyWdqlvlDqdmSlvDlyJumpOffset3W::new(self, 8)
    }
    #[doc = "Bits 24:27 - Number of cycles to wait after changing the DQ slave delay setting during write data leveling for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_updt_wait_cnt_3(&mut self) -> PhyWdqlvlUpdtWaitCnt3W<DenaliPhy409Spec> {
        PhyWdqlvlUpdtWaitCnt3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_409::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_409::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy409Spec;
impl crate::RegisterSpec for DenaliPhy409Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_409::R`](R) reader structure"]
impl crate::Readable for DenaliPhy409Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_409::W`](W) writer structure"]
impl crate::Writable for DenaliPhy409Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_409 to value 0"]
impl crate::Resettable for DenaliPhy409Spec {
    const RESET_VALUE: u32 = 0;
}
