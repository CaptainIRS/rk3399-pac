#[doc = "Register `DDR_DENALI_PHY_515` reader"]
pub type R = crate::R<DdrDenaliPhy515Spec>;
#[doc = "Field `PHY_ADR_LPBK_RESULT_OBS_0` reader - Observation register containing loopback status/results for address slice 0. READ-ONLY"]
pub type PhyAdrLpbkResultObs0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register containing loopback status/results for address slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_lpbk_result_obs_0(&self) -> PhyAdrLpbkResultObs0R {
        PhyAdrLpbkResultObs0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_515::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy515Spec;
impl crate::RegisterSpec for DdrDenaliPhy515Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_515::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy515Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_515 to value 0"]
impl crate::Resettable for DdrDenaliPhy515Spec {
    const RESET_VALUE: u32 = 0;
}
