#[doc = "Register `DENALI_PHY_424` reader"]
pub type R = crate::R<DenaliPhy424Spec>;
#[doc = "Field `PHY_WRLVL_STATUS_OBS_3` reader - Observation register for write leveling status for slice 3."]
pub type PhyWrlvlStatusObs3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - Observation register for write leveling status for slice 3."]
    #[inline(always)]
    pub fn phy_wrlvl_status_obs_3(&self) -> PhyWrlvlStatusObs3R {
        PhyWrlvlStatusObs3R::new(self.bits & 0x0001_ffff)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_424::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy424Spec;
impl crate::RegisterSpec for DenaliPhy424Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_424::R`](R) reader structure"]
impl crate::Readable for DenaliPhy424Spec {}
#[doc = "`reset()` method sets DENALI_PHY_424 to value 0"]
impl crate::Resettable for DenaliPhy424Spec {
    const RESET_VALUE: u32 = 0;
}
