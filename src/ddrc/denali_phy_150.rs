#[doc = "Register `DENALI_PHY_150` reader"]
pub type R = crate::R<DenaliPhy150Spec>;
#[doc = "Register `DENALI_PHY_150` writer"]
pub type W = crate::W<DenaliPhy150Spec>;
#[doc = "Field `PHY_LVL_DEBUG_MODE_1` reader - Enables leveling debug mode for slice 1. Set to 1 to enable."]
pub type PhyLvlDebugMode1R = crate::BitReader;
#[doc = "Field `PHY_LVL_DEBUG_MODE_1` writer - Enables leveling debug mode for slice 1. Set to 1 to enable."]
pub type PhyLvlDebugMode1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC_PHY_LVL_DEBUG_CONT_1` writer - Allows the leveling state machine to advance (when in debug mode) for slice 1. Set to 1 to trigger."]
pub type ScPhyLvlDebugCont1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_WRLVL_CAPTURE_CNT_1` reader - Number of samples to take at each DQS slave delay setting during write leveling for slice 1."]
pub type PhyWrlvlCaptureCnt1R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_CAPTURE_CNT_1` writer - Number of samples to take at each DQS slave delay setting during write leveling for slice 1."]
pub type PhyWrlvlCaptureCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_WRLVL_UPDT_WAIT_CNT_1` reader - Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 1."]
pub type PhyWrlvlUpdtWaitCnt1R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_UPDT_WAIT_CNT_1` writer - Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 1."]
pub type PhyWrlvlUpdtWaitCnt1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Enables leveling debug mode for slice 1. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_lvl_debug_mode_1(&self) -> PhyLvlDebugMode1R {
        PhyLvlDebugMode1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:21 - Number of samples to take at each DQS slave delay setting during write leveling for slice 1."]
    #[inline(always)]
    pub fn phy_wrlvl_capture_cnt_1(&self) -> PhyWrlvlCaptureCnt1R {
        PhyWrlvlCaptureCnt1R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 1."]
    #[inline(always)]
    pub fn phy_wrlvl_updt_wait_cnt_1(&self) -> PhyWrlvlUpdtWaitCnt1R {
        PhyWrlvlUpdtWaitCnt1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables leveling debug mode for slice 1. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lvl_debug_mode_1(&mut self) -> PhyLvlDebugMode1W<DenaliPhy150Spec> {
        PhyLvlDebugMode1W::new(self, 0)
    }
    #[doc = "Bit 8 - Allows the leveling state machine to advance (when in debug mode) for slice 1. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_lvl_debug_cont_1(&mut self) -> ScPhyLvlDebugCont1W<DenaliPhy150Spec> {
        ScPhyLvlDebugCont1W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Number of samples to take at each DQS slave delay setting during write leveling for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_capture_cnt_1(&mut self) -> PhyWrlvlCaptureCnt1W<DenaliPhy150Spec> {
        PhyWrlvlCaptureCnt1W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_updt_wait_cnt_1(&mut self) -> PhyWrlvlUpdtWaitCnt1W<DenaliPhy150Spec> {
        PhyWrlvlUpdtWaitCnt1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_150::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_150::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy150Spec;
impl crate::RegisterSpec for DenaliPhy150Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_150::R`](R) reader structure"]
impl crate::Readable for DenaliPhy150Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_150::W`](W) writer structure"]
impl crate::Writable for DenaliPhy150Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_150 to value 0"]
impl crate::Resettable for DenaliPhy150Spec {
    const RESET_VALUE: u32 = 0;
}
