#[doc = "Register `DDR_DENALI_PHY_282` reader"]
pub type R = crate::R<DdrDenaliPhy282Spec>;
#[doc = "Register `DDR_DENALI_PHY_282` writer"]
pub type W = crate::W<DdrDenaliPhy282Spec>;
#[doc = "Field `PHY_WDQLVL_DQDM_OBS_SELECT_2` reader - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 2."]
pub type PhyWdqlvlDqdmObsSelect2R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_DQDM_OBS_SELECT_2` writer - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 2."]
pub type PhyWdqlvlDqdmObsSelect2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_WDQLVL_QTR_DLY_STEP_2` reader - Defines the step granularity for the logic to use once an edge is found. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
pub type PhyWdqlvlQtrDlyStep2R = crate::FieldReader;
#[doc = "Field `PHY_WDQLVL_QTR_DLY_STEP_2` writer - Defines the step granularity for the logic to use once an edge is found. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
pub type PhyWdqlvlQtrDlyStep2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SC_PHY_WDQLVL_CLR_PREV_RESULTS_2` writer - Clears the previous result value to allow a clean slate comparison for future write DQ leveling results for slice 2. Set to 1 to trigger."]
pub type ScPhyWdqlvlClrPrevResults2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 2."]
    #[inline(always)]
    pub fn phy_wdqlvl_dqdm_obs_select_2(&self) -> PhyWdqlvlDqdmObsSelect2R {
        PhyWdqlvlDqdmObsSelect2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Defines the step granularity for the logic to use once an edge is found. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
    #[inline(always)]
    pub fn phy_wdqlvl_qtr_dly_step_2(&self) -> PhyWdqlvlQtrDlyStep2R {
        PhyWdqlvlQtrDlyStep2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select value to map an individual DQ data window leading/trailing edge to the leading/trailing edge observation registers during write data leveling for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_dqdm_obs_select_2(
        &mut self,
    ) -> PhyWdqlvlDqdmObsSelect2W<DdrDenaliPhy282Spec> {
        PhyWdqlvlDqdmObsSelect2W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Defines the step granularity for the logic to use once an edge is found. When this occurs, the logic jumps back to the previous invalid value and uses this step size to determine a more accurate delay value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_wdqlvl_qtr_dly_step_2(&mut self) -> PhyWdqlvlQtrDlyStep2W<DdrDenaliPhy282Spec> {
        PhyWdqlvlQtrDlyStep2W::new(self, 8)
    }
    #[doc = "Bit 16 - Clears the previous result value to allow a clean slate comparison for future write DQ leveling results for slice 2. Set to 1 to trigger."]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_wdqlvl_clr_prev_results_2(
        &mut self,
    ) -> ScPhyWdqlvlClrPrevResults2W<DdrDenaliPhy282Spec> {
        ScPhyWdqlvlClrPrevResults2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_282::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_282::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy282Spec;
impl crate::RegisterSpec for DdrDenaliPhy282Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_282::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy282Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_282::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy282Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_282 to value 0"]
impl crate::Resettable for DdrDenaliPhy282Spec {
    const RESET_VALUE: u32 = 0;
}
