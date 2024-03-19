#[doc = "Register `DDR_DENALI_PHY_292` reader"]
pub type R = crate::R<DdrDenaliPhy292Spec>;
#[doc = "Field `PHY_RDDQ_SLV_DLY_ENC_OBS_2` reader - Observation register for read DQ slave delay encoded values for slice 2."]
pub type PhyRddqSlvDlyEncObs2R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_BASE_SLV_DLY_ENC_OBS_2` reader - Observation register for read DQS base slave delay encoded value for slice 2."]
pub type PhyRddqsBaseSlvDlyEncObs2R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_DQ_RISE_ADDER_SLV_DLY_ENC_OBS_2` reader - Observation register for read DQS DQ rising edge adder slave delay encoded value for slice 2."]
pub type PhyRddqsDqRiseAdderSlvDlyEncObs2R = crate::FieldReader;
#[doc = "Field `PHY_RDDQS_DQ_FALL_ADDER_SLV_DLY_ENC_OBS_2` reader - Observation register for read DQS DQ falling edge adder slave delay encoded value for slice 2."]
pub type PhyRddqsDqFallAdderSlvDlyEncObs2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Observation register for read DQ slave delay encoded values for slice 2."]
    #[inline(always)]
    pub fn phy_rddq_slv_dly_enc_obs_2(&self) -> PhyRddqSlvDlyEncObs2R {
        PhyRddqSlvDlyEncObs2R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:14 - Observation register for read DQS base slave delay encoded value for slice 2."]
    #[inline(always)]
    pub fn phy_rddqs_base_slv_dly_enc_obs_2(&self) -> PhyRddqsBaseSlvDlyEncObs2R {
        PhyRddqsBaseSlvDlyEncObs2R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:23 - Observation register for read DQS DQ rising edge adder slave delay encoded value for slice 2."]
    #[inline(always)]
    pub fn phy_rddqs_dq_rise_adder_slv_dly_enc_obs_2(&self) -> PhyRddqsDqRiseAdderSlvDlyEncObs2R {
        PhyRddqsDqRiseAdderSlvDlyEncObs2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Observation register for read DQS DQ falling edge adder slave delay encoded value for slice 2."]
    #[inline(always)]
    pub fn phy_rddqs_dq_fall_adder_slv_dly_enc_obs_2(&self) -> PhyRddqsDqFallAdderSlvDlyEncObs2R {
        PhyRddqsDqFallAdderSlvDlyEncObs2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_292::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy292Spec;
impl crate::RegisterSpec for DdrDenaliPhy292Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_292::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy292Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_292 to value 0"]
impl crate::Resettable for DdrDenaliPhy292Spec {
    const RESET_VALUE: u32 = 0;
}
