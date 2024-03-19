#[doc = "Register `DDR_DENALI_PHY_643` reader"]
pub type R = crate::R<DdrDenaliPhy643Spec>;
#[doc = "Field `PHY_ADR_LPBK_RESULT_OBS_1` reader - Observation register containing loopback status/results for address slice 1. READ-ONLY"]
pub type PhyAdrLpbkResultObs1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register containing loopback status/results for address slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_lpbk_result_obs_1(&self) -> PhyAdrLpbkResultObs1R {
        PhyAdrLpbkResultObs1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_643::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy643Spec;
impl crate::RegisterSpec for DdrDenaliPhy643Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_643::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy643Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_643 to value 0"]
impl crate::Resettable for DdrDenaliPhy643Spec {
    const RESET_VALUE: u32 = 0;
}
