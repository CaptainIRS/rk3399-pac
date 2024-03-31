#[doc = "Register `DENALI_PHY_183` reader"]
pub type R = crate::R<DenaliPhy183Spec>;
#[doc = "Register `DENALI_PHY_183` writer"]
pub type W = crate::W<DenaliPhy183Spec>;
#[doc = "Field `PHY_RX_CAL_DQ4_1` reader - RX Calibration codes for DQ4 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq4_1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ4_1` writer - RX Calibration codes for DQ4 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq4_1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PHY_RX_CAL_DQ5_1` reader - RX Calibration codes for DQ5 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq5_1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ5_1` writer - RX Calibration codes for DQ5 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq5_1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - RX Calibration codes for DQ4 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq4_1(&self) -> PhyRxCalDq4_1R {
        PhyRxCalDq4_1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQ5 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq5_1(&self) -> PhyRxCalDq5_1R {
        PhyRxCalDq5_1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - RX Calibration codes for DQ4 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq4_1(&mut self) -> PhyRxCalDq4_1W<DenaliPhy183Spec> {
        PhyRxCalDq4_1W::new(self, 0)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQ5 for slice 1. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq5_1(&mut self) -> PhyRxCalDq5_1W<DenaliPhy183Spec> {
        PhyRxCalDq5_1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_183::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_183::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy183Spec;
impl crate::RegisterSpec for DenaliPhy183Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_183::R`](R) reader structure"]
impl crate::Readable for DenaliPhy183Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_183::W`](W) writer structure"]
impl crate::Writable for DenaliPhy183Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_183 to value 0"]
impl crate::Resettable for DenaliPhy183Spec {
    const RESET_VALUE: u32 = 0;
}
