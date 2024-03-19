#[doc = "Register `DDR_DENALI_PHY_418` reader"]
pub type R = crate::R<DdrDenaliPhy418Spec>;
#[doc = "Field `PHY_LPBK_RESULT_OBS_3` reader - Observation register containing loopback status/results for slice 3."]
pub type PhyLpbkResultObs3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register containing loopback status/results for slice 3."]
    #[inline(always)]
    pub fn phy_lpbk_result_obs_3(&self) -> PhyLpbkResultObs3R {
        PhyLpbkResultObs3R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_418::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy418Spec;
impl crate::RegisterSpec for DdrDenaliPhy418Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_418::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy418Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_418 to value 0"]
impl crate::Resettable for DdrDenaliPhy418Spec {
    const RESET_VALUE: u32 = 0;
}
