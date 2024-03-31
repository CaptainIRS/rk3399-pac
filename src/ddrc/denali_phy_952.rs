#[doc = "Register `DENALI_PHY_952` reader"]
pub type R = crate::R<DenaliPhy952Spec>;
#[doc = "Field `PHY_AC_LPBK_RESULT_OBS` reader - Observation register for the address/control slices."]
pub type PhyAcLpbkResultObsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register for the address/control slices."]
    #[inline(always)]
    pub fn phy_ac_lpbk_result_obs(&self) -> PhyAcLpbkResultObsR {
        PhyAcLpbkResultObsR::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_952::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy952Spec;
impl crate::RegisterSpec for DenaliPhy952Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_952::R`](R) reader structure"]
impl crate::Readable for DenaliPhy952Spec {}
#[doc = "`reset()` method sets DENALI_PHY_952 to value 0"]
impl crate::Resettable for DenaliPhy952Spec {
    const RESET_VALUE: u32 = 0;
}
