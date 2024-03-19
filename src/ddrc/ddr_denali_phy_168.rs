#[doc = "Register `DDR_DENALI_PHY_168` reader"]
pub type R = crate::R<DdrDenaliPhy168Spec>;
#[doc = "Field `PHY_WRLVL_STATUS_OBS_1` reader - Observation register for write leveling status for slice 1."]
pub type PhyWrlvlStatusObs1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - Observation register for write leveling status for slice 1."]
    #[inline(always)]
    pub fn phy_wrlvl_status_obs_1(&self) -> PhyWrlvlStatusObs1R {
        PhyWrlvlStatusObs1R::new(self.bits & 0x0001_ffff)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_168::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy168Spec;
impl crate::RegisterSpec for DdrDenaliPhy168Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_168::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy168Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_168 to value 0"]
impl crate::Resettable for DdrDenaliPhy168Spec {
    const RESET_VALUE: u32 = 0;
}
