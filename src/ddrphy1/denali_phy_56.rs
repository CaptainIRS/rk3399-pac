#[doc = "Register `DENALI_PHY_56` reader"]
pub type R = crate::R<DenaliPhy56Spec>;
#[doc = "Register `DENALI_PHY_56` writer"]
pub type W = crate::W<DenaliPhy56Spec>;
#[doc = "Field `PHY_RX_CAL_DQ6_0` reader - RX Calibration codes for DQ6 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq6_0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ6_0` writer - RX Calibration codes for DQ6 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq6_0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PHY_RX_CAL_DQ7_0` reader - RX Calibration codes for DQ7 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq7_0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ7_0` writer - RX Calibration codes for DQ7 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq7_0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - RX Calibration codes for DQ6 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq6_0(&self) -> PhyRxCalDq6_0R {
        PhyRxCalDq6_0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQ7 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq7_0(&self) -> PhyRxCalDq7_0R {
        PhyRxCalDq7_0R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - RX Calibration codes for DQ6 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq6_0(&mut self) -> PhyRxCalDq6_0W<DenaliPhy56Spec> {
        PhyRxCalDq6_0W::new(self, 0)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQ7 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq7_0(&mut self) -> PhyRxCalDq7_0W<DenaliPhy56Spec> {
        PhyRxCalDq7_0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_56::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_56::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy56Spec;
impl crate::RegisterSpec for DenaliPhy56Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_56::R`](R) reader structure"]
impl crate::Readable for DenaliPhy56Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_56::W`](W) writer structure"]
impl crate::Writable for DenaliPhy56Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_56 to value 0"]
impl crate::Resettable for DenaliPhy56Spec {
    const RESET_VALUE: u32 = 0;
}
