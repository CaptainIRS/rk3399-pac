#[doc = "Register `DENALI_PHY_167` reader"]
pub type R = crate::R<DenaliPhy167Spec>;
#[doc = "Field `PHY_WRLVL_HARD1_DELAY_OBS_1` reader - Observation register for write leveling first hard 1 DQS slave delay for slice 1."]
pub type PhyWrlvlHard1DelayObs1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Observation register for write leveling first hard 1 DQS slave delay for slice 1."]
    #[inline(always)]
    pub fn phy_wrlvl_hard1_delay_obs_1(&self) -> PhyWrlvlHard1DelayObs1R {
        PhyWrlvlHard1DelayObs1R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_167::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy167Spec;
impl crate::RegisterSpec for DenaliPhy167Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_167::R`](R) reader structure"]
impl crate::Readable for DenaliPhy167Spec {}
#[doc = "`reset()` method sets DENALI_PHY_167 to value 0"]
impl crate::Resettable for DenaliPhy167Spec {
    const RESET_VALUE: u32 = 0;
}
