#[doc = "Register `DDR_DENALI_PHY_423` reader"]
pub type R = crate::R<DdrDenaliPhy423Spec>;
#[doc = "Field `PHY_WRLVL_HARD1_DELAY_OBS_3` reader - Observation register for write leveling first hard 1 DQS slave delay for slice 3."]
pub type PhyWrlvlHard1DelayObs3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Observation register for write leveling first hard 1 DQS slave delay for slice 3."]
    #[inline(always)]
    pub fn phy_wrlvl_hard1_delay_obs_3(&self) -> PhyWrlvlHard1DelayObs3R {
        PhyWrlvlHard1DelayObs3R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_423::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy423Spec;
impl crate::RegisterSpec for DdrDenaliPhy423Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_423::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy423Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_423 to value 0"]
impl crate::Resettable for DdrDenaliPhy423Spec {
    const RESET_VALUE: u32 = 0;
}
