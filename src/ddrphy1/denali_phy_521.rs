#[doc = "Register `DENALI_PHY_521` reader"]
pub type R = crate::R<DenaliPhy521Spec>;
#[doc = "Field `PHY_ADR_DDL_TEST_OBS_0` reader - DDL test observation for address slice 0. READ-ONLY"]
pub type PhyAdrDdlTestObs0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DDL test observation for address slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_adr_ddl_test_obs_0(&self) -> PhyAdrDdlTestObs0R {
        PhyAdrDdlTestObs0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_521::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy521Spec;
impl crate::RegisterSpec for DenaliPhy521Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_521::R`](R) reader structure"]
impl crate::Readable for DenaliPhy521Spec {}
#[doc = "`reset()` method sets DENALI_PHY_521 to value 0"]
impl crate::Resettable for DenaliPhy521Spec {
    const RESET_VALUE: u32 = 0;
}
