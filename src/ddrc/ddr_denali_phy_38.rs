#[doc = "Register `DDR_DENALI_PHY_38` reader"]
pub type R = crate::R<DdrDenaliPhy38Spec>;
#[doc = "Field `PHY_WR_ADDER_SLV_DLY_ENC_OBS_0` reader - Observation register for write adder slave delay encoded value for slice 0. READ-ONLY"]
pub type PhyWrAdderSlvDlyEncObs0R = crate::FieldReader;
#[doc = "Field `PHY_WR_SHIFT_OBS_0` reader - Observation register for automatic half cycle and cycle shift values for slice 0. READ-ONLY"]
pub type PhyWrShiftObs0R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_HARD0_DELAY_OBS_0` reader - Observation register for write leveling last hard 0 DQS slave delay for slice 0. READ-ONLY"]
pub type PhyWrlvlHard0DelayObs0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Observation register for write adder slave delay encoded value for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wr_adder_slv_dly_enc_obs_0(&self) -> PhyWrAdderSlvDlyEncObs0R {
        PhyWrAdderSlvDlyEncObs0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Observation register for automatic half cycle and cycle shift values for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wr_shift_obs_0(&self) -> PhyWrShiftObs0R {
        PhyWrShiftObs0R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:25 - Observation register for write leveling last hard 0 DQS slave delay for slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn phy_wrlvl_hard0_delay_obs_0(&self) -> PhyWrlvlHard0DelayObs0R {
        PhyWrlvlHard0DelayObs0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_38::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy38Spec;
impl crate::RegisterSpec for DdrDenaliPhy38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_38::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy38Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_38 to value 0"]
impl crate::Resettable for DdrDenaliPhy38Spec {
    const RESET_VALUE: u32 = 0;
}
