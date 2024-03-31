#[doc = "Register `DENALI_PHY_51` reader"]
pub type R = crate::R<DenaliPhy51Spec>;
#[doc = "Field `PHY_DDL_TEST_MSTR_DLY_OBS_0` reader - DDL test observation delays for slice 0 master DDL."]
pub type PhyDdlTestMstrDlyObs0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DDL test observation delays for slice 0 master DDL."]
    #[inline(always)]
    pub fn phy_ddl_test_mstr_dly_obs_0(&self) -> PhyDdlTestMstrDlyObs0R {
        PhyDdlTestMstrDlyObs0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_51::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy51Spec;
impl crate::RegisterSpec for DenaliPhy51Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_51::R`](R) reader structure"]
impl crate::Readable for DenaliPhy51Spec {}
#[doc = "`reset()` method sets DENALI_PHY_51 to value 0"]
impl crate::Resettable for DenaliPhy51Spec {
    const RESET_VALUE: u32 = 0;
}
