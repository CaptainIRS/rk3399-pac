#[doc = "Register `DENALI_PHY_410` reader"]
pub type R = crate::R<DenaliPhy410Spec>;
#[doc = "Register `DENALI_PHY_410` writer"]
pub type W = crate::W<DenaliPhy410Spec>;
#[doc = "Field `PHY_WDQLVL_DQDM_OBS_SELECT_3` reader - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 3."]
pub type PhyWdqlvlDqdmObsSelect3R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_DQDM_OBS_SELECT_3` writer - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 3."]
pub type PhyWdqlvlDqdmObsSelect3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WDQLVL_QTR_DLY_STEP_3` reader - Defines the step granularity for the logic to use once an edge is found. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
pub type PhyWdqlvlQtrDlyStep3R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_QTR_DLY_STEP_3` writer - Defines the step granularity for the logic to use once an edge is found. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
pub type PhyWdqlvlQtrDlyStep3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SC_PHY_WDQLVL_CLR_PREV_RESULTS_3` writer - Clears the previous result value to allow a clean slate comparison for future write DQ leveling results for slice 3. Set to 1 to trigger."]
pub type ScPhyWdqlvlClrPrevResults3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 3."]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_obs_select_3(&self) -> PhyWdqlvlDqdmObsSelect3R {
        PhyWdqlvlDqdmObsSelect3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Defines the step granularity for the logic to use once an edge is found. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
    #[inline(always)]
    pub fn phy_wdqlvl_qtr_dly_step_3(&self) -> PhyWdqlvlQtrDlyStep3R {
        PhyWdqlvlQtrDlyStep3R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dqdm_obs_select_3(&mut self) -> PhyWdqlvlDqdmObsSelect3W<DenaliPhy410Spec> {
        PhyWdqlvlDqdmObsSelect3W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Defines the step granularity for the logic to use once an edge is found. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_qtr_dly_step_3(&mut self) -> PhyWdqlvlQtrDlyStep3W<DenaliPhy410Spec> {
        PhyWdqlvlQtrDlyStep3W::new(self, 8)
    }
    #[doc = "Bit 16 - Clears the previous result value to allow a clean slate comparison for future write DQ leveling results for slice 3. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_wdqlvl_clr_prev_results_3(
        &mut self,
    ) -> ScPhyWdqlvlClrPrevResults3W<DenaliPhy410Spec> {
        ScPhyWdqlvlClrPrevResults3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_410::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_410::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy410Spec;
impl crate::RegisterSpec for DenaliPhy410Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_410::R`](R) reader structure"]
impl crate::Readable for DenaliPhy410Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_410::W`](W) writer structure"]
impl crate::Writable for DenaliPhy410Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_410 to value 0"]
impl crate::Resettable for DenaliPhy410Spec {
    const RESET_VALUE: u32 = 0;
}
