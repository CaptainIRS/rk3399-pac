#[doc = "Register `DENALI_PHY_43` reader"]
pub type R = crate::R<DenaliPhy43Spec>;
#[doc = "Field `PHY_GTLVL_HARD1_DELAY_OBS_0` reader - Observation register for gate training last hard 1 DQS slave delay for slice 0. READ-ONLY"]
pub type PhyGtlvlHard1DelayObs0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GTLVL_STATUS_OBS_0` reader - Observation register for gate training status for slice 0. READ- ONLY"]
pub type PhyGtlvlStatusObs0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Observation register for gate training last hard 1 DQS slave delay for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_gtlvl_hard1_delay_obs_0(&self) -> PhyGtlvlHard1DelayObs0R {
        PhyGtlvlHard1DelayObs0R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:27 - Observation register for gate training status for slice 0. READ- ONLY"]
    #[inline(always)]
    pub fn phy_gtlvl_status_obs_0(&self) -> PhyGtlvlStatusObs0R {
        PhyGtlvlStatusObs0R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_43::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy43Spec;
impl crate::RegisterSpec for DenaliPhy43Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_43::R`](R) reader structure"]
impl crate::Readable for DenaliPhy43Spec {}
#[doc = "`reset()` method sets DENALI_PHY_43 to value 0"]
impl crate::Resettable for DenaliPhy43Spec {
    const RESET_VALUE: u32 = 0;
}
