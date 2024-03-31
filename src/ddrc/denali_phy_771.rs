#[doc = "Register `DENALI_PHY_771` reader"]
pub type R = crate::R<DenaliPhy771Spec>;
#[doc = "Field `PHY_ADR_LPBK_RESULT_OBS_2` reader - Observation register containing loopback status/results for address slice 2."]
pub type PhyAdrLpbkResultObs2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register containing loopback status/results for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_lpbk_result_obs_2(&self) -> PhyAdrLpbkResultObs2R {
        PhyAdrLpbkResultObs2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_771::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy771Spec;
impl crate::RegisterSpec for DenaliPhy771Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_771::R`](R) reader structure"]
impl crate::Readable for DenaliPhy771Spec {}
#[doc = "`reset()` method sets DENALI_PHY_771 to value 0"]
impl crate::Resettable for DenaliPhy771Spec {
    const RESET_VALUE: u32 = 0;
}
