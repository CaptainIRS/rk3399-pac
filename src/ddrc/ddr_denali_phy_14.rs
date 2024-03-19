#[doc = "Register `DDR_DENALI_PHY_14` reader"]
pub type R = crate::R<DdrDenaliPhy14Spec>;
#[doc = "Field `PHY_GATE_TRACKING_OBS_0` reader - Report the on the fly gate measurement result for slice 0."]
pub type PhyGateTrackingObs0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Report the on the fly gate measurement result for slice 0."]
    #[inline(always)]
    pub fn phy_gate_tracking_obs_0(&self) -> PhyGateTrackingObs0R {
        PhyGateTrackingObs0R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_14::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy14Spec;
impl crate::RegisterSpec for DdrDenaliPhy14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_14::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy14Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_14 to value 0"]
impl crate::Resettable for DdrDenaliPhy14Spec {
    const RESET_VALUE: u32 = 0;
}
