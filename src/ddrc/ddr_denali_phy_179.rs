#[doc = "Register `DDR_DENALI_PHY_179` reader"]
pub type R = crate::R<DdrDenaliPhy179Spec>;
#[doc = "Field `PHY_DDL_TEST_MSTR_DLY_OBS_1` reader - DDL test observation delays for slice 1 master DDL."]
pub type PhyDdlTestMstrDlyObs1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DDL test observation delays for slice 1 master DDL."]
    #[inline(always)]
    pub fn phy_ddl_test_mstr_dly_obs_1(&self) -> PhyDdlTestMstrDlyObs1R {
        PhyDdlTestMstrDlyObs1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_179::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy179Spec;
impl crate::RegisterSpec for DdrDenaliPhy179Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_179::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy179Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_179 to value 0"]
impl crate::Resettable for DdrDenaliPhy179Spec {
    const RESET_VALUE: u32 = 0;
}
