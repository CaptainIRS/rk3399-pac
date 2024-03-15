#[doc = "Register `DENALI_PHY_302` reader"]
pub type R = crate::R<DenaliPhy302Spec>;
#[doc = "Field `PHY_RDLVL_STATUS_OBS_2` reader - Observation register for read leveling status for slice 2. READ- ONLY"]
pub type PhyRdlvlStatusObs2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register for read leveling status for slice 2. READ- ONLY"]
    #[inline(always)]
    pub fn phy_rdlvl_status_obs_2(&self) -> PhyRdlvlStatusObs2R {
        PhyRdlvlStatusObs2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_302::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy302Spec;
impl crate::RegisterSpec for DenaliPhy302Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_302::R`](R) reader structure"]
impl crate::Readable for DenaliPhy302Spec {}
#[doc = "`reset()` method sets DENALI_PHY_302 to value 0"]
impl crate::Resettable for DenaliPhy302Spec {
    const RESET_VALUE: u32 = 0;
}
