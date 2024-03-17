#[doc = "Register `DENALI_PHY_164` reader"]
pub type R = crate::R<DenaliPhy164Spec>;
#[doc = "Field `PHY_RDDQ_SLV_DLY_ENC_OBS_1` reader - Observation register for read DQ slave delay encoded values for slice 1. READ-ONLY"]
pub type PhyRddqSlvDlyEncObs1R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_BASE_SLV_DLY_ENC_OBS_1` reader - Observation register for read DQS base slave delay encoded value for slice 1. READ-ONLY"]
pub type PhyRddqsBaseSlvDlyEncObs1R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_DQ_RISE_ADDER_SLV_DLY_ENC_OBS_1` reader - Observation register for read DQS DQ rising edge adder slave delay encoded value for slice 1. READ- ONLY"]
pub type PhyRddqsDqRiseAdderSlvDlyEncObs1R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_DQ_FALL_ADDER_SLV_DLY_ENC_OBS_1` reader - Observation register for read DQS DQ falling edge adder slave delay encoded value for slice 1. READ- ONLY"]
pub type PhyRddqsDqFallAdderSlvDlyEncObs1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Observation register for read DQ slave delay encoded values for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_rddq_slv_dly_enc_obs_1(&self) -> PhyRddqSlvDlyEncObs1R {
        PhyRddqSlvDlyEncObs1R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:14 - Observation register for read DQS base slave delay encoded value for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_rddqs_base_slv_dly_enc_obs_1(&self) -> PhyRddqsBaseSlvDlyEncObs1R {
        PhyRddqsBaseSlvDlyEncObs1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:23 - Observation register for read DQS DQ rising edge adder slave delay encoded value for slice 1. READ- ONLY"]
    #[inline(always)]
    pub fn phy_rddqs_dq_rise_adder_slv_dly_enc_obs_1(&self) -> PhyRddqsDqRiseAdderSlvDlyEncObs1R {
        PhyRddqsDqRiseAdderSlvDlyEncObs1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Observation register for read DQS DQ falling edge adder slave delay encoded value for slice 1. READ- ONLY"]
    #[inline(always)]
    pub fn phy_rddqs_dq_fall_adder_slv_dly_enc_obs_1(&self) -> PhyRddqsDqFallAdderSlvDlyEncObs1R {
        PhyRddqsDqFallAdderSlvDlyEncObs1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_164::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy164Spec;
impl crate::RegisterSpec for DenaliPhy164Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_164::R`](R) reader structure"]
impl crate::Readable for DenaliPhy164Spec {}
#[doc = "`reset()` method sets DENALI_PHY_164 to value 0"]
impl crate::Resettable for DenaliPhy164Spec {
    const RESET_VALUE: u32 = 0;
}
