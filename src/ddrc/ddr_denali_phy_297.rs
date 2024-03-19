#[doc = "Register `DDR_DENALI_PHY_297` reader"]
pub type R = crate::R<DdrDenaliPhy297Spec>;
#[doc = "Field `PHY_GATE_SMPL1_SLV_DLY_ENC_OBS_2` reader - Observation register for gate sample1 slave delay encoded values for slice 2."]
pub type PhyGateSmpl1SlvDlyEncObs2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GATE_SMPL2_SLV_DLY_ENC_OBS_2` reader - Observation register for gate sample1 slave delay encoded values for slice 2."]
pub type PhyGateSmpl2SlvDlyEncObs2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - Observation register for gate sample1 slave delay encoded values for slice 2."]
    #[inline(always)]
    pub fn phy_gate_smpl1_slv_dly_enc_obs_2(&self) -> PhyGateSmpl1SlvDlyEncObs2R {
        PhyGateSmpl1SlvDlyEncObs2R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Observation register for gate sample1 slave delay encoded values for slice 2."]
    #[inline(always)]
    pub fn phy_gate_smpl2_slv_dly_enc_obs_2(&self) -> PhyGateSmpl2SlvDlyEncObs2R {
        PhyGateSmpl2SlvDlyEncObs2R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_297::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy297Spec;
impl crate::RegisterSpec for DdrDenaliPhy297Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_297::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy297Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_297 to value 0"]
impl crate::Resettable for DdrDenaliPhy297Spec {
    const RESET_VALUE: u32 = 0;
}
