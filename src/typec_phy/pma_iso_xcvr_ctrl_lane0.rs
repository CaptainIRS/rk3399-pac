#[doc = "Register `PMA_ISO_XCVR_CTRL_LANE0` reader"]
pub type R = crate::R<PmaIsoXcvrCtrlLane0Spec>;
#[doc = "Register `PMA_ISO_XCVR_CTRL_LANE0` writer"]
pub type W = crate::W<PmaIsoXcvrCtrlLane0Spec>;
#[doc = "Field `FIELD15` reader - Drives xcvr_lane_en PMA input for the associated lane when in PMA \n\nisolation mode."]
pub type Field15R = crate::BitReader;
#[doc = "Field `FIELD15` writer - Drives xcvr_lane_en PMA input for the associated lane when in PMA \n\nisolation mode."]
pub type Field15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD14` reader - Drives rx_termination PMA input for the associated lane when in PMA \n\nisolation mode. Valid for PMA lanes 2 and 3 only. For PMA lanes 0 and \n\n3, reserved."]
pub type Field14R = crate::BitReader;
#[doc = "Field `FIELD14` writer - Drives rx_termination PMA input for the associated lane when in PMA \n\nisolation mode. Valid for PMA lanes 2 and 3 only. For PMA lanes 0 and \n\n3, reserved."]
pub type Field14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD13` reader - Current value of rx_signal_detect PMA output for the associated lane. \n\nValid for PMA lanes 2 and 3 only. For PMA lanes 0 and 3, reserved."]
pub type Field13R = crate::BitReader;
#[doc = "Field `FIELD12` reader - Current value of rx_lfps_detect PMA output for the assocaited lane. Valid \n\nfor PMA lanes 2 and 3 only. For PMA lanes 0 and 3, reserved."]
pub type Field12R = crate::BitReader;
#[doc = "Field `FIELD11` reader - Drives xcvr_lane_suspend PMA input for the associated lane when in PMA \n\nisolation mode."]
pub type Field11R = crate::BitReader;
#[doc = "Field `FIELD11` writer - Drives xcvr_lane_suspend PMA input for the associated lane when in PMA \n\nisolation mode."]
pub type Field11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD10` reader - Drives xcvr_link_reset_n PMA input for the associated lane when in PMA \n\nisolation mode."]
pub type Field10R = crate::BitReader;
#[doc = "Field `FIELD10` writer - Drives xcvr_link_reset_n PMA input for the associated lane when in PMA \n\nisolation mode."]
pub type Field10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD9` reader - Drives rx_eq_training PMA input for the associated lane when in PMA \n\nisolation mode. Valid for PMA lanes 2 and 3 only. For PMA lanes 0 and \n\n3, reserved."]
pub type Field9R = crate::BitReader;
#[doc = "Field `FIELD9` writer - Drives rx_eq_training PMA input for the associated lane when in PMA \n\nisolation mode. Valid for PMA lanes 2 and 3 only. For PMA lanes 0 and \n\n3, reserved."]
pub type Field9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD8` reader - Drives \n\nrx_eq_training_data_valid_ln_{nnnn} PMA \n\ninput \n\nfor \n\nthe \n\nassociated lane when in PMA isolation mode. Valid for PMA lanes 2 \n\nand 3 only. For PMA lanes 0 and 3, reserved."]
pub type Field8R = crate::BitReader;
#[doc = "Field `FIELD8` writer - Drives \n\nrx_eq_training_data_valid_ln_{nnnn} PMA \n\ninput \n\nfor \n\nthe \n\nassociated lane when in PMA isolation mode. Valid for PMA lanes 2 \n\nand 3 only. For PMA lanes 0 and 3, reserved."]
pub type Field8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD7` reader - Drives tx_rcv_detect_en PMA input for the associated lane when in PMA \n\nisolation mode."]
pub type Field7R = crate::BitReader;
#[doc = "Field `FIELD7` writer - Drives tx_rcv_detect_en PMA input for the associated lane when in PMA \n\nisolation mode."]
pub type Field7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD6` reader - Current value of tx_rcv_detect_done PMA ouptut for the associated lane."]
pub type Field6R = crate::BitReader;
#[doc = "Field `FIELD5` reader - Current value of tx_rcv_detected PMA output for the associated lane."]
pub type Field5R = crate::BitReader;
#[doc = "Field `FIELD4` reader - Current value of xcvr_psm_ready PMA output for the associated lane."]
pub type Field4R = crate::BitReader;
#[doc = "Field `FIELD3` reader - Drives tx_elec_idle PMA input for the associated lane when in PMA \n\nisolation mode."]
pub type Field3R = crate::BitReader;
#[doc = "Field `FIELD3` writer - Drives tx_elec_idle PMA input for the associated lane when in PMA \n\nisolation mode."]
pub type Field3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD2` reader - Drives tx_lfps_en PMA input for the associated lane when in PMA isolation \n\nmode."]
pub type Field2R = crate::BitReader;
#[doc = "Field `FIELD2` writer - Drives tx_lfps_en PMA input for the associated lane when in PMA isolation \n\nmode."]
pub type Field2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD1` reader - Drives xcvr_pll_clk_en PMA input for the associated lane when in PMA \n\nisolation mode."]
pub type Field1R = crate::BitReader;
#[doc = "Field `FIELD1` writer - Drives xcvr_pll_clk_en PMA input for the associated lane when in PMA \n\nisolation mode."]
pub type Field1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD0` reader - Current value of xcvr_pll_clk_en_ack PMA output for the associated lane."]
pub type Field0R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Drives xcvr_lane_en PMA input for the associated lane when in PMA \n\nisolation mode."]
    #[inline(always)]
    pub fn field15(&self) -> Field15R {
        Field15R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Drives rx_termination PMA input for the associated lane when in PMA \n\nisolation mode. Valid for PMA lanes 2 and 3 only. For PMA lanes 0 and \n\n3, reserved."]
    #[inline(always)]
    pub fn field14(&self) -> Field14R {
        Field14R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Current value of rx_signal_detect PMA output for the associated lane. \n\nValid for PMA lanes 2 and 3 only. For PMA lanes 0 and 3, reserved."]
    #[inline(always)]
    pub fn field13(&self) -> Field13R {
        Field13R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Current value of rx_lfps_detect PMA output for the assocaited lane. Valid \n\nfor PMA lanes 2 and 3 only. For PMA lanes 0 and 3, reserved."]
    #[inline(always)]
    pub fn field12(&self) -> Field12R {
        Field12R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Drives xcvr_lane_suspend PMA input for the associated lane when in PMA \n\nisolation mode."]
    #[inline(always)]
    pub fn field11(&self) -> Field11R {
        Field11R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Drives xcvr_link_reset_n PMA input for the associated lane when in PMA \n\nisolation mode."]
    #[inline(always)]
    pub fn field10(&self) -> Field10R {
        Field10R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Drives rx_eq_training PMA input for the associated lane when in PMA \n\nisolation mode. Valid for PMA lanes 2 and 3 only. For PMA lanes 0 and \n\n3, reserved."]
    #[inline(always)]
    pub fn field9(&self) -> Field9R {
        Field9R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Drives \n\nrx_eq_training_data_valid_ln_{nnnn} PMA \n\ninput \n\nfor \n\nthe \n\nassociated lane when in PMA isolation mode. Valid for PMA lanes 2 \n\nand 3 only. For PMA lanes 0 and 3, reserved."]
    #[inline(always)]
    pub fn field8(&self) -> Field8R {
        Field8R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Drives tx_rcv_detect_en PMA input for the associated lane when in PMA \n\nisolation mode."]
    #[inline(always)]
    pub fn field7(&self) -> Field7R {
        Field7R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Current value of tx_rcv_detect_done PMA ouptut for the associated lane."]
    #[inline(always)]
    pub fn field6(&self) -> Field6R {
        Field6R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Current value of tx_rcv_detected PMA output for the associated lane."]
    #[inline(always)]
    pub fn field5(&self) -> Field5R {
        Field5R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Current value of xcvr_psm_ready PMA output for the associated lane."]
    #[inline(always)]
    pub fn field4(&self) -> Field4R {
        Field4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Drives tx_elec_idle PMA input for the associated lane when in PMA \n\nisolation mode."]
    #[inline(always)]
    pub fn field3(&self) -> Field3R {
        Field3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Drives tx_lfps_en PMA input for the associated lane when in PMA isolation \n\nmode."]
    #[inline(always)]
    pub fn field2(&self) -> Field2R {
        Field2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Drives xcvr_pll_clk_en PMA input for the associated lane when in PMA \n\nisolation mode."]
    #[inline(always)]
    pub fn field1(&self) -> Field1R {
        Field1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Current value of xcvr_pll_clk_en_ack PMA output for the associated lane."]
    #[inline(always)]
    pub fn field0(&self) -> Field0R {
        Field0R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Drives xcvr_lane_en PMA input for the associated lane when in PMA \n\nisolation mode."]
    #[inline(always)]
    #[must_use]
    pub fn field15(&mut self) -> Field15W<PmaIsoXcvrCtrlLane0Spec> {
        Field15W::new(self, 0)
    }
    #[doc = "Bit 1 - Drives rx_termination PMA input for the associated lane when in PMA \n\nisolation mode. Valid for PMA lanes 2 and 3 only. For PMA lanes 0 and \n\n3, reserved."]
    #[inline(always)]
    #[must_use]
    pub fn field14(&mut self) -> Field14W<PmaIsoXcvrCtrlLane0Spec> {
        Field14W::new(self, 1)
    }
    #[doc = "Bit 4 - Drives xcvr_lane_suspend PMA input for the associated lane when in PMA \n\nisolation mode."]
    #[inline(always)]
    #[must_use]
    pub fn field11(&mut self) -> Field11W<PmaIsoXcvrCtrlLane0Spec> {
        Field11W::new(self, 4)
    }
    #[doc = "Bit 5 - Drives xcvr_link_reset_n PMA input for the associated lane when in PMA \n\nisolation mode."]
    #[inline(always)]
    #[must_use]
    pub fn field10(&mut self) -> Field10W<PmaIsoXcvrCtrlLane0Spec> {
        Field10W::new(self, 5)
    }
    #[doc = "Bit 6 - Drives rx_eq_training PMA input for the associated lane when in PMA \n\nisolation mode. Valid for PMA lanes 2 and 3 only. For PMA lanes 0 and \n\n3, reserved."]
    #[inline(always)]
    #[must_use]
    pub fn field9(&mut self) -> Field9W<PmaIsoXcvrCtrlLane0Spec> {
        Field9W::new(self, 6)
    }
    #[doc = "Bit 7 - Drives \n\nrx_eq_training_data_valid_ln_{nnnn} PMA \n\ninput \n\nfor \n\nthe \n\nassociated lane when in PMA isolation mode. Valid for PMA lanes 2 \n\nand 3 only. For PMA lanes 0 and 3, reserved."]
    #[inline(always)]
    #[must_use]
    pub fn field8(&mut self) -> Field8W<PmaIsoXcvrCtrlLane0Spec> {
        Field8W::new(self, 7)
    }
    #[doc = "Bit 8 - Drives tx_rcv_detect_en PMA input for the associated lane when in PMA \n\nisolation mode."]
    #[inline(always)]
    #[must_use]
    pub fn field7(&mut self) -> Field7W<PmaIsoXcvrCtrlLane0Spec> {
        Field7W::new(self, 8)
    }
    #[doc = "Bit 12 - Drives tx_elec_idle PMA input for the associated lane when in PMA \n\nisolation mode."]
    #[inline(always)]
    #[must_use]
    pub fn field3(&mut self) -> Field3W<PmaIsoXcvrCtrlLane0Spec> {
        Field3W::new(self, 12)
    }
    #[doc = "Bit 13 - Drives tx_lfps_en PMA input for the associated lane when in PMA isolation \n\nmode."]
    #[inline(always)]
    #[must_use]
    pub fn field2(&mut self) -> Field2W<PmaIsoXcvrCtrlLane0Spec> {
        Field2W::new(self, 13)
    }
    #[doc = "Bit 14 - Drives xcvr_pll_clk_en PMA input for the associated lane when in PMA \n\nisolation mode."]
    #[inline(always)]
    #[must_use]
    pub fn field1(&mut self) -> Field1W<PmaIsoXcvrCtrlLane0Spec> {
        Field1W::new(self, 14)
    }
}
#[doc = "PMA Isolation Tansceiver control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_xcvr_ctrl_lane0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_xcvr_ctrl_lane0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmaIsoXcvrCtrlLane0Spec;
impl crate::RegisterSpec for PmaIsoXcvrCtrlLane0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pma_iso_xcvr_ctrl_lane0::R`](R) reader structure"]
impl crate::Readable for PmaIsoXcvrCtrlLane0Spec {}
#[doc = "`write(|w| ..)` method takes [`pma_iso_xcvr_ctrl_lane0::W`](W) writer structure"]
impl crate::Writable for PmaIsoXcvrCtrlLane0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PMA_ISO_XCVR_CTRL_LANE0 to value 0"]
impl crate::Resettable for PmaIsoXcvrCtrlLane0Spec {
    const RESET_VALUE: u16 = 0;
}
