#[doc = "Register `DENALI_PHY_650` reader"]
pub type R = crate::R<DenaliPhy650Spec>;
#[doc = "Field `PHY_ADR_DDL_TEST_MSTR_DLY_OBS_1` reader - DDL test observation delays for address slice 1 master DDL. READ-ONLY"]
pub type PhyAdrDdlTestMstrDlyObs1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DDL test observation delays for address slice 1 master DDL. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_ddl_test_mstr_dly_obs_1(&self) -> PhyAdrDdlTestMstrDlyObs1R {
        PhyAdrDdlTestMstrDlyObs1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_650::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy650Spec;
impl crate::RegisterSpec for DenaliPhy650Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_650::R`](R) reader structure"]
impl crate::Readable for DenaliPhy650Spec {}
#[doc = "`reset()` method sets DENALI_PHY_650 to value 0"]
impl crate::Resettable for DenaliPhy650Spec {
    const RESET_VALUE: u32 = 0;
}
