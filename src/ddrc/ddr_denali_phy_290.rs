#[doc = "Register `DDR_DENALI_PHY_290` reader"]
pub type R = crate::R<DdrDenaliPhy290Spec>;
#[doc = "Field `PHY_LPBK_RESULT_OBS_2` reader - Observation register containing loopback status/results for slice 2."]
pub type PhyLpbkResultObs2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register containing loopback status/results for slice 2."]
    #[inline(always)]
    pub fn phy_lpbk_result_obs_2(&self) -> PhyLpbkResultObs2R {
        PhyLpbkResultObs2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_290::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy290Spec;
impl crate::RegisterSpec for DdrDenaliPhy290Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_290::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy290Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_290 to value 0"]
impl crate::Resettable for DdrDenaliPhy290Spec {
    const RESET_VALUE: u32 = 0;
}
