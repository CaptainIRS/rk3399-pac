#[doc = "Register `DENALI_PHY_48` reader"]
pub type R = crate::R<DenaliPhy48Spec>;
#[doc = "Field `PHY_WDQLVL_STATUS_OBS_0` reader - Observation register for write data leveling status for slice 0. READ- ONLY"]
pub type PhyWdqlvlStatusObs0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register for write data leveling status for slice 0. READ- ONLY"]
    #[inline(always)]
    pub fn phy_wdqlvl_status_obs_0(&self) -> PhyWdqlvlStatusObs0R {
        PhyWdqlvlStatusObs0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_48::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy48Spec;
impl crate::RegisterSpec for DenaliPhy48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_48::R`](R) reader structure"]
impl crate::Readable for DenaliPhy48Spec {}
#[doc = "`reset()` method sets DENALI_PHY_48 to value 0"]
impl crate::Resettable for DenaliPhy48Spec {
    const RESET_VALUE: u32 = 0;
}
