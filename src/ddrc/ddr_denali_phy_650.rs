#[doc = "Register `DDR_DENALI_PHY_650` reader"]
pub type R = crate::R<DdrDenaliPhy650Spec>;
#[doc = "Field `PHY_ADR_DDL_TEST_MSTR_DLY_OBS_1` reader - DDL test observation delays for address slice 1 master DDL."]
pub type PhyAdrDdlTestMstrDlyObs1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DDL test observation delays for address slice 1 master DDL."]
    #[inline(always)]
    pub fn phy_adr_ddl_test_mstr_dly_obs_1(&self) -> PhyAdrDdlTestMstrDlyObs1R {
        PhyAdrDdlTestMstrDlyObs1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_650::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy650Spec;
impl crate::RegisterSpec for DdrDenaliPhy650Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_650::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy650Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_650 to value 0"]
impl crate::Resettable for DdrDenaliPhy650Spec {
    const RESET_VALUE: u32 = 0;
}
