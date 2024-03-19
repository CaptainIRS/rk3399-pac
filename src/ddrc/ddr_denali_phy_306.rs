#[doc = "Register `DDR_DENALI_PHY_306` reader"]
pub type R = crate::R<DdrDenaliPhy306Spec>;
#[doc = "Field `PHY_DDL_TEST_OBS_2` reader - DDL test observation for slice 2. READ-ONLY"]
pub type PhyDdlTestObs2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DDL test observation for slice 2. READ-ONLY"]
    #[inline(always)]
    pub fn phy_ddl_test_obs_2(&self) -> PhyDdlTestObs2R {
        PhyDdlTestObs2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_306::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy306Spec;
impl crate::RegisterSpec for DdrDenaliPhy306Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_306::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy306Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_306 to value 0"]
impl crate::Resettable for DdrDenaliPhy306Spec {
    const RESET_VALUE: u32 = 0;
}
