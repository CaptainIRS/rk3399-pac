#[doc = "Register `DDR_DENALI_PHY_439` reader"]
pub type R = crate::R<DdrDenaliPhy439Spec>;
#[doc = "Register `DDR_DENALI_PHY_439` writer"]
pub type W = crate::W<DdrDenaliPhy439Spec>;
#[doc = "Field `PHY_RX_CAL_DQ4_3` reader - RX Calibration codes for DQ4 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq4_3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ4_3` writer - RX Calibration codes for DQ4 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq4_3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PHY_RX_CAL_DQ5_3` reader - RX Calibration codes for DQ5 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq5_3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ5_3` writer - RX Calibration codes for DQ5 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq5_3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - RX Calibration codes for DQ4 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq4_3(&self) -> PhyRxCalDq4_3R {
        PhyRxCalDq4_3R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQ5 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq5_3(&self) -> PhyRxCalDq5_3R {
        PhyRxCalDq5_3R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - RX Calibration codes for DQ4 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq4_3(&mut self) -> PhyRxCalDq4_3W<DdrDenaliPhy439Spec> {
        PhyRxCalDq4_3W::new(self, 0)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQ5 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq5_3(&mut self) -> PhyRxCalDq5_3W<DdrDenaliPhy439Spec> {
        PhyRxCalDq5_3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_439::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_439::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy439Spec;
impl crate::RegisterSpec for DdrDenaliPhy439Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_439::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy439Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_439::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy439Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_439 to value 0"]
impl crate::Resettable for DdrDenaliPhy439Spec {
    const RESET_VALUE: u32 = 0;
}
