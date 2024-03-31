#[doc = "Register `DENALI_PHY_296` reader"]
pub type R = crate::R<DenaliPhy296Spec>;
#[doc = "Field `PHY_WRLVL_STATUS_OBS_2` reader - Observation register for write leveling status for slice 2."]
pub type PhyWrlvlStatusObs2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - Observation register for write leveling status for slice 2."]
    #[inline(always)]
    pub fn phy_wrlvl_status_obs_2(&self) -> PhyWrlvlStatusObs2R {
        PhyWrlvlStatusObs2R::new(self.bits & 0x0001_ffff)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_296::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy296Spec;
impl crate::RegisterSpec for DenaliPhy296Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_296::R`](R) reader structure"]
impl crate::Readable for DenaliPhy296Spec {}
#[doc = "`reset()` method sets DENALI_PHY_296 to value 0"]
impl crate::Resettable for DenaliPhy296Spec {
    const RESET_VALUE: u32 = 0;
}
