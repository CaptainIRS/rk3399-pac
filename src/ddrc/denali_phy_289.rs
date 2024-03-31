#[doc = "Register `DENALI_PHY_289` reader"]
pub type R = crate::R<DenaliPhy289Spec>;
#[doc = "Field `PHY_FIFO_PTR_OBS_2` reader - Observation register for read entry FIFO pointers for slice 2."]
pub type PhyFifoPtrObs2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Observation register for read entry FIFO pointers for slice 2."]
    #[inline(always)]
    pub fn phy_fifo_ptr_obs_2(&self) -> PhyFifoPtrObs2R {
        PhyFifoPtrObs2R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_289::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy289Spec;
impl crate::RegisterSpec for DenaliPhy289Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_289::R`](R) reader structure"]
impl crate::Readable for DenaliPhy289Spec {}
#[doc = "`reset()` method sets DENALI_PHY_289 to value 0"]
impl crate::Resettable for DenaliPhy289Spec {
    const RESET_VALUE: u32 = 0;
}
