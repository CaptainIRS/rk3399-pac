#[doc = "Register `DDR_DENALI_PHY_184` reader"]
pub type R = crate::R<DdrDenaliPhy184Spec>;
#[doc = "Register `DDR_DENALI_PHY_184` writer"]
pub type W = crate::W<DdrDenaliPhy184Spec>;
#[doc = "Field `PHY_RX_CAL_DQ6_1` reader - RX Calibration codes for DQ6 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq6_1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ6_1` writer - RX Calibration codes for DQ6 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq6_1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PHY_RX_CAL_DQ7_1` reader - RX Calibration codes for DQ7 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq7_1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ7_1` writer - RX Calibration codes for DQ7 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq7_1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - RX Calibration codes for DQ6 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq6_1(&self) -> PhyRxCalDq6_1R {
        PhyRxCalDq6_1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQ7 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq7_1(&self) -> PhyRxCalDq7_1R {
        PhyRxCalDq7_1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - RX Calibration codes for DQ6 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq6_1(&mut self) -> PhyRxCalDq6_1W<DdrDenaliPhy184Spec> {
        PhyRxCalDq6_1W::new(self, 0)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQ7 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq7_1(&mut self) -> PhyRxCalDq7_1W<DdrDenaliPhy184Spec> {
        PhyRxCalDq7_1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_184::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_184::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy184Spec;
impl crate::RegisterSpec for DdrDenaliPhy184Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_184::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy184Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_184::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy184Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_184 to value 0"]
impl crate::Resettable for DdrDenaliPhy184Spec {
    const RESET_VALUE: u32 = 0;
}
