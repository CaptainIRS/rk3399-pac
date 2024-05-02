#[doc = "Register `DP_TX_CTL_LANE0` reader"]
pub type R = crate::R<DpTxCtlLane0Spec>;
#[doc = "Register `DP_TX_CTL_LANE0` writer"]
pub type W = crate::W<DpTxCtlLane0Spec>;
#[doc = "Field `FIELD3` reader - TX Deemphasis setting - Drives tx_deemphasis PMA input for the \n\nmapped PMA lane . This field is used to set the DP Pre-emphasis Level \n\n(0b00 = Level 0, 0b01 = Level 1, 0b10 = Level 2 and 0b11 = Level \n\n3). TBD if Pre-emphasis Level 3 supported."]
pub type Field3R = crate::FieldReader;
#[doc = "Field `FIELD3` writer - TX Deemphasis setting - Drives tx_deemphasis PMA input for the \n\nmapped PMA lane . This field is used to set the DP Pre-emphasis Level \n\n(0b00 = Level 0, 0b01 = Level 1, 0b10 = Level 2 and 0b11 = Level \n\n3). TBD if Pre-emphasis Level 3 supported."]
pub type Field3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIELD2` reader - Reserved"]
pub type Field2R = crate::FieldReader;
#[doc = "Field `FIELD1` reader - TX Voltage Level - Dxrives tx_vmargin PMA input for the mapped \n\nPMA lane (for functional and isolation modes). This field is used to \n\nset the DP Voltage Swing Level (0b00 = Level 0, 0b01 = Level 1, \n\n0b10 = Level 2 and 0b11 = Level 3). TBD if Voltage Swing Level \n\n3 supported."]
pub type Field1R = crate::FieldReader;
#[doc = "Field `FIELD1` writer - TX Voltage Level - Dxrives tx_vmargin PMA input for the mapped \n\nPMA lane (for functional and isolation modes). This field is used to \n\nset the DP Voltage Swing Level (0b00 = Level 0, 0b01 = Level 1, \n\n0b10 = Level 2 and 0b11 = Level 3). TBD if Voltage Swing Level \n\n3 supported."]
pub type Field1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIELD0` reader - Reserved"]
pub type Field0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - TX Deemphasis setting - Drives tx_deemphasis PMA input for the \n\nmapped PMA lane . This field is used to set the DP Pre-emphasis Level \n\n(0b00 = Level 0, 0b01 = Level 1, 0b10 = Level 2 and 0b11 = Level \n\n3). TBD if Pre-emphasis Level 3 supported."]
    #[inline(always)]
    pub fn field3(&self) -> Field3R {
        Field3R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Reserved"]
    #[inline(always)]
    pub fn field2(&self) -> Field2R {
        Field2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - TX Voltage Level - Dxrives tx_vmargin PMA input for the mapped \n\nPMA lane (for functional and isolation modes). This field is used to \n\nset the DP Voltage Swing Level (0b00 = Level 0, 0b01 = Level 1, \n\n0b10 = Level 2 and 0b11 = Level 3). TBD if Voltage Swing Level \n\n3 supported."]
    #[inline(always)]
    pub fn field1(&self) -> Field1R {
        Field1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:15 - Reserved"]
    #[inline(always)]
    pub fn field0(&self) -> Field0R {
        Field0R::new((self.bits >> 6) & 0x03ff)
    }
}
impl W {
    #[doc = "Bits 0:1 - TX Deemphasis setting - Drives tx_deemphasis PMA input for the \n\nmapped PMA lane . This field is used to set the DP Pre-emphasis Level \n\n(0b00 = Level 0, 0b01 = Level 1, 0b10 = Level 2 and 0b11 = Level \n\n3). TBD if Pre-emphasis Level 3 supported."]
    #[inline(always)]
    #[must_use]
    pub fn field3(&mut self) -> Field3W<DpTxCtlLane0Spec> {
        Field3W::new(self, 0)
    }
    #[doc = "Bits 4:5 - TX Voltage Level - Dxrives tx_vmargin PMA input for the mapped \n\nPMA lane (for functional and isolation modes). This field is used to \n\nset the DP Voltage Swing Level (0b00 = Level 0, 0b01 = Level 1, \n\n0b10 = Level 2 and 0b11 = Level 3). TBD if Voltage Swing Level \n\n3 supported."]
    #[inline(always)]
    #[must_use]
    pub fn field1(&mut self) -> Field1W<DpTxCtlLane0Spec> {
        Field1W::new(self, 4)
    }
}
#[doc = "DP Lane Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_tx_ctl_lane0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_tx_ctl_lane0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpTxCtlLane0Spec;
impl crate::RegisterSpec for DpTxCtlLane0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dp_tx_ctl_lane0::R`](R) reader structure"]
impl crate::Readable for DpTxCtlLane0Spec {}
#[doc = "`write(|w| ..)` method takes [`dp_tx_ctl_lane0::W`](W) writer structure"]
impl crate::Writable for DpTxCtlLane0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DP_TX_CTL_LANE0 to value 0"]
impl crate::Resettable for DpTxCtlLane0Spec {
    const RESET_VALUE: u16 = 0;
}
