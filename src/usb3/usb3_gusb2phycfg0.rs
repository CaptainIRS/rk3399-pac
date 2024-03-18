#[doc = "Register `USB3_GUSB2PHYCFG0` reader"]
pub type R = crate::R<Usb3Gusb2phycfg0Spec>;
#[doc = "Register `USB3_GUSB2PHYCFG0` writer"]
pub type W = crate::W<Usb3Gusb2phycfg0Spec>;
#[doc = "Field `TOUTCAL` reader - HS/FS Timeout Calibration The number of PHY clocks, as indicated by the application in this field, is multiplied by a bit-time factor; this factor is added to the high-speed/full-speed interpacket timeout duration in the core to account for additional delays introduced by the PHY. This may be required, since the delay introduced by the PHY in generating the linestate condition may vary among PHYs. The USB standard timeout value for high-speed operation is 736 to 816 (inclusive) bit times. The USB standard timeout value for full-speed operation is 16 to 18 (inclusive) bit times. The application must program this field based on the speed of connection. The number of bit times added per PHY clock are: High-speed operation: One 30-MHz PHY clock = 16 bit times One 60-MHz PHY clock = 8 bit times Full-speed operation: One 30-MHz PHY clock = 0.4 bit times One 60-MHz PHY clock = 0.2 bit times"]
pub type ToutcalR = crate::FieldReader;
#[doc = "Field `TOUTCAL` writer - HS/FS Timeout Calibration The number of PHY clocks, as indicated by the application in this field, is multiplied by a bit-time factor; this factor is added to the high-speed/full-speed interpacket timeout duration in the core to account for additional delays introduced by the PHY. This may be required, since the delay introduced by the PHY in generating the linestate condition may vary among PHYs. The USB standard timeout value for high-speed operation is 736 to 816 (inclusive) bit times. The USB standard timeout value for full-speed operation is 16 to 18 (inclusive) bit times. The application must program this field based on the speed of connection. The number of bit times added per PHY clock are: High-speed operation: One 30-MHz PHY clock = 16 bit times One 60-MHz PHY clock = 8 bit times Full-speed operation: One 30-MHz PHY clock = 0.4 bit times One 60-MHz PHY clock = 0.2 bit times"]
pub type ToutcalW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "PHY Interface If UTMI+ is selected, the application uses this bit to configure the core to support a UTMI+ PHY with an 8- or 16-bit interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phyif {
    #[doc = "0: 16 bits"]
    B0 = 0,
    #[doc = "1: 16 bits"]
    B1 = 1,
}
impl From<Phyif> for bool {
    #[inline(always)]
    fn from(variant: Phyif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHYIF` reader - PHY Interface If UTMI+ is selected, the application uses this bit to configure the core to support a UTMI+ PHY with an 8- or 16-bit interface."]
pub type PhyifR = crate::BitReader<Phyif>;
impl PhyifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Phyif {
        match self.bits {
            false => Phyif::B0,
            true => Phyif::B1,
        }
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Phyif::B0
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Phyif::B1
    }
}
#[doc = "Field `PHYIF` writer - PHY Interface If UTMI+ is selected, the application uses this bit to configure the core to support a UTMI+ PHY with an 8- or 16-bit interface."]
pub type PhyifW<'a, REG> = crate::BitWriter<'a, REG, Phyif>;
impl<'a, REG> PhyifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Phyif::B0)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Phyif::B1)
    }
}
#[doc = "ULPI or UTMI+ Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UlpiUtmiSel {
    #[doc = "0: ULPI Interface"]
    B0 = 0,
    #[doc = "1: ULPI Interface"]
    B1 = 1,
}
impl From<UlpiUtmiSel> for bool {
    #[inline(always)]
    fn from(variant: UlpiUtmiSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPI_UTMI_SEL` reader - ULPI or UTMI+ Select"]
pub type UlpiUtmiSelR = crate::BitReader<UlpiUtmiSel>;
impl UlpiUtmiSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UlpiUtmiSel {
        match self.bits {
            false => UlpiUtmiSel::B0,
            true => UlpiUtmiSel::B1,
        }
    }
    #[doc = "ULPI Interface"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == UlpiUtmiSel::B0
    }
    #[doc = "ULPI Interface"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == UlpiUtmiSel::B1
    }
}
#[doc = "Field `SUSPENDUSB20` reader - Suspend USB2.0 HS/FS/LS PHY When set, USB2.0 PHY enters Suspend mode if Suspend conditions are valid. For DRD/OTG configurations, it is recommended that this bit is set to 0 during coreConsultant configuration. If it is set to 1, then the application must clear this bit after power-on reset. Application needs to set it to 1 after the core initialization completes. For all other configurations, this bit can be set to 1 during core configuration. Note: In host mode, on reset, this bit is set to 1. Software can override this bit after reset. In device mode, before issuing any device endpoint command when operating in 2.0 speeds, disable this bit and enable it after the command completes. If you issue a command without disabling this bit when the device is in L2 state and if mac2_clk (utmi_clk/ulpi_clk) is gated off, the command will not get completed."]
pub type Suspendusb20R = crate::BitReader;
#[doc = "Field `SUSPENDUSB20` writer - Suspend USB2.0 HS/FS/LS PHY When set, USB2.0 PHY enters Suspend mode if Suspend conditions are valid. For DRD/OTG configurations, it is recommended that this bit is set to 0 during coreConsultant configuration. If it is set to 1, then the application must clear this bit after power-on reset. Application needs to set it to 1 after the core initialization completes. For all other configurations, this bit can be set to 1 during core configuration. Note: In host mode, on reset, this bit is set to 1. Software can override this bit after reset. In device mode, before issuing any device endpoint command when operating in 2.0 speeds, disable this bit and enable it after the command completes. If you issue a command without disabling this bit when the device is in L2 state and if mac2_clk (utmi_clk/ulpi_clk) is gated off, the command will not get completed."]
pub type Suspendusb20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "USB 2.0 High-Speed PHY or USB 1.1 Full-Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Physel {
    #[doc = "0: USB 1.1 full-speed serial transceiver."]
    B0 = 0,
    #[doc = "1: USB 1.1 full-speed serial transceiver."]
    B1 = 1,
}
impl From<Physel> for bool {
    #[inline(always)]
    fn from(variant: Physel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHYSEL` reader - USB 2.0 High-Speed PHY or USB 1.1 Full-Speed"]
pub type PhyselR = crate::BitReader<Physel>;
impl PhyselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Physel {
        match self.bits {
            false => Physel::B0,
            true => Physel::B1,
        }
    }
    #[doc = "USB 1.1 full-speed serial transceiver."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Physel::B0
    }
    #[doc = "USB 1.1 full-speed serial transceiver."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Physel::B1
    }
}
#[doc = "Enable utmi_sleep_n and utmi_l1_suspend_n The application uses this bit to control utmi_sleep_n and utmi_l1_suspend_n assertion to the PHY in the L1 state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enblslpm {
    #[doc = "0: utmi_sleep_n and utmi_l1_suspend_n assertion from the core is transferred to the external PHY. Note: In Device mode - Before issuing any device endpoint command when operating in 2.0 speeds, disable this bit and enable it after the command completes. Without disabling this bit, if a command is issued when the device is in L1 state and if mac2_clk (utmi_clk/ulpi_clk) is gated off, the command will not get completed."]
    B0 = 0,
    #[doc = "1: utmi_sleep_n and utmi_l1_suspend_n assertion from the core is transferred to the external PHY. Note: In Device mode - Before issuing any device endpoint command when operating in 2.0 speeds, disable this bit and enable it after the command completes. Without disabling this bit, if a command is issued when the device is in L1 state and if mac2_clk (utmi_clk/ulpi_clk) is gated off, the command will not get completed."]
    B1 = 1,
}
impl From<Enblslpm> for bool {
    #[inline(always)]
    fn from(variant: Enblslpm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENBLSLPM` reader - Enable utmi_sleep_n and utmi_l1_suspend_n The application uses this bit to control utmi_sleep_n and utmi_l1_suspend_n assertion to the PHY in the L1 state."]
pub type EnblslpmR = crate::BitReader<Enblslpm>;
impl EnblslpmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enblslpm {
        match self.bits {
            false => Enblslpm::B0,
            true => Enblslpm::B1,
        }
    }
    #[doc = "utmi_sleep_n and utmi_l1_suspend_n assertion from the core is transferred to the external PHY. Note: In Device mode - Before issuing any device endpoint command when operating in 2.0 speeds, disable this bit and enable it after the command completes. Without disabling this bit, if a command is issued when the device is in L1 state and if mac2_clk (utmi_clk/ulpi_clk) is gated off, the command will not get completed."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Enblslpm::B0
    }
    #[doc = "utmi_sleep_n and utmi_l1_suspend_n assertion from the core is transferred to the external PHY. Note: In Device mode - Before issuing any device endpoint command when operating in 2.0 speeds, disable this bit and enable it after the command completes. Without disabling this bit, if a command is issued when the device is in L1 state and if mac2_clk (utmi_clk/ulpi_clk) is gated off, the command will not get completed."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Enblslpm::B1
    }
}
#[doc = "Field `ENBLSLPM` writer - Enable utmi_sleep_n and utmi_l1_suspend_n The application uses this bit to control utmi_sleep_n and utmi_l1_suspend_n assertion to the PHY in the L1 state."]
pub type EnblslpmW<'a, REG> = crate::BitWriter<'a, REG, Enblslpm>;
impl<'a, REG> EnblslpmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "utmi_sleep_n and utmi_l1_suspend_n assertion from the core is transferred to the external PHY. Note: In Device mode - Before issuing any device endpoint command when operating in 2.0 speeds, disable this bit and enable it after the command completes. Without disabling this bit, if a command is issued when the device is in L1 state and if mac2_clk (utmi_clk/ulpi_clk) is gated off, the command will not get completed."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Enblslpm::B0)
    }
    #[doc = "utmi_sleep_n and utmi_l1_suspend_n assertion from the core is transferred to the external PHY. Note: In Device mode - Before issuing any device endpoint command when operating in 2.0 speeds, disable this bit and enable it after the command completes. Without disabling this bit, if a command is issued when the device is in L1 state and if mac2_clk (utmi_clk/ulpi_clk) is gated off, the command will not get completed."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Enblslpm::B1)
    }
}
#[doc = "Field `XCVRDLY` reader - Transceiver Delay Enables a delay between the assertion of the UTMI/ULPI Transceiver Select signal (for HS) and the assertion of the TxValid signal during a HS Chirp. When this bit is set to 1, a delay (of approximately 2.5 us) is introduced from the time when the Transceiver Select is set to 2'b00 (HS) to the time the TxValid is driven to 0 for sending the chirp-K. This delay is required for some UTMI/ULPI PHYs. Note: If you enable the hibernation feature when the device core comes out of power-off, you must re-initialize this bit with the appropriate value because the core does not save and restore this bit value during hibernation. This bit is valid only in device mode."]
pub type XcvrdlyR = crate::BitReader;
#[doc = "Field `XCVRDLY` writer - Transceiver Delay Enables a delay between the assertion of the UTMI/ULPI Transceiver Select signal (for HS) and the assertion of the TxValid signal during a HS Chirp. When this bit is set to 1, a delay (of approximately 2.5 us) is introduced from the time when the Transceiver Select is set to 2'b00 (HS) to the time the TxValid is driven to 0 for sending the chirp-K. This delay is required for some UTMI/ULPI PHYs. Note: If you enable the hibernation feature when the device core comes out of power-off, you must re-initialize this bit with the appropriate value because the core does not save and restore this bit value during hibernation. This bit is valid only in device mode."]
pub type XcvrdlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "USB 2.0 Turnaround Time Sets the turnaround time in PHY clocks. Specifies the response time for a MAC request to the Packet FIFO Controller (PFC) to fetch data from the DFIFO (SPRAM). The following are the required values for the minimum SoC bus frequency of 60 MHz. USB turnaround time is a critical certification criteria when using long cables and five hub levels. The required values for this field:\n\nValue on reset: 9"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usbtrdtim {
    #[doc = "5: When the MAC interface is 8-bit UTMI+/ULPI. If SoC bus clock is less than 60 MHz, and USB turnaround time is not critical, this field can be set to a larger value. Note: This field is valid only in device mode."]
    H5 = 5,
    #[doc = "9: When the MAC interface is 8-bit UTMI+/ULPI. If SoC bus clock is less than 60 MHz, and USB turnaround time is not critical, this field can be set to a larger value. Note: This field is valid only in device mode."]
    H9 = 9,
}
impl From<Usbtrdtim> for u8 {
    #[inline(always)]
    fn from(variant: Usbtrdtim) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usbtrdtim {
    type Ux = u8;
}
#[doc = "Field `USBTRDTIM` reader - USB 2.0 Turnaround Time Sets the turnaround time in PHY clocks. Specifies the response time for a MAC request to the Packet FIFO Controller (PFC) to fetch data from the DFIFO (SPRAM). The following are the required values for the minimum SoC bus frequency of 60 MHz. USB turnaround time is a critical certification criteria when using long cables and five hub levels. The required values for this field:"]
pub type UsbtrdtimR = crate::FieldReader<Usbtrdtim>;
impl UsbtrdtimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Usbtrdtim> {
        match self.bits {
            5 => Some(Usbtrdtim::H5),
            9 => Some(Usbtrdtim::H9),
            _ => None,
        }
    }
    #[doc = "When the MAC interface is 8-bit UTMI+/ULPI. If SoC bus clock is less than 60 MHz, and USB turnaround time is not critical, this field can be set to a larger value. Note: This field is valid only in device mode."]
    #[inline(always)]
    pub fn is_h5(&self) -> bool {
        *self == Usbtrdtim::H5
    }
    #[doc = "When the MAC interface is 8-bit UTMI+/ULPI. If SoC bus clock is less than 60 MHz, and USB turnaround time is not critical, this field can be set to a larger value. Note: This field is valid only in device mode."]
    #[inline(always)]
    pub fn is_h9(&self) -> bool {
        *self == Usbtrdtim::H9
    }
}
#[doc = "Field `USBTRDTIM` writer - USB 2.0 Turnaround Time Sets the turnaround time in PHY clocks. Specifies the response time for a MAC request to the Packet FIFO Controller (PFC) to fetch data from the DFIFO (SPRAM). The following are the required values for the minimum SoC bus frequency of 60 MHz. USB turnaround time is a critical certification criteria when using long cables and five hub levels. The required values for this field:"]
pub type UsbtrdtimW<'a, REG> = crate::FieldWriter<'a, REG, 4, Usbtrdtim>;
impl<'a, REG> UsbtrdtimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "When the MAC interface is 8-bit UTMI+/ULPI. If SoC bus clock is less than 60 MHz, and USB turnaround time is not critical, this field can be set to a larger value. Note: This field is valid only in device mode."]
    #[inline(always)]
    pub fn h5(self) -> &'a mut crate::W<REG> {
        self.variant(Usbtrdtim::H5)
    }
    #[doc = "When the MAC interface is 8-bit UTMI+/ULPI. If SoC bus clock is less than 60 MHz, and USB turnaround time is not critical, this field can be set to a larger value. Note: This field is valid only in device mode."]
    #[inline(always)]
    pub fn h9(self) -> &'a mut crate::W<REG> {
        self.variant(Usbtrdtim::H9)
    }
}
#[doc = "LS Inter-Packet Time This field indicates the value of Tx-to-Tx packet gap for LS devices. The encoding is as follows:\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lsipd {
    #[doc = "0: 5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    D0 = 0,
    #[doc = "1: 5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    D1 = 1,
    #[doc = "2: 5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    D2 = 2,
    #[doc = "3: 5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    D3 = 3,
    #[doc = "4: 5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    D4 = 4,
    #[doc = "5: 5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    D5 = 5,
    #[doc = "6: 5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    D6 = 6,
    #[doc = "7: 5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    D7 = 7,
}
impl From<Lsipd> for u8 {
    #[inline(always)]
    fn from(variant: Lsipd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lsipd {
    type Ux = u8;
}
#[doc = "Field `LSIPD` reader - LS Inter-Packet Time This field indicates the value of Tx-to-Tx packet gap for LS devices. The encoding is as follows:"]
pub type LsipdR = crate::FieldReader<Lsipd>;
impl LsipdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsipd {
        match self.bits {
            0 => Lsipd::D0,
            1 => Lsipd::D1,
            2 => Lsipd::D2,
            3 => Lsipd::D3,
            4 => Lsipd::D4,
            5 => Lsipd::D5,
            6 => Lsipd::D6,
            7 => Lsipd::D7,
            _ => unreachable!(),
        }
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == Lsipd::D0
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == Lsipd::D1
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == Lsipd::D2
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == Lsipd::D3
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == Lsipd::D4
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == Lsipd::D5
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == Lsipd::D6
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn is_d7(&self) -> bool {
        *self == Lsipd::D7
    }
}
#[doc = "Field `LSIPD` writer - LS Inter-Packet Time This field indicates the value of Tx-to-Tx packet gap for LS devices. The encoding is as follows:"]
pub type LsipdW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Lsipd>;
impl<'a, REG> LsipdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(Lsipd::D0)
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(Lsipd::D1)
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(Lsipd::D2)
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(Lsipd::D3)
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn d4(self) -> &'a mut crate::W<REG> {
        self.variant(Lsipd::D4)
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn d5(self) -> &'a mut crate::W<REG> {
        self.variant(Lsipd::D5)
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn d6(self) -> &'a mut crate::W<REG> {
        self.variant(Lsipd::D6)
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h2 (3 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the AOpen LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn d7(self) -> &'a mut crate::W<REG> {
        self.variant(Lsipd::D7)
    }
}
#[doc = "LS Turnaround Time This field indicates the value of the Rx-to-Tx packet gap for LS devices. The encoding is as follows:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lstrd {
    #[doc = "0: 5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    D0 = 0,
    #[doc = "1: 5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    D1 = 1,
    #[doc = "2: 5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    D2 = 2,
    #[doc = "3: 5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    D3 = 3,
    #[doc = "4: 5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    D4 = 4,
    #[doc = "5: 5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    D5 = 5,
    #[doc = "6: 5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    D6 = 6,
    #[doc = "7: 5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    D7 = 7,
}
impl From<Lstrd> for u8 {
    #[inline(always)]
    fn from(variant: Lstrd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lstrd {
    type Ux = u8;
}
#[doc = "Field `LSTRD` reader - LS Turnaround Time This field indicates the value of the Rx-to-Tx packet gap for LS devices. The encoding is as follows:"]
pub type LstrdR = crate::FieldReader<Lstrd>;
impl LstrdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lstrd {
        match self.bits {
            0 => Lstrd::D0,
            1 => Lstrd::D1,
            2 => Lstrd::D2,
            3 => Lstrd::D3,
            4 => Lstrd::D4,
            5 => Lstrd::D5,
            6 => Lstrd::D6,
            7 => Lstrd::D7,
            _ => unreachable!(),
        }
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == Lstrd::D0
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == Lstrd::D1
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == Lstrd::D2
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == Lstrd::D3
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == Lstrd::D4
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn is_d5(&self) -> bool {
        *self == Lstrd::D5
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn is_d6(&self) -> bool {
        *self == Lstrd::D6
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn is_d7(&self) -> bool {
        *self == Lstrd::D7
    }
}
#[doc = "Field `LSTRD` writer - LS Turnaround Time This field indicates the value of the Rx-to-Tx packet gap for LS devices. The encoding is as follows:"]
pub type LstrdW<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, Lstrd>;
impl<'a, REG> LstrdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(Lstrd::D0)
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(Lstrd::D1)
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(Lstrd::D2)
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(Lstrd::D3)
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn d4(self) -> &'a mut crate::W<REG> {
        self.variant(Lstrd::D4)
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn d5(self) -> &'a mut crate::W<REG> {
        self.variant(Lstrd::D5)
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn d6(self) -> &'a mut crate::W<REG> {
        self.variant(Lstrd::D6)
    }
    #[doc = "5.5 bit times Note: This field is applicable only in Host mode. For normal operation (to work with most LS devices), set the default value of this field to 3'h0 (2 bit times). The programmable LS device inter-packet gap and turnaround delays are provided to support some legacy LS devices that might require different delays than the default/fixed ones. For instance, the Open LS mouse requires 3 bit times of inter-packet gap to work correctly. Include your PHY delays when programming the LSIPD/LSTRDTIM values. For example, if your PHY's TxEndDelay in LS mode is 30 UTMI/ULPI CLKs, then subtract this delay (~1 LS bit time) from the device's delay requirement."]
    #[inline(always)]
    pub fn d7(self) -> &'a mut crate::W<REG> {
        self.variant(Lstrd::D7)
    }
}
#[doc = "U2_FREECLK_EXISTS Specifies whether your USB 2.0 PHY provides a free-running PHY clock, which is active when the clock control input is active. If your USB 2.0 PHY provides a free-running PHY clock, it must be connected to the utmi_clk\\[0\\]
input. The remaining utmi_clk\\[n\\]
must be connected to the respective port clocks. The core uses the Port-0 clock for generating the internal mac2 clock.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U2FreeclkExists {
    #[doc = "0: USB 2.0 free clock exists Note: When the core is configured as device-only, do not set this bit to 1."]
    B0 = 0,
    #[doc = "1: USB 2.0 free clock exists Note: When the core is configured as device-only, do not set this bit to 1."]
    B1 = 1,
}
impl From<U2FreeclkExists> for bool {
    #[inline(always)]
    fn from(variant: U2FreeclkExists) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `U2_FREECLK_EXISTS` reader - U2_FREECLK_EXISTS Specifies whether your USB 2.0 PHY provides a free-running PHY clock, which is active when the clock control input is active. If your USB 2.0 PHY provides a free-running PHY clock, it must be connected to the utmi_clk\\[0\\]
input. The remaining utmi_clk\\[n\\]
must be connected to the respective port clocks. The core uses the Port-0 clock for generating the internal mac2 clock."]
pub type U2FreeclkExistsR = crate::BitReader<U2FreeclkExists>;
impl U2FreeclkExistsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U2FreeclkExists {
        match self.bits {
            false => U2FreeclkExists::B0,
            true => U2FreeclkExists::B1,
        }
    }
    #[doc = "USB 2.0 free clock exists Note: When the core is configured as device-only, do not set this bit to 1."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == U2FreeclkExists::B0
    }
    #[doc = "USB 2.0 free clock exists Note: When the core is configured as device-only, do not set this bit to 1."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == U2FreeclkExists::B1
    }
}
#[doc = "Field `U2_FREECLK_EXISTS` writer - U2_FREECLK_EXISTS Specifies whether your USB 2.0 PHY provides a free-running PHY clock, which is active when the clock control input is active. If your USB 2.0 PHY provides a free-running PHY clock, it must be connected to the utmi_clk\\[0\\]
input. The remaining utmi_clk\\[n\\]
must be connected to the respective port clocks. The core uses the Port-0 clock for generating the internal mac2 clock."]
pub type U2FreeclkExistsW<'a, REG> = crate::BitWriter<'a, REG, U2FreeclkExists>;
impl<'a, REG> U2FreeclkExistsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB 2.0 free clock exists Note: When the core is configured as device-only, do not set this bit to 1."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(U2FreeclkExists::B0)
    }
    #[doc = "USB 2.0 free clock exists Note: When the core is configured as device-only, do not set this bit to 1."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(U2FreeclkExists::B1)
    }
}
#[doc = "Field `PHYSOFTRST` reader - UTMI PHY Soft Reset Causes the usb2phy_reset signal to be asserted to reset a UTMI PHY. Not applicable to ULPI because ULPI PHYs are reset via their FunctionControl.Reset register, and the core automatically writes to this register when the core is reset (vcc_reset_n, USBCMD.HCRST, DCTL.SoftReset, or GCTL.SoftReset)"]
pub type PhysoftrstR = crate::BitReader;
#[doc = "Field `PHYSOFTRST` writer - UTMI PHY Soft Reset Causes the usb2phy_reset signal to be asserted to reset a UTMI PHY. Not applicable to ULPI because ULPI PHYs are reset via their FunctionControl.Reset register, and the core automatically writes to this register when the core is reset (vcc_reset_n, USBCMD.HCRST, DCTL.SoftReset, or GCTL.SoftReset)"]
pub type PhysoftrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - HS/FS Timeout Calibration The number of PHY clocks, as indicated by the application in this field, is multiplied by a bit-time factor; this factor is added to the high-speed/full-speed interpacket timeout duration in the core to account for additional delays introduced by the PHY. This may be required, since the delay introduced by the PHY in generating the linestate condition may vary among PHYs. The USB standard timeout value for high-speed operation is 736 to 816 (inclusive) bit times. The USB standard timeout value for full-speed operation is 16 to 18 (inclusive) bit times. The application must program this field based on the speed of connection. The number of bit times added per PHY clock are: High-speed operation: One 30-MHz PHY clock = 16 bit times One 60-MHz PHY clock = 8 bit times Full-speed operation: One 30-MHz PHY clock = 0.4 bit times One 60-MHz PHY clock = 0.2 bit times"]
    #[inline(always)]
    pub fn toutcal(&self) -> ToutcalR {
        ToutcalR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - PHY Interface If UTMI+ is selected, the application uses this bit to configure the core to support a UTMI+ PHY with an 8- or 16-bit interface."]
    #[inline(always)]
    pub fn phyif(&self) -> PhyifR {
        PhyifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ULPI or UTMI+ Select"]
    #[inline(always)]
    pub fn ulpi_utmi_sel(&self) -> UlpiUtmiSelR {
        UlpiUtmiSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Suspend USB2.0 HS/FS/LS PHY When set, USB2.0 PHY enters Suspend mode if Suspend conditions are valid. For DRD/OTG configurations, it is recommended that this bit is set to 0 during coreConsultant configuration. If it is set to 1, then the application must clear this bit after power-on reset. Application needs to set it to 1 after the core initialization completes. For all other configurations, this bit can be set to 1 during core configuration. Note: In host mode, on reset, this bit is set to 1. Software can override this bit after reset. In device mode, before issuing any device endpoint command when operating in 2.0 speeds, disable this bit and enable it after the command completes. If you issue a command without disabling this bit when the device is in L2 state and if mac2_clk (utmi_clk/ulpi_clk) is gated off, the command will not get completed."]
    #[inline(always)]
    pub fn suspendusb20(&self) -> Suspendusb20R {
        Suspendusb20R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB 2.0 High-Speed PHY or USB 1.1 Full-Speed"]
    #[inline(always)]
    pub fn physel(&self) -> PhyselR {
        PhyselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable utmi_sleep_n and utmi_l1_suspend_n The application uses this bit to control utmi_sleep_n and utmi_l1_suspend_n assertion to the PHY in the L1 state."]
    #[inline(always)]
    pub fn enblslpm(&self) -> EnblslpmR {
        EnblslpmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transceiver Delay Enables a delay between the assertion of the UTMI/ULPI Transceiver Select signal (for HS) and the assertion of the TxValid signal during a HS Chirp. When this bit is set to 1, a delay (of approximately 2.5 us) is introduced from the time when the Transceiver Select is set to 2'b00 (HS) to the time the TxValid is driven to 0 for sending the chirp-K. This delay is required for some UTMI/ULPI PHYs. Note: If you enable the hibernation feature when the device core comes out of power-off, you must re-initialize this bit with the appropriate value because the core does not save and restore this bit value during hibernation. This bit is valid only in device mode."]
    #[inline(always)]
    pub fn xcvrdly(&self) -> XcvrdlyR {
        XcvrdlyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - USB 2.0 Turnaround Time Sets the turnaround time in PHY clocks. Specifies the response time for a MAC request to the Packet FIFO Controller (PFC) to fetch data from the DFIFO (SPRAM). The following are the required values for the minimum SoC bus frequency of 60 MHz. USB turnaround time is a critical certification criteria when using long cables and five hub levels. The required values for this field:"]
    #[inline(always)]
    pub fn usbtrdtim(&self) -> UsbtrdtimR {
        UsbtrdtimR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 19:21 - LS Inter-Packet Time This field indicates the value of Tx-to-Tx packet gap for LS devices. The encoding is as follows:"]
    #[inline(always)]
    pub fn lsipd(&self) -> LsipdR {
        LsipdR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - LS Turnaround Time This field indicates the value of the Rx-to-Tx packet gap for LS devices. The encoding is as follows:"]
    #[inline(always)]
    pub fn lstrd(&self) -> LstrdR {
        LstrdR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 30 - U2_FREECLK_EXISTS Specifies whether your USB 2.0 PHY provides a free-running PHY clock, which is active when the clock control input is active. If your USB 2.0 PHY provides a free-running PHY clock, it must be connected to the utmi_clk\\[0\\]
input. The remaining utmi_clk\\[n\\]
must be connected to the respective port clocks. The core uses the Port-0 clock for generating the internal mac2 clock."]
    #[inline(always)]
    pub fn u2_freeclk_exists(&self) -> U2FreeclkExistsR {
        U2FreeclkExistsR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UTMI PHY Soft Reset Causes the usb2phy_reset signal to be asserted to reset a UTMI PHY. Not applicable to ULPI because ULPI PHYs are reset via their FunctionControl.Reset register, and the core automatically writes to this register when the core is reset (vcc_reset_n, USBCMD.HCRST, DCTL.SoftReset, or GCTL.SoftReset)"]
    #[inline(always)]
    pub fn physoftrst(&self) -> PhysoftrstR {
        PhysoftrstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - HS/FS Timeout Calibration The number of PHY clocks, as indicated by the application in this field, is multiplied by a bit-time factor; this factor is added to the high-speed/full-speed interpacket timeout duration in the core to account for additional delays introduced by the PHY. This may be required, since the delay introduced by the PHY in generating the linestate condition may vary among PHYs. The USB standard timeout value for high-speed operation is 736 to 816 (inclusive) bit times. The USB standard timeout value for full-speed operation is 16 to 18 (inclusive) bit times. The application must program this field based on the speed of connection. The number of bit times added per PHY clock are: High-speed operation: One 30-MHz PHY clock = 16 bit times One 60-MHz PHY clock = 8 bit times Full-speed operation: One 30-MHz PHY clock = 0.4 bit times One 60-MHz PHY clock = 0.2 bit times"]
    #[inline(always)]
    #[must_use]
    pub fn toutcal(&mut self) -> ToutcalW<Usb3Gusb2phycfg0Spec> {
        ToutcalW::new(self, 0)
    }
    #[doc = "Bit 3 - PHY Interface If UTMI+ is selected, the application uses this bit to configure the core to support a UTMI+ PHY with an 8- or 16-bit interface."]
    #[inline(always)]
    #[must_use]
    pub fn phyif(&mut self) -> PhyifW<Usb3Gusb2phycfg0Spec> {
        PhyifW::new(self, 3)
    }
    #[doc = "Bit 6 - Suspend USB2.0 HS/FS/LS PHY When set, USB2.0 PHY enters Suspend mode if Suspend conditions are valid. For DRD/OTG configurations, it is recommended that this bit is set to 0 during coreConsultant configuration. If it is set to 1, then the application must clear this bit after power-on reset. Application needs to set it to 1 after the core initialization completes. For all other configurations, this bit can be set to 1 during core configuration. Note: In host mode, on reset, this bit is set to 1. Software can override this bit after reset. In device mode, before issuing any device endpoint command when operating in 2.0 speeds, disable this bit and enable it after the command completes. If you issue a command without disabling this bit when the device is in L2 state and if mac2_clk (utmi_clk/ulpi_clk) is gated off, the command will not get completed."]
    #[inline(always)]
    #[must_use]
    pub fn suspendusb20(&mut self) -> Suspendusb20W<Usb3Gusb2phycfg0Spec> {
        Suspendusb20W::new(self, 6)
    }
    #[doc = "Bit 8 - Enable utmi_sleep_n and utmi_l1_suspend_n The application uses this bit to control utmi_sleep_n and utmi_l1_suspend_n assertion to the PHY in the L1 state."]
    #[inline(always)]
    #[must_use]
    pub fn enblslpm(&mut self) -> EnblslpmW<Usb3Gusb2phycfg0Spec> {
        EnblslpmW::new(self, 8)
    }
    #[doc = "Bit 9 - Transceiver Delay Enables a delay between the assertion of the UTMI/ULPI Transceiver Select signal (for HS) and the assertion of the TxValid signal during a HS Chirp. When this bit is set to 1, a delay (of approximately 2.5 us) is introduced from the time when the Transceiver Select is set to 2'b00 (HS) to the time the TxValid is driven to 0 for sending the chirp-K. This delay is required for some UTMI/ULPI PHYs. Note: If you enable the hibernation feature when the device core comes out of power-off, you must re-initialize this bit with the appropriate value because the core does not save and restore this bit value during hibernation. This bit is valid only in device mode."]
    #[inline(always)]
    #[must_use]
    pub fn xcvrdly(&mut self) -> XcvrdlyW<Usb3Gusb2phycfg0Spec> {
        XcvrdlyW::new(self, 9)
    }
    #[doc = "Bits 10:13 - USB 2.0 Turnaround Time Sets the turnaround time in PHY clocks. Specifies the response time for a MAC request to the Packet FIFO Controller (PFC) to fetch data from the DFIFO (SPRAM). The following are the required values for the minimum SoC bus frequency of 60 MHz. USB turnaround time is a critical certification criteria when using long cables and five hub levels. The required values for this field:"]
    #[inline(always)]
    #[must_use]
    pub fn usbtrdtim(&mut self) -> UsbtrdtimW<Usb3Gusb2phycfg0Spec> {
        UsbtrdtimW::new(self, 10)
    }
    #[doc = "Bits 19:21 - LS Inter-Packet Time This field indicates the value of Tx-to-Tx packet gap for LS devices. The encoding is as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn lsipd(&mut self) -> LsipdW<Usb3Gusb2phycfg0Spec> {
        LsipdW::new(self, 19)
    }
    #[doc = "Bits 22:24 - LS Turnaround Time This field indicates the value of the Rx-to-Tx packet gap for LS devices. The encoding is as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn lstrd(&mut self) -> LstrdW<Usb3Gusb2phycfg0Spec> {
        LstrdW::new(self, 22)
    }
    #[doc = "Bit 30 - U2_FREECLK_EXISTS Specifies whether your USB 2.0 PHY provides a free-running PHY clock, which is active when the clock control input is active. If your USB 2.0 PHY provides a free-running PHY clock, it must be connected to the utmi_clk\\[0\\]
input. The remaining utmi_clk\\[n\\]
must be connected to the respective port clocks. The core uses the Port-0 clock for generating the internal mac2 clock."]
    #[inline(always)]
    #[must_use]
    pub fn u2_freeclk_exists(&mut self) -> U2FreeclkExistsW<Usb3Gusb2phycfg0Spec> {
        U2FreeclkExistsW::new(self, 30)
    }
    #[doc = "Bit 31 - UTMI PHY Soft Reset Causes the usb2phy_reset signal to be asserted to reset a UTMI PHY. Not applicable to ULPI because ULPI PHYs are reset via their FunctionControl.Reset register, and the core automatically writes to this register when the core is reset (vcc_reset_n, USBCMD.HCRST, DCTL.SoftReset, or GCTL.SoftReset)"]
    #[inline(always)]
    #[must_use]
    pub fn physoftrst(&mut self) -> PhysoftrstW<Usb3Gusb2phycfg0Spec> {
        PhysoftrstW::new(self, 31)
    }
}
#[doc = "Global USB2 PHY Configuration Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gusb2phycfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gusb2phycfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3Gusb2phycfg0Spec;
impl crate::RegisterSpec for Usb3Gusb2phycfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_gusb2phycfg0::R`](R) reader structure"]
impl crate::Readable for Usb3Gusb2phycfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`usb3_gusb2phycfg0::W`](W) writer structure"]
impl crate::Writable for Usb3Gusb2phycfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_GUSB2PHYCFG0 to value 0x4010_2400"]
impl crate::Resettable for Usb3Gusb2phycfg0Spec {
    const RESET_VALUE: u32 = 0x4010_2400;
}
