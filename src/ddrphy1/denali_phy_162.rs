#[doc = "Register `DENALI_PHY_162` reader"]
pub type R = crate::R<DenaliPhy162Spec>;
#[doc = "Field `PHY_LPBK_RESULT_OBS_1` reader - Observation register containing loopback status/results for slice 1. READ-ONLY"]
pub type PhyLpbkResultObs1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register containing loopback status/results for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_lpbk_result_obs_1(&self) -> PhyLpbkResultObs1R {
        PhyLpbkResultObs1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_162::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy162Spec;
impl crate::RegisterSpec for DenaliPhy162Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_162::R`](R) reader structure"]
impl crate::Readable for DenaliPhy162Spec {}
#[doc = "`reset()` method sets DENALI_PHY_162 to value 0"]
impl crate::Resettable for DenaliPhy162Spec {
    const RESET_VALUE: u32 = 0;
}
