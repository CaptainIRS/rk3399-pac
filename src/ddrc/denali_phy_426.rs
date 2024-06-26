#[doc = "Register `DENALI_PHY_426` reader"]
pub type R = crate::R<DenaliPhy426Spec>;
#[doc = "Field `PHY_WRLVL_ERROR_OBS_3` reader - Observation register for write leveling error status for slice 3."]
pub type PhyWrlvlErrorObs3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GTLVL_HARD0_DELAY_OBS_3` reader - Observation register for gate training first hard 0 DQS slave delay for slice 3."]
pub type PhyGtlvlHard0DelayObs3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Observation register for write leveling error status for slice 3."]
    #[inline(always)]
    pub fn phy_wrlvl_error_obs_3(&self) -> PhyWrlvlErrorObs3R {
        PhyWrlvlErrorObs3R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:29 - Observation register for gate training first hard 0 DQS slave delay for slice 3."]
    #[inline(always)]
    pub fn phy_gtlvl_hard0_delay_obs_3(&self) -> PhyGtlvlHard0DelayObs3R {
        PhyGtlvlHard0DelayObs3R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_426::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy426Spec;
impl crate::RegisterSpec for DenaliPhy426Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_426::R`](R) reader structure"]
impl crate::Readable for DenaliPhy426Spec {}
#[doc = "`reset()` method sets DENALI_PHY_426 to value 0"]
impl crate::Resettable for DenaliPhy426Spec {
    const RESET_VALUE: u32 = 0;
}
