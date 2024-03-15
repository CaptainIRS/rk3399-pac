#[doc = "Register `DENALI_PHY_169` reader"]
pub type R = crate::R<DenaliPhy169Spec>;
#[doc = "Field `PHY_GATE_SMPL1_SLV_DLY_ENC_OBS_1` reader - Observation register for gate sample1 slave delay encoded values for slice 1. READ-ONLY"]
pub type PhyGateSmpl1SlvDlyEncObs1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GATE_SMPL2_SLV_DLY_ENC_OBS_1` reader - Observation register for gate sample1 slave delay encoded values for slice 1. READ-ONLY"]
pub type PhyGateSmpl2SlvDlyEncObs1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - Observation register for gate sample1 slave delay encoded values for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_gate_smpl1_slv_dly_enc_obs_1(&self) -> PhyGateSmpl1SlvDlyEncObs1R {
        PhyGateSmpl1SlvDlyEncObs1R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Observation register for gate sample1 slave delay encoded values for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_gate_smpl2_slv_dly_enc_obs_1(&self) -> PhyGateSmpl2SlvDlyEncObs1R {
        PhyGateSmpl2SlvDlyEncObs1R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_169::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy169Spec;
impl crate::RegisterSpec for DenaliPhy169Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_169::R`](R) reader structure"]
impl crate::Readable for DenaliPhy169Spec {}
#[doc = "`reset()` method sets DENALI_PHY_169 to value 0"]
impl crate::Resettable for DenaliPhy169Spec {
    const RESET_VALUE: u32 = 0;
}
