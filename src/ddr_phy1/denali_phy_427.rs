#[doc = "Register `DENALI_PHY_427` reader"]
pub type R = crate::R<DenaliPhy427Spec>;
#[doc = "Field `PHY_GTLVL_HARD1_DELAY_OBS_3` reader - Observation register for gate training last hard 1 DQS slave delay for slice 3. READ-ONLY"]
pub type PhyGtlvlHard1DelayObs3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GTLVL_STATUS_OBS_3` reader - Observation register for gate training status for slice 3. READ- ONLY"]
pub type PhyGtlvlStatusObs3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Observation register for gate training last hard 1 DQS slave delay for slice 3. READ-ONLY"]
    #[inline(always)]
    pub fn phy_gtlvl_hard1_delay_obs_3(&self) -> PhyGtlvlHard1DelayObs3R {
        PhyGtlvlHard1DelayObs3R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:27 - Observation register for gate training status for slice 3. READ- ONLY"]
    #[inline(always)]
    pub fn phy_gtlvl_status_obs_3(&self) -> PhyGtlvlStatusObs3R {
        PhyGtlvlStatusObs3R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_427::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy427Spec;
impl crate::RegisterSpec for DenaliPhy427Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_427::R`](R) reader structure"]
impl crate::Readable for DenaliPhy427Spec {}
#[doc = "`reset()` method sets DENALI_PHY_427 to value 0"]
impl crate::Resettable for DenaliPhy427Spec {
    const RESET_VALUE: u32 = 0;
}
