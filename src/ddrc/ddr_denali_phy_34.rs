#[doc = "Register `DDR_DENALI_PHY_34` reader"]
pub type R = crate::R<DdrDenaliPhy34Spec>;
#[doc = "Field `PHY_LPBK_RESULT_OBS_0` reader - Observation register containing loopback status/results for slice 0. READ-ONLY"]
pub type PhyLpbkResultObs0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register containing loopback status/results for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_lpbk_result_obs_0(&self) -> PhyLpbkResultObs0R {
        PhyLpbkResultObs0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_34::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy34Spec;
impl crate::RegisterSpec for DdrDenaliPhy34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_34::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy34Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_34 to value 0"]
impl crate::Resettable for DdrDenaliPhy34Spec {
    const RESET_VALUE: u32 = 0;
}
