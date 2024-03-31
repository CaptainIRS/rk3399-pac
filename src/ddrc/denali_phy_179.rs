#[doc = "Register `DENALI_PHY_179` reader"]
pub type R = crate::R<DenaliPhy179Spec>;
#[doc = "Field `PHY_DDL_TEST_MSTR_DLY_OBS_1` reader - DDL test observation delays for slice 1 master DDL."]
pub type PhyDdlTestMstrDlyObs1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DDL test observation delays for slice 1 master DDL."]
    #[inline(always)]
    pub fn phy_ddl_test_mstr_dly_obs_1(&self) -> PhyDdlTestMstrDlyObs1R {
        PhyDdlTestMstrDlyObs1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_179::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy179Spec;
impl crate::RegisterSpec for DenaliPhy179Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_179::R`](R) reader structure"]
impl crate::Readable for DenaliPhy179Spec {}
#[doc = "`reset()` method sets DENALI_PHY_179 to value 0"]
impl crate::Resettable for DenaliPhy179Spec {
    const RESET_VALUE: u32 = 0;
}
