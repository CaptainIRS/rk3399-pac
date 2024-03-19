#[doc = "Register `DDR_DENALI_PHY_417` reader"]
pub type R = crate::R<DdrDenaliPhy417Spec>;
#[doc = "Field `PHY_FIFO_PTR_OBS_3` reader - Observation register for read entry FIFO pointers for slice 3. READ- ONLY"]
pub type PhyFifoPtrObs3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Observation register for read entry FIFO pointers for slice 3. READ- ONLY"]
    #[inline(always)]
    pub fn phy_fifo_ptr_obs_3(&self) -> PhyFifoPtrObs3R {
        PhyFifoPtrObs3R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_417::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy417Spec;
impl crate::RegisterSpec for DdrDenaliPhy417Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_417::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy417Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_417 to value 0"]
impl crate::Resettable for DdrDenaliPhy417Spec {
    const RESET_VALUE: u32 = 0;
}
