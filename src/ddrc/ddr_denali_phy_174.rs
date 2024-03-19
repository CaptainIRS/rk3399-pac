#[doc = "Register `DDR_DENALI_PHY_174` reader"]
pub type R = crate::R<DdrDenaliPhy174Spec>;
#[doc = "Field `PHY_RDLVL_STATUS_OBS_1` reader - Observation register for read leveling status for slice 1. READ- ONLY"]
pub type PhyRdlvlStatusObs1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register for read leveling status for slice 1. READ- ONLY"]
    #[inline(always)]
    pub fn phy_rdlvl_status_obs_1(&self) -> PhyRdlvlStatusObs1R {
        PhyRdlvlStatusObs1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_174::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy174Spec;
impl crate::RegisterSpec for DdrDenaliPhy174Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_174::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy174Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_174 to value 0"]
impl crate::Resettable for DdrDenaliPhy174Spec {
    const RESET_VALUE: u32 = 0;
}
