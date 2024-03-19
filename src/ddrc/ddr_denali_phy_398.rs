#[doc = "Register `DDR_DENALI_PHY_398` reader"]
pub type R = crate::R<DdrDenaliPhy398Spec>;
#[doc = "Field `PHY_GATE_TRACKING_OBS_3` reader - Report the on the fly gate measurement result for slice 3."]
pub type PhyGateTrackingObs3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Report the on the fly gate measurement result for slice 3."]
    #[inline(always)]
    pub fn phy_gate_tracking_obs_3(&self) -> PhyGateTrackingObs3R {
        PhyGateTrackingObs3R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_398::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy398Spec;
impl crate::RegisterSpec for DdrDenaliPhy398Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_398::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy398Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_398 to value 0"]
impl crate::Resettable for DdrDenaliPhy398Spec {
    const RESET_VALUE: u32 = 0;
}
