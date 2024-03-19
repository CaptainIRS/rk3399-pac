#[doc = "Register `DDR_DENALI_PHY_430` reader"]
pub type R = crate::R<DdrDenaliPhy430Spec>;
#[doc = "Field `PHY_RDLVL_STATUS_OBS_3` reader - Observation register for read leveling status for slice 3. READ- ONLY"]
pub type PhyRdlvlStatusObs3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register for read leveling status for slice 3. READ- ONLY"]
    #[inline(always)]
    pub fn phy_rdlvl_status_obs_3(&self) -> PhyRdlvlStatusObs3R {
        PhyRdlvlStatusObs3R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_430::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy430Spec;
impl crate::RegisterSpec for DdrDenaliPhy430Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_430::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy430Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_430 to value 0"]
impl crate::Resettable for DdrDenaliPhy430Spec {
    const RESET_VALUE: u32 = 0;
}
