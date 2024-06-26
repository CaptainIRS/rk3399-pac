#[doc = "Register `DENALI_PHY_904` reader"]
pub type R = crate::R<DenaliPhy904Spec>;
#[doc = "Field `PHY_CSLVL_OBS1` reader - Observation register for CS training."]
pub type PhyCslvlObs1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register for CS training."]
    #[inline(always)]
    pub fn phy_cslvl_obs1(&self) -> PhyCslvlObs1R {
        PhyCslvlObs1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_904::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy904Spec;
impl crate::RegisterSpec for DenaliPhy904Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_904::R`](R) reader structure"]
impl crate::Readable for DenaliPhy904Spec {}
#[doc = "`reset()` method sets DENALI_PHY_904 to value 0"]
impl crate::Resettable for DenaliPhy904Spec {
    const RESET_VALUE: u32 = 0;
}
