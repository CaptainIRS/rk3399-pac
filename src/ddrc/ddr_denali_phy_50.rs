#[doc = "Register `DDR_DENALI_PHY_50` reader"]
pub type R = crate::R<DdrDenaliPhy50Spec>;
#[doc = "Field `PHY_DDL_TEST_OBS_0` reader - DDL test observation for slice 0. READ-ONLY"]
pub type PhyDdlTestObs0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DDL test observation for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_ddl_test_obs_0(&self) -> PhyDdlTestObs0R {
        PhyDdlTestObs0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_50::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy50Spec;
impl crate::RegisterSpec for DdrDenaliPhy50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_50::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy50Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_50 to value 0"]
impl crate::Resettable for DdrDenaliPhy50Spec {
    const RESET_VALUE: u32 = 0;
}
