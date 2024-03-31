#[doc = "Register `DENALI_PHY_299` reader"]
pub type R = crate::R<DenaliPhy299Spec>;
#[doc = "Field `PHY_GTLVL_HARD1_DELAY_OBS_2` reader - Observation register for gate training last hard 1 DQS slave delay for slice 2."]
pub type PhyGtlvlHard1DelayObs2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GTLVL_STATUS_OBS_2` reader - Observation register for gate training status for slice 2."]
pub type PhyGtlvlStatusObs2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Observation register for gate training last hard 1 DQS slave delay for slice 2."]
    #[inline(always)]
    pub fn phy_gtlvl_hard1_delay_obs_2(&self) -> PhyGtlvlHard1DelayObs2R {
        PhyGtlvlHard1DelayObs2R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:27 - Observation register for gate training status for slice 2."]
    #[inline(always)]
    pub fn phy_gtlvl_status_obs_2(&self) -> PhyGtlvlStatusObs2R {
        PhyGtlvlStatusObs2R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_299::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy299Spec;
impl crate::RegisterSpec for DenaliPhy299Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_299::R`](R) reader structure"]
impl crate::Readable for DenaliPhy299Spec {}
#[doc = "`reset()` method sets DENALI_PHY_299 to value 0"]
impl crate::Resettable for DenaliPhy299Spec {
    const RESET_VALUE: u32 = 0;
}
