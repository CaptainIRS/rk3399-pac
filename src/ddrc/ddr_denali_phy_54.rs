#[doc = "Register `DDR_DENALI_PHY_54` reader"]
pub type R = crate::R<DdrDenaliPhy54Spec>;
#[doc = "Register `DDR_DENALI_PHY_54` writer"]
pub type W = crate::W<DdrDenaliPhy54Spec>;
#[doc = "Field `PHY_RX_CAL_DQ2_0` reader - RX Calibration codes for DQ2 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq2_0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ2_0` writer - RX Calibration codes for DQ2 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq2_0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PHY_RX_CAL_DQ3_0` reader - RX Calibration codes for DQ3 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq3_0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ3_0` writer - RX Calibration codes for DQ3 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq3_0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - RX Calibration codes for DQ2 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq2_0(&self) -> PhyRxCalDq2_0R {
        PhyRxCalDq2_0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQ3 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq3_0(&self) -> PhyRxCalDq3_0R {
        PhyRxCalDq3_0R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - RX Calibration codes for DQ2 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq2_0(&mut self) -> PhyRxCalDq2_0W<DdrDenaliPhy54Spec> {
        PhyRxCalDq2_0W::new(self, 0)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQ3 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq3_0(&mut self) -> PhyRxCalDq3_0W<DdrDenaliPhy54Spec> {
        PhyRxCalDq3_0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_54::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_54::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy54Spec;
impl crate::RegisterSpec for DdrDenaliPhy54Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_54::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy54Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_54::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy54Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_54 to value 0"]
impl crate::Resettable for DdrDenaliPhy54Spec {
    const RESET_VALUE: u32 = 0;
}
