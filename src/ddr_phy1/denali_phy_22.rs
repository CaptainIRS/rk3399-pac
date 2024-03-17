#[doc = "Register `DENALI_PHY_22` reader"]
pub type R = crate::R<DenaliPhy22Spec>;
#[doc = "Register `DENALI_PHY_22` writer"]
pub type W = crate::W<DenaliPhy22Spec>;
#[doc = "Field `PHY_LVL_DEBUG_MODE_0` reader - Enables leveling debug mode for slice 0. Set to 1 to enable."]
pub type PhyLvlDebugMode0R = crate::BitReader;
#[doc = "Field `PHY_LVL_DEBUG_MODE_0` writer - Enables leveling debug mode for slice 0. Set to 1 to enable."]
pub type PhyLvlDebugMode0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC_PHY_LVL_DEBUG_CONT_0` writer - Allows the leveling state machine to advance (when in debug mode) for slice 0. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyLvlDebugCont0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_WRLVL_CAPTURE_CNT_0` reader - Number of samples to take at each DQS slave delay setting during write leveling for slice 0."]
pub type PhyWrlvlCaptureCnt0R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_CAPTURE_CNT_0` writer - Number of samples to take at each DQS slave delay setting during write leveling for slice 0."]
pub type PhyWrlvlCaptureCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_WRLVL_UPDT_WAIT_CNT_0` reader - Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 0."]
pub type PhyWrlvlUpdtWaitCnt0R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_UPDT_WAIT_CNT_0` writer - Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 0."]
pub type PhyWrlvlUpdtWaitCnt0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Enables leveling debug mode for slice 0. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_lvl_debug_mode_0(&self) -> PhyLvlDebugMode0R {
        PhyLvlDebugMode0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:21 - Number of samples to take at each DQS slave delay setting during write leveling for slice 0."]
    #[inline(always)]
    pub fn phy_wrlvl_capture_cnt_0(&self) -> PhyWrlvlCaptureCnt0R {
        PhyWrlvlCaptureCnt0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 0."]
    #[inline(always)]
    pub fn phy_wrlvl_updt_wait_cnt_0(&self) -> PhyWrlvlUpdtWaitCnt0R {
        PhyWrlvlUpdtWaitCnt0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables leveling debug mode for slice 0. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lvl_debug_mode_0(&mut self) -> PhyLvlDebugMode0W<DenaliPhy22Spec> {
        PhyLvlDebugMode0W::new(self, 0)
    }
    #[doc = "Bit 8 - Allows the leveling state machine to advance (when in debug mode) for slice 0. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_lvl_debug_cont_0(&mut self) -> ScPhyLvlDebugCont0W<DenaliPhy22Spec> {
        ScPhyLvlDebugCont0W::new(self, 8)
    }
    #[doc = "Bits 16:21 - Number of samples to take at each DQS slave delay setting during write leveling for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_capture_cnt_0(&mut self) -> PhyWrlvlCaptureCnt0W<DenaliPhy22Spec> {
        PhyWrlvlCaptureCnt0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Number of cycles to wait after changing DQS slave delay setting during write leveling for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wrlvl_updt_wait_cnt_0(&mut self) -> PhyWrlvlUpdtWaitCnt0W<DenaliPhy22Spec> {
        PhyWrlvlUpdtWaitCnt0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_22::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_22::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy22Spec;
impl crate::RegisterSpec for DenaliPhy22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_22::R`](R) reader structure"]
impl crate::Readable for DenaliPhy22Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_22::W`](W) writer structure"]
impl crate::Writable for DenaliPhy22Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_22 to value 0"]
impl crate::Resettable for DenaliPhy22Spec {
    const RESET_VALUE: u32 = 0;
}
