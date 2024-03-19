#[doc = "Register `DDR_DENALI_PHY_778` reader"]
pub type R = crate::R<DdrDenaliPhy778Spec>;
#[doc = "Field `PHY_ADR_DDL_TEST_MSTR_DLY_OBS_2` reader - DDL test observation delays for address slice 2 master DDL."]
pub type PhyAdrDdlTestMstrDlyObs2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DDL test observation delays for address slice 2 master DDL."]
    #[inline(always)]
    pub fn phy_adr_ddl_test_mstr_dly_obs_2(&self) -> PhyAdrDdlTestMstrDlyObs2R {
        PhyAdrDdlTestMstrDlyObs2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_778::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy778Spec;
impl crate::RegisterSpec for DdrDenaliPhy778Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_778::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy778Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_778 to value 0"]
impl crate::Resettable for DdrDenaliPhy778Spec {
    const RESET_VALUE: u32 = 0;
}
