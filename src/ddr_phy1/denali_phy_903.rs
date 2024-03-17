#[doc = "Register `DENALI_PHY_903` reader"]
pub type R = crate::R<DenaliPhy903Spec>;
#[doc = "Field `PHY_CSLVL_OBS0` reader - Observation register for CS training. READ-ONLY"]
pub type PhyCslvlObs0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register for CS training. READ-ONLY"]
    #[inline(always)]
    pub fn phy_cslvl_obs0(&self) -> PhyCslvlObs0R {
        PhyCslvlObs0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_903::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy903Spec;
impl crate::RegisterSpec for DenaliPhy903Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_903::R`](R) reader structure"]
impl crate::Readable for DenaliPhy903Spec {}
#[doc = "`reset()` method sets DENALI_PHY_903 to value 0"]
impl crate::Resettable for DenaliPhy903Spec {
    const RESET_VALUE: u32 = 0;
}
