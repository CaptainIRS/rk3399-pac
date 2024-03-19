#[doc = "Register `DDR_DENALI_PHY_310` reader"]
pub type R = crate::R<DdrDenaliPhy310Spec>;
#[doc = "Register `DDR_DENALI_PHY_310` writer"]
pub type W = crate::W<DdrDenaliPhy310Spec>;
#[doc = "Field `PHY_RX_CAL_DQ2_2` reader - RX Calibration codes for DQ2 for slice 2. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq2_2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ2_2` writer - RX Calibration codes for DQ2 for slice 2. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq2_2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PHY_RX_CAL_DQ3_2` reader - RX Calibration codes for DQ3 for slice 2. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq3_2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ3_2` writer - RX Calibration codes for DQ3 for slice 2. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq3_2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - RX Calibration codes for DQ2 for slice 2. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq2_2(&self) -> PhyRxCalDq2_2R {
        PhyRxCalDq2_2R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQ3 for slice 2. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq3_2(&self) -> PhyRxCalDq3_2R {
        PhyRxCalDq3_2R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - RX Calibration codes for DQ2 for slice 2. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq2_2(&mut self) -> PhyRxCalDq2_2W<DdrDenaliPhy310Spec> {
        PhyRxCalDq2_2W::new(self, 0)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQ3 for slice 2. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq3_2(&mut self) -> PhyRxCalDq3_2W<DdrDenaliPhy310Spec> {
        PhyRxCalDq3_2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_310::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_310::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy310Spec;
impl crate::RegisterSpec for DdrDenaliPhy310Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_310::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy310Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_310::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy310Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_310 to value 0"]
impl crate::Resettable for DdrDenaliPhy310Spec {
    const RESET_VALUE: u32 = 0;
}
