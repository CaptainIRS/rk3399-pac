#[doc = "Register `DENALI_PHY_298` reader"]
pub type R = crate::R<DenaliPhy298Spec>;
#[doc = "Field `PHY_WRLVL_ERROR_OBS_2` reader - Observation register for write leveling error status for slice 2."]
pub type PhyWrlvlErrorObs2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GTLVL_HARD0_DELAY_OBS_2` reader - Observation register for gate training first hard 0 DQS slave delay for slice 2."]
pub type PhyGtlvlHard0DelayObs2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Observation register for write leveling error status for slice 2."]
    #[inline(always)]
    pub fn phy_wrlvl_error_obs_2(&self) -> PhyWrlvlErrorObs2R {
        PhyWrlvlErrorObs2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:29 - Observation register for gate training first hard 0 DQS slave delay for slice 2."]
    #[inline(always)]
    pub fn phy_gtlvl_hard0_delay_obs_2(&self) -> PhyGtlvlHard0DelayObs2R {
        PhyGtlvlHard0DelayObs2R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_298::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy298Spec;
impl crate::RegisterSpec for DenaliPhy298Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_298::R`](R) reader structure"]
impl crate::Readable for DenaliPhy298Spec {}
#[doc = "`reset()` method sets DENALI_PHY_298 to value 0"]
impl crate::Resettable for DenaliPhy298Spec {
    const RESET_VALUE: u32 = 0;
}
