#[doc = "Register `DENALI_PHY_290` reader"]
pub type R = crate::R<DenaliPhy290Spec>;
#[doc = "Field `PHY_LPBK_RESULT_OBS_2` reader - Observation register containing loopback status/results for slice 2. READ-ONLY"]
pub type PhyLpbkResultObs2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register containing loopback status/results for slice 2. READ-ONLY"]
    #[inline(always)]
    pub fn phy_lpbk_result_obs_2(&self) -> PhyLpbkResultObs2R {
        PhyLpbkResultObs2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_290::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy290Spec;
impl crate::RegisterSpec for DenaliPhy290Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_290::R`](R) reader structure"]
impl crate::Readable for DenaliPhy290Spec {}
#[doc = "`reset()` method sets DENALI_PHY_290 to value 0"]
impl crate::Resettable for DenaliPhy290Spec {
    const RESET_VALUE: u32 = 0;
}
