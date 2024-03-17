#[doc = "Register `DENALI_PHY_522` reader"]
pub type R = crate::R<DenaliPhy522Spec>;
#[doc = "Field `PHY_ADR_DDL_TEST_MSTR_DLY_OBS_0` reader - DDL test observation delays for address slice 0 master DDL. READ-ONLY"]
pub type PhyAdrDdlTestMstrDlyObs0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DDL test observation delays for address slice 0 master DDL. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_ddl_test_mstr_dly_obs_0(&self) -> PhyAdrDdlTestMstrDlyObs0R {
        PhyAdrDdlTestMstrDlyObs0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_522::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy522Spec;
impl crate::RegisterSpec for DenaliPhy522Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_522::R`](R) reader structure"]
impl crate::Readable for DenaliPhy522Spec {}
#[doc = "`reset()` method sets DENALI_PHY_522 to value 0"]
impl crate::Resettable for DenaliPhy522Spec {
    const RESET_VALUE: u32 = 0;
}
