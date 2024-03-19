#[doc = "Register `DDR_DENALI_PHY_432` reader"]
pub type R = crate::R<DdrDenaliPhy432Spec>;
#[doc = "Field `PHY_WDQLVL_STATUS_OBS_3` reader - Observation register for write data leveling status for slice 3. READ- ONLY"]
pub type PhyWdqlvlStatusObs3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register for write data leveling status for slice 3. READ- ONLY"]
    #[inline(always)]
    pub fn phy_wdqlvl_status_obs_3(&self) -> PhyWdqlvlStatusObs3R {
        PhyWdqlvlStatusObs3R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_432::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy432Spec;
impl crate::RegisterSpec for DdrDenaliPhy432Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_432::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy432Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_432 to value 0"]
impl crate::Resettable for DdrDenaliPhy432Spec {
    const RESET_VALUE: u32 = 0;
}
