#[doc = "Register `DDR_DENALI_PHY_142` reader"]
pub type R = crate::R<DdrDenaliPhy142Spec>;
#[doc = "Field `PHY_GATE_TRACKING_OBS_1` reader - Report the on the fly gate measurement result for slice 1."]
pub type PhyGateTrackingObs1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Report the on the fly gate measurement result for slice 1."]
    #[inline(always)]
    pub fn phy_gate_tracking_obs_1(&self) -> PhyGateTrackingObs1R {
        PhyGateTrackingObs1R::new(self.bits)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_142::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy142Spec;
impl crate::RegisterSpec for DdrDenaliPhy142Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_142::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy142Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_142 to value 0"]
impl crate::Resettable for DdrDenaliPhy142Spec {
    const RESET_VALUE: u32 = 0;
}
