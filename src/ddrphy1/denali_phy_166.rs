#[doc = "Register `DENALI_PHY_166` reader"]
pub type R = crate::R<DenaliPhy166Spec>;
#[doc = "Field `PHY_WR_ADDER_SLV_DLY_ENC_OBS_1` reader - Observation register for write adder slave delay encoded value for slice 1. READ-ONLY"]
pub type PhyWrAdderSlvDlyEncObs1R = crate::FieldReader;
#[doc = "Field `PHY_WR_SHIFT_OBS_1` reader - Observation register for automatic half cycle and cycle shift values for slice 1. READ-ONLY"]
pub type PhyWrShiftObs1R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_HARD0_DELAY_OBS_1` reader - Observation register for write leveling last hard 0 DQS slave delay for slice 1. READ-ONLY"]
pub type PhyWrlvlHard0DelayObs1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Observation register for write adder slave delay encoded value for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wr_adder_slv_dly_enc_obs_1(&self) -> PhyWrAdderSlvDlyEncObs1R {
        PhyWrAdderSlvDlyEncObs1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Observation register for automatic half cycle and cycle shift values for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wr_shift_obs_1(&self) -> PhyWrShiftObs1R {
        PhyWrShiftObs1R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:25 - Observation register for write leveling last hard 0 DQS slave delay for slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wrlvl_hard0_delay_obs_1(&self) -> PhyWrlvlHard0DelayObs1R {
        PhyWrlvlHard0DelayObs1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_166::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy166Spec;
impl crate::RegisterSpec for DenaliPhy166Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_166::R`](R) reader structure"]
impl crate::Readable for DenaliPhy166Spec {}
#[doc = "`reset()` method sets DENALI_PHY_166 to value 0"]
impl crate::Resettable for DenaliPhy166Spec {
    const RESET_VALUE: u32 = 0;
}
