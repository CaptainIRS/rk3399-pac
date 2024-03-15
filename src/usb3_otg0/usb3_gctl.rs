#[doc = "Register `USB3_GCTL` reader"]
pub type R = crate::R<Usb3GctlSpec>;
#[doc = "Register `USB3_GCTL` writer"]
pub type W = crate::W<Usb3GctlSpec>;
#[doc = "Field `DSBLCLKGTNG` reader - Disable Clock Gating (DsblClkGtng) This bit is set to 1 and the core is in Low Power mode, internal clock gating is disabled. You can set this bit to 1'b1 after Power On Reset."]
pub type DsblclkgtngR = crate::BitReader;
#[doc = "Field `DSBLCLKGTNG` writer - Disable Clock Gating (DsblClkGtng) This bit is set to 1 and the core is in Low Power mode, internal clock gating is disabled. You can set this bit to 1'b1 after Power On Reset."]
pub type DsblclkgtngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GBLHIBERNATIONEN` reader - GblHibernationEn This bit enables hibernation at the global level. If hibernation is not enabled through this bit, the PMU immediately accepts the D0->D3 and D3->D0 power state change requests, but does not save or restore any core state. In addition, the PMUs never drive the PHY interfaces and let the core continue to drive the PHY interfaces."]
pub type GblhibernationenR = crate::BitReader;
#[doc = "Field `U2EXIT_LFPS` reader - U2EXIT_LFPS If this bit is: 0: the link treats 248ns LFPS as a valid U2 exit. 1: the link waits for 8us of LFPS before it detects a valid U2 exit. This bit is added to improve interoperability with a third party host controller. This host controller in U2 state while performing receiver detection generates an LFPS glitch of about 4ms duration. This causes the device to exit from U2 state because the LFPS filter value is 248ns. With the new functionality enabled, the device can stay in U2 while ignoring this glitch from the host controller."]
pub type U2exitLfpsR = crate::BitReader;
#[doc = "Field `U2EXIT_LFPS` writer - U2EXIT_LFPS If this bit is: 0: the link treats 248ns LFPS as a valid U2 exit. 1: the link waits for 8us of LFPS before it detects a valid U2 exit. This bit is added to improve interoperability with a third party host controller. This host controller in U2 state while performing receiver detection generates an LFPS glitch of about 4ms duration. This causes the device to exit from U2 state because the LFPS filter value is 248ns. With the new functionality enabled, the device can stay in U2 while ignoring this glitch from the host controller."]
pub type U2exitLfpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISSCRAMBLE` reader - Disable Scrambling (DisScramble) Transmit request to Link Partner on next transition to Recovery or Polling."]
pub type DisscrambleR = crate::BitReader;
#[doc = "Field `DISSCRAMBLE` writer - Disable Scrambling (DisScramble) Transmit request to Link Partner on next transition to Recovery or Polling."]
pub type DisscrambleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCALEDOWN` reader - Scale-Down Mode (ScaleDown) When Scale-Down mode is enabled for simulation, the core uses scaled-down timing values, resulting in faster simulations. When Scale-Down mode is disabled, actual timing values are used. This is required for hardware operation. HS/FS/LS Modes: 2'b00: Disables all scale-downs. Actual timing values are used. 2'b01: Enables scale-down of all timing values except Device mode suspend and resume. These include Speed enumeration, HNP/SRP, and Host mode suspend and resume 2'b10: Enables scale-down of Device mode suspend and resume timing values only. 2'b11: Enables bit 0 and bit 1 scale-down timing values. SS Mode: 2'b00: Disables all scale-downs. Actual timing values are used. 2'b01: Enables scaled down SS timing and repeat values including: (1) Number of TxEq training sequences reduce to 8; (2) LFPS polling burst time reduce to 256 nS; (3) LFPS warm reset receive reduce to 30 uS. 2'b10: No TxEq training sequences are sent. Overrides Bit 4. 2'b11: Enables bit 0 and bit 1 scale-down timing values."]
pub type ScaledownR = crate::FieldReader;
#[doc = "Field `SCALEDOWN` writer - Scale-Down Mode (ScaleDown) When Scale-Down mode is enabled for simulation, the core uses scaled-down timing values, resulting in faster simulations. When Scale-Down mode is disabled, actual timing values are used. This is required for hardware operation. HS/FS/LS Modes: 2'b00: Disables all scale-downs. Actual timing values are used. 2'b01: Enables scale-down of all timing values except Device mode suspend and resume. These include Speed enumeration, HNP/SRP, and Host mode suspend and resume 2'b10: Enables scale-down of Device mode suspend and resume timing values only. 2'b11: Enables bit 0 and bit 1 scale-down timing values. SS Mode: 2'b00: Disables all scale-downs. Actual timing values are used. 2'b01: Enables scaled down SS timing and repeat values including: (1) Number of TxEq training sequences reduce to 8; (2) LFPS polling burst time reduce to 256 nS; (3) LFPS warm reset receive reduce to 30 uS. 2'b10: No TxEq training sequences are sent. Overrides Bit 4. 2'b11: Enables bit 0 and bit 1 scale-down timing values."]
pub type ScaledownW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RAMCLKSEL` reader - RAM Clock Select (RAMClkSel) 2'b00: bus clock 2'b01: pipe clock (Only used in device mode) 2'b10: In device mode, pipe/2 clock. In Host mode, controller switches ram_clk between pipe/2 clock, mac2_clk and bus_clk based on the status of the U2/U3 ports 2'b11: In device mode, selects mac2_clk as ram_clk (when 8-bit UTMI or ULPI used. Not supported in 16-bit UTMI mode); In Host mode, controller switches ram_clk between pipe_clk, mac2_clk and bus_clk based on the status of the U2/U3 ports. In device mode, upon a USB reset and USB disconnect, the hardware clears these bits to 2'b00."]
pub type RamclkselR = crate::FieldReader;
#[doc = "Field `RAMCLKSEL` writer - RAM Clock Select (RAMClkSel) 2'b00: bus clock 2'b01: pipe clock (Only used in device mode) 2'b10: In device mode, pipe/2 clock. In Host mode, controller switches ram_clk between pipe/2 clock, mac2_clk and bus_clk based on the status of the U2/U3 ports 2'b11: In device mode, selects mac2_clk as ram_clk (when 8-bit UTMI or ULPI used. Not supported in 16-bit UTMI mode); In Host mode, controller switches ram_clk between pipe_clk, mac2_clk and bus_clk based on the status of the U2/U3 ports. In device mode, upon a USB reset and USB disconnect, the hardware clears these bits to 2'b00."]
pub type RamclkselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DEBUGATTACH` reader - Debug Attach When this bit is set: 1. SS Link proceeds directly to the Polling link state (after RUN/STOP in the DCTL register is asserted) without checking remote termination; 2. Link LFPS polling timeout is infinite; 3. Polling timeout during TS1 is infinite (in case link is waiting for TXEQ to finish)."]
pub type DebugattachR = crate::BitReader;
#[doc = "Field `DEBUGATTACH` writer - Debug Attach When this bit is set: 1. SS Link proceeds directly to the Polling link state (after RUN/STOP in the DCTL register is asserted) without checking remote termination; 2. Link LFPS polling timeout is infinite; 3. Polling timeout during TS1 is infinite (in case link is waiting for TXEQ to finish)."]
pub type DebugattachW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `U1U2TIMERSCALE` reader - Disable U1/U2 timer Scaledown (U1U2TimerScale). If set to 1 along with GCTL\\[5:4\\]
(ScaleDown) = 2'bX1, disables the scale down of U1/U2 inactive timer values. This is for simulation mode only."]
pub type U1u2timerscaleR = crate::BitReader;
#[doc = "Field `U1U2TIMERSCALE` writer - Disable U1/U2 timer Scaledown (U1U2TimerScale). If set to 1 along with GCTL\\[5:4\\]
(ScaleDown) = 2'bX1, disables the scale down of U1/U2 inactive timer values. This is for simulation mode only."]
pub type U1u2timerscaleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFITPSYNC` reader - SOFITPSYNC If this bit is set to 0 operating in host mode, the core keeps the UTMI/ULPI PHY on the first port in a non-suspended state whenever there is a SuperSpeed port that is not in Rx.Detect, SS.Disable and U3. If this bit is set to 1 operating in host mode, the core keeps the UTMI/ULPI PHY on the first port in a non-suspended state whenever the other non-SuperSpeed ports are not in a suspended state. This feature is useful because it saves power by suspending UTMI/ULPI when SuperSpeed only is active, and it helps resolve when the PHY does not transmit a host resume unless it is placed in suspend state. This bit must be programmed as a part of initialization at power-on reset, and must not be dynamically changed afterwards. Note: USB2PHYCFGn\\[6\\].PhySusp eventually decides to put the UTMI/ULPI PHY in to suspend state. In addition, when this bit is set to 1, the core generates ITP from the ref_clk based counter. Otherwise, ITP and SOF are generated from utmi/ulpi_clk\\[0\\]
based counter. To program the reference clock period inside the core, refer to GUCTL\\[31:22\\].REFCLKPER. This feature is valid in Host and DRD/OTG configurations and used only in Host mode operation. If you never use this feature or the GFLADJ.GFLADJ_REFCLK_LPM_SEL, the minimum frequency for the ref_clk can be as low as 32KHz. You can connect the suspend_clk (as low as 32 KHz) to the ref_clk. If you plan to enable hardware-based LPM or software-based LPM (PORTPMSC. HLE=1), then you cannot use this feature. Turn off this feature by setting this bit to 0 and use the GFLADJ.GFLADJ_REFCLK_LPM_SEL feature. If you set this bit to 1, the GUSB2PHYCFG.U2_FREECLK_EXISTS bit and the DWC_USB3_FREECLK_USB2_EXIST parameter must be set to 0. Program this bit to 0 if the core is intended to be operated in USB 3.0 mode."]
pub type SofitpsyncR = crate::BitReader;
#[doc = "Field `SOFITPSYNC` writer - SOFITPSYNC If this bit is set to 0 operating in host mode, the core keeps the UTMI/ULPI PHY on the first port in a non-suspended state whenever there is a SuperSpeed port that is not in Rx.Detect, SS.Disable and U3. If this bit is set to 1 operating in host mode, the core keeps the UTMI/ULPI PHY on the first port in a non-suspended state whenever the other non-SuperSpeed ports are not in a suspended state. This feature is useful because it saves power by suspending UTMI/ULPI when SuperSpeed only is active, and it helps resolve when the PHY does not transmit a host resume unless it is placed in suspend state. This bit must be programmed as a part of initialization at power-on reset, and must not be dynamically changed afterwards. Note: USB2PHYCFGn\\[6\\].PhySusp eventually decides to put the UTMI/ULPI PHY in to suspend state. In addition, when this bit is set to 1, the core generates ITP from the ref_clk based counter. Otherwise, ITP and SOF are generated from utmi/ulpi_clk\\[0\\]
based counter. To program the reference clock period inside the core, refer to GUCTL\\[31:22\\].REFCLKPER. This feature is valid in Host and DRD/OTG configurations and used only in Host mode operation. If you never use this feature or the GFLADJ.GFLADJ_REFCLK_LPM_SEL, the minimum frequency for the ref_clk can be as low as 32KHz. You can connect the suspend_clk (as low as 32 KHz) to the ref_clk. If you plan to enable hardware-based LPM or software-based LPM (PORTPMSC. HLE=1), then you cannot use this feature. Turn off this feature by setting this bit to 0 and use the GFLADJ.GFLADJ_REFCLK_LPM_SEL feature. If you set this bit to 1, the GUSB2PHYCFG.U2_FREECLK_EXISTS bit and the DWC_USB3_FREECLK_USB2_EXIST parameter must be set to 0. Program this bit to 0 if the core is intended to be operated in USB 3.0 mode."]
pub type SofitpsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORESOFTRESET` reader - Core Soft Reset (CoreSoftReset) 1'b0 - No soft reset; 1'b1 - Soft reset to core Clears the interrupts and all the CSRs except the following registers: GCTL; GUCTL; GSTS; GSNPSID; GGPIO; GUID; GUSB2PHYCFGn registers; GUSB3PIPECTLn registers; DCFG; DCTL; DEVTEN; DSTS. When you reset PHYs (using GUBS3PHYCFG or GUSB3PIPECTL registers), you must keep the core in reset state until PHY clocks are stable. This controls the bus, ram, and mac domain resets. Note: This bit is for debug purposes only. Use USBCMD.HCRESET in xHCI Mode and DCTL.SoftReset in device mode for soft reset."]
pub type CoresoftresetR = crate::BitReader;
#[doc = "Field `CORESOFTRESET` writer - Core Soft Reset (CoreSoftReset) 1'b0 - No soft reset; 1'b1 - Soft reset to core Clears the interrupts and all the CSRs except the following registers: GCTL; GUCTL; GSTS; GSNPSID; GGPIO; GUID; GUSB2PHYCFGn registers; GUSB3PIPECTLn registers; DCFG; DCTL; DEVTEN; DSTS. When you reset PHYs (using GUBS3PHYCFG or GUSB3PIPECTL registers), you must keep the core in reset state until PHY clocks are stable. This controls the bus, ram, and mac domain resets. Note: This bit is for debug purposes only. Use USBCMD.HCRESET in xHCI Mode and DCTL.SoftReset in device mode for soft reset."]
pub type CoresoftresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTCAPDIR` reader - PRTCAPDIR: Port Capability Direction (PrtCapDir) 2'b01: for Host configurations 2'b10: for Device configurations SW should base on IDDIG input to set usb3 controller as an OTG 2.0/3.0 device with A-device or B-device."]
pub type PrtcapdirR = crate::FieldReader;
#[doc = "Field `PRTCAPDIR` writer - PRTCAPDIR: Port Capability Direction (PrtCapDir) 2'b01: for Host configurations 2'b10: for Device configurations SW should base on IDDIG input to set usb3 controller as an OTG 2.0/3.0 device with A-device or B-device."]
pub type PrtcapdirW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FRMSCLDWN` reader - FRMSCLDWN This field scales down device view of a SOF/USOF/ITP duration. For SS/HS mode: Value of 2'h3 implements interval to be 15.625 us Value of 2'h2 implements interval to be 31.25 us Value of 2'h1 implements interval to be 62.5 us Value of 2'h0 implements interval to be 125us For FS mode, the scale-down value is multiplied by 8. When xHCI Debug Capability is enabled, this field also scales down the MaxPacketSize of the IN and OUT bulk endpoint to allow more traffic during simulation. It can only be changed from a non-zero value during simulation. 2'h0: 1024 bytes 2'h1: 512 bytes 2'h2: 256 bytes 2'h3: 128 bytes"]
pub type FrmscldwnR = crate::FieldReader;
#[doc = "Field `FRMSCLDWN` writer - FRMSCLDWN This field scales down device view of a SOF/USOF/ITP duration. For SS/HS mode: Value of 2'h3 implements interval to be 15.625 us Value of 2'h2 implements interval to be 31.25 us Value of 2'h1 implements interval to be 62.5 us Value of 2'h0 implements interval to be 125us For FS mode, the scale-down value is multiplied by 8. When xHCI Debug Capability is enabled, this field also scales down the MaxPacketSize of the IN and OUT bulk endpoint to allow more traffic during simulation. It can only be changed from a non-zero value during simulation. 2'h0: 1024 bytes 2'h1: 512 bytes 2'h2: 256 bytes 2'h3: 128 bytes"]
pub type FrmscldwnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `U2RSTECN` reader - U2RSTECN If the SuperSpeed connection fails during POLL or LMP exchange, the device connects at non-SS mode. If this bit is set, then device attempts three more times to connect at SS, even if it previously failed to operate in SS mode. For each attempt, the device checks receiver termination eight times. From 2.60a release, this bit controls whether to check for Rx.Detect eight times or one time for every attempt. Device controller on USB 2.0 reset checks for receiver termination eight times per attempt if this bit is set to zero, or only once per attempt if the bit is set to one. Note: This bit is applicable only in device mode."]
pub type U2rstecnR = crate::BitReader;
#[doc = "Field `U2RSTECN` writer - U2RSTECN If the SuperSpeed connection fails during POLL or LMP exchange, the device connects at non-SS mode. If this bit is set, then device attempts three more times to connect at SS, even if it previously failed to operate in SS mode. For each attempt, the device checks receiver termination eight times. From 2.60a release, this bit controls whether to check for Rx.Detect eight times or one time for every attempt. Device controller on USB 2.0 reset checks for receiver termination eight times per attempt if this bit is set to zero, or only once per attempt if the bit is set to one. Note: This bit is applicable only in device mode."]
pub type U2rstecnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPSSETADDR` reader - Bypass SetAddress in Device Mode. When BYPSSETADDR bit is set, the device core uses the value in the DCFG\\[DevAddr\\]
bits directly for comparing the device address in the tokens. For simulation, you can use this feature to avoid sending an actual SET ADDRESS control transfer on the USB, and make the device core respond to a new address. When the xHCI Debug capability is enabled and this bit is set, the Debug Target immediately enters the configured state without requiring the Debug Host to send a SetAddress or SetConfig request. Note: You can set this bit for simulation purposes only. In the actual hardware, this bit must be set to 1'b0."]
pub type BypssetaddrR = crate::BitReader;
#[doc = "Field `BYPSSETADDR` writer - Bypass SetAddress in Device Mode. When BYPSSETADDR bit is set, the device core uses the value in the DCFG\\[DevAddr\\]
bits directly for comparing the device address in the tokens. For simulation, you can use this feature to avoid sending an actual SET ADDRESS control transfer on the USB, and make the device core respond to a new address. When the xHCI Debug capability is enabled and this bit is set, the Debug Target immediately enters the configured state without requiring the Debug Host to send a SetAddress or SetConfig request. Note: You can set this bit for simulation purposes only. In the actual hardware, this bit must be set to 1'b0."]
pub type BypssetaddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTERFILTBYPASS` reader - Master Filter Bypass When this bit is set to 1'b1, all the filters are bypassed. The double synchronizers to mac_clk preceding the filters are also bypassed. For enabling the filters, this bit must be 1'b0."]
pub type MasterfiltbypassR = crate::BitReader;
#[doc = "Field `MASTERFILTBYPASS` writer - Master Filter Bypass When this bit is set to 1'b1, all the filters are bypassed. The double synchronizers to mac_clk preceding the filters are also bypassed. For enabling the filters, this bit must be 1'b0."]
pub type MasterfiltbypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRDNSCALE` reader - Power Down Scale (PwrDnScale) The USB3 suspend_clk input replaces pipe3_rx_pclk as a clock source to a small part of the USB3 core that operates when the SS PHY is in its lowest power (P3) state, and therefore does not provide a clock. The Power Down Scale field specifies how many suspend_clk periods fit into a 16 kHz clock period. When performing the division, round up the remainder. For example, when using an 8-bit/16-bit/32-bit PHY and 25-MHz Suspend clock, Power Down Scale = 25000 kHz/16 kHz = 13'd1563 (rounder up) Note: Minimum Suspend clock frequency is 32 kHz Maximum Suspend clock frequency is 125 MHz The LTSSM uses Suspend clock for 12-ms and 100-ms timers during suspend mode. According to the USB 3.0 specification, the accuracy on these timers is 0% to +50%. 12 ms + 0~+50% accuracy = 18 ms (Range is 12 ms - 18 ms) 100 ms + 0~+50% accuracy = 150 ms (Range is 100 ms - 150ms). The suspend clock accuracy requirement is: (12,000/62.5) * (GCTL\\[31:19\\]) * actual suspend_clk_period must be between 12,000 and 18,000 (100,0000/62.5) * (GCTL\\[31:19\\]) * actual suspend_clk_period must be between 100,000 and 150,000 For example, if your suspend_clk frequency varies from 7.5 MHz to 10.5MHz, then the value needs to programmed is: Power Down Scale = 10500/16 = 657 (rounded up; and fastest frequency used)."]
pub type PwrdnscaleR = crate::FieldReader<u16>;
#[doc = "Field `PWRDNSCALE` writer - Power Down Scale (PwrDnScale) The USB3 suspend_clk input replaces pipe3_rx_pclk as a clock source to a small part of the USB3 core that operates when the SS PHY is in its lowest power (P3) state, and therefore does not provide a clock. The Power Down Scale field specifies how many suspend_clk periods fit into a 16 kHz clock period. When performing the division, round up the remainder. For example, when using an 8-bit/16-bit/32-bit PHY and 25-MHz Suspend clock, Power Down Scale = 25000 kHz/16 kHz = 13'd1563 (rounder up) Note: Minimum Suspend clock frequency is 32 kHz Maximum Suspend clock frequency is 125 MHz The LTSSM uses Suspend clock for 12-ms and 100-ms timers during suspend mode. According to the USB 3.0 specification, the accuracy on these timers is 0% to +50%. 12 ms + 0~+50% accuracy = 18 ms (Range is 12 ms - 18 ms) 100 ms + 0~+50% accuracy = 150 ms (Range is 100 ms - 150ms). The suspend clock accuracy requirement is: (12,000/62.5) * (GCTL\\[31:19\\]) * actual suspend_clk_period must be between 12,000 and 18,000 (100,0000/62.5) * (GCTL\\[31:19\\]) * actual suspend_clk_period must be between 100,000 and 150,000 For example, if your suspend_clk frequency varies from 7.5 MHz to 10.5MHz, then the value needs to programmed is: Power Down Scale = 10500/16 = 657 (rounded up; and fastest frequency used)."]
pub type PwrdnscaleW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bit 0 - Disable Clock Gating (DsblClkGtng) This bit is set to 1 and the core is in Low Power mode, internal clock gating is disabled. You can set this bit to 1'b1 after Power On Reset."]
    #[inline(always)]
    pub fn dsblclkgtng(&self) -> DsblclkgtngR {
        DsblclkgtngR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GblHibernationEn This bit enables hibernation at the global level. If hibernation is not enabled through this bit, the PMU immediately accepts the D0->D3 and D3->D0 power state change requests, but does not save or restore any core state. In addition, the PMUs never drive the PHY interfaces and let the core continue to drive the PHY interfaces."]
    #[inline(always)]
    pub fn gblhibernationen(&self) -> GblhibernationenR {
        GblhibernationenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - U2EXIT_LFPS If this bit is: 0: the link treats 248ns LFPS as a valid U2 exit. 1: the link waits for 8us of LFPS before it detects a valid U2 exit. This bit is added to improve interoperability with a third party host controller. This host controller in U2 state while performing receiver detection generates an LFPS glitch of about 4ms duration. This causes the device to exit from U2 state because the LFPS filter value is 248ns. With the new functionality enabled, the device can stay in U2 while ignoring this glitch from the host controller."]
    #[inline(always)]
    pub fn u2exit_lfps(&self) -> U2exitLfpsR {
        U2exitLfpsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable Scrambling (DisScramble) Transmit request to Link Partner on next transition to Recovery or Polling."]
    #[inline(always)]
    pub fn disscramble(&self) -> DisscrambleR {
        DisscrambleR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Scale-Down Mode (ScaleDown) When Scale-Down mode is enabled for simulation, the core uses scaled-down timing values, resulting in faster simulations. When Scale-Down mode is disabled, actual timing values are used. This is required for hardware operation. HS/FS/LS Modes: 2'b00: Disables all scale-downs. Actual timing values are used. 2'b01: Enables scale-down of all timing values except Device mode suspend and resume. These include Speed enumeration, HNP/SRP, and Host mode suspend and resume 2'b10: Enables scale-down of Device mode suspend and resume timing values only. 2'b11: Enables bit 0 and bit 1 scale-down timing values. SS Mode: 2'b00: Disables all scale-downs. Actual timing values are used. 2'b01: Enables scaled down SS timing and repeat values including: (1) Number of TxEq training sequences reduce to 8; (2) LFPS polling burst time reduce to 256 nS; (3) LFPS warm reset receive reduce to 30 uS. 2'b10: No TxEq training sequences are sent. Overrides Bit 4. 2'b11: Enables bit 0 and bit 1 scale-down timing values."]
    #[inline(always)]
    pub fn scaledown(&self) -> ScaledownR {
        ScaledownR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - RAM Clock Select (RAMClkSel) 2'b00: bus clock 2'b01: pipe clock (Only used in device mode) 2'b10: In device mode, pipe/2 clock. In Host mode, controller switches ram_clk between pipe/2 clock, mac2_clk and bus_clk based on the status of the U2/U3 ports 2'b11: In device mode, selects mac2_clk as ram_clk (when 8-bit UTMI or ULPI used. Not supported in 16-bit UTMI mode); In Host mode, controller switches ram_clk between pipe_clk, mac2_clk and bus_clk based on the status of the U2/U3 ports. In device mode, upon a USB reset and USB disconnect, the hardware clears these bits to 2'b00."]
    #[inline(always)]
    pub fn ramclksel(&self) -> RamclkselR {
        RamclkselR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Debug Attach When this bit is set: 1. SS Link proceeds directly to the Polling link state (after RUN/STOP in the DCTL register is asserted) without checking remote termination; 2. Link LFPS polling timeout is infinite; 3. Polling timeout during TS1 is infinite (in case link is waiting for TXEQ to finish)."]
    #[inline(always)]
    pub fn debugattach(&self) -> DebugattachR {
        DebugattachR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable U1/U2 timer Scaledown (U1U2TimerScale). If set to 1 along with GCTL\\[5:4\\]
(ScaleDown) = 2'bX1, disables the scale down of U1/U2 inactive timer values. This is for simulation mode only."]
    #[inline(always)]
    pub fn u1u2timerscale(&self) -> U1u2timerscaleR {
        U1u2timerscaleR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SOFITPSYNC If this bit is set to 0 operating in host mode, the core keeps the UTMI/ULPI PHY on the first port in a non-suspended state whenever there is a SuperSpeed port that is not in Rx.Detect, SS.Disable and U3. If this bit is set to 1 operating in host mode, the core keeps the UTMI/ULPI PHY on the first port in a non-suspended state whenever the other non-SuperSpeed ports are not in a suspended state. This feature is useful because it saves power by suspending UTMI/ULPI when SuperSpeed only is active, and it helps resolve when the PHY does not transmit a host resume unless it is placed in suspend state. This bit must be programmed as a part of initialization at power-on reset, and must not be dynamically changed afterwards. Note: USB2PHYCFGn\\[6\\].PhySusp eventually decides to put the UTMI/ULPI PHY in to suspend state. In addition, when this bit is set to 1, the core generates ITP from the ref_clk based counter. Otherwise, ITP and SOF are generated from utmi/ulpi_clk\\[0\\]
based counter. To program the reference clock period inside the core, refer to GUCTL\\[31:22\\].REFCLKPER. This feature is valid in Host and DRD/OTG configurations and used only in Host mode operation. If you never use this feature or the GFLADJ.GFLADJ_REFCLK_LPM_SEL, the minimum frequency for the ref_clk can be as low as 32KHz. You can connect the suspend_clk (as low as 32 KHz) to the ref_clk. If you plan to enable hardware-based LPM or software-based LPM (PORTPMSC. HLE=1), then you cannot use this feature. Turn off this feature by setting this bit to 0 and use the GFLADJ.GFLADJ_REFCLK_LPM_SEL feature. If you set this bit to 1, the GUSB2PHYCFG.U2_FREECLK_EXISTS bit and the DWC_USB3_FREECLK_USB2_EXIST parameter must be set to 0. Program this bit to 0 if the core is intended to be operated in USB 3.0 mode."]
    #[inline(always)]
    pub fn sofitpsync(&self) -> SofitpsyncR {
        SofitpsyncR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Core Soft Reset (CoreSoftReset) 1'b0 - No soft reset; 1'b1 - Soft reset to core Clears the interrupts and all the CSRs except the following registers: GCTL; GUCTL; GSTS; GSNPSID; GGPIO; GUID; GUSB2PHYCFGn registers; GUSB3PIPECTLn registers; DCFG; DCTL; DEVTEN; DSTS. When you reset PHYs (using GUBS3PHYCFG or GUSB3PIPECTL registers), you must keep the core in reset state until PHY clocks are stable. This controls the bus, ram, and mac domain resets. Note: This bit is for debug purposes only. Use USBCMD.HCRESET in xHCI Mode and DCTL.SoftReset in device mode for soft reset."]
    #[inline(always)]
    pub fn coresoftreset(&self) -> CoresoftresetR {
        CoresoftresetR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - PRTCAPDIR: Port Capability Direction (PrtCapDir) 2'b01: for Host configurations 2'b10: for Device configurations SW should base on IDDIG input to set usb3 controller as an OTG 2.0/3.0 device with A-device or B-device."]
    #[inline(always)]
    pub fn prtcapdir(&self) -> PrtcapdirR {
        PrtcapdirR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - FRMSCLDWN This field scales down device view of a SOF/USOF/ITP duration. For SS/HS mode: Value of 2'h3 implements interval to be 15.625 us Value of 2'h2 implements interval to be 31.25 us Value of 2'h1 implements interval to be 62.5 us Value of 2'h0 implements interval to be 125us For FS mode, the scale-down value is multiplied by 8. When xHCI Debug Capability is enabled, this field also scales down the MaxPacketSize of the IN and OUT bulk endpoint to allow more traffic during simulation. It can only be changed from a non-zero value during simulation. 2'h0: 1024 bytes 2'h1: 512 bytes 2'h2: 256 bytes 2'h3: 128 bytes"]
    #[inline(always)]
    pub fn frmscldwn(&self) -> FrmscldwnR {
        FrmscldwnR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - U2RSTECN If the SuperSpeed connection fails during POLL or LMP exchange, the device connects at non-SS mode. If this bit is set, then device attempts three more times to connect at SS, even if it previously failed to operate in SS mode. For each attempt, the device checks receiver termination eight times. From 2.60a release, this bit controls whether to check for Rx.Detect eight times or one time for every attempt. Device controller on USB 2.0 reset checks for receiver termination eight times per attempt if this bit is set to zero, or only once per attempt if the bit is set to one. Note: This bit is applicable only in device mode."]
    #[inline(always)]
    pub fn u2rstecn(&self) -> U2rstecnR {
        U2rstecnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bypass SetAddress in Device Mode. When BYPSSETADDR bit is set, the device core uses the value in the DCFG\\[DevAddr\\]
bits directly for comparing the device address in the tokens. For simulation, you can use this feature to avoid sending an actual SET ADDRESS control transfer on the USB, and make the device core respond to a new address. When the xHCI Debug capability is enabled and this bit is set, the Debug Target immediately enters the configured state without requiring the Debug Host to send a SetAddress or SetConfig request. Note: You can set this bit for simulation purposes only. In the actual hardware, this bit must be set to 1'b0."]
    #[inline(always)]
    pub fn bypssetaddr(&self) -> BypssetaddrR {
        BypssetaddrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Master Filter Bypass When this bit is set to 1'b1, all the filters are bypassed. The double synchronizers to mac_clk preceding the filters are also bypassed. For enabling the filters, this bit must be 1'b0."]
    #[inline(always)]
    pub fn masterfiltbypass(&self) -> MasterfiltbypassR {
        MasterfiltbypassR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:31 - Power Down Scale (PwrDnScale) The USB3 suspend_clk input replaces pipe3_rx_pclk as a clock source to a small part of the USB3 core that operates when the SS PHY is in its lowest power (P3) state, and therefore does not provide a clock. The Power Down Scale field specifies how many suspend_clk periods fit into a 16 kHz clock period. When performing the division, round up the remainder. For example, when using an 8-bit/16-bit/32-bit PHY and 25-MHz Suspend clock, Power Down Scale = 25000 kHz/16 kHz = 13'd1563 (rounder up) Note: Minimum Suspend clock frequency is 32 kHz Maximum Suspend clock frequency is 125 MHz The LTSSM uses Suspend clock for 12-ms and 100-ms timers during suspend mode. According to the USB 3.0 specification, the accuracy on these timers is 0% to +50%. 12 ms + 0~+50% accuracy = 18 ms (Range is 12 ms - 18 ms) 100 ms + 0~+50% accuracy = 150 ms (Range is 100 ms - 150ms). The suspend clock accuracy requirement is: (12,000/62.5) * (GCTL\\[31:19\\]) * actual suspend_clk_period must be between 12,000 and 18,000 (100,0000/62.5) * (GCTL\\[31:19\\]) * actual suspend_clk_period must be between 100,000 and 150,000 For example, if your suspend_clk frequency varies from 7.5 MHz to 10.5MHz, then the value needs to programmed is: Power Down Scale = 10500/16 = 657 (rounded up; and fastest frequency used)."]
    #[inline(always)]
    pub fn pwrdnscale(&self) -> PwrdnscaleR {
        PwrdnscaleR::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Disable Clock Gating (DsblClkGtng) This bit is set to 1 and the core is in Low Power mode, internal clock gating is disabled. You can set this bit to 1'b1 after Power On Reset."]
    #[inline(always)]
    #[must_use]
    pub fn dsblclkgtng(&mut self) -> DsblclkgtngW<Usb3GctlSpec> {
        DsblclkgtngW::new(self, 0)
    }
    #[doc = "Bit 2 - U2EXIT_LFPS If this bit is: 0: the link treats 248ns LFPS as a valid U2 exit. 1: the link waits for 8us of LFPS before it detects a valid U2 exit. This bit is added to improve interoperability with a third party host controller. This host controller in U2 state while performing receiver detection generates an LFPS glitch of about 4ms duration. This causes the device to exit from U2 state because the LFPS filter value is 248ns. With the new functionality enabled, the device can stay in U2 while ignoring this glitch from the host controller."]
    #[inline(always)]
    #[must_use]
    pub fn u2exit_lfps(&mut self) -> U2exitLfpsW<Usb3GctlSpec> {
        U2exitLfpsW::new(self, 2)
    }
    #[doc = "Bit 3 - Disable Scrambling (DisScramble) Transmit request to Link Partner on next transition to Recovery or Polling."]
    #[inline(always)]
    #[must_use]
    pub fn disscramble(&mut self) -> DisscrambleW<Usb3GctlSpec> {
        DisscrambleW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Scale-Down Mode (ScaleDown) When Scale-Down mode is enabled for simulation, the core uses scaled-down timing values, resulting in faster simulations. When Scale-Down mode is disabled, actual timing values are used. This is required for hardware operation. HS/FS/LS Modes: 2'b00: Disables all scale-downs. Actual timing values are used. 2'b01: Enables scale-down of all timing values except Device mode suspend and resume. These include Speed enumeration, HNP/SRP, and Host mode suspend and resume 2'b10: Enables scale-down of Device mode suspend and resume timing values only. 2'b11: Enables bit 0 and bit 1 scale-down timing values. SS Mode: 2'b00: Disables all scale-downs. Actual timing values are used. 2'b01: Enables scaled down SS timing and repeat values including: (1) Number of TxEq training sequences reduce to 8; (2) LFPS polling burst time reduce to 256 nS; (3) LFPS warm reset receive reduce to 30 uS. 2'b10: No TxEq training sequences are sent. Overrides Bit 4. 2'b11: Enables bit 0 and bit 1 scale-down timing values."]
    #[inline(always)]
    #[must_use]
    pub fn scaledown(&mut self) -> ScaledownW<Usb3GctlSpec> {
        ScaledownW::new(self, 4)
    }
    #[doc = "Bits 6:7 - RAM Clock Select (RAMClkSel) 2'b00: bus clock 2'b01: pipe clock (Only used in device mode) 2'b10: In device mode, pipe/2 clock. In Host mode, controller switches ram_clk between pipe/2 clock, mac2_clk and bus_clk based on the status of the U2/U3 ports 2'b11: In device mode, selects mac2_clk as ram_clk (when 8-bit UTMI or ULPI used. Not supported in 16-bit UTMI mode); In Host mode, controller switches ram_clk between pipe_clk, mac2_clk and bus_clk based on the status of the U2/U3 ports. In device mode, upon a USB reset and USB disconnect, the hardware clears these bits to 2'b00."]
    #[inline(always)]
    #[must_use]
    pub fn ramclksel(&mut self) -> RamclkselW<Usb3GctlSpec> {
        RamclkselW::new(self, 6)
    }
    #[doc = "Bit 8 - Debug Attach When this bit is set: 1. SS Link proceeds directly to the Polling link state (after RUN/STOP in the DCTL register is asserted) without checking remote termination; 2. Link LFPS polling timeout is infinite; 3. Polling timeout during TS1 is infinite (in case link is waiting for TXEQ to finish)."]
    #[inline(always)]
    #[must_use]
    pub fn debugattach(&mut self) -> DebugattachW<Usb3GctlSpec> {
        DebugattachW::new(self, 8)
    }
    #[doc = "Bit 9 - Disable U1/U2 timer Scaledown (U1U2TimerScale). If set to 1 along with GCTL\\[5:4\\]
(ScaleDown) = 2'bX1, disables the scale down of U1/U2 inactive timer values. This is for simulation mode only."]
    #[inline(always)]
    #[must_use]
    pub fn u1u2timerscale(&mut self) -> U1u2timerscaleW<Usb3GctlSpec> {
        U1u2timerscaleW::new(self, 9)
    }
    #[doc = "Bit 10 - SOFITPSYNC If this bit is set to 0 operating in host mode, the core keeps the UTMI/ULPI PHY on the first port in a non-suspended state whenever there is a SuperSpeed port that is not in Rx.Detect, SS.Disable and U3. If this bit is set to 1 operating in host mode, the core keeps the UTMI/ULPI PHY on the first port in a non-suspended state whenever the other non-SuperSpeed ports are not in a suspended state. This feature is useful because it saves power by suspending UTMI/ULPI when SuperSpeed only is active, and it helps resolve when the PHY does not transmit a host resume unless it is placed in suspend state. This bit must be programmed as a part of initialization at power-on reset, and must not be dynamically changed afterwards. Note: USB2PHYCFGn\\[6\\].PhySusp eventually decides to put the UTMI/ULPI PHY in to suspend state. In addition, when this bit is set to 1, the core generates ITP from the ref_clk based counter. Otherwise, ITP and SOF are generated from utmi/ulpi_clk\\[0\\]
based counter. To program the reference clock period inside the core, refer to GUCTL\\[31:22\\].REFCLKPER. This feature is valid in Host and DRD/OTG configurations and used only in Host mode operation. If you never use this feature or the GFLADJ.GFLADJ_REFCLK_LPM_SEL, the minimum frequency for the ref_clk can be as low as 32KHz. You can connect the suspend_clk (as low as 32 KHz) to the ref_clk. If you plan to enable hardware-based LPM or software-based LPM (PORTPMSC. HLE=1), then you cannot use this feature. Turn off this feature by setting this bit to 0 and use the GFLADJ.GFLADJ_REFCLK_LPM_SEL feature. If you set this bit to 1, the GUSB2PHYCFG.U2_FREECLK_EXISTS bit and the DWC_USB3_FREECLK_USB2_EXIST parameter must be set to 0. Program this bit to 0 if the core is intended to be operated in USB 3.0 mode."]
    #[inline(always)]
    #[must_use]
    pub fn sofitpsync(&mut self) -> SofitpsyncW<Usb3GctlSpec> {
        SofitpsyncW::new(self, 10)
    }
    #[doc = "Bit 11 - Core Soft Reset (CoreSoftReset) 1'b0 - No soft reset; 1'b1 - Soft reset to core Clears the interrupts and all the CSRs except the following registers: GCTL; GUCTL; GSTS; GSNPSID; GGPIO; GUID; GUSB2PHYCFGn registers; GUSB3PIPECTLn registers; DCFG; DCTL; DEVTEN; DSTS. When you reset PHYs (using GUBS3PHYCFG or GUSB3PIPECTL registers), you must keep the core in reset state until PHY clocks are stable. This controls the bus, ram, and mac domain resets. Note: This bit is for debug purposes only. Use USBCMD.HCRESET in xHCI Mode and DCTL.SoftReset in device mode for soft reset."]
    #[inline(always)]
    #[must_use]
    pub fn coresoftreset(&mut self) -> CoresoftresetW<Usb3GctlSpec> {
        CoresoftresetW::new(self, 11)
    }
    #[doc = "Bits 12:13 - PRTCAPDIR: Port Capability Direction (PrtCapDir) 2'b01: for Host configurations 2'b10: for Device configurations SW should base on IDDIG input to set usb3 controller as an OTG 2.0/3.0 device with A-device or B-device."]
    #[inline(always)]
    #[must_use]
    pub fn prtcapdir(&mut self) -> PrtcapdirW<Usb3GctlSpec> {
        PrtcapdirW::new(self, 12)
    }
    #[doc = "Bits 14:15 - FRMSCLDWN This field scales down device view of a SOF/USOF/ITP duration. For SS/HS mode: Value of 2'h3 implements interval to be 15.625 us Value of 2'h2 implements interval to be 31.25 us Value of 2'h1 implements interval to be 62.5 us Value of 2'h0 implements interval to be 125us For FS mode, the scale-down value is multiplied by 8. When xHCI Debug Capability is enabled, this field also scales down the MaxPacketSize of the IN and OUT bulk endpoint to allow more traffic during simulation. It can only be changed from a non-zero value during simulation. 2'h0: 1024 bytes 2'h1: 512 bytes 2'h2: 256 bytes 2'h3: 128 bytes"]
    #[inline(always)]
    #[must_use]
    pub fn frmscldwn(&mut self) -> FrmscldwnW<Usb3GctlSpec> {
        FrmscldwnW::new(self, 14)
    }
    #[doc = "Bit 16 - U2RSTECN If the SuperSpeed connection fails during POLL or LMP exchange, the device connects at non-SS mode. If this bit is set, then device attempts three more times to connect at SS, even if it previously failed to operate in SS mode. For each attempt, the device checks receiver termination eight times. From 2.60a release, this bit controls whether to check for Rx.Detect eight times or one time for every attempt. Device controller on USB 2.0 reset checks for receiver termination eight times per attempt if this bit is set to zero, or only once per attempt if the bit is set to one. Note: This bit is applicable only in device mode."]
    #[inline(always)]
    #[must_use]
    pub fn u2rstecn(&mut self) -> U2rstecnW<Usb3GctlSpec> {
        U2rstecnW::new(self, 16)
    }
    #[doc = "Bit 17 - Bypass SetAddress in Device Mode. When BYPSSETADDR bit is set, the device core uses the value in the DCFG\\[DevAddr\\]
bits directly for comparing the device address in the tokens. For simulation, you can use this feature to avoid sending an actual SET ADDRESS control transfer on the USB, and make the device core respond to a new address. When the xHCI Debug capability is enabled and this bit is set, the Debug Target immediately enters the configured state without requiring the Debug Host to send a SetAddress or SetConfig request. Note: You can set this bit for simulation purposes only. In the actual hardware, this bit must be set to 1'b0."]
    #[inline(always)]
    #[must_use]
    pub fn bypssetaddr(&mut self) -> BypssetaddrW<Usb3GctlSpec> {
        BypssetaddrW::new(self, 17)
    }
    #[doc = "Bit 18 - Master Filter Bypass When this bit is set to 1'b1, all the filters are bypassed. The double synchronizers to mac_clk preceding the filters are also bypassed. For enabling the filters, this bit must be 1'b0."]
    #[inline(always)]
    #[must_use]
    pub fn masterfiltbypass(&mut self) -> MasterfiltbypassW<Usb3GctlSpec> {
        MasterfiltbypassW::new(self, 18)
    }
    #[doc = "Bits 19:31 - Power Down Scale (PwrDnScale) The USB3 suspend_clk input replaces pipe3_rx_pclk as a clock source to a small part of the USB3 core that operates when the SS PHY is in its lowest power (P3) state, and therefore does not provide a clock. The Power Down Scale field specifies how many suspend_clk periods fit into a 16 kHz clock period. When performing the division, round up the remainder. For example, when using an 8-bit/16-bit/32-bit PHY and 25-MHz Suspend clock, Power Down Scale = 25000 kHz/16 kHz = 13'd1563 (rounder up) Note: Minimum Suspend clock frequency is 32 kHz Maximum Suspend clock frequency is 125 MHz The LTSSM uses Suspend clock for 12-ms and 100-ms timers during suspend mode. According to the USB 3.0 specification, the accuracy on these timers is 0% to +50%. 12 ms + 0~+50% accuracy = 18 ms (Range is 12 ms - 18 ms) 100 ms + 0~+50% accuracy = 150 ms (Range is 100 ms - 150ms). The suspend clock accuracy requirement is: (12,000/62.5) * (GCTL\\[31:19\\]) * actual suspend_clk_period must be between 12,000 and 18,000 (100,0000/62.5) * (GCTL\\[31:19\\]) * actual suspend_clk_period must be between 100,000 and 150,000 For example, if your suspend_clk frequency varies from 7.5 MHz to 10.5MHz, then the value needs to programmed is: Power Down Scale = 10500/16 = 657 (rounded up; and fastest frequency used)."]
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
