#[doc = "Register `DENALI_PHY_425` reader"]
pub type R = crate::R<DenaliPhy425Spec>;
#[doc = "Field `PHY_GATE_SMPL1_SLV_DLY_ENC_OBS_3` reader - Observation register for gate sample1 slave delay encoded values for slice 3."]
pub type PhyGateSmpl1SlvDlyEncObs3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GATE_SMPL2_SLV_DLY_ENC_OBS_3` reader - Observation register for gate sample1 slave delay encoded values for slice 3."]
pub type PhyGateSmpl2SlvDlyEncObs3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - Observation register for gate sample1 slave delay encoded values for slice 3."]
    #[inline(always)]
    pub fn phy_gate_smpl1_slv_dly_enc_obs_3(&self) -> PhyGateSmpl1SlvDlyEncObs3R {
        PhyGateSmpl1SlvDlyEncObs3R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Observation register for gate sample1 slave delay encoded values for slice 3."]
    #[inline(always)]
    pub fn phy_gate_smpl2_slv_dly_enc_obs_3(&self) -> PhyGateSmpl2SlvDlyEncObs3R {
        PhyGateSmpl2SlvDlyEncObs3R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_425::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy425Spec;
impl crate::RegisterSpec for DenaliPhy425Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_425::R`](R) reader structure"]
impl crate::Readable for DenaliPhy425Spec {}
#[doc = "`reset()` method sets DENALI_PHY_425 to value 0"]
impl crate::Resettable for DenaliPhy425Spec {
    const RESET_VALUE: u32 = 0;
}
