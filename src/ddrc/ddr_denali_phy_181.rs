#[doc = "Register `DDR_DENALI_PHY_181` reader"]
pub type R = crate::R<DdrDenaliPhy181Spec>;
#[doc = "Register `DDR_DENALI_PHY_181` writer"]
pub type W = crate::W<DdrDenaliPhy181Spec>;
#[doc = "Field `PHY_RX_CAL_DQ0_1` reader - RX Calibration codes for DQ0 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq0_1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ0_1` writer - RX Calibration codes for DQ0 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq0_1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PHY_RX_CAL_DQ1_1` reader - RX Calibration codes for DQ1 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq1_1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ1_1` writer - RX Calibration codes for DQ1 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq1_1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - RX Calibration codes for DQ0 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq0_1(&self) -> PhyRxCalDq0_1R {
        PhyRxCalDq0_1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQ1 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq1_1(&self) -> PhyRxCalDq1_1R {
        PhyRxCalDq1_1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - RX Calibration codes for DQ0 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq0_1(&mut self) -> PhyRxCalDq0_1W<DdrDenaliPhy181Spec> {
        PhyRxCalDq0_1W::new(self, 0)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQ1 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq1_1(&mut self) -> PhyRxCalDq1_1W<DdrDenaliPhy181Spec> {
        PhyRxCalDq1_1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_181::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_181::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy181Spec;
impl crate::RegisterSpec for DdrDenaliPhy181Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_181::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy181Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_181::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy181Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_181 to value 0"]
impl crate::Resettable for DdrDenaliPhy181Spec {
    const RESET_VALUE: u32 = 0;
}
