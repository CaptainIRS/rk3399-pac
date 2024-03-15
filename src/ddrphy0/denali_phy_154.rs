#[doc = "Register `DENALI_PHY_154` reader"]
pub type R = crate::R<DenaliPhy154Spec>;
#[doc = "Register `DENALI_PHY_154` writer"]
pub type W = crate::W<DenaliPhy154Spec>;
#[doc = "Field `PHY_WDQLVL_DQDM_OBS_SELECT_1` reader - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 1."]
pub type PhyWdqlvlDqdmObsSelect1R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_DQDM_OBS_SELECT_1` writer - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 1."]
pub type PhyWdqlvlDqdmObsSelect1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WDQLVL_QTR_DLY_STEP_1` reader - Defines the step granularity for the logic to use once an edge is found. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
pub type PhyWdqlvlQtrDlyStep1R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_QTR_DLY_STEP_1` writer - Defines the step granularity for the logic to use once an edge is found. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
pub type PhyWdqlvlQtrDlyStep1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SC_PHY_WDQLVL_CLR_PREV_RESULTS_1` writer - Clears the previous result value to allow a clean slate comparison for future write DQ leveling results for slice 1. Set to 1 to trigger. WRITE- ONLY"]
pub type ScPhyWdqlvlClrPrevResults1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 1."]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_obs_select_1(&self) -> PhyWdqlvlDqdmObsSelect1R {
        PhyWdqlvlDqdmObsSelect1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Defines the step granularity for the logic to use once an edge is found. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
    #[inline(always)]
    pub fn phy_wdqlvl_qtr_dly_step_1(&self) -> PhyWdqlvlQtrDlyStep1R {
        PhyWdqlvlQtrDlyStep1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dqdm_obs_select_1(&mut self) -> PhyWdqlvlDqdmObsSelect1W<DenaliPhy154Spec> {
        PhyWdqlvlDqdmObsSelect1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Defines the step granularity for the logic to use once an edge is found. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_qtr_dly_step_1(&mut self) -> PhyWdqlvlQtrDlyStep1W<DenaliPhy154Spec> {
        PhyWdqlvlQtrDlyStep1W::new(self, 8)
    }
    #[doc = "Bit 16 - Clears the previous result value to allow a clean slate comparison for future write DQ leveling results for slice 1. Set to 1 to trigger. WRITE- ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_wdqlvl_clr_prev_results_1(
        &mut self,
    ) -> ScPhyWdqlvlClrPrevResults1W<DenaliPhy154Spec> {
        ScPhyWdqlvlClrPrevResults1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_154::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_154::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy154Spec;
impl crate::RegisterSpec for DenaliPhy154Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_154::R`](R) reader structure"]
impl crate::Readable for DenaliPhy154Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_154::W`](W) writer structure"]
impl crate::Writable for DenaliPhy154Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_154 to value 0"]
impl crate::Resettable for DenaliPhy154Spec {
    const RESET_VALUE: u32 = 0;
}
