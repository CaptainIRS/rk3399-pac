#[doc = "Register `DENALI_PHY_417` reader"]
pub type R = crate::R<DenaliPhy417Spec>;
#[doc = "Field `PHY_FIFO_PTR_OBS_3` reader - Observation register for read entry FIFO pointers for slice 3."]
pub type PhyFifoPtrObs3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Observation register for read entry FIFO pointers for slice 3."]
    #[inline(always)]
    pub fn phy_fifo_ptr_obs_3(&self) -> PhyFifoPtrObs3R {
        PhyFifoPtrObs3R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_417::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy417Spec;
impl crate::RegisterSpec for DenaliPhy417Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_417::R`](R) reader structure"]
impl crate::Readable for DenaliPhy417Spec {}
#[doc = "`reset()` method sets DENALI_PHY_417 to value 0"]
impl crate::Resettable for DenaliPhy417Spec {
    const RESET_VALUE: u32 = 0;
}
