#[doc = "Register `USB3_GCTL` reader"]
pub type R = crate::R<Usb3GctlSpec>;
#[doc = "Register `USB3_GCTL` writer"]
pub type W = crate::W<Usb3GctlSpec>;
#[doc = "Field `DSBLCLKGTNG` reader - Disable Clock Gating (DsblClkGtng)\n\nThis bit is set to 1 and the core is in Low Power mode, internal\n\nclock gating is disabled. You can set this bit to 1'b1 after Power\n\nOn Reset."]
pub type DsblclkgtngR = crate::BitReader;
#[doc = "Field `DSBLCLKGTNG` writer - Disable Clock Gating (DsblClkGtng)\n\nThis bit is set to 1 and the core is in Low Power mode, internal\n\nclock gating is disabled. You can set this bit to 1'b1 after Power\n\nOn Reset."]
pub type DsblclkgtngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GBLHIBERNATIONEN` reader - GblHibernationEn\n\nThis bit enables hibernation at the global level. If hibernation is\n\nnot enabled through this bit, the PMU immediately accepts the\n\nD0->D3 and D3->D0 power state change requests, but does not\n\nsave or restore any core state. In addition, the PMUs never drive\n\nthe PHY interfaces and let the core continue to drive the PHY\n\ninterfaces."]
pub type GblhibernationenR = crate::BitReader;
#[doc = "U2EXIT_LFPS\n\nIf this bit is:\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U2exitLfps {
    #[doc = "0: the link treats 248ns LFPS as a valid U2 exit."]
    B0 = 0,
    #[doc = "1: the link waits for 8us of LFPS before it detects a valid U2 exit. This bit is added to improve interoperability with a third party host controller. This host controller in U2 state while performing receiver detection generates an LFPS glitch of about 4ms duration. This causes the device to exit from U2 state because the LFPS filter value is 248ns. With the new functionality enabled, the device can stay in U2 while ignoring this glitch from the host controller."]
    B1 = 1,
}
impl From<U2exitLfps> for bool {
    #[inline(always)]
    fn from(variant: U2exitLfps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `U2EXIT_LFPS` reader - U2EXIT_LFPS\n\nIf this bit is:"]
pub type U2exitLfpsR = crate::BitReader<U2exitLfps>;
impl U2exitLfpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U2exitLfps {
        match self.bits {
            false => U2exitLfps::B0,
            true => U2exitLfps::B1,
        }
    }
    #[doc = "the link treats 248ns LFPS as a valid U2 exit."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == U2exitLfps::B0
    }
    #[doc = "the link waits for 8us of LFPS before it detects a valid U2 exit. This bit is added to improve interoperability with a third party host controller. This host controller in U2 state while performing receiver detection generates an LFPS glitch of about 4ms duration. This causes the device to exit from U2 state because the LFPS filter value is 248ns. With the new functionality enabled, the device can stay in U2 while ignoring this glitch from the host controller."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == U2exitLfps::B1
    }
}
#[doc = "Field `U2EXIT_LFPS` writer - U2EXIT_LFPS\n\nIf this bit is:"]
pub type U2exitLfpsW<'a, REG> = crate::BitWriter<'a, REG, U2exitLfps>;
impl<'a, REG> U2exitLfpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the link treats 248ns LFPS as a valid U2 exit."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(U2exitLfps::B0)
    }
    #[doc = "the link waits for 8us of LFPS before it detects a valid U2 exit. This bit is added to improve interoperability with a third party host controller. This host controller in U2 state while performing receiver detection generates an LFPS glitch of about 4ms duration. This causes the device to exit from U2 state because the LFPS filter value is 248ns. With the new functionality enabled, the device can stay in U2 while ignoring this glitch from the host controller."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(U2exitLfps::B1)
    }
}
#[doc = "Field `DISSCRAMBLE` reader - Disable Scrambling (DisScramble)\n\nTransmit request to Link Partner on next transition to Recovery or\n\nPolling."]
pub type DisscrambleR = crate::BitReader;
#[doc = "Field `DISSCRAMBLE` writer - Disable Scrambling (DisScramble)\n\nTransmit request to Link Partner on next transition to Recovery or\n\nPolling."]
pub type DisscrambleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCALEDOWN` reader - Scale-Down Mode (ScaleDown)\n\nWhen Scale-Down mode is enabled for simulation, the core uses\n\nscaled-down timing values, resulting in faster simulations.\n\nWhen Scale-Down mode is disabled, actual timing values are\n\nused. This is required for hardware operation.\n\nHS/FS/LS Modes:\n\n2'b00: Disables all scale-downs. Actual timing values are used.\n\n2'b01: Enables scale-down of all timing values except Device\n\nmode suspend and resume. These include Speed enumeration,\n\nHNP/SRP, and Host mode suspend and resume\n\n2'b10: Enables scale-down of Device mode suspend and resume\n\ntiming values only.\n\n2'b11: Enables bit 0 and bit 1 scale-down timing values.\n\nSS Mode:\n\n2'b00: Disables all scale-downs. Actual timing values are used.\n\n2'b01: Enables scaled down SS timing and repeat values\n\nincluding: (1) Number of TxEq training sequences reduce to 8;\n\n(2) LFPS polling burst time reduce to 256 nS; (3) LFPS warm\n\nreset receive reduce to 30 uS. 2'b10: No TxEq training sequences\n\nare sent. Overrides Bit 4.\n\n2'b11: Enables bit 0 and bit 1 scale-down timing values."]
pub type ScaledownR = crate::FieldReader;
#[doc = "Field `SCALEDOWN` writer - Scale-Down Mode (ScaleDown)\n\nWhen Scale-Down mode is enabled for simulation, the core uses\n\nscaled-down timing values, resulting in faster simulations.\n\nWhen Scale-Down mode is disabled, actual timing values are\n\nused. This is required for hardware operation.\n\nHS/FS/LS Modes:\n\n2'b00: Disables all scale-downs. Actual timing values are used.\n\n2'b01: Enables scale-down of all timing values except Device\n\nmode suspend and resume. These include Speed enumeration,\n\nHNP/SRP, and Host mode suspend and resume\n\n2'b10: Enables scale-down of Device mode suspend and resume\n\ntiming values only.\n\n2'b11: Enables bit 0 and bit 1 scale-down timing values.\n\nSS Mode:\n\n2'b00: Disables all scale-downs. Actual timing values are used.\n\n2'b01: Enables scaled down SS timing and repeat values\n\nincluding: (1) Number of TxEq training sequences reduce to 8;\n\n(2) LFPS polling burst time reduce to 256 nS; (3) LFPS warm\n\nreset receive reduce to 30 uS. 2'b10: No TxEq training sequences\n\nare sent. Overrides Bit 4.\n\n2'b11: Enables bit 0 and bit 1 scale-down timing values."]
pub type ScaledownW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "RAM Clock Select (RAMClkSel)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ramclksel {
    #[doc = "0: bus clock"]
    B00 = 0,
    #[doc = "1: pipe clock (Only used in device mode)"]
    B01 = 1,
    #[doc = "2: In device mode, pipe/2 clock. In Host mode, controller switches ram_clk between pipe/2 clock, mac2_clk and bus_clk based on the status of the U2/U3 ports"]
    B10 = 2,
    #[doc = "3: In device mode, selects mac2_clk as ram_clk (when 8-bit UTMI or ULPI used. Not supported in 16-bit UTMI mode); In Host mode, controller switches ram_clk between pipe_clk, mac2_clk and bus_clk based on the status of the U2/U3 ports. In device mode, upon a USB reset and USB disconnect, the hardware clears these bits to 2'b00."]
    B11 = 3,
}
impl From<Ramclksel> for u8 {
    #[inline(always)]
    fn from(variant: Ramclksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ramclksel {
    type Ux = u8;
}
#[doc = "Field `RAMCLKSEL` reader - RAM Clock Select (RAMClkSel)"]
pub type RamclkselR = crate::FieldReader<Ramclksel>;
impl RamclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramclksel {
        match self.bits {
            0 => Ramclksel::B00,
            1 => Ramclksel::B01,
            2 => Ramclksel::B10,
            3 => Ramclksel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "bus clock"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Ramclksel::B00
    }
    #[doc = "pipe clock (Only used in device mode)"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Ramclksel::B01
    }
    #[doc = "In device mode, pipe/2 clock. In Host mode, controller switches ram_clk between pipe/2 clock, mac2_clk and bus_clk based on the status of the U2/U3 ports"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Ramclksel::B10
    }
    #[doc = "In device mode, selects mac2_clk as ram_clk (when 8-bit UTMI or ULPI used. Not supported in 16-bit UTMI mode); In Host mode, controller switches ram_clk between pipe_clk, mac2_clk and bus_clk based on the status of the U2/U3 ports. In device mode, upon a USB reset and USB disconnect, the hardware clears these bits to 2'b00."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Ramclksel::B11
    }
}
#[doc = "Field `RAMCLKSEL` writer - RAM Clock Select (RAMClkSel)"]
pub type RamclkselW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Ramclksel>;
impl<'a, REG> RamclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "bus clock"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Ramclksel::B00)
    }
    #[doc = "pipe clock (Only used in device mode)"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Ramclksel::B01)
    }
    #[doc = "In device mode, pipe/2 clock. In Host mode, controller switches ram_clk between pipe/2 clock, mac2_clk and bus_clk based on the status of the U2/U3 ports"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Ramclksel::B10)
    }
    #[doc = "In device mode, selects mac2_clk as ram_clk (when 8-bit UTMI or ULPI used. Not supported in 16-bit UTMI mode); In Host mode, controller switches ram_clk between pipe_clk, mac2_clk and bus_clk based on the status of the U2/U3 ports. In device mode, upon a USB reset and USB disconnect, the hardware clears these bits to 2'b00."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Ramclksel::B11)
    }
}
#[doc = "Field `DEBUGATTACH` reader - Debug Attach\n\nWhen this bit is set:\n\n1. SS Link proceeds directly to the Polling link state (after\n\nRUN/STOP in the DCTL register is asserted) without checking\n\nremote termination;\n\n2. Link LFPS polling timeout is infinite;\n\n3. Polling timeout during TS1 is infinite (in case link is waiting for\n\nTXEQ to finish)."]
pub type DebugattachR = crate::BitReader;
#[doc = "Field `DEBUGATTACH` writer - Debug Attach\n\nWhen this bit is set:\n\n1. SS Link proceeds directly to the Polling link state (after\n\nRUN/STOP in the DCTL register is asserted) without checking\n\nremote termination;\n\n2. Link LFPS polling timeout is infinite;\n\n3. Polling timeout during TS1 is infinite (in case link is waiting for\n\nTXEQ to finish)."]
pub type DebugattachW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `U1U2TIMERSCALE` reader - Disable U1/U2 timer Scaledown (U1U2TimerScale).\n\nIf set to 1 along with GCTL\\[5:4\\]
(ScaleDown) = 2'bX1, disables\n\nthe scale down of U1/U2 inactive timer values. This is for\n\nsimulation mode only."]
pub type U1u2timerscaleR = crate::BitReader;
#[doc = "Field `U1U2TIMERSCALE` writer - Disable U1/U2 timer Scaledown (U1U2TimerScale).\n\nIf set to 1 along with GCTL\\[5:4\\]
(ScaleDown) = 2'bX1, disables\n\nthe scale down of U1/U2 inactive timer values. This is for\n\nsimulation mode only."]
pub type U1u2timerscaleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFITPSYNC` reader - SOFITPSYNC\n\nIf this bit is set to 0 operating in host mode, the core keeps the\n\nUTMI/ULPI PHY on the first port in a non-suspended state\n\nwhenever there is a SuperSpeed port that is not in Rx.Detect,\n\nSS.Disable and U3.\n\nIf this bit is set to 1 operating in host mode, the core keeps the\n\nUTMI/ULPI PHY on the first port in a non-suspended state\n\nwhenever the other non-SuperSpeed ports are not in a\n\nsuspended state. This feature is useful because it saves power by\n\nsuspending UTMI/ULPI when SuperSpeed only is active, and it\n\nhelps resolve when the PHY does not transmit a host resume\n\nunless it is placed in suspend state. This bit must be programmed\n\nas a part of initialization at power-on reset, and must not be\n\ndynamically changed afterwards.\n\nNote:\n\nUSB2PHYCFGn\\[6\\].PhySusp eventually decides to put the\n\nUTMI/ULPI PHY in to suspend state. In addition, when this bit is\n\nset to 1, the core generates ITP from the ref_clk based counter.\n\nOtherwise, ITP and SOF are generated from utmi/ulpi_clk\\[0\\]\n\nbased counter. To program the reference clock period inside the\n\ncore, refer to GUCTL\\[31:22\\].REFCLKPER.\n\nThis feature is valid in Host and DRD/OTG configurations and\n\nused only in Host mode operation.\n\nIf you never use this feature or the\n\nGFLADJ.GFLADJ_REFCLK_LPM_SEL, the minimum frequency for\n\nthe ref_clk can be as low as 32KHz. You can connect the\n\nsuspend_clk (as low as 32 KHz) to the ref_clk.\n\nIf you plan to enable hardware-based LPM or software-based LPM\n\n(PORTPMSC. HLE=1), then you cannot use this feature.\n\nTurn off this feature by setting this bit to 0 and use the\n\nGFLADJ.GFLADJ_REFCLK_LPM_SEL feature.\n\nIf you set this bit to 1, the GUSB2PHYCFG.U2_FREECLK_EXISTS\n\nbit and the DWC_USB3_FREECLK_USB2_EXIST parameter must\n\nbe set to 0.\n\nProgram this bit to 0 if the core is intended to be operated in USB\n\n3.0 mode."]
pub type SofitpsyncR = crate::BitReader;
#[doc = "Field `SOFITPSYNC` writer - SOFITPSYNC\n\nIf this bit is set to 0 operating in host mode, the core keeps the\n\nUTMI/ULPI PHY on the first port in a non-suspended state\n\nwhenever there is a SuperSpeed port that is not in Rx.Detect,\n\nSS.Disable and U3.\n\nIf this bit is set to 1 operating in host mode, the core keeps the\n\nUTMI/ULPI PHY on the first port in a non-suspended state\n\nwhenever the other non-SuperSpeed ports are not in a\n\nsuspended state. This feature is useful because it saves power by\n\nsuspending UTMI/ULPI when SuperSpeed only is active, and it\n\nhelps resolve when the PHY does not transmit a host resume\n\nunless it is placed in suspend state. This bit must be programmed\n\nas a part of initialization at power-on reset, and must not be\n\ndynamically changed afterwards.\n\nNote:\n\nUSB2PHYCFGn\\[6\\].PhySusp eventually decides to put the\n\nUTMI/ULPI PHY in to suspend state. In addition, when this bit is\n\nset to 1, the core generates ITP from the ref_clk based counter.\n\nOtherwise, ITP and SOF are generated from utmi/ulpi_clk\\[0\\]\n\nbased counter. To program the reference clock period inside the\n\ncore, refer to GUCTL\\[31:22\\].REFCLKPER.\n\nThis feature is valid in Host and DRD/OTG configurations and\n\nused only in Host mode operation.\n\nIf you never use this feature or the\n\nGFLADJ.GFLADJ_REFCLK_LPM_SEL, the minimum frequency for\n\nthe ref_clk can be as low as 32KHz. You can connect the\n\nsuspend_clk (as low as 32 KHz) to the ref_clk.\n\nIf you plan to enable hardware-based LPM or software-based LPM\n\n(PORTPMSC. HLE=1), then you cannot use this feature.\n\nTurn off this feature by setting this bit to 0 and use the\n\nGFLADJ.GFLADJ_REFCLK_LPM_SEL feature.\n\nIf you set this bit to 1, the GUSB2PHYCFG.U2_FREECLK_EXISTS\n\nbit and the DWC_USB3_FREECLK_USB2_EXIST parameter must\n\nbe set to 0.\n\nProgram this bit to 0 if the core is intended to be operated in USB\n\n3.0 mode."]
pub type SofitpsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORESOFTRESET` reader - Core Soft Reset (CoreSoftReset)\n\n1'b0 - No soft reset;\n\n1'b1 - Soft reset to core\n\nClears the interrupts and all the CSRs except the following\n\nregisters:\n\nGCTL; GUCTL; GSTS; GSNPSID; GGPIO; GUID; GUSB2PHYCFGn\n\nregisters; GUSB3PIPECTLn registers; DCFG; DCTL; DEVTEN;\n\nDSTS.\n\nWhen you reset PHYs (using GUBS3PHYCFG or GUSB3PIPECTL\n\nregisters), you must keep the core in reset state until PHY clocks\n\nare stable. This controls the bus, ram, and mac domain resets.\n\nNote: This bit is for debug purposes only. Use USBCMD.HCRESET\n\nin xHCI Mode and DCTL.SoftReset in device mode for soft reset."]
pub type CoresoftresetR = crate::BitReader;
#[doc = "Field `CORESOFTRESET` writer - Core Soft Reset (CoreSoftReset)\n\n1'b0 - No soft reset;\n\n1'b1 - Soft reset to core\n\nClears the interrupts and all the CSRs except the following\n\nregisters:\n\nGCTL; GUCTL; GSTS; GSNPSID; GGPIO; GUID; GUSB2PHYCFGn\n\nregisters; GUSB3PIPECTLn registers; DCFG; DCTL; DEVTEN;\n\nDSTS.\n\nWhen you reset PHYs (using GUBS3PHYCFG or GUSB3PIPECTL\n\nregisters), you must keep the core in reset state until PHY clocks\n\nare stable. This controls the bus, ram, and mac domain resets.\n\nNote: This bit is for debug purposes only. Use USBCMD.HCRESET\n\nin xHCI Mode and DCTL.SoftReset in device mode for soft reset."]
pub type CoresoftresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PRTCAPDIR: Port Capability Direction (PrtCapDir)\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prtcapdir {
    #[doc = "1: for Host configurations"]
    B01 = 1,
    #[doc = "2: for Device configurations SW should base on IDDIG input to set usb3 controller as an OTG 2.0/3.0 device with A-device or B-device."]
    B10 = 2,
}
impl From<Prtcapdir> for u8 {
    #[inline(always)]
    fn from(variant: Prtcapdir) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prtcapdir {
    type Ux = u8;
}
#[doc = "Field `PRTCAPDIR` reader - PRTCAPDIR: Port Capability Direction (PrtCapDir)"]
pub type PrtcapdirR = crate::FieldReader<Prtcapdir>;
impl PrtcapdirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prtcapdir> {
        match self.bits {
            1 => Some(Prtcapdir::B01),
            2 => Some(Prtcapdir::B10),
            _ => None,
        }
    }
    #[doc = "for Host configurations"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Prtcapdir::B01
    }
    #[doc = "for Device configurations SW should base on IDDIG input to set usb3 controller as an OTG 2.0/3.0 device with A-device or B-device."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Prtcapdir::B10
    }
}
#[doc = "Field `PRTCAPDIR` writer - PRTCAPDIR: Port Capability Direction (PrtCapDir)"]
pub type PrtcapdirW<'a, REG> = crate::FieldWriter<'a, REG, 2, Prtcapdir>;
impl<'a, REG> PrtcapdirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "for Host configurations"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Prtcapdir::B01)
    }
    #[doc = "for Device configurations SW should base on IDDIG input to set usb3 controller as an OTG 2.0/3.0 device with A-device or B-device."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Prtcapdir::B10)
    }
}
#[doc = "FRMSCLDWN\n\nThis field scales down device view of a SOF/USOF/ITP duration.\n\nFor SS/HS mode:\n\nValue of 2'h3 implements interval to be 15.625 us\n\nValue of 2'h2 implements interval to be 31.25 us\n\nValue of 2'h1 implements interval to be 62.5 us\n\nValue of 2'h0 implements interval to be 125us\n\nFor FS mode, the scale-down value is multiplied by 8.\n\nWhen xHCI Debug Capability is enabled, this field also scales\n\ndown the MaxPacketSize of the IN and OUT bulk endpoint to\n\nallow more traffic during simulation. It can only be changed from\n\na non-zero\n\nvalue during simulation.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frmscldwn {
    #[doc = "0: 1024 bytes"]
    H0 = 0,
    #[doc = "1: 512 bytes"]
    H1 = 1,
    #[doc = "2: 256 bytes"]
    H2 = 2,
    #[doc = "3: 128 bytes"]
    H3 = 3,
}
impl From<Frmscldwn> for u8 {
    #[inline(always)]
    fn from(variant: Frmscldwn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frmscldwn {
    type Ux = u8;
}
#[doc = "Field `FRMSCLDWN` reader - FRMSCLDWN\n\nThis field scales down device view of a SOF/USOF/ITP duration.\n\nFor SS/HS mode:\n\nValue of 2'h3 implements interval to be 15.625 us\n\nValue of 2'h2 implements interval to be 31.25 us\n\nValue of 2'h1 implements interval to be 62.5 us\n\nValue of 2'h0 implements interval to be 125us\n\nFor FS mode, the scale-down value is multiplied by 8.\n\nWhen xHCI Debug Capability is enabled, this field also scales\n\ndown the MaxPacketSize of the IN and OUT bulk endpoint to\n\nallow more traffic during simulation. It can only be changed from\n\na non-zero\n\nvalue during simulation."]
pub type FrmscldwnR = crate::FieldReader<Frmscldwn>;
impl FrmscldwnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frmscldwn {
        match self.bits {
            0 => Frmscldwn::H0,
            1 => Frmscldwn::H1,
            2 => Frmscldwn::H2,
            3 => Frmscldwn::H3,
            _ => unreachable!(),
        }
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn is_h0(&self) -> bool {
        *self == Frmscldwn::H0
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn is_h1(&self) -> bool {
        *self == Frmscldwn::H1
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn is_h2(&self) -> bool {
        *self == Frmscldwn::H2
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn is_h3(&self) -> bool {
        *self == Frmscldwn::H3
    }
}
#[doc = "Field `FRMSCLDWN` writer - FRMSCLDWN\n\nThis field scales down device view of a SOF/USOF/ITP duration.\n\nFor SS/HS mode:\n\nValue of 2'h3 implements interval to be 15.625 us\n\nValue of 2'h2 implements interval to be 31.25 us\n\nValue of 2'h1 implements interval to be 62.5 us\n\nValue of 2'h0 implements interval to be 125us\n\nFor FS mode, the scale-down value is multiplied by 8.\n\nWhen xHCI Debug Capability is enabled, this field also scales\n\ndown the MaxPacketSize of the IN and OUT bulk endpoint to\n\nallow more traffic during simulation. It can only be changed from\n\na non-zero\n\nvalue during simulation."]
pub type FrmscldwnW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Frmscldwn>;
impl<'a, REG> FrmscldwnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn h0(self) -> &'a mut crate::W<REG> {
        self.variant(Frmscldwn::H0)
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn h1(self) -> &'a mut crate::W<REG> {
        self.variant(Frmscldwn::H1)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn h2(self) -> &'a mut crate::W<REG> {
        self.variant(Frmscldwn::H2)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn h3(self) -> &'a mut crate::W<REG> {
        self.variant(Frmscldwn::H3)
    }
}
#[doc = "Field `U2RSTECN` reader - U2RSTECN\n\nIf the SuperSpeed connection fails during POLL or LMP exchange,\n\nthe device connects at non-SS mode.\n\nIf this bit is set, then device attempts three more times to\n\nconnect at SS, even if it previously failed to operate in SS mode.\n\nFor each attempt, the device checks receiver termination eight\n\ntimes.\n\nFrom 2.60a release, this bit controls whether to check for\n\nRx.Detect eight times or one time for every attempt. Device\n\ncontroller on USB 2.0 reset checks for receiver termination eight\n\ntimes per attempt if\n\nthis bit is set to zero, or only once per attempt if the bit is set to\n\none.\n\nNote: This bit is applicable only in device mode."]
pub type U2rstecnR = crate::BitReader;
#[doc = "Field `U2RSTECN` writer - U2RSTECN\n\nIf the SuperSpeed connection fails during POLL or LMP exchange,\n\nthe device connects at non-SS mode.\n\nIf this bit is set, then device attempts three more times to\n\nconnect at SS, even if it previously failed to operate in SS mode.\n\nFor each attempt, the device checks receiver termination eight\n\ntimes.\n\nFrom 2.60a release, this bit controls whether to check for\n\nRx.Detect eight times or one time for every attempt. Device\n\ncontroller on USB 2.0 reset checks for receiver termination eight\n\ntimes per attempt if\n\nthis bit is set to zero, or only once per attempt if the bit is set to\n\none.\n\nNote: This bit is applicable only in device mode."]
pub type U2rstecnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPSSETADDR` reader - Bypass SetAddress in Device Mode.\n\nWhen BYPSSETADDR bit is set, the device core uses the value in\n\nthe DCFG\\[DevAddr\\]
bits directly for comparing the device\n\naddress in the tokens.\n\nFor simulation, you can use this feature to avoid sending an\n\nactual SET ADDRESS control transfer on the USB, and make the\n\ndevice core respond to a new address.\n\nWhen the xHCI Debug capability is enabled and this bit is set, the\n\nDebug Target immediately enters the configured state without\n\nrequiring the Debug Host to send a SetAddress or SetConfig\n\nrequest.\n\nNote: You can set this bit for simulation purposes only. In the\n\nactual hardware, this bit must be set to 1'b0."]
pub type BypssetaddrR = crate::BitReader;
#[doc = "Field `BYPSSETADDR` writer - Bypass SetAddress in Device Mode.\n\nWhen BYPSSETADDR bit is set, the device core uses the value in\n\nthe DCFG\\[DevAddr\\]
bits directly for comparing the device\n\naddress in the tokens.\n\nFor simulation, you can use this feature to avoid sending an\n\nactual SET ADDRESS control transfer on the USB, and make the\n\ndevice core respond to a new address.\n\nWhen the xHCI Debug capability is enabled and this bit is set, the\n\nDebug Target immediately enters the configured state without\n\nrequiring the Debug Host to send a SetAddress or SetConfig\n\nrequest.\n\nNote: You can set this bit for simulation purposes only. In the\n\nactual hardware, this bit must be set to 1'b0."]
pub type BypssetaddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTERFILTBYPASS` reader - Master Filter Bypass\n\nWhen this bit is set to 1'b1, all the filters are bypassed. The\n\ndouble synchronizers to mac_clk preceding the filters are also\n\nbypassed. For enabling the filters, this bit must be 1'b0."]
pub type MasterfiltbypassR = crate::BitReader;
#[doc = "Field `MASTERFILTBYPASS` writer - Master Filter Bypass\n\nWhen this bit is set to 1'b1, all the filters are bypassed. The\n\ndouble synchronizers to mac_clk preceding the filters are also\n\nbypassed. For enabling the filters, this bit must be 1'b0."]
pub type MasterfiltbypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRDNSCALE` reader - Power Down Scale (PwrDnScale)\n\nThe USB3 suspend_clk input replaces pipe3_rx_pclk as a clock\n\nsource to a small part of the USB3 core that operates when the\n\nSS PHY is in its lowest power (P3) state, and therefore does not\n\nprovide a clock.\n\nThe Power Down Scale field specifies how many suspend_clk\n\nperiods fit into a 16 kHz clock period. When performing the\n\ndivision, round up the remainder.\n\nFor example, when using an 8-bit/16-bit/32-bit PHY and 25-MHz\n\nSuspend clock,\n\nPower Down Scale = 25000 kHz/16 kHz = 13'd1563 (rounder up)\n\nNote:\n\nMinimum Suspend clock frequency is 32 kHz\n\nMaximum Suspend clock frequency is 125 MHz\n\nThe LTSSM uses Suspend clock for 12-ms and 100-ms timers\n\nduring suspend mode. According to the USB 3.0 specification, the\n\naccuracy on these timers is 0% to +50%.\n\n12 ms + 0~+50% accuracy = 18 ms (Range is 12 ms - 18 ms)\n\n100 ms + 0~+50% accuracy = 150 ms (Range is 100 ms -\n\n150ms).\n\nThe suspend clock accuracy requirement is:\n\n(12,000/62.5) * (GCTL\\[31:19\\]) * actual suspend_clk_period\n\nmust be between 12,000 and 18,000\n\n(100,0000/62.5) * (GCTL\\[31:19\\]) * actual suspend_clk_period\n\nmust be between 100,000 and 150,000\n\nFor example, if your suspend_clk frequency varies from 7.5 MHz\n\nto 10.5MHz, then the value needs to programmed is:\n\nPower Down Scale = 10500/16 = 657 (rounded up; and fastest\n\nfrequency used)."]
pub type PwrdnscaleR = crate::FieldReader<u16>;
#[doc = "Field `PWRDNSCALE` writer - Power Down Scale (PwrDnScale)\n\nThe USB3 suspend_clk input replaces pipe3_rx_pclk as a clock\n\nsource to a small part of the USB3 core that operates when the\n\nSS PHY is in its lowest power (P3) state, and therefore does not\n\nprovide a clock.\n\nThe Power Down Scale field specifies how many suspend_clk\n\nperiods fit into a 16 kHz clock period. When performing the\n\ndivision, round up the remainder.\n\nFor example, when using an 8-bit/16-bit/32-bit PHY and 25-MHz\n\nSuspend clock,\n\nPower Down Scale = 25000 kHz/16 kHz = 13'd1563 (rounder up)\n\nNote:\n\nMinimum Suspend clock frequency is 32 kHz\n\nMaximum Suspend clock frequency is 125 MHz\n\nThe LTSSM uses Suspend clock for 12-ms and 100-ms timers\n\nduring suspend mode. According to the USB 3.0 specification, the\n\naccuracy on these timers is 0% to +50%.\n\n12 ms + 0~+50% accuracy = 18 ms (Range is 12 ms - 18 ms)\n\n100 ms + 0~+50% accuracy = 150 ms (Range is 100 ms -\n\n150ms).\n\nThe suspend clock accuracy requirement is:\n\n(12,000/62.5) * (GCTL\\[31:19\\]) * actual suspend_clk_period\n\nmust be between 12,000 and 18,000\n\n(100,0000/62.5) * (GCTL\\[31:19\\]) * actual suspend_clk_period\n\nmust be between 100,000 and 150,000\n\nFor example, if your suspend_clk frequency varies from 7.5 MHz\n\nto 10.5MHz, then the value needs to programmed is:\n\nPower Down Scale = 10500/16 = 657 (rounded up; and fastest\n\nfrequency used)."]
pub type PwrdnscaleW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bit 0 - Disable Clock Gating (DsblClkGtng)\n\nThis bit is set to 1 and the core is in Low Power mode, internal\n\nclock gating is disabled. You can set this bit to 1'b1 after Power\n\nOn Reset."]
    #[inline(always)]
    pub fn dsblclkgtng(&self) -> DsblclkgtngR {
        DsblclkgtngR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GblHibernationEn\n\nThis bit enables hibernation at the global level. If hibernation is\n\nnot enabled through this bit, the PMU immediately accepts the\n\nD0->D3 and D3->D0 power state change requests, but does not\n\nsave or restore any core state. In addition, the PMUs never drive\n\nthe PHY interfaces and let the core continue to drive the PHY\n\ninterfaces."]
    #[inline(always)]
    pub fn gblhibernationen(&self) -> GblhibernationenR {
        GblhibernationenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - U2EXIT_LFPS\n\nIf this bit is:"]
    #[inline(always)]
    pub fn u2exit_lfps(&self) -> U2exitLfpsR {
        U2exitLfpsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable Scrambling (DisScramble)\n\nTransmit request to Link Partner on next transition to Recovery or\n\nPolling."]
    #[inline(always)]
    pub fn disscramble(&self) -> DisscrambleR {
        DisscrambleR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Scale-Down Mode (ScaleDown)\n\nWhen Scale-Down mode is enabled for simulation, the core uses\n\nscaled-down timing values, resulting in faster simulations.\n\nWhen Scale-Down mode is disabled, actual timing values are\n\nused. This is required for hardware operation.\n\nHS/FS/LS Modes:\n\n2'b00: Disables all scale-downs. Actual timing values are used.\n\n2'b01: Enables scale-down of all timing values except Device\n\nmode suspend and resume. These include Speed enumeration,\n\nHNP/SRP, and Host mode suspend and resume\n\n2'b10: Enables scale-down of Device mode suspend and resume\n\ntiming values only.\n\n2'b11: Enables bit 0 and bit 1 scale-down timing values.\n\nSS Mode:\n\n2'b00: Disables all scale-downs. Actual timing values are used.\n\n2'b01: Enables scaled down SS timing and repeat values\n\nincluding: (1) Number of TxEq training sequences reduce to 8;\n\n(2) LFPS polling burst time reduce to 256 nS; (3) LFPS warm\n\nreset receive reduce to 30 uS. 2'b10: No TxEq training sequences\n\nare sent. Overrides Bit 4.\n\n2'b11: Enables bit 0 and bit 1 scale-down timing values."]
    #[inline(always)]
    pub fn scaledown(&self) -> ScaledownR {
        ScaledownR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - RAM Clock Select (RAMClkSel)"]
    #[inline(always)]
    pub fn ramclksel(&self) -> RamclkselR {
        RamclkselR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Debug Attach\n\nWhen this bit is set:\n\n1. SS Link proceeds directly to the Polling link state (after\n\nRUN/STOP in the DCTL register is asserted) without checking\n\nremote termination;\n\n2. Link LFPS polling timeout is infinite;\n\n3. Polling timeout during TS1 is infinite (in case link is waiting for\n\nTXEQ to finish)."]
    #[inline(always)]
    pub fn debugattach(&self) -> DebugattachR {
        DebugattachR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable U1/U2 timer Scaledown (U1U2TimerScale).\n\nIf set to 1 along with GCTL\\[5:4\\]
(ScaleDown) = 2'bX1, disables\n\nthe scale down of U1/U2 inactive timer values. This is for\n\nsimulation mode only."]
    #[inline(always)]
    pub fn u1u2timerscale(&self) -> U1u2timerscaleR {
        U1u2timerscaleR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SOFITPSYNC\n\nIf this bit is set to 0 operating in host mode, the core keeps the\n\nUTMI/ULPI PHY on the first port in a non-suspended state\n\nwhenever there is a SuperSpeed port that is not in Rx.Detect,\n\nSS.Disable and U3.\n\nIf this bit is set to 1 operating in host mode, the core keeps the\n\nUTMI/ULPI PHY on the first port in a non-suspended state\n\nwhenever the other non-SuperSpeed ports are not in a\n\nsuspended state. This feature is useful because it saves power by\n\nsuspending UTMI/ULPI when SuperSpeed only is active, and it\n\nhelps resolve when the PHY does not transmit a host resume\n\nunless it is placed in suspend state. This bit must be programmed\n\nas a part of initialization at power-on reset, and must not be\n\ndynamically changed afterwards.\n\nNote:\n\nUSB2PHYCFGn\\[6\\].PhySusp eventually decides to put the\n\nUTMI/ULPI PHY in to suspend state. In addition, when this bit is\n\nset to 1, the core generates ITP from the ref_clk based counter.\n\nOtherwise, ITP and SOF are generated from utmi/ulpi_clk\\[0\\]\n\nbased counter. To program the reference clock period inside the\n\ncore, refer to GUCTL\\[31:22\\].REFCLKPER.\n\nThis feature is valid in Host and DRD/OTG configurations and\n\nused only in Host mode operation.\n\nIf you never use this feature or the\n\nGFLADJ.GFLADJ_REFCLK_LPM_SEL, the minimum frequency for\n\nthe ref_clk can be as low as 32KHz. You can connect the\n\nsuspend_clk (as low as 32 KHz) to the ref_clk.\n\nIf you plan to enable hardware-based LPM or software-based LPM\n\n(PORTPMSC. HLE=1), then you cannot use this feature.\n\nTurn off this feature by setting this bit to 0 and use the\n\nGFLADJ.GFLADJ_REFCLK_LPM_SEL feature.\n\nIf you set this bit to 1, the GUSB2PHYCFG.U2_FREECLK_EXISTS\n\nbit and the DWC_USB3_FREECLK_USB2_EXIST parameter must\n\nbe set to 0.\n\nProgram this bit to 0 if the core is intended to be operated in USB\n\n3.0 mode."]
    #[inline(always)]
    pub fn sofitpsync(&self) -> SofitpsyncR {
        SofitpsyncR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Core Soft Reset (CoreSoftReset)\n\n1'b0 - No soft reset;\n\n1'b1 - Soft reset to core\n\nClears the interrupts and all the CSRs except the following\n\nregisters:\n\nGCTL; GUCTL; GSTS; GSNPSID; GGPIO; GUID; GUSB2PHYCFGn\n\nregisters; GUSB3PIPECTLn registers; DCFG; DCTL; DEVTEN;\n\nDSTS.\n\nWhen you reset PHYs (using GUBS3PHYCFG or GUSB3PIPECTL\n\nregisters), you must keep the core in reset state until PHY clocks\n\nare stable. This controls the bus, ram, and mac domain resets.\n\nNote: This bit is for debug purposes only. Use USBCMD.HCRESET\n\nin xHCI Mode and DCTL.SoftReset in device mode for soft reset."]
    #[inline(always)]
    pub fn coresoftreset(&self) -> CoresoftresetR {
        CoresoftresetR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - PRTCAPDIR: Port Capability Direction (PrtCapDir)"]
    #[inline(always)]
    pub fn prtcapdir(&self) -> PrtcapdirR {
        PrtcapdirR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - FRMSCLDWN\n\nThis field scales down device view of a SOF/USOF/ITP duration.\n\nFor SS/HS mode:\n\nValue of 2'h3 implements interval to be 15.625 us\n\nValue of 2'h2 implements interval to be 31.25 us\n\nValue of 2'h1 implements interval to be 62.5 us\n\nValue of 2'h0 implements interval to be 125us\n\nFor FS mode, the scale-down value is multiplied by 8.\n\nWhen xHCI Debug Capability is enabled, this field also scales\n\ndown the MaxPacketSize of the IN and OUT bulk endpoint to\n\nallow more traffic during simulation. It can only be changed from\n\na non-zero\n\nvalue during simulation."]
    #[inline(always)]
    pub fn frmscldwn(&self) -> FrmscldwnR {
        FrmscldwnR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - U2RSTECN\n\nIf the SuperSpeed connection fails during POLL or LMP exchange,\n\nthe device connects at non-SS mode.\n\nIf this bit is set, then device attempts three more times to\n\nconnect at SS, even if it previously failed to operate in SS mode.\n\nFor each attempt, the device checks receiver termination eight\n\ntimes.\n\nFrom 2.60a release, this bit controls whether to check for\n\nRx.Detect eight times or one time for every attempt. Device\n\ncontroller on USB 2.0 reset checks for receiver termination eight\n\ntimes per attempt if\n\nthis bit is set to zero, or only once per attempt if the bit is set to\n\none.\n\nNote: This bit is applicable only in device mode."]
    #[inline(always)]
    pub fn u2rstecn(&self) -> U2rstecnR {
        U2rstecnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bypass SetAddress in Device Mode.\n\nWhen BYPSSETADDR bit is set, the device core uses the value in\n\nthe DCFG\\[DevAddr\\]
bits directly for comparing the device\n\naddress in the tokens.\n\nFor simulation, you can use this feature to avoid sending an\n\nactual SET ADDRESS control transfer on the USB, and make the\n\ndevice core respond to a new address.\n\nWhen the xHCI Debug capability is enabled and this bit is set, the\n\nDebug Target immediately enters the configured state without\n\nrequiring the Debug Host to send a SetAddress or SetConfig\n\nrequest.\n\nNote: You can set this bit for simulation purposes only. In the\n\nactual hardware, this bit must be set to 1'b0."]
    #[inline(always)]
    pub fn bypssetaddr(&self) -> BypssetaddrR {
        BypssetaddrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Master Filter Bypass\n\nWhen this bit is set to 1'b1, all the filters are bypassed. The\n\ndouble synchronizers to mac_clk preceding the filters are also\n\nbypassed. For enabling the filters, this bit must be 1'b0."]
    #[inline(always)]
    pub fn masterfiltbypass(&self) -> MasterfiltbypassR {
        MasterfiltbypassR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:31 - Power Down Scale (PwrDnScale)\n\nThe USB3 suspend_clk input replaces pipe3_rx_pclk as a clock\n\nsource to a small part of the USB3 core that operates when the\n\nSS PHY is in its lowest power (P3) state, and therefore does not\n\nprovide a clock.\n\nThe Power Down Scale field specifies how many suspend_clk\n\nperiods fit into a 16 kHz clock period. When performing the\n\ndivision, round up the remainder.\n\nFor example, when using an 8-bit/16-bit/32-bit PHY and 25-MHz\n\nSuspend clock,\n\nPower Down Scale = 25000 kHz/16 kHz = 13'd1563 (rounder up)\n\nNote:\n\nMinimum Suspend clock frequency is 32 kHz\n\nMaximum Suspend clock frequency is 125 MHz\n\nThe LTSSM uses Suspend clock for 12-ms and 100-ms timers\n\nduring suspend mode. According to the USB 3.0 specification, the\n\naccuracy on these timers is 0% to +50%.\n\n12 ms + 0~+50% accuracy = 18 ms (Range is 12 ms - 18 ms)\n\n100 ms + 0~+50% accuracy = 150 ms (Range is 100 ms -\n\n150ms).\n\nThe suspend clock accuracy requirement is:\n\n(12,000/62.5) * (GCTL\\[31:19\\]) * actual suspend_clk_period\n\nmust be between 12,000 and 18,000\n\n(100,0000/62.5) * (GCTL\\[31:19\\]) * actual suspend_clk_period\n\nmust be between 100,000 and 150,000\n\nFor example, if your suspend_clk frequency varies from 7.5 MHz\n\nto 10.5MHz, then the value needs to programmed is:\n\nPower Down Scale = 10500/16 = 657 (rounded up; and fastest\n\nfrequency used)."]
    #[inline(always)]
    pub fn pwrdnscale(&self) -> PwrdnscaleR {
        PwrdnscaleR::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Disable Clock Gating (DsblClkGtng)\n\nThis bit is set to 1 and the core is in Low Power mode, internal\n\nclock gating is disabled. You can set this bit to 1'b1 after Power\n\nOn Reset."]
    #[inline(always)]
    #[must_use]
    pub fn dsblclkgtng(&mut self) -> DsblclkgtngW<Usb3GctlSpec> {
        DsblclkgtngW::new(self, 0)
    }
    #[doc = "Bit 2 - U2EXIT_LFPS\n\nIf this bit is:"]
    #[inline(always)]
    #[must_use]
    pub fn u2exit_lfps(&mut self) -> U2exitLfpsW<Usb3GctlSpec> {
        U2exitLfpsW::new(self, 2)
    }
    #[doc = "Bit 3 - Disable Scrambling (DisScramble)\n\nTransmit request to Link Partner on next transition to Recovery or\n\nPolling."]
    #[inline(always)]
    #[must_use]
    pub fn disscramble(&mut self) -> DisscrambleW<Usb3GctlSpec> {
        DisscrambleW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Scale-Down Mode (ScaleDown)\n\nWhen Scale-Down mode is enabled for simulation, the core uses\n\nscaled-down timing values, resulting in faster simulations.\n\nWhen Scale-Down mode is disabled, actual timing values are\n\nused. This is required for hardware operation.\n\nHS/FS/LS Modes:\n\n2'b00: Disables all scale-downs. Actual timing values are used.\n\n2'b01: Enables scale-down of all timing values except Device\n\nmode suspend and resume. These include Speed enumeration,\n\nHNP/SRP, and Host mode suspend and resume\n\n2'b10: Enables scale-down of Device mode suspend and resume\n\ntiming values only.\n\n2'b11: Enables bit 0 and bit 1 scale-down timing values.\n\nSS Mode:\n\n2'b00: Disables all scale-downs. Actual timing values are used.\n\n2'b01: Enables scaled down SS timing and repeat values\n\nincluding: (1) Number of TxEq training sequences reduce to 8;\n\n(2) LFPS polling burst time reduce to 256 nS; (3) LFPS warm\n\nreset receive reduce to 30 uS. 2'b10: No TxEq training sequences\n\nare sent. Overrides Bit 4.\n\n2'b11: Enables bit 0 and bit 1 scale-down timing values."]
    #[inline(always)]
    #[must_use]
    pub fn scaledown(&mut self) -> ScaledownW<Usb3GctlSpec> {
        ScaledownW::new(self, 4)
    }
    #[doc = "Bits 6:7 - RAM Clock Select (RAMClkSel)"]
    #[inline(always)]
    #[must_use]
    pub fn ramclksel(&mut self) -> RamclkselW<Usb3GctlSpec> {
        RamclkselW::new(self, 6)
    }
    #[doc = "Bit 8 - Debug Attach\n\nWhen this bit is set:\n\n1. SS Link proceeds directly to the Polling link state (after\n\nRUN/STOP in the DCTL register is asserted) without checking\n\nremote termination;\n\n2. Link LFPS polling timeout is infinite;\n\n3. Polling timeout during TS1 is infinite (in case link is waiting for\n\nTXEQ to finish)."]
    #[inline(always)]
    #[must_use]
    pub fn debugattach(&mut self) -> DebugattachW<Usb3GctlSpec> {
        DebugattachW::new(self, 8)
    }
    #[doc = "Bit 9 - Disable U1/U2 timer Scaledown (U1U2TimerScale).\n\nIf set to 1 along with GCTL\\[5:4\\]
(ScaleDown) = 2'bX1, disables\n\nthe scale down of U1/U2 inactive timer values. This is for\n\nsimulation mode only."]
    #[inline(always)]
    #[must_use]
    pub fn u1u2timerscale(&mut self) -> U1u2timerscaleW<Usb3GctlSpec> {
        U1u2timerscaleW::new(self, 9)
    }
    #[doc = "Bit 10 - SOFITPSYNC\n\nIf this bit is set to 0 operating in host mode, the core keeps the\n\nUTMI/ULPI PHY on the first port in a non-suspended state\n\nwhenever there is a SuperSpeed port that is not in Rx.Detect,\n\nSS.Disable and U3.\n\nIf this bit is set to 1 operating in host mode, the core keeps the\n\nUTMI/ULPI PHY on the first port in a non-suspended state\n\nwhenever the other non-SuperSpeed ports are not in a\n\nsuspended state. This feature is useful because it saves power by\n\nsuspending UTMI/ULPI when SuperSpeed only is active, and it\n\nhelps resolve when the PHY does not transmit a host resume\n\nunless it is placed in suspend state. This bit must be programmed\n\nas a part of initialization at power-on reset, and must not be\n\ndynamically changed afterwards.\n\nNote:\n\nUSB2PHYCFGn\\[6\\].PhySusp eventually decides to put the\n\nUTMI/ULPI PHY in to suspend state. In addition, when this bit is\n\nset to 1, the core generates ITP from the ref_clk based counter.\n\nOtherwise, ITP and SOF are generated from utmi/ulpi_clk\\[0\\]\n\nbased counter. To program the reference clock period inside the\n\ncore, refer to GUCTL\\[31:22\\].REFCLKPER.\n\nThis feature is valid in Host and DRD/OTG configurations and\n\nused only in Host mode operation.\n\nIf you never use this feature or the\n\nGFLADJ.GFLADJ_REFCLK_LPM_SEL, the minimum frequency for\n\nthe ref_clk can be as low as 32KHz. You can connect the\n\nsuspend_clk (as low as 32 KHz) to the ref_clk.\n\nIf you plan to enable hardware-based LPM or software-based LPM\n\n(PORTPMSC. HLE=1), then you cannot use this feature.\n\nTurn off this feature by setting this bit to 0 and use the\n\nGFLADJ.GFLADJ_REFCLK_LPM_SEL feature.\n\nIf you set this bit to 1, the GUSB2PHYCFG.U2_FREECLK_EXISTS\n\nbit and the DWC_USB3_FREECLK_USB2_EXIST parameter must\n\nbe set to 0.\n\nProgram this bit to 0 if the core is intended to be operated in USB\n\n3.0 mode."]
    #[inline(always)]
    #[must_use]
    pub fn sofitpsync(&mut self) -> SofitpsyncW<Usb3GctlSpec> {
        SofitpsyncW::new(self, 10)
    }
    #[doc = "Bit 11 - Core Soft Reset (CoreSoftReset)\n\n1'b0 - No soft reset;\n\n1'b1 - Soft reset to core\n\nClears the interrupts and all the CSRs except the following\n\nregisters:\n\nGCTL; GUCTL; GSTS; GSNPSID; GGPIO; GUID; GUSB2PHYCFGn\n\nregisters; GUSB3PIPECTLn registers; DCFG; DCTL; DEVTEN;\n\nDSTS.\n\nWhen you reset PHYs (using GUBS3PHYCFG or GUSB3PIPECTL\n\nregisters), you must keep the core in reset state until PHY clocks\n\nare stable. This controls the bus, ram, and mac domain resets.\n\nNote: This bit is for debug purposes only. Use USBCMD.HCRESET\n\nin xHCI Mode and DCTL.SoftReset in device mode for soft reset."]
    #[inline(always)]
    #[must_use]
    pub fn coresoftreset(&mut self) -> CoresoftresetW<Usb3GctlSpec> {
        CoresoftresetW::new(self, 11)
    }
    #[doc = "Bits 12:13 - PRTCAPDIR: Port Capability Direction (PrtCapDir)"]
    #[inline(always)]
    #[must_use]
    pub fn prtcapdir(&mut self) -> PrtcapdirW<Usb3GctlSpec> {
        PrtcapdirW::new(self, 12)
    }
    #[doc = "Bits 14:15 - FRMSCLDWN\n\nThis field scales down device view of a SOF/USOF/ITP duration.\n\nFor SS/HS mode:\n\nValue of 2'h3 implements interval to be 15.625 us\n\nValue of 2'h2 implements interval to be 31.25 us\n\nValue of 2'h1 implements interval to be 62.5 us\n\nValue of 2'h0 implements interval to be 125us\n\nFor FS mode, the scale-down value is multiplied by 8.\n\nWhen xHCI Debug Capability is enabled, this field also scales\n\ndown the MaxPacketSize of the IN and OUT bulk endpoint to\n\nallow more traffic during simulation. It can only be changed from\n\na non-zero\n\nvalue during simulation."]
    #[inline(always)]
    #[must_use]
    pub fn frmscldwn(&mut self) -> FrmscldwnW<Usb3GctlSpec> {
        FrmscldwnW::new(self, 14)
    }
    #[doc = "Bit 16 - U2RSTECN\n\nIf the SuperSpeed connection fails during POLL or LMP exchange,\n\nthe device connects at non-SS mode.\n\nIf this bit is set, then device attempts three more times to\n\nconnect at SS, even if it previously failed to operate in SS mode.\n\nFor each attempt, the device checks receiver termination eight\n\ntimes.\n\nFrom 2.60a release, this bit controls whether to check for\n\nRx.Detect eight times or one time for every attempt. Device\n\ncontroller on USB 2.0 reset checks for receiver termination eight\n\ntimes per attempt if\n\nthis bit is set to zero, or only once per attempt if the bit is set to\n\none.\n\nNote: This bit is applicable only in device mode."]
    #[inline(always)]
    #[must_use]
    pub fn u2rstecn(&mut self) -> U2rstecnW<Usb3GctlSpec> {
        U2rstecnW::new(self, 16)
    }
    #[doc = "Bit 17 - Bypass SetAddress in Device Mode.\n\nWhen BYPSSETADDR bit is set, the device core uses the value in\n\nthe DCFG\\[DevAddr\\]
bits directly for comparing the device\n\naddress in the tokens.\n\nFor simulation, you can use this feature to avoid sending an\n\nactual SET ADDRESS control transfer on the USB, and make the\n\ndevice core respond to a new address.\n\nWhen the xHCI Debug capability is enabled and this bit is set, the\n\nDebug Target immediately enters the configured state without\n\nrequiring the Debug Host to send a SetAddress or SetConfig\n\nrequest.\n\nNote: You can set this bit for simulation purposes only. In the\n\nactual hardware, this bit must be set to 1'b0."]
    #[inline(always)]
    #[must_use]
    pub fn bypssetaddr(&mut self) -> BypssetaddrW<Usb3GctlSpec> {
        BypssetaddrW::new(self, 17)
    }
    #[doc = "Bit 18 - Master Filter Bypass\n\nWhen this bit is set to 1'b1, all the filters are bypassed. The\n\ndouble synchronizers to mac_clk preceding the filters are also\n\nbypassed. For enabling the filters, this bit must be 1'b0."]
    #[inline(always)]
    #[must_use]
    pub fn masterfiltbypass(&mut self) -> MasterfiltbypassW<Usb3GctlSpec> {
        MasterfiltbypassW::new(self, 18)
    }
    #[doc = "Bits 19:31 - Power Down Scale (PwrDnScale)\n\nThe USB3 suspend_clk input replaces pipe3_rx_pclk as a clock\n\nsource to a small part of the USB3 core that operates when the\n\nSS PHY is in its lowest power (P3) state, and therefore does not\n\nprovide a clock.\n\nThe Power Down Scale field specifies how many suspend_clk\n\nperiods fit into a 16 kHz clock period. When performing the\n\ndivision, round up the remainder.\n\nFor example, when using an 8-bit/16-bit/32-bit PHY and 25-MHz\n\nSuspend clock,\n\nPower Down Scale = 25000 kHz/16 kHz = 13'd1563 (rounder up)\n\nNote:\n\nMinimum Suspend clock frequency is 32 kHz\n\nMaximum Suspend clock frequency is 125 MHz\n\nThe LTSSM uses Suspend clock for 12-ms and 100-ms timers\n\nduring suspend mode. According to the USB 3.0 specification, the\n\naccuracy on these timers is 0% to +50%.\n\n12 ms + 0~+50% accuracy = 18 ms (Range is 12 ms - 18 ms)\n\n100 ms + 0~+50% accuracy = 150 ms (Range is 100 ms -\n\n150ms).\n\nThe suspend clock accuracy requirement is:\n\n(12,000/62.5) * (GCTL\\[31:19\\]) * actual suspend_clk_period\n\nmust be between 12,000 and 18,000\n\n(100,0000/62.5) * (GCTL\\[31:19\\]) * actual suspend_clk_period\n\nmust be between 100,000 and 150,000\n\nFor example, if your suspend_clk frequency varies from 7.5 MHz\n\nto 10.5MHz, then the value needs to programmed is:\n\nPower Down Scale = 10500/16 = 657 (rounded up; and fastest\n\nfrequency used)."]
    #[inline(always)]
    #[must_use]
    pub fn pwrdnscale(&mut self) -> PwrdnscaleW<Usb3GctlSpec> {
        PwrdnscaleW::new(self, 19)
    }
}
#[doc = "Global Core Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3GctlSpec;
impl crate::RegisterSpec for Usb3GctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_gctl::R`](R) reader structure"]
impl crate::Readable for Usb3GctlSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_gctl::W`](W) writer structure"]
impl crate::Writable for Usb3GctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_GCTL to value 0x30c1_2004"]
impl crate::Resettable for Usb3GctlSpec {
    const RESET_VALUE: u32 = 0x30c1_2004;
}
