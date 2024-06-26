#[doc = "Register `DENALI_PHY_36` reader"]
pub type R = crate::R<DenaliPhy36Spec>;
#[doc = "Field `PHY_RDDQ_SLV_DLY_ENC_OBS_0` reader - Observation register for read DQ slave delay encoded values for slice 0."]
pub type PhyRddqSlvDlyEncObs0R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_BASE_SLV_DLY_ENC_OBS_0` reader - Observation register for read DQS base slave delay encoded value for slice 0."]
pub type PhyRddqsBaseSlvDlyEncObs0R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_DQ_RISE_ADDER_SLV_DLY_ENC_OBS_0` reader - Observation register for read DQS DQ rising edge adder slave delay encoded value for slice 0."]
pub type PhyRddqsDqRiseAdderSlvDlyEncObs0R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_DQ_FALL_ADDER_SLV_DLY_ENC_OBS_0` reader - Observation register for read DQS DQ falling edge adder slave delay encoded value for slice 0."]
pub type PhyRddqsDqFallAdderSlvDlyEncObs0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Observation register for read DQ slave delay encoded values for slice 0."]
    #[inline(always)]
    pub fn phy_rddq_slv_dly_enc_obs_0(&self) -> PhyRddqSlvDlyEncObs0R {
        PhyRddqSlvDlyEncObs0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:14 - Observation register for read DQS base slave delay encoded value for slice 0."]
    #[inline(always)]
    pub fn phy_rddqs_base_slv_dly_enc_obs_0(&self) -> PhyRddqsBaseSlvDlyEncObs0R {
        PhyRddqsBaseSlvDlyEncObs0R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:23 - Observation register for read DQS DQ rising edge adder slave delay encoded value for slice 0."]
    #[inline(always)]
    pub fn phy_rddqs_dq_rise_adder_slv_dly_enc_obs_0(&self) -> PhyRddqsDqRiseAdderSlvDlyEncObs0R {
        PhyRddqsDqRiseAdderSlvDlyEncObs0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Observation register for read DQS DQ falling edge adder slave delay encoded value for slice 0."]
    #[inline(always)]
    pub fn phy_rddqs_dq_fall_adder_slv_dly_enc_obs_0(&self) -> PhyRddqsDqFallAdderSlvDlyEncObs0R {
        PhyRddqsDqFallAdderSlvDlyEncObs0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_36::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy36Spec;
impl crate::RegisterSpec for DenaliPhy36Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_36::R`](R) reader structure"]
impl crate::Readable for DenaliPhy36Spec {}
#[doc = "`reset()` method sets DENALI_PHY_36 to value 0"]
impl crate::Resettable for DenaliPhy36Spec {
    const RESET_VALUE: u32 = 0;
}
