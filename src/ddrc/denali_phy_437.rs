#[doc = "Register `DENALI_PHY_437` reader"]
pub type R = crate::R<DenaliPhy437Spec>;
#[doc = "Register `DENALI_PHY_437` writer"]
pub type W = crate::W<DenaliPhy437Spec>;
#[doc = "Field `PHY_RX_CAL_DQ0_3` reader - RX Calibration codes for DQ0 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq0_3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ0_3` writer - RX Calibration codes for DQ0 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq0_3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PHY_RX_CAL_DQ1_3` reader - RX Calibration codes for DQ1 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq1_3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RX_CAL_DQ1_3` writer - RX Calibration codes for DQ1 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
pub type PhyRxCalDq1_3W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - RX Calibration codes for DQ0 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq0_3(&self) -> PhyRxCalDq0_3R {
        PhyRxCalDq0_3R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQ1 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    pub fn phy_rx_cal_dq1_3(&self) -> PhyRxCalDq1_3R {
        PhyRxCalDq1_3R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - RX Calibration codes for DQ0 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq0_3(&mut self) -> PhyRxCalDq0_3W<DenaliPhy437Spec> {
        PhyRxCalDq0_3W::new(self, 0)
    }
    #[doc = "Bits 16:27 - RX Calibration codes for DQ1 for slice 3. Bits (5:0) contain rx_cal_code_down. Bits (11:6) contain rx_cal_code_up. Bits (17:12) contain rx_cal_code2_down. Bits (23:18) contain rx_cal_code2_up."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rx_cal_dq1_3(&mut self) -> PhyRxCalDq1_3W<DenaliPhy437Spec> {
        PhyRxCalDq1_3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_437::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_437::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy437Spec;
impl crate::RegisterSpec for DenaliPhy437Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_437::R`](R) reader structure"]
impl crate::Readable for DenaliPhy437Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_437::W`](W) writer structure"]
impl crate::Writable for DenaliPhy437Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_437 to value 0"]
impl crate::Resettable for DenaliPhy437Spec {
    const RESET_VALUE: u32 = 0;
}
