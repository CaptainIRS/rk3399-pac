#[doc = "Register `DENALI_PHY_174` reader"]
pub type R = crate::R<DenaliPhy174Spec>;
#[doc = "Field `PHY_RDLVL_STATUS_OBS_1` reader - Observation register for read leveling status for slice 1."]
pub type PhyRdlvlStatusObs1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register for read leveling status for slice 1."]
    #[inline(always)]
    pub fn phy_rdlvl_status_obs_1(&self) -> PhyRdlvlStatusObs1R {
        PhyRdlvlStatusObs1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_174::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy174Spec;
impl crate::RegisterSpec for DenaliPhy174Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_174::R`](R) reader structure"]
impl crate::Readable for DenaliPhy174Spec {}
#[doc = "`reset()` method sets DENALI_PHY_174 to value 0"]
impl crate::Resettable for DenaliPhy174Spec {
    const RESET_VALUE: u32 = 0;
}
