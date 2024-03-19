#[doc = "Register `DDR_DENALI_PHY_295` reader"]
pub type R = crate::R<DdrDenaliPhy295Spec>;
#[doc = "Field `PHY_WRLVL_HARD1_DELAY_OBS_2` reader - Observation register for write leveling first hard 1 DQS slave delay for slice 2. READ-ONLY"]
pub type PhyWrlvlHard1DelayObs2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Observation register for write leveling first hard 1 DQS slave delay for slice 2. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wrlvl_hard1_delay_obs_2(&self) -> PhyWrlvlHard1DelayObs2R {
        PhyWrlvlHard1DelayObs2R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_295::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy295Spec;
impl crate::RegisterSpec for DdrDenaliPhy295Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_295::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy295Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_295 to value 0"]
impl crate::Resettable for DdrDenaliPhy295Spec {
    const RESET_VALUE: u32 = 0;
}
