#[doc = "Register `DENALI_PHY_438` reader"]
pub type R = crate::R<DenaliPhy438Spec>;
#[doc = "Register `DENALI_PHY_438` writer"]
pub type W = crate::W<DenaliPhy438Spec>;
#[doc = "Field `PHY_RX_CAL_DQ2_3` reader - RX Calibration codes for DQ2 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq2_3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ2_3` writer - RX Calibration codes for DQ2 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq2_3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PHY_RX_CAL_DQ3_3` reader - RX Calibration codes for DQ3 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq3_3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ3_3` writer - RX Calibration codes for DQ3 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq3_3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - RX Calibration codes for DQ2 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq2_3(&self) -> PhyRxCalDq2_3R {
        PhyRxCalDq2_3R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQ3 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq3_3(&self) -> PhyRxCalDq3_3R {
        PhyRxCalDq3_3R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - RX Calibration codes for DQ2 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq2_3(&mut self) -> PhyRxCalDq2_3W<DenaliPhy438Spec> {
        PhyRxCalDq2_3W::new(self, 0)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQ3 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq3_3(&mut self) -> PhyRxCalDq3_3W<DenaliPhy438Spec> {
        PhyRxCalDq3_3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_438::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_438::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy438Spec;
impl crate::RegisterSpec for DenaliPhy438Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_438::R`](R) reader structure"]
impl crate::Readable for DenaliPhy438Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_438::W`](W) writer structure"]
impl crate::Writable for DenaliPhy438Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_438 to value 0"]
impl crate::Resettable for DenaliPhy438Spec {
    const RESET_VALUE: u32 = 0;
}
