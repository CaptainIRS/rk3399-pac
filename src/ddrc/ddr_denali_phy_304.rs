#[doc = "Register `DDR_DENALI_PHY_304` reader"]
pub type R = crate::R<DdrDenaliPhy304Spec>;
#[doc = "Field `PHY_WDQLVL_STATUS_OBS_2` reader - Observation register for write data leveling status for slice 2. READ- ONLY"]
pub type PhyWdqlvlStatusObs2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Observation register for write data leveling status for slice 2. READ- ONLY"]
    #[inline(always)]
    pub fn phy_wdqlvl_status_obs_2(&self) -> PhyWdqlvlStatusObs2R {
        PhyWdqlvlStatusObs2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_304::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy304Spec;
impl crate::RegisterSpec for DdrDenaliPhy304Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_304::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy304Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_304 to value 0"]
impl crate::Resettable for DdrDenaliPhy304Spec {
    const RESET_VALUE: u32 = 0;
}
