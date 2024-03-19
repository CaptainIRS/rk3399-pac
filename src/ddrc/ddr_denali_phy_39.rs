#[doc = "Register `DDR_DENALI_PHY_39` reader"]
pub type R = crate::R<DdrDenaliPhy39Spec>;
#[doc = "Field `PHY_WRLVL_HARD1_DELAY_OBS_0` reader - Observation register for write leveling first hard 1 DQS slave delay for slice 0."]
pub type PhyWrlvlHard1DelayObs0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Observation register for write leveling first hard 1 DQS slave delay for slice 0."]
    #[inline(always)]
    pub fn phy_wrlvl_hard1_delay_obs_0(&self) -> PhyWrlvlHard1DelayObs0R {
        PhyWrlvlHard1DelayObs0R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_39::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy39Spec;
impl crate::RegisterSpec for DdrDenaliPhy39Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_39::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy39Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_39 to value 0"]
impl crate::Resettable for DdrDenaliPhy39Spec {
    const RESET_VALUE: u32 = 0;
}
