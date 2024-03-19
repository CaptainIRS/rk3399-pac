#[doc = "Register `DDR_DENALI_PHY_162` reader"]
pub type R = crate::R<DdrDenaliPhy162Spec>;
#[doc = "Field `PHY_LPBK_RESULT_OBS_1` reader - Observation register containing loopback status/results for slice 1."]
pub type PhyLpbkResultObs1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register containing loopback status/results for slice 1."]
    #[inline(always)]
    pub fn phy_lpbk_result_obs_1(&self) -> PhyLpbkResultObs1R {
        PhyLpbkResultObs1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_162::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy162Spec;
impl crate::RegisterSpec for DdrDenaliPhy162Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_162::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy162Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_162 to value 0"]
impl crate::Resettable for DdrDenaliPhy162Spec {
    const RESET_VALUE: u32 = 0;
}
