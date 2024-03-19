#[doc = "Register `DDR_DENALI_PHY_649` reader"]
pub type R = crate::R<DdrDenaliPhy649Spec>;
#[doc = "Field `PHY_ADR_DDL_TEST_OBS_1` reader - DDL test observation for address slice 1."]
pub type PhyAdrDdlTestObs1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DDL test observation for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_ddl_test_obs_1(&self) -> PhyAdrDdlTestObs1R {
        PhyAdrDdlTestObs1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_649::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy649Spec;
impl crate::RegisterSpec for DdrDenaliPhy649Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_649::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy649Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_649 to value 0"]
impl crate::Resettable for DdrDenaliPhy649Spec {
    const RESET_VALUE: u32 = 0;
}
