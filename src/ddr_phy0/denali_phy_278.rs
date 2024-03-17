#[doc = "Register `DENALI_PHY_278` reader"]
pub type R = crate::R<DenaliPhy278Spec>;
#[doc = "Register `DENALI_PHY_278` writer"]
pub type W = crate::W<DenaliPhy278Spec>;
#[doc = "Field `PHY_LVL_DEBUG_MODE_2` reader - Enables leveling debug mode for slice 2. Set to 1 to enable."]
pub type PhyLvlDebugMode2R = crate::BitReader;
#[doc = "Field `PHY_LVL_DEBUG_MODE_2` writer - Enables leveling debug mode for slice 2. Set to 1 to enable."]
pub type PhyLvlDebugMode2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC_PHY_LVL_DEBUG_CONT_2` writer - Allows the leveling state machine to advance (when in debug mode) for slice 2. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyLvlDebugCont2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_WRLVL_CAPTURE_CNT_2` reader - Number of samples to take at each DQS slave delay setting during write leveling for slice 2."]
pub type PhyWrlvlCaptureCnt2R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_CAPTURE_CNT_2` writer - Number of samples to take at each DQS slave delay setting during write leveling for slice 2."]
pub type PhyWrlvlCaptureCnt2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_WRLVL_UPDT_WAIT_CNT_2` reader - Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 2."]
pub type PhyWrlvlUpdtWaitCnt2R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_UPDT_WAIT_CNT_2` writer - Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 2."]
pub type PhyWrlvlUpdtWaitCnt2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Enables leveling debug mode for slice 2. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_lvl_debug_mode_2(&self) -> PhyLvlDebugMode2R {
        PhyLvlDebugMode2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:21 - Number of samples to take at each DQS slave delay setting during write leveling for slice 2."]
    #[inline(always)]
    pub fn phy_wrlvl_capture_cnt_2(&self) -> PhyWrlvlCaptureCnt2R {
        PhyWrlvlCaptureCnt2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 2."]
    #[inline(always)]
    pub fn phy_wrlvl_updt_wait_cnt_2(&self) -> PhyWrlvlUpdtWaitCnt2R {
        PhyWrlvlUpdtWaitCnt2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables leveling debug mode for slice 2. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lvl_debug_mode_2(&mut self) -> PhyLvlDebugMode2W<DenaliPhy278Spec> {
        PhyLvlDebugMode2W::new(self, 0)
    }
    #[doc = "Bit 8 - Allows the leveling state machine to advance (when in debug mode) for slice 2. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_lvl_debug_cont_2(&mut self) -> ScPhyLvlDebugCont2W<DenaliPhy278Spec> {
        ScPhyLvlDebugCont2W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Number of samples to take at each DQS slave delay setting during write leveling for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_capture_cnt_2(&mut self) -> PhyWrlvlCaptureCnt2W<DenaliPhy278Spec> {
        PhyWrlvlCaptureCnt2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_updt_wait_cnt_2(&mut self) -> PhyWrlvlUpdtWaitCnt2W<DenaliPhy278Spec> {
        PhyWrlvlUpdtWaitCnt2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_278::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_278::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy278Spec;
impl crate::RegisterSpec for DenaliPhy278Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_278::R`](R) reader structure"]
impl crate::Readable for DenaliPhy278Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_278::W`](W) writer structure"]
impl crate::Writable for DenaliPhy278Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_278 to value 0"]
impl crate::Resettable for DenaliPhy278Spec {
    const RESET_VALUE: u32 = 0;
}
