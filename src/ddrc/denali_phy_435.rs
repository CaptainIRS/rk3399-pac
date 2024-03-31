#[doc = "Register `DENALI_PHY_435` reader"]
pub type R = crate::R<DenaliPhy435Spec>;
#[doc = "Field `PHY_DDL_TEST_MSTR_DLY_OBS_3` reader - DDL test observation delays for slice 3 master DDL."]
pub type PhyDdlTestMstrDlyObs3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DDL test observation delays for slice 3 master DDL."]
    #[inline(always)]
    pub fn phy_ddl_test_mstr_dly_obs_3(&self) -> PhyDdlTestMstrDlyObs3R {
        PhyDdlTestMstrDlyObs3R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_435::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy435Spec;
impl crate::RegisterSpec for DenaliPhy435Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_435::R`](R) reader structure"]
impl crate::Readable for DenaliPhy435Spec {}
#[doc = "`reset()` method sets DENALI_PHY_435 to value 0"]
impl crate::Resettable for DenaliPhy435Spec {
    const RESET_VALUE: u32 = 0;
}
