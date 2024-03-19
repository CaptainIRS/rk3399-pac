#[doc = "Register `DDR_DENALI_PHY_170` reader"]
pub type R = crate::R<DdrDenaliPhy170Spec>;
#[doc = "Field `PHY_WRLVL_ERROR_OBS_1` reader - Observation register for write leveling error status for slice 1. READ-ONLY"]
pub type PhyWrlvlErrorObs1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GTLVL_HARD0_DELAY_OBS_1` reader - Observation register for gate training first hard 0 DQS slave delay for slice 1. READ-ONLY"]
pub type PhyGtlvlHard0DelayObs1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Observation register for write leveling error status for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wrlvl_error_obs_1(&self) -> PhyWrlvlErrorObs1R {
        PhyWrlvlErrorObs1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:29 - Observation register for gate training first hard 0 DQS slave delay for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_gtlvl_hard0_delay_obs_1(&self) -> PhyGtlvlHard0DelayObs1R {
        PhyGtlvlHard0DelayObs1R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_170::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy170Spec;
impl crate::RegisterSpec for DdrDenaliPhy170Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_170::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy170Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_170 to value 0"]
impl crate::Resettable for DdrDenaliPhy170Spec {
    const RESET_VALUE: u32 = 0;
}
