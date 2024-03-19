#[doc = "Register `DDR_DENALI_PHY_178` reader"]
pub type R = crate::R<DdrDenaliPhy178Spec>;
#[doc = "Field `PHY_DDL_TEST_OBS_1` reader - DDL test observation for slice 1."]
pub type PhyDdlTestObs1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DDL test observation for slice 1."]
    #[inline(always)]
    pub fn phy_ddl_test_obs_1(&self) -> PhyDdlTestObs1R {
        PhyDdlTestObs1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_178::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy178Spec;
impl crate::RegisterSpec for DdrDenaliPhy178Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_178::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy178Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_178 to value 0"]
impl crate::Resettable for DdrDenaliPhy178Spec {
    const RESET_VALUE: u32 = 0;
}
