#[doc = "Register `DENALI_PHY_304` reader"]
pub type R = crate::R<DenaliPhy304Spec>;
#[doc = "Field `PHY_WDQLVL_STATUS_OBS_2` reader - Observation register for write data leveling status for slice 2."]
pub type PhyWdqlvlStatusObs2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register for write data leveling status for slice 2."]
    #[inline(always)]
    pub fn phy_wdqlvl_status_obs_2(&self) -> PhyWdqlvlStatusObs2R {
        PhyWdqlvlStatusObs2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_304::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy304Spec;
impl crate::RegisterSpec for DenaliPhy304Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_304::R`](R) reader structure"]
impl crate::Readable for DenaliPhy304Spec {}
#[doc = "`reset()` method sets DENALI_PHY_304 to value 0"]
impl crate::Resettable for DenaliPhy304Spec {
    const RESET_VALUE: u32 = 0;
}
