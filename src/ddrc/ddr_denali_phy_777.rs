#[doc = "Register `DDR_DENALI_PHY_777` reader"]
pub type R = crate::R<DdrDenaliPhy777Spec>;
#[doc = "Field `PHY_ADR_DDL_TEST_OBS_2` reader - DDL test observation for address slice 2."]
pub type PhyAdrDdlTestObs2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DDL test observation for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_ddl_test_obs_2(&self) -> PhyAdrDdlTestObs2R {
        PhyAdrDdlTestObs2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_777::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy777Spec;
impl crate::RegisterSpec for DdrDenaliPhy777Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_777::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy777Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_777 to value 0"]
impl crate::Resettable for DdrDenaliPhy777Spec {
    const RESET_VALUE: u32 = 0;
}
