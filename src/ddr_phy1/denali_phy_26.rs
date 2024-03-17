#[doc = "Register `DENALI_PHY_26` reader"]
pub type R = crate::R<DenaliPhy26Spec>;
#[doc = "Register `DENALI_PHY_26` writer"]
pub type W = crate::W<DenaliPhy26Spec>;
#[doc = "Field `PHY_WDQLVL_DQDM_OBS_SELECT_0` reader - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 0."]
pub type PhyWdqlvlDqdmObsSelect0R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_DQDM_OBS_SELECT_0` writer - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 0."]
pub type PhyWdqlvlDqdmObsSelect0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WDQLVL_QTR_DLY_STEP_0` reader - Defines the step granularity for the logic to use once an edge is found. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
pub type PhyWdqlvlQtrDlyStep0R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_QTR_DLY_STEP_0` writer - Defines the step granularity for the logic to use once an edge is found. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
pub type PhyWdqlvlQtrDlyStep0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SC_PHY_WDQLVL_CLR_PREV_RESULTS_0` writer - Clears the previous result value to allow a clean slate comparison for future write DQ leveling results for slice 0. Set to 1 to trigger. WRITE- ONLY"]
pub type ScPhyWdqlvlClrPrevResults0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 0."]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_obs_select_0(&self) -> PhyWdqlvlDqdmObsSelect0R {
        PhyWdqlvlDqdmObsSelect0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Defines the step granularity for the logic to use once an edge is found. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
    #[inline(always)]
    pub fn phy_wdqlvl_qtr_dly_step_0(&self) -> PhyWdqlvlQtrDlyStep0R {
        PhyWdqlvlQtrDlyStep0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dqdm_obs_select_0(&mut self) -> PhyWdqlvlDqdmObsSelect0W<DenaliPhy26Spec> {
        PhyWdqlvlDqdmObsSelect0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Defines the step granularity for the logic to use once an edge is found. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_qtr_dly_step_0(&mut self) -> PhyWdqlvlQtrDlyStep0W<DenaliPhy26Spec> {
        PhyWdqlvlQtrDlyStep0W::new(self, 8)
    }
    #[doc = "Bit 16 - Clears the previous result value to allow a clean slate comparison for future write DQ leveling results for slice 0. Set to 1 to trigger. WRITE- ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_wdqlvl_clr_prev_results_0(
        &mut self,
    ) -> ScPhyWdqlvlClrPrevResults0W<DenaliPhy26Spec> {
        ScPhyWdqlvlClrPrevResults0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_26::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_26::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy26Spec;
impl crate::RegisterSpec for DenaliPhy26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_26::R`](R) reader structure"]
impl crate::Readable for DenaliPhy26Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_26::W`](W) writer structure"]
impl crate::Writable for DenaliPhy26Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_26 to value 0"]
impl crate::Resettable for DenaliPhy26Spec {
    const RESET_VALUE: u32 = 0;
}
