#[doc = "Register `DENALI_PHY_307` reader"]
pub type R = crate::R<DenaliPhy307Spec>;
#[doc = "Field `PHY_DDL_TEST_MSTR_DLY_OBS_2` reader - DDL test observation delays for slice 2 master DDL."]
pub type PhyDdlTestMstrDlyObs2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DDL test observation delays for slice 2 master DDL."]
    #[inline(always)]
    pub fn phy_ddl_test_mstr_dly_obs_2(&self) -> PhyDdlTestMstrDlyObs2R {
        PhyDdlTestMstrDlyObs2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_307::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy307Spec;
impl crate::RegisterSpec for DenaliPhy307Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_307::R`](R) reader structure"]
impl crate::Readable for DenaliPhy307Spec {}
#[doc = "`reset()` method sets DENALI_PHY_307 to value 0"]
impl crate::Resettable for DenaliPhy307Spec {
    const RESET_VALUE: u32 = 0;
}
