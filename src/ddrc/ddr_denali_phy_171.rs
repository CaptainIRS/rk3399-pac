#[doc = "Register `DDR_DENALI_PHY_171` reader"]
pub type R = crate::R<DdrDenaliPhy171Spec>;
#[doc = "Field `PHY_GTLVL_HARD1_DELAY_OBS_1` reader - Observation register for gate training last hard 1 DQS slave delay for slice 1."]
pub type PhyGtlvlHard1DelayObs1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GTLVL_STATUS_OBS_1` reader - Observation register for gate training status for slice 1."]
pub type PhyGtlvlStatusObs1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Observation register for gate training last hard 1 DQS slave delay for slice 1."]
    #[inline(always)]
    pub fn phy_gtlvl_hard1_delay_obs_1(&self) -> PhyGtlvlHard1DelayObs1R {
        PhyGtlvlHard1DelayObs1R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:27 - Observation register for gate training status for slice 1."]
    #[inline(always)]
    pub fn phy_gtlvl_status_obs_1(&self) -> PhyGtlvlStatusObs1R {
        PhyGtlvlStatusObs1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_171::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy171Spec;
impl crate::RegisterSpec for DdrDenaliPhy171Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_171::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy171Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_171 to value 0"]
impl crate::Resettable for DdrDenaliPhy171Spec {
    const RESET_VALUE: u32 = 0;
}
