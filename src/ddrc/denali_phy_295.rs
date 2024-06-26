#[doc = "Register `DENALI_PHY_295` reader"]
pub type R = crate::R<DenaliPhy295Spec>;
#[doc = "Field `PHY_WRLVL_HARD1_DELAY_OBS_2` reader - Observation register for write leveling first hard 1 DQS slave delay for slice 2."]
pub type PhyWrlvlHard1DelayObs2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Observation register for write leveling first hard 1 DQS slave delay for slice 2."]
    #[inline(always)]
    pub fn phy_wrlvl_hard1_delay_obs_2(&self) -> PhyWrlvlHard1DelayObs2R {
        PhyWrlvlHard1DelayObs2R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_295::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy295Spec;
impl crate::RegisterSpec for DenaliPhy295Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_295::R`](R) reader structure"]
impl crate::Readable for DenaliPhy295Spec {}
#[doc = "`reset()` method sets DENALI_PHY_295 to value 0"]
impl crate::Resettable for DenaliPhy295Spec {
    const RESET_VALUE: u32 = 0;
}
