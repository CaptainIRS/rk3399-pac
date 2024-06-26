#[doc = "Register `DENALI_PHY_34` reader"]
pub type R = crate::R<DenaliPhy34Spec>;
#[doc = "Field `PHY_LPBK_RESULT_OBS_0` reader - Observation register containing loopback status/results for slice 0."]
pub type PhyLpbkResultObs0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register containing loopback status/results for slice 0."]
    #[inline(always)]
    pub fn phy_lpbk_result_obs_0(&self) -> PhyLpbkResultObs0R {
        PhyLpbkResultObs0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_34::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy34Spec;
impl crate::RegisterSpec for DenaliPhy34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_34::R`](R) reader structure"]
impl crate::Readable for DenaliPhy34Spec {}
#[doc = "`reset()` method sets DENALI_PHY_34 to value 0"]
impl crate::Resettable for DenaliPhy34Spec {
    const RESET_VALUE: u32 = 0;
}
