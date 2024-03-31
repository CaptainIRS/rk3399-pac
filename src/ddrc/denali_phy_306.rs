#[doc = "Register `DENALI_PHY_306` reader"]
pub type R = crate::R<DenaliPhy306Spec>;
#[doc = "Field `PHY_DDL_TEST_OBS_2` reader - DDL test observation for slice 2."]
pub type PhyDdlTestObs2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DDL test observation for slice 2."]
    #[inline(always)]
    pub fn phy_ddl_test_obs_2(&self) -> PhyDdlTestObs2R {
        PhyDdlTestObs2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_306::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy306Spec;
impl crate::RegisterSpec for DenaliPhy306Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_306::R`](R) reader structure"]
impl crate::Readable for DenaliPhy306Spec {}
#[doc = "`reset()` method sets DENALI_PHY_306 to value 0"]
impl crate::Resettable for DenaliPhy306Spec {
    const RESET_VALUE: u32 = 0;
}
