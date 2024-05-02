#[doc = "Register `PMA_ISO_PWRST_CTRL_LANE3` reader"]
pub type R = crate::R<PmaIsoPwrstCtrlLane3Spec>;
#[doc = "Register `PMA_ISO_PWRST_CTRL_LANE3` writer"]
pub type W = crate::W<PmaIsoPwrstCtrlLane3Spec>;
#[doc = "Field `FIELD4` reader - Drives xcvr_power_state_req PMA input for the associated lane \n\nwhen in PMA isolation mode."]
pub type Field4R = crate::FieldReader;
#[doc = "Field `FIELD4` writer - Drives xcvr_power_state_req PMA input for the associated lane \n\nwhen in PMA isolation mode."]
pub type Field4W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FIELD3` reader - Reserved"]
pub type Field3R = crate::FieldReader;
#[doc = "Field `FIELD2` reader - Current value of xcvr_power_state_ack PMA output for the associated \n\nlane."]
pub type Field2R = crate::FieldReader;
#[doc = "Field `FIELD1` reader - Drives \n\ntx_cmn_mode_en_ext \n\nPMA \n\ninput \n\nfor \n\nthe \n\nassociated \n\nlanewhen in PMA isolation mode. (Used for PCIe)"]
pub type Field1R = crate::BitReader;
#[doc = "Field `FIELD1` writer - Drives \n\ntx_cmn_mode_en_ext \n\nPMA \n\ninput \n\nfor \n\nthe \n\nassociated \n\nlanewhen in PMA isolation mode. (Used for PCIe)"]
pub type Field1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD0` reader - Drives rx_sig_det_en_ext_} PMA input for the associated lane when \n\nin PMA isolation mode. Valid for PMA lanes 2 and 3 only. For PMA \n\nlanes 0 and 3, reserved. (Used for PCIe)"]
pub type Field0R = crate::BitReader;
#[doc = "Field `FIELD0` writer - Drives rx_sig_det_en_ext_} PMA input for the associated lane when \n\nin PMA isolation mode. Valid for PMA lanes 2 and 3 only. For PMA \n\nlanes 0 and 3, reserved. (Used for PCIe)"]
pub type Field0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Drives xcvr_power_state_req PMA input for the associated lane \n\nwhen in PMA isolation mode."]
    #[inline(always)]
    pub fn field4(&self) -> Field4R {
        Field4R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Reserved"]
    #[inline(always)]
    pub fn field3(&self) -> Field3R {
        Field3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13 - Current value of xcvr_power_state_ack PMA output for the associated \n\nlane."]
    #[inline(always)]
    pub fn field2(&self) -> Field2R {
        Field2R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Drives \n\ntx_cmn_mode_en_ext \n\nPMA \n\ninput \n\nfor \n\nthe \n\nassociated \n\nlanewhen in PMA isolation mode. (Used for PCIe)"]
    #[inline(always)]
    pub fn field1(&self) -> Field1R {
        Field1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Drives rx_sig_det_en_ext_} PMA input for the associated lane when \n\nin PMA isolation mode. Valid for PMA lanes 2 and 3 only. For PMA \n\nlanes 0 and 3, reserved. (Used for PCIe)"]
    #[inline(always)]
    pub fn field0(&self) -> Field0R {
        Field0R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Drives xcvr_power_state_req PMA input for the associated lane \n\nwhen in PMA isolation mode."]
    #[inline(always)]
    #[must_use]
    pub fn field4(&mut self) -> Field4W<PmaIsoPwrstCtrlLane3Spec> {
        Field4W::new(self, 0)
    }
    #[doc = "Bit 14 - Drives \n\ntx_cmn_mode_en_ext \n\nPMA \n\ninput \n\nfor \n\nthe \n\nassociated \n\nlanewhen in PMA isolation mode. (Used for PCIe)"]
    #[inline(always)]
    #[must_use]
    pub fn field1(&mut self) -> Field1W<PmaIsoPwrstCtrlLane3Spec> {
        Field1W::new(self, 14)
    }
    #[doc = "Bit 15 - Drives rx_sig_det_en_ext_} PMA input for the associated lane when \n\nin PMA isolation mode. Valid for PMA lanes 2 and 3 only. For PMA \n\nlanes 0 and 3, reserved. (Used for PCIe)"]
    #[inline(always)]
    #[must_use]
    pub fn field0(&mut self) -> Field0W<PmaIsoPwrstCtrlLane3Spec> {
        Field0W::new(self, 15)
    }
}
#[doc = "PMA Isolation power state control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_iso_pwrst_ctrl_lane3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_iso_pwrst_ctrl_lane3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmaIsoPwrstCtrlLane3Spec;
impl crate::RegisterSpec for PmaIsoPwrstCtrlLane3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pma_iso_pwrst_ctrl_lane3::R`](R) reader structure"]
impl crate::Readable for PmaIsoPwrstCtrlLane3Spec {}
#[doc = "`write(|w| ..)` method takes [`pma_iso_pwrst_ctrl_lane3::W`](W) writer structure"]
impl crate::Writable for PmaIsoPwrstCtrlLane3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PMA_ISO_PWRST_CTRL_LANE3 to value 0"]
impl crate::Resettable for PmaIsoPwrstCtrlLane3Spec {
    const RESET_VALUE: u16 = 0;
}
