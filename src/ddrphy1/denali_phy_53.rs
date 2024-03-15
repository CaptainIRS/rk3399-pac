#[doc = "Register `DENALI_PHY_53` reader"]
pub type R = crate::R<DenaliPhy53Spec>;
#[doc = "Register `DENALI_PHY_53` writer"]
pub type W = crate::W<DenaliPhy53Spec>;
#[doc = "Field `PHY_RX_CAL_DQ0_0` reader - RX Calibration codes for DQ0 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq0_0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ0_0` writer - RX Calibration codes for DQ0 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq0_0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PHY_RX_CAL_DQ1_0` reader - RX Calibration codes for DQ1 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq1_0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ1_0` writer - RX Calibration codes for DQ1 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq1_0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - RX Calibration codes for DQ0 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq0_0(&self) -> PhyRxCalDq0_0R {
        PhyRxCalDq0_0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQ1 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq1_0(&self) -> PhyRxCalDq1_0R {
        PhyRxCalDq1_0R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - RX Calibration codes for DQ0 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq0_0(&mut self) -> PhyRxCalDq0_0W<DenaliPhy53Spec> {
        PhyRxCalDq0_0W::new(self, 0)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQ1 for slice 0. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq1_0(&mut self) -> PhyRxCalDq1_0W<DenaliPhy53Spec> {
        PhyRxCalDq1_0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_53::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_53::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy53Spec;
impl crate::RegisterSpec for DenaliPhy53Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_53::R`](R) reader structure"]
impl crate::Readable for DenaliPhy53Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_53::W`](W) writer structure"]
impl crate::Writable for DenaliPhy53Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_53 to value 0"]
impl crate::Resettable for DenaliPhy53Spec {
    const RESET_VALUE: u32 = 0;
}
