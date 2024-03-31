#[doc = "Register `DENALI_PHY_176` reader"]
pub type R = crate::R<DenaliPhy176Spec>;
#[doc = "Field `PHY_WDQLVL_STATUS_OBS_1` reader - Observation register for write data leveling status for slice 1."]
pub type PhyWdqlvlStatusObs1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register for write data leveling status for slice 1."]
    #[inline(always)]
    pub fn phy_wdqlvl_status_obs_1(&self) -> PhyWdqlvlStatusObs1R {
        PhyWdqlvlStatusObs1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_176::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy176Spec;
impl crate::RegisterSpec for DenaliPhy176Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_176::R`](R) reader structure"]
impl crate::Readable for DenaliPhy176Spec {}
#[doc = "`reset()` method sets DENALI_PHY_176 to value 0"]
impl crate::Resettable for DenaliPhy176Spec {
    const RESET_VALUE: u32 = 0;
}
