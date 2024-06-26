#[doc = "Register `DENALI_PHY_515` reader"]
pub type R = crate::R<DenaliPhy515Spec>;
#[doc = "Field `PHY_ADR_LPBK_RESULT_OBS_0` reader - Observation register containing loopback status/results for address slice 0."]
pub type PhyAdrLpbkResultObs0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register containing loopback status/results for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_lpbk_result_obs_0(&self) -> PhyAdrLpbkResultObs0R {
        PhyAdrLpbkResultObs0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_515::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy515Spec;
impl crate::RegisterSpec for DenaliPhy515Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_515::R`](R) reader structure"]
impl crate::Readable for DenaliPhy515Spec {}
#[doc = "`reset()` method sets DENALI_PHY_515 to value 0"]
impl crate::Resettable for DenaliPhy515Spec {
    const RESET_VALUE: u32 = 0;
}
