#[doc = "Register `DDR_DENALI_PHY_40` reader"]
pub type R = crate::R<DdrDenaliPhy40Spec>;
#[doc = "Field `PHY_WRLVL_STATUS_OBS_0` reader - Observation register for write leveling status for slice 0. READ- ONLY"]
pub type PhyWrlvlStatusObs0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - Observation register for write leveling status for slice 0. READ- ONLY"]
    #[inline(always)]
    pub fn phy_wrlvl_status_obs_0(&self) -> PhyWrlvlStatusObs0R {
        PhyWrlvlStatusObs0R::new(self.bits & 0x0001_ffff)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_40::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy40Spec;
impl crate::RegisterSpec for DdrDenaliPhy40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_40::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy40Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_40 to value 0"]
impl crate::Resettable for DdrDenaliPhy40Spec {
    const RESET_VALUE: u32 = 0;
}
