#[doc = "Register `DDR_DENALI_PHY_41` reader"]
pub type R = crate::R<DdrDenaliPhy41Spec>;
#[doc = "Field `PHY_GATE_SMPL1_SLV_DLY_ENC_OBS_0` reader - Observation register for gate sample1 slave delay encoded values for slice 0."]
pub type PhyGateSmpl1SlvDlyEncObs0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GATE_SMPL2_SLV_DLY_ENC_OBS_0` reader - Observation register for gate sample1 slave delay encoded values for slice 0."]
pub type PhyGateSmpl2SlvDlyEncObs0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - Observation register for gate sample1 slave delay encoded values for slice 0."]
    #[inline(always)]
    pub fn phy_gate_smpl1_slv_dly_enc_obs_0(&self) -> PhyGateSmpl1SlvDlyEncObs0R {
        PhyGateSmpl1SlvDlyEncObs0R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Observation register for gate sample1 slave delay encoded values for slice 0."]
    #[inline(always)]
    pub fn phy_gate_smpl2_slv_dly_enc_obs_0(&self) -> PhyGateSmpl2SlvDlyEncObs0R {
        PhyGateSmpl2SlvDlyEncObs0R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_41::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy41Spec;
impl crate::RegisterSpec for DdrDenaliPhy41Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_41::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy41Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_41 to value 0"]
impl crate::Resettable for DdrDenaliPhy41Spec {
    const RESET_VALUE: u32 = 0;
}
