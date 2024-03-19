#[doc = "Register `DDR_DENALI_PHY_270` reader"]
pub type R = crate::R<DdrDenaliPhy270Spec>;
#[doc = "Field `PHY_GATE_TRACKING_OBS_2` reader - Report the on the fly gate measurement result for slice 2."]
pub type PhyGateTrackingObs2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Report the on the fly gate measurement result for slice 2."]
    #[inline(always)]
    pub fn phy_gate_tracking_obs_2(&self) -> PhyGateTrackingObs2R {
        PhyGateTrackingObs2R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_270::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy270Spec;
impl crate::RegisterSpec for DdrDenaliPhy270Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_270::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy270Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_270 to value 0"]
impl crate::Resettable for DdrDenaliPhy270Spec {
    const RESET_VALUE: u32 = 0;
}
