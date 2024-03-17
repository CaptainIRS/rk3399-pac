#[doc = "Register `DENALI_PHY_178` reader"]
pub type R = crate::R<DenaliPhy178Spec>;
#[doc = "Field `PHY_DDL_TEST_OBS_1` reader - DDL test observation for slice 1. READ-ONLY"]
pub type PhyDdlTestObs1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DDL test observation for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_ddl_test_obs_1(&self) -> PhyDdlTestObs1R {
        PhyDdlTestObs1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_178::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy178Spec;
impl crate::RegisterSpec for DenaliPhy178Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_178::R`](R) reader structure"]
impl crate::Readable for DenaliPhy178Spec {}
#[doc = "`reset()` method sets DENALI_PHY_178 to value 0"]
impl crate::Resettable for DenaliPhy178Spec {
    const RESET_VALUE: u32 = 0;
}
