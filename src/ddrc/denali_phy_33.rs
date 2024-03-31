#[doc = "Register `DENALI_PHY_33` reader"]
pub type R = crate::R<DenaliPhy33Spec>;
#[doc = "Field `PHY_FIFO_PTR_OBS_0` reader - Observation register for read entry FIFO pointers for slice 0."]
pub type PhyFifoPtrObs0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Observation register for read entry FIFO pointers for slice 0."]
    #[inline(always)]
    pub fn phy_fifo_ptr_obs_0(&self) -> PhyFifoPtrObs0R {
        PhyFifoPtrObs0R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_33::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy33Spec;
impl crate::RegisterSpec for DenaliPhy33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_33::R`](R) reader structure"]
impl crate::Readable for DenaliPhy33Spec {}
#[doc = "`reset()` method sets DENALI_PHY_33 to value 0"]
impl crate::Resettable for DenaliPhy33Spec {
    const RESET_VALUE: u32 = 0;
}
