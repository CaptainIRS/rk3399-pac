#[doc = "Register `DDR_DENALI_PHY_161` reader"]
pub type R = crate::R<DdrDenaliPhy161Spec>;
#[doc = "Field `PHY_FIFO_PTR_OBS_1` reader - Observation register for read entry FIFO pointers for slice 1. READ- ONLY"]
pub type PhyFifoPtrObs1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Observation register for read entry FIFO pointers for slice 1. READ- ONLY"]
    #[inline(always)]
    pub fn phy_fifo_ptr_obs_1(&self) -> PhyFifoPtrObs1R {
        PhyFifoPtrObs1R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_161::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy161Spec;
impl crate::RegisterSpec for DdrDenaliPhy161Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_161::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy161Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_161 to value 0"]
impl crate::Resettable for DdrDenaliPhy161Spec {
    const RESET_VALUE: u32 = 0;
}
