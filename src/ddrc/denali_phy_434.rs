#[doc = "Register `DENALI_PHY_434` reader"]
pub type R = crate::R<DenaliPhy434Spec>;
#[doc = "Field `PHY_DDL_TEST_OBS_3` reader - DDL test observation for slice 3."]
pub type PhyDdlTestObs3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DDL test observation for slice 3."]
    #[inline(always)]
    pub fn phy_ddl_test_obs_3(&self) -> PhyDdlTestObs3R {
        PhyDdlTestObs3R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_434::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy434Spec;
impl crate::RegisterSpec for DenaliPhy434Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_434::R`](R) reader structure"]
impl crate::Readable for DenaliPhy434Spec {}
#[doc = "`reset()` method sets DENALI_PHY_434 to value 0"]
impl crate::Resettable for DenaliPhy434Spec {
    const RESET_VALUE: u32 = 0;
}
