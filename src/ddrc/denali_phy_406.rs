#[doc = "Register `DENALI_PHY_406` reader"]
pub type R = crate::R<DenaliPhy406Spec>;
#[doc = "Register `DENALI_PHY_406` writer"]
pub type W = crate::W<DenaliPhy406Spec>;
#[doc = "Field `PHY_LVL_DEBUG_MODE_3` reader - Enables leveling debug mode for slice 3. Set to 1 to enable."]
pub type PhyLvlDebugMode3R = crate::BitReader;
#[doc = "Field `PHY_LVL_DEBUG_MODE_3` writer - Enables leveling debug mode for slice 3. Set to 1 to enable."]
pub type PhyLvlDebugMode3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC_PHY_LVL_DEBUG_CONT_3` writer - Allows the leveling state machine to advance (when in debug mode) for slice 3. Set to 1 to trigger."]
pub type ScPhyLvlDebugCont3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_WRLVL_CAPTURE_CNT_3` reader - Number of samples to take at each DQS slave delay setting during write leveling for slice 3."]
pub type PhyWrlvlCaptureCnt3R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_CAPTURE_CNT_3` writer - Number of samples to take at each DQS slave delay setting during write leveling for slice 3."]
pub type PhyWrlvlCaptureCnt3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_WRLVL_UPDT_WAIT_CNT_3` reader - Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 3."]
pub type PhyWrlvlUpdtWaitCnt3R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_UPDT_WAIT_CNT_3` writer - Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 3."]
pub type PhyWrlvlUpdtWaitCnt3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Enables leveling debug mode for slice 3. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_lvl_debug_mode_3(&self) -> PhyLvlDebugMode3R {
        PhyLvlDebugMode3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:21 - Number of samples to take at each DQS slave delay setting during write leveling for slice 3."]
    #[inline(always)]
    pub fn phy_wrlvl_capture_cnt_3(&self) -> PhyWrlvlCaptureCnt3R {
        PhyWrlvlCaptureCnt3R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 3."]
    #[inline(always)]
    pub fn phy_wrlvl_updt_wait_cnt_3(&self) -> PhyWrlvlUpdtWaitCnt3R {
        PhyWrlvlUpdtWaitCnt3R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables leveling debug mode for slice 3. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lvl_debug_mode_3(&mut self) -> PhyLvlDebugMode3W<DenaliPhy406Spec> {
        PhyLvlDebugMode3W::new(self, 0)
    }
    #[doc = "Bit 8 - Allows the leveling state machine to advance (when in debug mode) for slice 3. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_lvl_debug_cont_3(&mut self) -> ScPhyLvlDebugCont3W<DenaliPhy406Spec> {
        ScPhyLvlDebugCont3W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Number of samples to take at each DQS slave delay setting during write leveling for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_capture_cnt_3(&mut self) -> PhyWrlvlCaptureCnt3W<DenaliPhy406Spec> {
        PhyWrlvlCaptureCnt3W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_updt_wait_cnt_3(&mut self) -> PhyWrlvlUpdtWaitCnt3W<DenaliPhy406Spec> {
        PhyWrlvlUpdtWaitCnt3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_406::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_406::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy406Spec;
impl crate::RegisterSpec for DenaliPhy406Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_406::R`](R) reader structure"]
impl crate::Readable for DenaliPhy406Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_406::W`](W) writer structure"]
impl crate::Writable for DenaliPhy406Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_406 to value 0"]
impl crate::Resettable for DenaliPhy406Spec {
    const RESET_VALUE: u32 = 0;
}
