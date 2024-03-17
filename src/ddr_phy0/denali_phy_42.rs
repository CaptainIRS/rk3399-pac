#[doc = "Register `DENALI_PHY_42` reader"]
pub type R = crate::R<DenaliPhy42Spec>;
#[doc = "Field `PHY_WRLVL_ERROR_OBS_0` reader - Observation register for write leveling error status for slice 0. READ-ONLY"]
pub type PhyWrlvlErrorObs0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GTLVL_HARD0_DELAY_OBS_0` reader - Observation register for gate training first hard 0 DQS slave delay for slice 0. READ-ONLY"]
pub type PhyGtlvlHard0DelayObs0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Observation register for write leveling error status for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wrlvl_error_obs_0(&self) -> PhyWrlvlErrorObs0R {
        PhyWrlvlErrorObs0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:29 - Observation register for gate training first hard 0 DQS slave delay for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_gtlvl_hard0_delay_obs_0(&self) -> PhyGtlvlHard0DelayObs0R {
        PhyGtlvlHard0DelayObs0R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_42::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy42Spec;
impl crate::RegisterSpec for DenaliPhy42Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_42::R`](R) reader structure"]
impl crate::Readable for DenaliPhy42Spec {}
#[doc = "`reset()` method sets DENALI_PHY_42 to value 0"]
impl crate::Resettable for DenaliPhy42Spec {
    const RESET_VALUE: u32 = 0;
}
