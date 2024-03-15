#[doc = "Register `DENALI_PHY_312` reader"]
pub type R = crate::R<DenaliPhy312Spec>;
#[doc = "Register `DENALI_PHY_312` writer"]
pub type W = crate::W<DenaliPhy312Spec>;
#[doc = "Field `PHY_RX_CAL_DQ6_2` reader - RX Calibration codes for DQ6 for slice 2. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq6_2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ6_2` writer - RX Calibration codes for DQ6 for slice 2. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq6_2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PHY_RX_CAL_DQ7_2` reader - RX Calibration codes for DQ7 for slice 2. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq7_2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ7_2` writer - RX Calibration codes for DQ7 for slice 2. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq7_2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - RX Calibration codes for DQ6 for slice 2. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq6_2(&self) -> PhyRxCalDq6_2R {
        PhyRxCalDq6_2R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQ7 for slice 2. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq7_2(&self) -> PhyRxCalDq7_2R {
        PhyRxCalDq7_2R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - RX Calibration codes for DQ6 for slice 2. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq6_2(&mut self) -> PhyRxCalDq6_2W<DenaliPhy312Spec> {
        PhyRxCalDq6_2W::new(self, 0)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQ7 for slice 2. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq7_2(&mut self) -> PhyRxCalDq7_2W<DenaliPhy312Spec> {
        PhyRxCalDq7_2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_312::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_312::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy312Spec;
impl crate::RegisterSpec for DenaliPhy312Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_312::R`](R) reader structure"]
impl crate::Readable for DenaliPhy312Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_312::W`](W) writer structure"]
impl crate::Writable for DenaliPhy312Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_312 to value 0"]
impl crate::Resettable for DenaliPhy312Spec {
    const RESET_VALUE: u32 = 0;
}
