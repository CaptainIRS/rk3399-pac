#[doc = "Register `DDR_DENALI_PHY_294` reader"]
pub type R = crate::R<DdrDenaliPhy294Spec>;
#[doc = "Field `PHY_WR_ADDER_SLV_DLY_ENC_OBS_2` reader - Observation register for write adder slave delay encoded value for slice 2."]
pub type PhyWrAdderSlvDlyEncObs2R = crate::FieldReader;
#[doc = "Field `PHY_WR_SHIFT_OBS_2` reader - Observation register for automatic half cycle and cycle shift values for slice 2."]
pub type PhyWrShiftObs2R = crate::FieldReader;
#[doc = "Field `PHY_WRLVL_HARD0_DELAY_OBS_2` reader - Observation register for write leveling last hard 0 DQS slave delay for slice 2."]
pub type PhyWrlvlHard0DelayObs2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Observation register for write adder slave delay encoded value for slice 2."]
    #[inline(always)]
    pub fn phy_wr_adder_slv_dly_enc_obs_2(&self) -> PhyWrAdderSlvDlyEncObs2R {
        PhyWrAdderSlvDlyEncObs2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Observation register for automatic half cycle and cycle shift values for slice 2."]
    #[inline(always)]
    pub fn phy_wr_shift_obs_2(&self) -> PhyWrShiftObs2R {
        PhyWrShiftObs2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:25 - Observation register for write leveling last hard 0 DQS slave delay for slice 2."]
    #[inline(always)]
    pub fn phy_wrlvl_hard0_delay_obs_2(&self) -> PhyWrlvlHard0DelayObs2R {
        PhyWrlvlHard0DelayObs2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_294::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy294Spec;
impl crate::RegisterSpec for DdrDenaliPhy294Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_294::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy294Spec {}
#[doc = "`reset()` method sets DDR_DENALI_PHY_294 to value 0"]
impl crate::Resettable for DdrDenaliPhy294Spec {
    const RESET_VALUE: u32 = 0;
}
