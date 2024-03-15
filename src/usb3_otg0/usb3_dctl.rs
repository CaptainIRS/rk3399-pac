#[doc = "Register `USB3_DCTL` reader"]
pub type R = crate::R<Usb3DctlSpec>;
#[doc = "Register `USB3_DCTL` writer"]
pub type W = crate::W<Usb3DctlSpec>;
#[doc = "Field `TSTCTL` reader - Test Control 4'b000: Test mode disabled 4'b001: Test_J mode 4'b010: Test_K mode 4'b011: Test_SE0_NAK mode 4'b100: Test_Packet mode 4'b101: Test_Force_Enable Others: Reserved"]
pub type TstctlR = crate::FieldReader;
#[doc = "Field `TSTCTL` writer - Test Control 4'b000: Test mode disabled 4'b001: Test_J mode 4'b010: Test_K mode 4'b011: Test_SE0_NAK mode 4'b100: Test_Packet mode 4'b101: Test_Force_Enable Others: Reserved"]
pub type TstctlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ULSTCHNGREQ` reader - ULSTCHNGREQ Software writes this field to issue a USB/Link state change request. A change in this field indicates a new request to the core. If software wants to issue the same request back-to-back, it must write a 0 to this field between the two requests. The result of the state change request is reflected in the USB/Link State in DSTS. These bits are self-cleared on the MAC Layer exiting suspended state. If software is updating other fields of the DCTL register and not intending to force any link state change, then it must write a 0 to this field. SS Compliance mode is normally entered and controlled by the remote link partner. Refer to the USB 3.0 specification. Alternatively, you can force the local link directly into compliance mode, by resetting the SS link with the RUN/STOP bit set to zero. If you then write 10 to the USB/Link State Change field and 1 to RUN/STOP, the link goes to compliance mode. Once you are in compliance, you may alternately write zero and 10 to this field to advance the compliance pattern. In SS mode: Value: Requested Link State Transition/Action 0: No Action 4: SS.Disabled 5: Rx.Detect 6: SS.Inactive 8: Recovery 10: Compliance Others: Reserved In HS/FS/LS mode: Value:Requested USB state transition 8: Remote wakeup request Others: Reserved The Remote wakeup request must be issued 2us after the device goes into suspend state (DSTS\\[21:18\\]
is 3 ). Note: After coming out of hibernation, software must write 8 (Recovery) into this field to confirm exit from the suspended state."]
pub type UlstchngreqR = crate::FieldReader;
#[doc = "Field `ULSTCHNGREQ` writer - ULSTCHNGREQ Software writes this field to issue a USB/Link state change request. A change in this field indicates a new request to the core. If software wants to issue the same request back-to-back, it must write a 0 to this field between the two requests. The result of the state change request is reflected in the USB/Link State in DSTS. These bits are self-cleared on the MAC Layer exiting suspended state. If software is updating other fields of the DCTL register and not intending to force any link state change, then it must write a 0 to this field. SS Compliance mode is normally entered and controlled by the remote link partner. Refer to the USB 3.0 specification. Alternatively, you can force the local link directly into compliance mode, by resetting the SS link with the RUN/STOP bit set to zero. If you then write 10 to the USB/Link State Change field and 1 to RUN/STOP, the link goes to compliance mode. Once you are in compliance, you may alternately write zero and 10 to this field to advance the compliance pattern. In SS mode: Value: Requested Link State Transition/Action 0: No Action 4: SS.Disabled 5: Rx.Detect 6: SS.Inactive 8: Recovery 10: Compliance Others: Reserved In HS/FS/LS mode: Value:Requested USB state transition 8: Remote wakeup request Others: Reserved The Remote wakeup request must be issued 2us after the device goes into suspend state (DSTS\\[21:18\\]
is 3 ). Note: After coming out of hibernation, software must write 8 (Recovery) into this field to confirm exit from the suspended state."]
pub type UlstchngreqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ACCEPTU1ENA` reader - Accept U1 Enable 1'b0: Core rejects U1 except when Force_LinkPM_Accept bit is set (default) 1'b1: Core accepts transition to U1 state if nothing is pending on the application side. On USB reset, hardware clears this bit to 0. Software sets this bit after receiving a SetConfiguration command."]
pub type Acceptu1enaR = crate::BitReader;
#[doc = "Field `ACCEPTU1ENA` writer - Accept U1 Enable 1'b0: Core rejects U1 except when Force_LinkPM_Accept bit is set (default) 1'b1: Core accepts transition to U1 state if nothing is pending on the application side. On USB reset, hardware clears this bit to 0. Software sets this bit after receiving a SetConfiguration command."]
pub type Acceptu1enaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITU1ENA` reader - Initiate U1 Enable 1'b0: May not initiate U1 (default); 1'b1: May initiate U1. On USB reset, hardware clears this bit to 0. Software sets this bit after receiving SetFeature(U1_ENABLE), and clears this bit when ClearFeature(U1_ENABLE) is received. If DCTL\\[9\\]
(AcceptU1Ena) is 0, the link immediately exits U1 state."]
pub type Initu1enaR = crate::BitReader;
#[doc = "Field `INITU1ENA` writer - Initiate U1 Enable 1'b0: May not initiate U1 (default); 1'b1: May initiate U1. On USB reset, hardware clears this bit to 0. Software sets this bit after receiving SetFeature(U1_ENABLE), and clears this bit when ClearFeature(U1_ENABLE) is received. If DCTL\\[9\\]
(AcceptU1Ena) is 0, the link immediately exits U1 state."]
pub type Initu1enaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCEPTU2ENA` reader - Accept U2 Enable 1'b0: Reject U2 except when Force_LinkPM_Accept bit is set (default) 1'b1: Core accepts transition to U2 state if nothing is pending on the application side. On USB reset, hardware clears this bit to 0. Software sets this bit after receiving a SetConfiguration command."]
pub type Acceptu2enaR = crate::BitReader;
#[doc = "Field `ACCEPTU2ENA` writer - Accept U2 Enable 1'b0: Reject U2 except when Force_LinkPM_Accept bit is set (default) 1'b1: Core accepts transition to U2 state if nothing is pending on the application side. On USB reset, hardware clears this bit to 0. Software sets this bit after receiving a SetConfiguration command."]
pub type Acceptu2enaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INITU2ENA` reader - Initiate U2 Enable 1'b0: May not initiate U2 (default) 1'b1: May initiate U2 On USB reset, hardware clears this bit to 0. Software sets this bit after receiving SetFeature(U2_ENABLE), and clears this bit when ClearFeature(U2_ENABLE) is received. If DCTL\\[11\\]
(AcceptU2Ena) is 0, the link immediately exits U2 state."]
pub type Initu2enaR = crate::BitReader;
#[doc = "Field `INITU2ENA` writer - Initiate U2 Enable 1'b0: May not initiate U2 (default) 1'b1: May initiate U2 On USB reset, hardware clears this bit to 0. Software sets this bit after receiving SetFeature(U2_ENABLE), and clears this bit when ClearFeature(U2_ENABLE) is received. If DCTL\\[11\\]
(AcceptU2Ena) is 0, the link immediately exits U2 state."]
pub type Initu2enaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSS` reader - Controller Save State This command is similar to the USBCMD.CSS bit in host mode and initiates the save process. When software sets this bit to 1, the controller immediately sets DSTS.SSS to 1. When the controller has finished the save process, it sets DSTS.SSS to 0. Note: When read, this field always returns 0."]
pub type CssR = crate::BitReader;
#[doc = "Field `CSS` writer - Controller Save State This command is similar to the USBCMD.CSS bit in host mode and initiates the save process. When software sets this bit to 1, the controller immediately sets DSTS.SSS to 1. When the controller has finished the save process, it sets DSTS.SSS to 0. Note: When read, this field always returns 0."]
pub type CssW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRS` reader - Controller Restore State This command is similar to the USBCMD.CRS bit in host mode and initiates the restore process. When software sets this bit to 1, the controller immediately sets DSTS.RSS to 1. When the controller has finished the restore process, it sets DSTS.RSS to 0. Note: When read, this field always returns 0."]
pub type CrsR = crate::BitReader;
#[doc = "Field `CRS` writer - Controller Restore State This command is similar to the USBCMD.CRS bit in host mode and initiates the restore process. When software sets this bit to 1, the controller immediately sets DSTS.RSS to 1. When the controller has finished the restore process, it sets DSTS.RSS to 0. Note: When read, this field always returns 0."]
pub type CrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1HIBERNATIONEN` reader - L1HibernationEn When this bit is set along with KeepConnect, the device core generates a Hibernation Request Event if L1 is enabled and the HIRD value in the LPM token is larger than the threshold programmed in DCTL.HIRD_Thres. The core does not exit the LPM L1 state until software writes Recovery into the DCTL.ULStChngReq field. This prevents corner cases where the device is entering hibernation at the same time the host is attempting to exit L1."]
pub type L1hibernationenR = crate::BitReader;
#[doc = "Field `L1HIBERNATIONEN` writer - L1HibernationEn When this bit is set along with KeepConnect, the device core generates a Hibernation Request Event if L1 is enabled and the HIRD value in the LPM token is larger than the threshold programmed in DCTL.HIRD_Thres. The core does not exit the LPM L1 state until software writes Recovery into the DCTL.ULStChngReq field. This prevents corner cases where the device is entering hibernation at the same time the host is attempting to exit L1."]
pub type L1hibernationenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEEPCONNECT` reader - KeepConnect When 1, this bit enables the save and restore programming model by preventing the core from disconnecting from the host when DCTL.RunStop is set to 0. It also enables the Hibernation Request Event to be generated when the link goes to U3 or L2. The device core disconnects from the host when DCTL.RunStop is set to 0. This bit indicates whether to preserve this behavior (0), or if the core must not disconnect when RunStop is set to 0 (1). This bit also prevents the LTSSM from automatically going to U0/L0 when the host requests resume from U3/L2."]
pub type KeepconnectR = crate::BitReader;
#[doc = "Field `KEEPCONNECT` writer - KeepConnect When 1, this bit enables the save and restore programming model by preventing the core from disconnecting from the host when DCTL.RunStop is set to 0. It also enables the Hibernation Request Event to be generated when the link goes to U3 or L2. The device core disconnects from the host when DCTL.RunStop is set to 0. This bit indicates whether to preserve this behavior (0), or if the core must not disconnect when RunStop is set to 0 (1). This bit also prevents the LTSSM from automatically going to U0/L0 when the host requests resume from U3/L2."]
pub type KeepconnectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM_NYET_THRES` reader - LPM NYET Threshold Handshake response to LPM token specified by device application. Response depends on DCFG.LPMCap. DCFG.LPMCap is 1'b0 - The core always responds with Timeout (that is, no response). DCFG.LPMCap is 1'b1 - The core responds with an ACK on successful LPM transaction, which requires that all of the following are satisfied: 1. There are no PID or CRC5 errors in both the EXT token and the LPM token (if not true, inactivity results in a timeout ERROR). 2. No data is pending in the Transmit FIFO and OUT endpoints not in flow controlled state (else NYET). 3. The BESL value in the LPM token is less than or equal to LPM_NYET_thres\\[3:0\\]"]
pub type LpmNyetThresR = crate::FieldReader;
#[doc = "Field `LPM_NYET_THRES` writer - LPM NYET Threshold Handshake response to LPM token specified by device application. Response depends on DCFG.LPMCap. DCFG.LPMCap is 1'b0 - The core always responds with Timeout (that is, no response). DCFG.LPMCap is 1'b1 - The core responds with an ACK on successful LPM transaction, which requires that all of the following are satisfied: 1. There are no PID or CRC5 errors in both the EXT token and the LPM token (if not true, inactivity results in a timeout ERROR). 2. No data is pending in the Transmit FIFO and OUT endpoints not in flow controlled state (else NYET). 3. The BESL value in the LPM token is less than or equal to LPM_NYET_thres\\[3:0\\]"]
pub type LpmNyetThresW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HIRDTHRES` reader - HIRD Threshold The core asserts output signals utmi_l1_suspend_n and utmi_sleep_n on the basis of this signal: The core asserts utmi_l1_suspend_n to put the PHY into Deep Low-Power mode in L1 when both of the following are true: 1. HIRD value is greater than or equal to the value in DCTL.HIRD_Thres\\[3:0\\]
2. HIRD_Thres\\[4\\]
is set to 1'b1. The core asserts utmi_sleep_n on L1 when one of the following is true: 1. If the HIRD value is less than HIRD_Thres\\[3:0\\]
or 2. HIRD_Thres\\[4\\]
is set to 1'b0. Note: This field must be set to '0' during SuperSpeed mode of operation."]
pub type HirdthresR = crate::FieldReader;
#[doc = "Field `HIRDTHRES` writer - HIRD Threshold The core asserts output signals utmi_l1_suspend_n and utmi_sleep_n on the basis of this signal: The core asserts utmi_l1_suspend_n to put the PHY into Deep Low-Power mode in L1 when both of the following are true: 1. HIRD value is greater than or equal to the value in DCTL.HIRD_Thres\\[3:0\\]
2. HIRD_Thres\\[4\\]
is set to 1'b1. The core asserts utmi_sleep_n on L1 when one of the following is true: 1. If the HIRD value is less than HIRD_Thres\\[3:0\\]
or 2. HIRD_Thres\\[4\\]
is set to 1'b0. Note: This field must be set to '0' during SuperSpeed mode of operation."]
pub type HirdthresW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CSFTRST` reader - Core Soft Reset Reset all clock domains as follows: 1. This bit clears the interrupts and all the CSRs except GSTS, GSNPSID, GGPIO, GUID, GUSB2PHYCFGn registers, GUSB3PIPECTLn registers, DCFG, DCTL, DEVTEN, and DSTS registers. 2. All module state machines (except the SoC Bus Slave Unit) are reset to the IDLE state, and all the TxFIFOs and the RxFIFO are flushed. 3. Any transactions on the SoC bus Master are terminated as soon as possible, after gracefully completing the last data phase of a SoC bus transfer. Any transactions on the USB are terminated immediately. The application can write this bit at any time to reset the core. This is a self-clearing bit; the core clears this bit after all necessary logic is reset in the core, which may take several clocks depending on the core's current state. Once this bit is cleared, the software must wait at least 3 PHY clocks before accessing the PHY domain (synchronization delay). Typically, software reset is used during software development and also when you dynamically change the PHY selection bits in the USB configuration registers listed above. When you change the PHY, the corresponding clock for the PHY is selected and used in the PHY domain. Once a new clock is selected, the PHY domain must be reset for proper operation."]
pub type CsftrstR = crate::BitReader;
#[doc = "Field `CSFTRST` writer - Core Soft Reset Reset all clock domains as follows: 1. This bit clears the interrupts and all the CSRs except GSTS, GSNPSID, GGPIO, GUID, GUSB2PHYCFGn registers, GUSB3PIPECTLn registers, DCFG, DCTL, DEVTEN, and DSTS registers. 2. All module state machines (except the SoC Bus Slave Unit) are reset to the IDLE state, and all the TxFIFOs and the RxFIFO are flushed. 3. Any transactions on the SoC bus Master are terminated as soon as possible, after gracefully completing the last data phase of a SoC bus transfer. Any transactions on the USB are terminated immediately. The application can write this bit at any time to reset the core. This is a self-clearing bit; the core clears this bit after all necessary logic is reset in the core, which may take several clocks depending on the core's current state. Once this bit is cleared, the software must wait at least 3 PHY clocks before accessing the PHY domain (synchronization delay). Typically, software reset is used during software development and also when you dynamically change the PHY selection bits in the USB configuration registers listed above. When you change the PHY, the corresponding clock for the PHY is selected and used in the PHY domain. Once a new clock is selected, the PHY domain must be reset for proper operation."]
pub type CsftrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUN_STOP` reader - Run/Stop The software writes 1 to this bit to start the device controller operation. To stop the device controller operation, the software must remove any active transfers and write 0 to this bit. When the controller is stopped, it sets the DSTS.DevCtrlHlt bit when the core is idle and the lower layer finishes the disconnect process. The Run/Stop bit must be used in following cases as specified: 1. After power-on reset and CSR initialization, the software must write 1 to this bit to start the device controller. The controller does not signal connect to the host until this bit is set. 2. The software uses this bit to control the device controller to perform a soft disconnect. When the software writes 0 to this bit, the host does not see that the device is connected. The device controller stays in the disconnected state until the software writes 1 to this bit. The minimum duration of keeping this bit cleared is specified in the Note below. If the software attempts a connect after the soft disconnect or detects a disconnect event, it must set DCTL\\[8:5\\]
to 5 before reasserting the Run/Stop bit. 3. When the USB or Link is in a lower power state and the Two Power Rails configuration is selected, software writes 0 to this bit to indicate that it is going to turn off the Core Power Rail. After the software turns on the Core Power Rail again and re-initializes the device controller, it must set this bit to start the device controller. Note: The following is the minimum duration under various conditions for which the soft disconnect (SftDiscon) bit must be set for the USB host to detect a device disconnect: 30ms: For SuperSpeed, when the device state is Suspended, Idle, Transmit, or Receive. 10ms: For high-speed, when the device state is Suspended, Idle, or not Idle/Suspended (performing transactions); For full- speed/low-speed, when the device state is Suspended, Idle, or not Idle/Supended (performing transactions) To accommodate clock jitter, it is recommended that the application add extra delay to the specified minimum duration."]
pub type RunStopR = crate::BitReader;
#[doc = "Field `RUN_STOP` writer - Run/Stop The software writes 1 to this bit to start the device controller operation. To stop the device controller operation, the software must remove any active transfers and write 0 to this bit. When the controller is stopped, it sets the DSTS.DevCtrlHlt bit when the core is idle and the lower layer finishes the disconnect process. The Run/Stop bit must be used in following cases as specified: 1. After power-on reset and CSR initialization, the software must write 1 to this bit to start the device controller. The controller does not signal connect to the host until this bit is set. 2. The software uses this bit to control the device controller to perform a soft disconnect. When the software writes 0 to this bit, the host does not see that the device is connected. The device controller stays in the disconnected state until the software writes 1 to this bit. The minimum duration of keeping this bit cleared is specified in the Note below. If the software attempts a connect after the soft disconnect or detects a disconnect event, it must set DCTL\\[8:5\\]
to 5 before reasserting the Run/Stop bit. 3. When the USB or Link is in a lower power state and the Two Power Rails configuration is selected, software writes 0 to this bit to indicate that it is going to turn off the Core Power Rail. After the software turns on the Core Power Rail again and re-initializes the device controller, it must set this bit to start the device controller. Note: The following is the minimum duration under various conditions for which the soft disconnect (SftDiscon) bit must be set for the USB host to detect a device disconnect: 30ms: For SuperSpeed, when the device state is Suspended, Idle, Transmit, or Receive. 10ms: For high-speed, when the device state is Suspended, Idle, or not Idle/Suspended (performing transactions); For full- speed/low-speed, when the device state is Suspended, Idle, or not Idle/Supended (performing transactions) To accommodate clock jitter, it is recommended that the application add extra delay to the specified minimum duration."]
pub type RunStopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 1:4 - Test Control 4'b000: Test mode disabled 4'b001: Test_J mode 4'b010: Test_K mode 4'b011: Test_SE0_NAK mode 4'b100: Test_Packet mode 4'b101: Test_Force_Enable Others: Reserved"]
    #[inline(always)]
    pub fn tstctl(&self) -> TstctlR {
        TstctlR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:8 - ULSTCHNGREQ Software writes this field to issue a USB/Link state change request. A change in this field indicates a new request to the core. If software wants to issue the same request back-to-back, it must write a 0 to this field between the two requests. The result of the state change request is reflected in the USB/Link State in DSTS. These bits are self-cleared on the MAC Layer exiting suspended state. If software is updating other fields of the DCTL register and not intending to force any link state change, then it must write a 0 to this field. SS Compliance mode is normally entered and controlled by the remote link partner. Refer to the USB 3.0 specification. Alternatively, you can force the local link directly into compliance mode, by resetting the SS link with the RUN/STOP bit set to zero. If you then write 10 to the USB/Link State Change field and 1 to RUN/STOP, the link goes to compliance mode. Once you are in compliance, you may alternately write zero and 10 to this field to advance the compliance pattern. In SS mode: Value: Requested Link State Transition/Action 0: No Action 4: SS.Disabled 5: Rx.Detect 6: SS.Inactive 8: Recovery 10: Compliance Others: Reserved In HS/FS/LS mode: Value:Requested USB state transition 8: Remote wakeup request Others: Reserved The Remote wakeup request must be issued 2us after the device goes into suspend state (DSTS\\[21:18\\]
is 3 ). Note: After coming out of hibernation, software must write 8 (Recovery) into this field to confirm exit from the suspended state."]
    #[inline(always)]
    pub fn ulstchngreq(&self) -> UlstchngreqR {
        UlstchngreqR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Accept U1 Enable 1'b0: Core rejects U1 except when Force_LinkPM_Accept bit is set (default) 1'b1: Core accepts transition to U1 state if nothing is pending on the application side. On USB reset, hardware clears this bit to 0. Software sets this bit after receiving a SetConfiguration command."]
    #[inline(always)]
    pub fn acceptu1ena(&self) -> Acceptu1enaR {
        Acceptu1enaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Initiate U1 Enable 1'b0: May not initiate U1 (default); 1'b1: May initiate U1. On USB reset, hardware clears this bit to 0. Software sets this bit after receiving SetFeature(U1_ENABLE), and clears this bit when ClearFeature(U1_ENABLE) is received. If DCTL\\[9\\]
(AcceptU1Ena) is 0, the link immediately exits U1 state."]
    #[inline(always)]
    pub fn initu1ena(&self) -> Initu1enaR {
        Initu1enaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Accept U2 Enable 1'b0: Reject U2 except when Force_LinkPM_Accept bit is set (default) 1'b1: Core accepts transition to U2 state if nothing is pending on the application side. On USB reset, hardware clears this bit to 0. Software sets this bit after receiving a SetConfiguration command."]
    #[inline(always)]
    pub fn acceptu2ena(&self) -> Acceptu2enaR {
        Acceptu2enaR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Initiate U2 Enable 1'b0: May not initiate U2 (default) 1'b1: May initiate U2 On USB reset, hardware clears this bit to 0. Software sets this bit after receiving SetFeature(U2_ENABLE), and clears this bit when ClearFeature(U2_ENABLE) is received. If DCTL\\[11\\]
(AcceptU2Ena) is 0, the link immediately exits U2 state."]
    #[inline(always)]
    pub fn initu2ena(&self) -> Initu2enaR {
        Initu2enaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Controller Save State This command is similar to the USBCMD.CSS bit in host mode and initiates the save process. When software sets this bit to 1, the controller immediately sets DSTS.SSS to 1. When the controller has finished the save process, it sets DSTS.SSS to 0. Note: When read, this field always returns 0."]
    #[inline(always)]
    pub fn css(&self) -> CssR {
        CssR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Controller Restore State This command is similar to the USBCMD.CRS bit in host mode and initiates the restore process. When software sets this bit to 1, the controller immediately sets DSTS.RSS to 1. When the controller has finished the restore process, it sets DSTS.RSS to 0. Note: When read, this field always returns 0."]
    #[inline(always)]
    pub fn crs(&self) -> CrsR {
        CrsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - L1HibernationEn When this bit is set along with KeepConnect, the device core generates a Hibernation Request Event if L1 is enabled and the HIRD value in the LPM token is larger than the threshold programmed in DCTL.HIRD_Thres. The core does not exit the LPM L1 state until software writes Recovery into the DCTL.ULStChngReq field. This prevents corner cases where the device is entering hibernation at the same time the host is attempting to exit L1."]
    #[inline(always)]
    pub fn l1hibernationen(&self) -> L1hibernationenR {
        L1hibernationenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - KeepConnect When 1, this bit enables the save and restore programming model by preventing the core from disconnecting from the host when DCTL.RunStop is set to 0. It also enables the Hibernation Request Event to be generated when the link goes to U3 or L2. The device core disconnects from the host when DCTL.RunStop is set to 0. This bit indicates whether to preserve this behavior (0), or if the core must not disconnect when RunStop is set to 0 (1). This bit also prevents the LTSSM from automatically going to U0/L0 when the host requests resume from U3/L2."]
    #[inline(always)]
    pub fn keepconnect(&self) -> KeepconnectR {
        KeepconnectR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - LPM NYET Threshold Handshake response to LPM token specified by device application. Response depends on DCFG.LPMCap. DCFG.LPMCap is 1'b0 - The core always responds with Timeout (that is, no response). DCFG.LPMCap is 1'b1 - The core responds with an ACK on successful LPM transaction, which requires that all of the following are satisfied: 1. There are no PID or CRC5 errors in both the EXT token and the LPM token (if not true, inactivity results in a timeout ERROR). 2. No data is pending in the Transmit FIFO and OUT endpoints not in flow controlled state (else NYET). 3. The BESL value in the LPM token is less than or equal to LPM_NYET_thres\\[3:0\\]"]
    #[inline(always)]
    pub fn lpm_nyet_thres(&self) -> LpmNyetThresR {
        LpmNyetThresR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:28 - HIRD Threshold The core asserts output signals utmi_l1_suspend_n and utmi_sleep_n on the basis of this signal: The core asserts utmi_l1_suspend_n to put the PHY into Deep Low-Power mode in L1 when both of the following are true: 1. HIRD value is greater than or equal to the value in DCTL.HIRD_Thres\\[3:0\\]
2. HIRD_Thres\\[4\\]
is set to 1'b1. The core asserts utmi_sleep_n on L1 when one of the following is true: 1. If the HIRD value is less than HIRD_Thres\\[3:0\\]
or 2. HIRD_Thres\\[4\\]
is set to 1'b0. Note: This field must be set to '0' during SuperSpeed mode of operation."]
    #[inline(always)]
    pub fn hirdthres(&self) -> HirdthresR {
        HirdthresR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - Core Soft Reset Reset all clock domains as follows: 1. This bit clears the interrupts and all the CSRs except GSTS, GSNPSID, GGPIO, GUID, GUSB2PHYCFGn registers, GUSB3PIPECTLn registers, DCFG, DCTL, DEVTEN, and DSTS registers. 2. All module state machines (except the SoC Bus Slave Unit) are reset to the IDLE state, and all the TxFIFOs and the RxFIFO are flushed. 3. Any transactions on the SoC bus Master are terminated as soon as possible, after gracefully completing the last data phase of a SoC bus transfer. Any transactions on the USB are terminated immediately. The application can write this bit at any time to reset the core. This is a self-clearing bit; the core clears this bit after all necessary logic is reset in the core, which may take several clocks depending on the core's current state. Once this bit is cleared, the software must wait at least 3 PHY clocks before accessing the PHY domain (synchronization delay). Typically, software reset is used during software development and also when you dynamically change the PHY selection bits in the USB configuration registers listed above. When you change the PHY, the corresponding clock for the PHY is selected and used in the PHY domain. Once a new clock is selected, the PHY domain must be reset for proper operation."]
    #[inline(always)]
    pub fn csftrst(&self) -> CsftrstR {
        CsftrstR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Run/Stop The software writes 1 to this bit to start the device controller operation. To stop the device controller operation, the software must remove any active transfers and write 0 to this bit. When the controller is stopped, it sets the DSTS.DevCtrlHlt bit when the core is idle and the lower layer finishes the disconnect process. The Run/Stop bit must be used in following cases as specified: 1. After power-on reset and CSR initialization, the software must write 1 to this bit to start the device controller. The controller does not signal connect to the host until this bit is set. 2. The software uses this bit to control the device controller to perform a soft disconnect. When the software writes 0 to this bit, the host does not see that the device is connected. The device controller stays in the disconnected state until the software writes 1 to this bit. The minimum duration of keeping this bit cleared is specified in the Note below. If the software attempts a connect after the soft disconnect or detects a disconnect event, it must set DCTL\\[8:5\\]
to 5 before reasserting the Run/Stop bit. 3. When the USB or Link is in a lower power state and the Two Power Rails configuration is selected, software writes 0 to this bit to indicate that it is going to turn off the Core Power Rail. After the software turns on the Core Power Rail again and re-initializes the device controller, it must set this bit to start the device controller. Note: The following is the minimum duration under various conditions for which the soft disconnect (SftDiscon) bit must be set for the USB host to detect a device disconnect: 30ms: For SuperSpeed, when the device state is Suspended, Idle, Transmit, or Receive. 10ms: For high-speed, when the device state is Suspended, Idle, or not Idle/Suspended (performing transactions); For full- speed/low-speed, when the device state is Suspended, Idle, or not Idle/Supended (performing transactions) To accommodate clock jitter, it is recommended that the application add extra delay to the specified minimum duration."]
    #[inline(always)]
    pub fn run_stop(&self) -> RunStopR {
        RunStopR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:4 - Test Control 4'b000: Test mode disabled 4'b001: Test_J mode 4'b010: Test_K mode 4'b011: Test_SE0_NAK mode 4'b100: Test_Packet mode 4'b101: Test_Force_Enable Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn tstctl(&mut self) -> TstctlW<Usb3DctlSpec> {
        TstctlW::new(self, 1)
    }
    #[doc = "Bits 5:8 - ULSTCHNGREQ Software writes this field to issue a USB/Link state change request. A change in this field indicates a new request to the core. If software wants to issue the same request back-to-back, it must write a 0 to this field between the two requests. The result of the state change request is reflected in the USB/Link State in DSTS. These bits are self-cleared on the MAC Layer exiting suspended state. If software is updating other fields of the DCTL register and not intending to force any link state change, then it must write a 0 to this field. SS Compliance mode is normally entered and controlled by the remote link partner. Refer to the USB 3.0 specification. Alternatively, you can force the local link directly into compliance mode, by resetting the SS link with the RUN/STOP bit set to zero. If you then write 10 to the USB/Link State Change field and 1 to RUN/STOP, the link goes to compliance mode. Once you are in compliance, you may alternately write zero and 10 to this field to advance the compliance pattern. In SS mode: Value: Requested Link State Transition/Action 0: No Action 4: SS.Disabled 5: Rx.Detect 6: SS.Inactive 8: Recovery 10: Compliance Others: Reserved In HS/FS/LS mode: Value:Requested USB state transition 8: Remote wakeup request Others: Reserved The Remote wakeup request must be issued 2us after the device goes into suspend state (DSTS\\[21:18\\]
is 3 ). Note: After coming out of hibernation, software must write 8 (Recovery) into this field to confirm exit from the suspended state."]
    #[inline(always)]
    #[must_use]
    pub fn ulstchngreq(&mut self) -> UlstchngreqW<Usb3DctlSpec> {
        UlstchngreqW::new(self, 5)
    }
    #[doc = "Bit 9 - Accept U1 Enable 1'b0: Core rejects U1 except when Force_LinkPM_Accept bit is set (default) 1'b1: Core accepts transition to U1 state if nothing is pending on the application side. On USB reset, hardware clears this bit to 0. Software sets this bit after receiving a SetConfiguration command."]
    #[inline(always)]
    #[must_use]
    pub fn acceptu1ena(&mut self) -> Acceptu1enaW<Usb3DctlSpec> {
        Acceptu1enaW::new(self, 9)
    }
    #[doc = "Bit 10 - Initiate U1 Enable 1'b0: May not initiate U1 (default); 1'b1: May initiate U1. On USB reset, hardware clears this bit to 0. Software sets this bit after receiving SetFeature(U1_ENABLE), and clears this bit when ClearFeature(U1_ENABLE) is received. If DCTL\\[9\\]
(AcceptU1Ena) is 0, the link immediately exits U1 state."]
    #[inline(always)]
    #[must_use]
    pub fn initu1ena(&mut self) -> Initu1enaW<Usb3DctlSpec> {
        Initu1enaW::new(self, 10)
    }
    #[doc = "Bit 11 - Accept U2 Enable 1'b0: Reject U2 except when Force_LinkPM_Accept bit is set (default) 1'b1: Core accepts transition to U2 state if nothing is pending on the application side. On USB reset, hardware clears this bit to 0. Software sets this bit after receiving a SetConfiguration command."]
    #[inline(always)]
    #[must_use]
    pub fn acceptu2ena(&mut self) -> Acceptu2enaW<Usb3DctlSpec> {
        Acceptu2enaW::new(self, 11)
    }
    #[doc = "Bit 12 - Initiate U2 Enable 1'b0: May not initiate U2 (default) 1'b1: May initiate U2 On USB reset, hardware clears this bit to 0. Software sets this bit after receiving SetFeature(U2_ENABLE), and clears this bit when ClearFeature(U2_ENABLE) is received. If DCTL\\[11\\]
(AcceptU2Ena) is 0, the link immediately exits U2 state."]
    #[inline(always)]
    #[must_use]
    pub fn initu2ena(&mut self) -> Initu2enaW<Usb3DctlSpec> {
        Initu2enaW::new(self, 12)
    }
    #[doc = "Bit 16 - Controller Save State This command is similar to the USBCMD.CSS bit in host mode and initiates the save process. When software sets this bit to 1, the controller immediately sets DSTS.SSS to 1. When the controller has finished the save process, it sets DSTS.SSS to 0. Note: When read, this field always returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn css(&mut self) -> CssW<Usb3DctlSpec> {
        CssW::new(self, 16)
    }
    #[doc = "Bit 17 - Controller Restore State This command is similar to the USBCMD.CRS bit in host mode and initiates the restore process. When software sets this bit to 1, the controller immediately sets DSTS.RSS to 1. When the controller has finished the restore process, it sets DSTS.RSS to 0. Note: When read, this field always returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn crs(&mut self) -> CrsW<Usb3DctlSpec> {
        CrsW::new(self, 17)
    }
    #[doc = "Bit 18 - L1HibernationEn When this bit is set along with KeepConnect, the device core generates a Hibernation Request Event if L1 is enabled and the HIRD value in the LPM token is larger than the threshold programmed in DCTL.HIRD_Thres. The core does not exit the LPM L1 state until software writes Recovery into the DCTL.ULStChngReq field. This prevents corner cases where the device is entering hibernation at the same time the host is attempting to exit L1."]
    #[inline(always)]
    #[must_use]
    pub fn l1hibernationen(&mut self) -> L1hibernationenW<Usb3DctlSpec> {
        L1hibernationenW::new(self, 18)
    }
    #[doc = "Bit 19 - KeepConnect When 1, this bit enables the save and restore programming model by preventing the core from disconnecting from the host when DCTL.RunStop is set to 0. It also enables the Hibernation Request Event to be generated when the link goes to U3 or L2. The device core disconnects from the host when DCTL.RunStop is set to 0. This bit indicates whether to preserve this behavior (0), or if the core must not disconnect when RunStop is set to 0 (1). This bit also prevents the LTSSM from automatically going to U0/L0 when the host requests resume from U3/L2."]
    #[inline(always)]
    #[must_use]
    pub fn keepconnect(&mut self) -> KeepconnectW<Usb3DctlSpec> {
        KeepconnectW::new(self, 19)
    }
    #[doc = "Bits 20:23 - LPM NYET Threshold Handshake response to LPM token specified by device application. Response depends on DCFG.LPMCap. DCFG.LPMCap is 1'b0 - The core always responds with Timeout (that is, no response). DCFG.LPMCap is 1'b1 - The core responds with an ACK on successful LPM transaction, which requires that all of the following are satisfied: 1. There are no PID or CRC5 errors in both the EXT token and the LPM token (if not true, inactivity results in a timeout ERROR). 2. No data is pending in the Transmit FIFO and OUT endpoints not in flow controlled state (else NYET). 3. The BESL value in the LPM token is less than or equal to LPM_NYET_thres\\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn lpm_nyet_thres(&mut self) -> LpmNyetThresW<Usb3DctlSpec> {
        LpmNyetThresW::new(self, 20)
    }
    #[doc = "Bits 24:28 - HIRD Threshold The core asserts output signals utmi_l1_suspend_n and utmi_sleep_n on the basis of this signal: The core asserts utmi_l1_suspend_n to put the PHY into Deep Low-Power mode in L1 when both of the following are true: 1. HIRD value is greater than or equal to the value in DCTL.HIRD_Thres\\[3:0\\]
2. HIRD_Thres\\[4\\]
is set to 1'b1. The core asserts utmi_sleep_n on L1 when one of the following is true: 1. If the HIRD value is less than HIRD_Thres\\[3:0\\]
or 2. HIRD_Thres\\[4\\]
is set to 1'b0. Note: This field must be set to '0' during SuperSpeed mode of operation."]
    #[inline(always)]
    #[must_use]
    pub fn hirdthres(&mut self) -> HirdthresW<Usb3DctlSpec> {
        HirdthresW::new(self, 24)
    }
    #[doc = "Bit 30 - Core Soft Reset Reset all clock domains as follows: 1. This bit clears the interrupts and all the CSRs except GSTS, GSNPSID, GGPIO, GUID, GUSB2PHYCFGn registers, GUSB3PIPECTLn registers, DCFG, DCTL, DEVTEN, and DSTS registers. 2. All module state machines (except the SoC Bus Slave Unit) are reset to the IDLE state, and all the TxFIFOs and the RxFIFO are flushed. 3. Any transactions on the SoC bus Master are terminated as soon as possible, after gracefully completing the last data phase of a SoC bus transfer. Any transactions on the USB are terminated immediately. The application can write this bit at any time to reset the core. This is a self-clearing bit; the core clears this bit after all necessary logic is reset in the core, which may take several clocks depending on the core's current state. Once this bit is cleared, the software must wait at least 3 PHY clocks before accessing the PHY domain (synchronization delay). Typically, software reset is used during software development and also when you dynamically change the PHY selection bits in the USB configuration registers listed above. When you change the PHY, the corresponding clock for the PHY is selected and used in the PHY domain. Once a new clock is selected, the PHY domain must be reset for proper operation."]
    #[inline(always)]
    #[must_use]
    pub fn csftrst(&mut self) -> CsftrstW<Usb3DctlSpec> {
        CsftrstW::new(self, 30)
    }
    #[doc = "Bit 31 - Run/Stop The software writes 1 to this bit to start the device controller operation. To stop the device controller operation, the software must remove any active transfers and write 0 to this bit. When the controller is stopped, it sets the DSTS.DevCtrlHlt bit when the core is idle and the lower layer finishes the disconnect process. The Run/Stop bit must be used in following cases as specified: 1. After power-on reset and CSR initialization, the software must write 1 to this bit to start the device controller. The controller does not signal connect to the host until this bit is set. 2. The software uses this bit to control the device controller to perform a soft disconnect. When the software writes 0 to this bit, the host does not see that the device is connected. The device controller stays in the disconnected state until the software writes 1 to this bit. The minimum duration of keeping this bit cleared is specified in the Note below. If the software attempts a connect after the soft disconnect or detects a disconnect event, it must set DCTL\\[8:5\\]
to 5 before reasserting the Run/Stop bit. 3. When the USB or Link is in a lower power state and the Two Power Rails configuration is selected, software writes 0 to this bit to indicate that it is going to turn off the Core Power Rail. After the software turns on the Core Power Rail again and re-initializes the device controller, it must set this bit to start the device controller. Note: The following is the minimum duration under various conditions for which the soft disconnect (SftDiscon) bit must be set for the USB host to detect a device disconnect: 30ms: For SuperSpeed, when the device state is Suspended, Idle, Transmit, or Receive. 10ms: For high-speed, when the device state is Suspended, Idle, or not Idle/Suspended (performing transactions); For full- speed/low-speed, when the device state is Suspended, Idle, or not Idle/Supended (performing transactions) To accommodate clock jitter, it is recommended that the application add extra delay to the specified minimum duration."]
    #[inline(always)]
    #[must_use]
    pub fn run_stop(&mut self) -> RunStopW<Usb3DctlSpec> {
        RunStopW::new(self, 31)
    }
}
#[doc = "Device Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_dctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_dctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3DctlSpec;
impl crate::RegisterSpec for Usb3DctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_dctl::R`](R) reader structure"]
impl crate::Readable for Usb3DctlSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_dctl::W`](W) writer structure"]
impl crate::Writable for Usb3DctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_DCTL to value 0x00f0_0000"]
impl crate::Resettable for Usb3DctlSpec {
    const RESET_VALUE: u32 = 0x00f0_0000;
}
