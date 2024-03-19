#[doc = "Register `DDR_DENALI_PHY_37` reader"]
pub type R = crate::R<DdrDenaliPhy37Spec>;
#[doc = "Field `PHY_RDDQS_GATE_SLV_DLY_ENC_OBS_0` reader - Observation register for read DQS gate slave delay encoded value for slice 0. READ-ONLY"]
pub type PhyRddqsGateSlvDlyEncObs0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_WRDQS_BASE_SLV_DLY_ENC_OBS_0` reader - Observation register for write DQS base slave delay encoded value for slice 0. READ-ONLY"]
pub type PhyWrdqsBaseSlvDlyEncObs0R = crate::FieldReader;
#[doc = "Field `PHY_WRDQ_BASE_SLV_DLY_ENC_OBS_0` reader - Observation register for write DQ base slave delay encoded value for slice 0. READ-ONLY"]
pub type PhyWrdqBaseSlvDlyEncObs0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:9 - Observation register for read DQS gate slave delay encoded value for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_rddqs_gate_slv_dly_enc_obs_0(&self) -> PhyRddqsGateSlvDlyEncObs0R {
        PhyRddqsGateSlvDlyEncObs0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:22 - Observation register for write DQS base slave delay encoded value for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wrdqs_base_slv_dly_enc_obs_0(&self) -> PhyWrdqsBaseSlvDlyEncObs0R {
        PhyWrdqsBaseSlvDlyEncObs0R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:31 - Observation register for write DQ base slave delay encoded value for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wrdq_base_slv_dly_enc_obs_0(&self) -> PhyWrdqBaseSlvDlyEncObs0R {
        PhyWrdqBaseSlvDlyEncObs0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_37::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy37Spec;
impl crate::RegisterSpec for DdrDenaliPhy37Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_37::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy37Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_37 to value 0"]
impl crate::Resettable for DdrDenaliPhy37Spec {
    const RESET_VALUE: u32 = 0;
}
