#[doc = "Register `MC_OPCTRL` reader"]
pub type R = crate::R<McOpctrlSpec>;
#[doc = "Register `MC_OPCTRL` writer"]
pub type W = crate::W<McOpctrlSpec>;
#[doc = "Block HDCP bypass mechanism\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HdcpBlockByp {
    #[doc = "0: This is the default value. You can write to the hdcp_clkdisable bit of the register mc_clkdis and bypass HDCP by acting on the register mc_clkdis bit 6 (hdcp_clkdisable)"]
    B0 = 0,
    #[doc = "1: You can still write to the hdcp_clkdisable bit of the register mc_clkdis but this action disables the HDCP module and blocks the bypass mechanism. The output data is frozen and the HDMI Tx and RX fail authentication. Once you set the value to 1'b1, you can change the value back to 1'b0 only by issuing a master reset to the Hdmi_tx. Otherwise, this field is a \"spare\" bit with no associated functionality."]
    B1 = 1,
}
impl From<HdcpBlockByp> for bool {
    #[inline(always)]
    fn from(variant: HdcpBlockByp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDCP_BLOCK_BYP` reader - Block HDCP bypass mechanism"]
pub type HdcpBlockBypR = crate::BitReader<HdcpBlockByp>;
impl HdcpBlockBypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HdcpBlockByp {
        match self.bits {
            false => HdcpBlockByp::B0,
            true => HdcpBlockByp::B1,
        }
    }
    #[doc = "This is the default value. You can write to the hdcp_clkdisable bit of the register mc_clkdis and bypass HDCP by acting on the register mc_clkdis bit 6 (hdcp_clkdisable)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HdcpBlockByp::B0
    }
    #[doc = "You can still write to the hdcp_clkdisable bit of the register mc_clkdis but this action disables the HDCP module and blocks the bypass mechanism. The output data is frozen and the HDMI Tx and RX fail authentication. Once you set the value to 1'b1, you can change the value back to 1'b0 only by issuing a master reset to the Hdmi_tx. Otherwise, this field is a \"spare\" bit with no associated functionality."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HdcpBlockByp::B1
    }
}
#[doc = "Field `HDCP_BLOCK_BYP` writer - Block HDCP bypass mechanism"]
pub type HdcpBlockBypW<'a, REG> = crate::BitWriter<'a, REG, HdcpBlockByp>;
impl<'a, REG> HdcpBlockBypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This is the default value. You can write to the hdcp_clkdisable bit of the register mc_clkdis and bypass HDCP by acting on the register mc_clkdis bit 6 (hdcp_clkdisable)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HdcpBlockByp::B0)
    }
    #[doc = "You can still write to the hdcp_clkdisable bit of the register mc_clkdis but this action disables the HDCP module and blocks the bypass mechanism. The output data is frozen and the HDMI Tx and RX fail authentication. Once you set the value to 1'b1, you can change the value back to 1'b0 only by issuing a master reset to the Hdmi_tx. Otherwise, this field is a \"spare\" bit with no associated functionality."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HdcpBlockByp::B1)
    }
}
#[doc = "HDCP 2.2 SNPS switch lock\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H22sSwitchLck {
    #[doc = "0: Enables you to change the direction of the HDCP 2.2 SNPS versus 1.4 switch by using the hdcp22snps_ovr_val."]
    B0 = 0,
    #[doc = "1: You can still write to hdcp22snps_ovr_val but has no effect over the HDCP 2.2 SNPS versus 1.4 switch, that keeps as it was configured by hdcp22snps_ovr_val at the time the 1'b1 was written to this bit field. Once you set the value to 1'b1, you can change the value back to 1'b0 only by issuing a master reset to the Hdmi_tx."]
    B1 = 1,
}
impl From<H22sSwitchLck> for bool {
    #[inline(always)]
    fn from(variant: H22sSwitchLck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H22S_SWITCH_LCK` reader - HDCP 2.2 SNPS switch lock"]
pub type H22sSwitchLckR = crate::BitReader<H22sSwitchLck>;
impl H22sSwitchLckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H22sSwitchLck {
        match self.bits {
            false => H22sSwitchLck::B0,
            true => H22sSwitchLck::B1,
        }
    }
    #[doc = "Enables you to change the direction of the HDCP 2.2 SNPS versus 1.4 switch by using the hdcp22snps_ovr_val."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == H22sSwitchLck::B0
    }
    #[doc = "You can still write to hdcp22snps_ovr_val but has no effect over the HDCP 2.2 SNPS versus 1.4 switch, that keeps as it was configured by hdcp22snps_ovr_val at the time the 1'b1 was written to this bit field. Once you set the value to 1'b1, you can change the value back to 1'b0 only by issuing a master reset to the Hdmi_tx."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == H22sSwitchLck::B1
    }
}
#[doc = "Field `H22S_SWITCH_LCK` writer - HDCP 2.2 SNPS switch lock"]
pub type H22sSwitchLckW<'a, REG> = crate::BitWriter<'a, REG, H22sSwitchLck>;
impl<'a, REG> H22sSwitchLckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enables you to change the direction of the HDCP 2.2 SNPS versus 1.4 switch by using the hdcp22snps_ovr_val."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(H22sSwitchLck::B0)
    }
    #[doc = "You can still write to hdcp22snps_ovr_val but has no effect over the HDCP 2.2 SNPS versus 1.4 switch, that keeps as it was configured by hdcp22snps_ovr_val at the time the 1'b1 was written to this bit field. Once you set the value to 1'b1, you can change the value back to 1'b0 only by issuing a master reset to the Hdmi_tx."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(H22sSwitchLck::B1)
    }
}
#[doc = "HDCP SNPS 2.2 versus 1.4 switch override value\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H22sOvrVal {
    #[doc = "0: The switch is routed to HDCP 1.4 signals when hdcp22snps_switch_lock is not set to 1'b1."]
    B0 = 0,
    #[doc = "1: The switch is routed to HDCP 2.2 SNPS signals when hdcp22snps_switch_lock is not set to 1'b1."]
    B1 = 1,
}
impl From<H22sOvrVal> for bool {
    #[inline(always)]
    fn from(variant: H22sOvrVal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H22S_OVR_VAL` reader - HDCP SNPS 2.2 versus 1.4 switch override value"]
pub type H22sOvrValR = crate::BitReader<H22sOvrVal>;
impl H22sOvrValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H22sOvrVal {
        match self.bits {
            false => H22sOvrVal::B0,
            true => H22sOvrVal::B1,
        }
    }
    #[doc = "The switch is routed to HDCP 1.4 signals when hdcp22snps_switch_lock is not set to 1'b1."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == H22sOvrVal::B0
    }
    #[doc = "The switch is routed to HDCP 2.2 SNPS signals when hdcp22snps_switch_lock is not set to 1'b1."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == H22sOvrVal::B1
    }
}
#[doc = "Field `H22S_OVR_VAL` writer - HDCP SNPS 2.2 versus 1.4 switch override value"]
pub type H22sOvrValW<'a, REG> = crate::BitWriter<'a, REG, H22sOvrVal>;
impl<'a, REG> H22sOvrValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The switch is routed to HDCP 1.4 signals when hdcp22snps_switch_lock is not set to 1'b1."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(H22sOvrVal::B0)
    }
    #[doc = "The switch is routed to HDCP 2.2 SNPS signals when hdcp22snps_switch_lock is not set to 1'b1."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(H22sOvrVal::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Block HDCP bypass mechanism"]
    #[inline(always)]
    pub fn hdcp_block_byp(&self) -> HdcpBlockBypR {
        HdcpBlockBypR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - HDCP 2.2 SNPS switch lock"]
    #[inline(always)]
    pub fn h22s_switch_lck(&self) -> H22sSwitchLckR {
        H22sSwitchLckR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HDCP SNPS 2.2 versus 1.4 switch override value"]
    #[inline(always)]
    pub fn h22s_ovr_val(&self) -> H22sOvrValR {
        H22sOvrValR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Block HDCP bypass mechanism"]
    #[inline(always)]
    #[must_use]
    pub fn hdcp_block_byp(&mut self) -> HdcpBlockBypW<McOpctrlSpec> {
        HdcpBlockBypW::new(self, 0)
    }
    #[doc = "Bit 4 - HDCP 2.2 SNPS switch lock"]
    #[inline(always)]
    #[must_use]
    pub fn h22s_switch_lck(&mut self) -> H22sSwitchLckW<McOpctrlSpec> {
        H22sSwitchLckW::new(self, 4)
    }
    #[doc = "Bit 5 - HDCP SNPS 2.2 versus 1.4 switch override value"]
    #[inline(always)]
    #[must_use]
    pub fn h22s_ovr_val(&mut self) -> H22sOvrValW<McOpctrlSpec> {
        H22sOvrValW::new(self, 5)
    }
}
#[doc = "Main Controller HDCP Bypass Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mc_opctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mc_opctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McOpctrlSpec;
impl crate::RegisterSpec for McOpctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mc_opctrl::R`](R) reader structure"]
impl crate::Readable for McOpctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mc_opctrl::W`](W) writer structure"]
impl crate::Writable for McOpctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MC_OPCTRL to value 0x20"]
impl crate::Resettable for McOpctrlSpec {
    const RESET_VALUE: u8 = 0x20;
}
