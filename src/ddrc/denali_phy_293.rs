#[doc = "Register `DENALI_PHY_293` reader"]
pub type R = crate::R<DenaliPhy293Spec>;
#[doc = "Field `PHY_RDDQS_GATE_SLV_DLY_ENC_OBS_2` reader - Observation register for read DQS gate slave delay encoded value for slice 2."]
pub type PhyRddqsGateSlvDlyEncObs2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WRDQS_BASE_SLV_DLY_ENC_OBS_2` reader - Observation register for write DQS base slave delay encoded value for slice 2."]
pub type PhyWrdqsBaseSlvDlyEncObs2R = crate::FieldReader;
#[doc = "Field `PHY_WRDQ_BASE_SLV_DLY_ENC_OBS_2` reader - Observation register for write DQ base slave delay encoded value for slice 2."]
pub type PhyWrdqBaseSlvDlyEncObs2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:9 - Observation register for read DQS gate slave delay encoded value for slice 2."]
    #[inline(always)]
    pub fn phy_rddqs_gate_slv_dly_enc_obs_2(&self) -> PhyRddqsGateSlvDlyEncObs2R {
        PhyRddqsGateSlvDlyEncObs2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:22 - Observation register for write DQS base slave delay encoded value for slice 2."]
    #[inline(always)]
    pub fn phy_wrdqs_base_slv_dly_enc_obs_2(&self) -> PhyWrdqsBaseSlvDlyEncObs2R {
        PhyWrdqsBaseSlvDlyEncObs2R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:31 - Observation register for write DQ base slave delay encoded value for slice 2."]
    #[inline(always)]
    pub fn phy_wrdq_base_slv_dly_enc_obs_2(&self) -> PhyWrdqBaseSlvDlyEncObs2R {
        PhyWrdqBaseSlvDlyEncObs2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_293::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy293Spec;
impl crate::RegisterSpec for DenaliPhy293Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_293::R`](R) reader structure"]
impl crate::Readable for DenaliPhy293Spec {}
#[doc = "`reset()` method sets DENALI_PHY_293 to value 0"]
impl crate::Resettable for DenaliPhy293Spec {
    const RESET_VALUE: u32 = 0;
}
