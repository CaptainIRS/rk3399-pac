#[doc = "Register `DENALI_PHY_420` reader"]
pub type R = crate::R<DenaliPhy420Spec>;
#[doc = "Field `PHY_RDDQ_SLV_DLY_ENC_OBS_3` reader - Observation register for read DQ slave delay encoded values for slice 3."]
pub type PhyRddqSlvDlyEncObs3R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_BASE_SLV_DLY_ENC_OBS_3` reader - Observation register for read DQS base slave delay encoded value for slice 3."]
pub type PhyRddqsBaseSlvDlyEncObs3R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_DQ_RISE_ADDER_SLV_DLY_ENC_OBS_3` reader - Observation register for read DQS DQ rising edge adder slave delay encoded value for slice 3."]
pub type PhyRddqsDqRiseAdderSlvDlyEncObs3R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_DQ_FALL_ADDER_SLV_DLY_ENC_OBS_3` reader - Observation register for read DQS DQ falling edge adder slave delay encoded value for slice 3."]
pub type PhyRddqsDqFallAdderSlvDlyEncObs3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Observation register for read DQ slave delay encoded values for slice 3."]
    #[inline(always)]
    pub fn phy_rddq_slv_dly_enc_obs_3(&self) -> PhyRddqSlvDlyEncObs3R {
        PhyRddqSlvDlyEncObs3R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:14 - Observation register for read DQS base slave delay encoded value for slice 3."]
    #[inline(always)]
    pub fn phy_rddqs_base_slv_dly_enc_obs_3(&self) -> PhyRddqsBaseSlvDlyEncObs3R {
        PhyRddqsBaseSlvDlyEncObs3R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:23 - Observation register for read DQS DQ rising edge adder slave delay encoded value for slice 3."]
    #[inline(always)]
    pub fn phy_rddqs_dq_rise_adder_slv_dly_enc_obs_3(&self) -> PhyRddqsDqRiseAdderSlvDlyEncObs3R {
        PhyRddqsDqRiseAdderSlvDlyEncObs3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Observation register for read DQS DQ falling edge adder slave delay encoded value for slice 3."]
    #[inline(always)]
    pub fn phy_rddqs_dq_fall_adder_slv_dly_enc_obs_3(&self) -> PhyRddqsDqFallAdderSlvDlyEncObs3R {
        PhyRddqsDqFallAdderSlvDlyEncObs3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_420::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy420Spec;
impl crate::RegisterSpec for DenaliPhy420Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_420::R`](R) reader structure"]
impl crate::Readable for DenaliPhy420Spec {}
#[doc = "`reset()` method sets DENALI_PHY_420 to value 0"]
impl crate::Resettable for DenaliPhy420Spec {
    const RESET_VALUE: u32 = 0;
}
