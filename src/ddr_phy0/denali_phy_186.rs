#[doc = "Register `DENALI_PHY_186` reader"]
pub type R = crate::R<DenaliPhy186Spec>;
#[doc = "Register `DENALI_PHY_186` writer"]
pub type W = crate::W<DenaliPhy186Spec>;
#[doc = "Field `PHY_RX_CAL_FDBK_1` reader - RX Calibration codes for FDBK for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalFdbk1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_FDBK_1` writer - RX Calibration codes for FDBK for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalFdbk1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PHY_RX_CAL_OBS_1` reader - RX Calibration results for slice 1. Bits (7:0) contain calibration results from DQ0-7. Bit (8) contains calibration result from DM. Bit (9) contains calibration result from DQS. Bit (10) contains calibration result from FDBK."]
pub type PhyRxCalObs1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - RX Calibration codes for FDBK for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_fdbk_1(&self) -> PhyRxCalFdbk1R {
        PhyRxCalFdbk1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:26 - RX Calibration results for slice 1. Bits (7:0) contain calibration results from DQ0-7. Bit (8) contains calibration result from DM. Bit (9) contains calibration result from DQS. Bit (10) contains calibration result from FDBK."]
    #[inline(always)]
    pub fn phy_rx_cal_obs_1(&self) -> PhyRxCalObs1R {
        PhyRxCalObs1R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - RX Calibration codes for FDBK for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_fdbk_1(&mut self) -> PhyRxCalFdbk1W<DenaliPhy186Spec> {
        PhyRxCalFdbk1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_186::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_186::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy186Spec;
impl crate::RegisterSpec for DenaliPhy186Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_186::R`](R) reader structure"]
impl crate::Readable for DenaliPhy186Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_186::W`](W) writer structure"]
impl crate::Writable for DenaliPhy186Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_186 to value 0"]
impl crate::Resettable for DenaliPhy186Spec {
    const RESET_VALUE: u32 = 0;
}
