#[doc = "Register `DCTL` reader"]
pub type R = crate::R<DctlSpec>;
#[doc = "Register `DCTL` writer"]
pub type W = crate::W<DctlSpec>;
#[doc = "Test Control\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tstctl {
    #[doc = "0: Test mode disabled"]
    B000 = 0,
    #[doc = "1: Test_J mode"]
    B001 = 1,
    #[doc = "2: Test_K mode"]
    B010 = 2,
    #[doc = "3: Test_SE0_NAK mode"]
    B011 = 3,
    #[doc = "4: Test_Packet mode"]
    B100 = 4,
    #[doc = "5: Test_Force_Enable Others: Reserved"]
    B101 = 5,
}
impl From<Tstctl> for u8 {
    #[inline(always)]
    fn from(variant: Tstctl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tstctl {
    type Ux = u8;
}
#[doc = "Field `TSTCTL` reader - Test Control"]
pub type TstctlR = crate::FieldReader<Tstctl>;
impl TstctlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tstctl> {
        match self.bits {
            0 => Some(Tstctl::B000),
            1 => Some(Tstctl::B001),
            2 => Some(Tstctl::B010),
            3 => Some(Tstctl::B011),
            4 => Some(Tstctl::B100),
            5 => Some(Tstctl::B101),
            _ => None,
        }
    }
    #[doc = "Test mode disabled"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Tstctl::B000
    }
    #[doc = "Test_J mode"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Tstctl::B001
    }
    #[doc = "Test_K mode"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Tstctl::B010
    }
    #[doc = "Test_SE0_NAK mode"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Tstctl::B011
    }
    #[doc = "Test_Packet mode"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Tstctl::B100
    }
    #[doc = "Test_Force_Enable Others: Reserved"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Tstctl::B101
    }
}
#[doc = "Field `TSTCTL` writer - Test Control"]
pub type TstctlW<'a, REG> = crate::FieldWriter<'a, REG, 4, Tstctl>;
impl<'a, REG> TstctlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Test mode disabled"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Tstctl::B000)
    }
    #[doc = "Test_J mode"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(Tstctl::B001)
    }
    #[doc = "Test_K mode"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(Tstctl::B010)
    }
    #[doc = "Test_SE0_NAK mode"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Tstctl::B011)
    }
    #[doc = "Test_Packet mode"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Tstctl::B100)
    }
    #[doc = "Test_Force_Enable Others: Reserved"]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(Tstctl::B101)
    }
}
#[doc = "Field `ULSTCHNGREQ` reader - ULSTCHNGREQ\n\nSoftware writes this field to issue a USB/Link state change\n\nrequest. A change in this field indicates a new request to the\n\ncore.\n\nIf software wants to issue the same request back-to-back, it must\n\nwrite a 0 to this field between the two requests. The result of the\n\nstate change request is reflected in the USB/Link State in DSTS.\n\nThese bits are self-cleared on the MAC Layer exiting suspended\n\nstate.\n\nIf software is updating other fields of the DCTL register and not\n\nintending to force any link state change, then it must write a 0 to\n\nthis field.\n\nSS Compliance mode is normally entered and controlled by the\n\nremote link partner. Refer to the USB 3.0 specification.\n\nAlternatively, you can force the local link directly into compliance\n\nmode, by resetting the SS link with the RUN/STOP bit set to zero.\n\nIf you then write 10 to the USB/Link State Change field and 1 to\n\nRUN/STOP, the link goes to compliance mode.\n\nOnce you are in compliance, you may alternately write zero and\n\n10 to this field to advance the compliance pattern.\n\nIn SS mode:\n\nValue: Requested Link State Transition/Action\n\n0: No Action\n\n4: SS.Disabled\n\n5: Rx.Detect\n\n6: SS.Inactive\n\n8: Recovery\n\n10: Compliance\n\nOthers: Reserved\n\nIn HS/FS/LS mode:\n\nValue:Requested USB state transition\n\n8: Remote wakeup request\n\nOthers: Reserved\n\nThe Remote wakeup request must be issued 2us after the device\n\ngoes into suspend state (DSTS\\[21:18\\]
is 3 ).\n\nNote: After coming out of hibernation, software must write 8\n\n(Recovery) into this field to confirm exit from the suspended\n\nstate."]
pub type UlstchngreqR = crate::FieldReader;
#[doc = "Field `ULSTCHNGREQ` writer - ULSTCHNGREQ\n\nSoftware writes this field to issue a USB/Link state change\n\nrequest. A change in this field indicates a new request to the\n\ncore.\n\nIf software wants to issue the same request back-to-back, it must\n\nwrite a 0 to this field between the two requests. The result of the\n\nstate change request is reflected in the USB/Link State in DSTS.\n\nThese bits are self-cleared on the MAC Layer exiting suspended\n\nstate.\n\nIf software is updating other fields of the DCTL register and not\n\nintending to force any link state change, then it must write a 0 to\n\nthis field.\n\nSS Compliance mode is normally entered and controlled by the\n\nremote link partner. Refer to the USB 3.0 specification.\n\nAlternatively, you can force the local link directly into compliance\n\nmode, by resetting the SS link with the RUN/STOP bit set to zero.\n\nIf you then write 10 to the USB/Link State Change field and 1 to\n\nRUN/STOP, the link goes to compliance mode.\n\nOnce you are in compliance, you may alternately write zero and\n\n10 to this field to advance the compliance pattern.\n\nIn SS mode:\n\nValue: Requested Link State Transition/Action\n\n0: No Action\n\n4: SS.Disabled\n\n5: Rx.Detect\n\n6: SS.Inactive\n\n8: Recovery\n\n10: Compliance\n\nOthers: Reserved\n\nIn HS/FS/LS mode:\n\nValue:Requested USB state transition\n\n8: Remote wakeup request\n\nOthers: Reserved\n\nThe Remote wakeup request must be issued 2us after the device\n\ngoes into suspend state (DSTS\\[21:18\\]
is 3 ).\n\nNote: After coming out of hibernation, software must write 8\n\n(Recovery) into this field to confirm exit from the suspended\n\nstate."]
pub type UlstchngreqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Accept U1 Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acceptu1ena {
    #[doc = "0: Core rejects U1 except when Force_LinkPM_Accept bit is set (default)"]
    B0 = 0,
    #[doc = "1: Core accepts transition to U1 state if nothing is pending on the application side. On USB reset, hardware clears this bit to 0. Software sets this bit after receiving a SetConfiguration command."]
    B1 = 1,
}
impl From<Acceptu1ena> for bool {
    #[inline(always)]
    fn from(variant: Acceptu1ena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCEPTU1ENA` reader - Accept U1 Enable"]
pub type Acceptu1enaR = crate::BitReader<Acceptu1ena>;
impl Acceptu1enaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acceptu1ena {
        match self.bits {
            false => Acceptu1ena::B0,
            true => Acceptu1ena::B1,
        }
    }
    #[doc = "Core rejects U1 except when Force_LinkPM_Accept bit is set (default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Acceptu1ena::B0
    }
    #[doc = "Core accepts transition to U1 state if nothing is pending on the application side. On USB reset, hardware clears this bit to 0. Software sets this bit after receiving a SetConfiguration command."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Acceptu1ena::B1
    }
}
#[doc = "Field `ACCEPTU1ENA` writer - Accept U1 Enable"]
pub type Acceptu1enaW<'a, REG> = crate::BitWriter<'a, REG, Acceptu1ena>;
impl<'a, REG> Acceptu1enaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Core rejects U1 except when Force_LinkPM_Accept bit is set (default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Acceptu1ena::B0)
    }
    #[doc = "Core accepts transition to U1 state if nothing is pending on the application side. On USB reset, hardware clears this bit to 0. Software sets this bit after receiving a SetConfiguration command."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Acceptu1ena::B1)
    }
}
#[doc = "Initiate U1 Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Initu1ena {
    #[doc = "0: May not initiate U1 (default);"]
    B0 = 0,
    #[doc = "1: May initiate U1. On USB reset, hardware clears this bit to 0. Software sets this bit after receiving SetFeature(U1_ENABLE), and clears this bit when ClearFeature(U1_ENABLE) is received. If DCTL\\[9\\]
(AcceptU1Ena) is 0, the link immediately exits U1 state."]
    B1 = 1,
}
impl From<Initu1ena> for bool {
    #[inline(always)]
    fn from(variant: Initu1ena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITU1ENA` reader - Initiate U1 Enable"]
pub type Initu1enaR = crate::BitReader<Initu1ena>;
impl Initu1enaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Initu1ena {
        match self.bits {
            false => Initu1ena::B0,
            true => Initu1ena::B1,
        }
    }
    #[doc = "May not initiate U1 (default);"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Initu1ena::B0
    }
    #[doc = "May initiate U1. On USB reset, hardware clears this bit to 0. Software sets this bit after receiving SetFeature(U1_ENABLE), and clears this bit when ClearFeature(U1_ENABLE) is received. If DCTL\\[9\\]
(AcceptU1Ena) is 0, the link immediately exits U1 state."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Initu1ena::B1
    }
}
#[doc = "Field `INITU1ENA` writer - Initiate U1 Enable"]
pub type Initu1enaW<'a, REG> = crate::BitWriter<'a, REG, Initu1ena>;
impl<'a, REG> Initu1enaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "May not initiate U1 (default);"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Initu1ena::B0)
    }
    #[doc = "May initiate U1. On USB reset, hardware clears this bit to 0. Software sets this bit after receiving SetFeature(U1_ENABLE), and clears this bit when ClearFeature(U1_ENABLE) is received. If DCTL\\[9\\]
(AcceptU1Ena) is 0, the link immediately exits U1 state."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Initu1ena::B1)
    }
}
#[doc = "Accept U2 Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acceptu2ena {
    #[doc = "0: Reject U2 except when Force_LinkPM_Accept bit is set (default)"]
    B0 = 0,
    #[doc = "1: Core accepts transition to U2 state if nothing is pending on the application side. On USB reset, hardware clears this bit to 0. Software sets this bit after receiving a SetConfiguration command."]
    B1 = 1,
}
impl From<Acceptu2ena> for bool {
    #[inline(always)]
    fn from(variant: Acceptu2ena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCEPTU2ENA` reader - Accept U2 Enable"]
pub type Acceptu2enaR = crate::BitReader<Acceptu2ena>;
impl Acceptu2enaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acceptu2ena {
        match self.bits {
            false => Acceptu2ena::B0,
            true => Acceptu2ena::B1,
        }
    }
    #[doc = "Reject U2 except when Force_LinkPM_Accept bit is set (default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Acceptu2ena::B0
    }
    #[doc = "Core accepts transition to U2 state if nothing is pending on the application side. On USB reset, hardware clears this bit to 0. Software sets this bit after receiving a SetConfiguration command."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Acceptu2ena::B1
    }
}
#[doc = "Field `ACCEPTU2ENA` writer - Accept U2 Enable"]
pub type Acceptu2enaW<'a, REG> = crate::BitWriter<'a, REG, Acceptu2ena>;
impl<'a, REG> Acceptu2enaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reject U2 except when Force_LinkPM_Accept bit is set (default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Acceptu2ena::B0)
    }
    #[doc = "Core accepts transition to U2 state if nothing is pending on the application side. On USB reset, hardware clears this bit to 0. Software sets this bit after receiving a SetConfiguration command."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Acceptu2ena::B1)
    }
}
#[doc = "Initiate U2 Enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Initu2ena {
    #[doc = "0: May not initiate U2 (default)"]
    B0 = 0,
    #[doc = "1: May initiate U2 On USB reset, hardware clears this bit to 0. Software sets this bit after receiving SetFeature(U2_ENABLE), and clears this bit when ClearFeature(U2_ENABLE) is received. If DCTL\\[11\\]
(AcceptU2Ena) is 0, the link immediately exits U2 state."]
    B1 = 1,
}
impl From<Initu2ena> for bool {
    #[inline(always)]
    fn from(variant: Initu2ena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITU2ENA` reader - Initiate U2 Enable"]
pub type Initu2enaR = crate::BitReader<Initu2ena>;
impl Initu2enaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Initu2ena {
        match self.bits {
            false => Initu2ena::B0,
            true => Initu2ena::B1,
        }
    }
    #[doc = "May not initiate U2 (default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Initu2ena::B0
    }
    #[doc = "May initiate U2 On USB reset, hardware clears this bit to 0. Software sets this bit after receiving SetFeature(U2_ENABLE), and clears this bit when ClearFeature(U2_ENABLE) is received. If DCTL\\[11\\]
(AcceptU2Ena) is 0, the link immediately exits U2 state."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Initu2ena::B1
    }
}
#[doc = "Field `INITU2ENA` writer - Initiate U2 Enable"]
pub type Initu2enaW<'a, REG> = crate::BitWriter<'a, REG, Initu2ena>;
impl<'a, REG> Initu2enaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "May not initiate U2 (default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Initu2ena::B0)
    }
    #[doc = "May initiate U2 On USB reset, hardware clears this bit to 0. Software sets this bit after receiving SetFeature(U2_ENABLE), and clears this bit when ClearFeature(U2_ENABLE) is received. If DCTL\\[11\\]
(AcceptU2Ena) is 0, the link immediately exits U2 state."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Initu2ena::B1)
    }
}
#[doc = "Field `CSS` reader - Controller Save State\n\nThis command is similar to the USBCMD.CSS bit in host mode\n\nand initiates the save process. When software sets this bit to 1,\n\nthe controller immediately sets DSTS.SSS to 1. When the\n\ncontroller has finished the save process, it sets DSTS.SSS to 0.\n\nNote: When read, this field always returns 0."]
pub type CssR = crate::BitReader;
#[doc = "Field `CSS` writer - Controller Save State\n\nThis command is similar to the USBCMD.CSS bit in host mode\n\nand initiates the save process. When software sets this bit to 1,\n\nthe controller immediately sets DSTS.SSS to 1. When the\n\ncontroller has finished the save process, it sets DSTS.SSS to 0.\n\nNote: When read, this field always returns 0."]
pub type CssW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRS` reader - Controller Restore State\n\nThis command is similar to the USBCMD.CRS bit in host mode\n\nand initiates the restore process. When software sets this bit to 1,\n\nthe controller immediately sets DSTS.RSS to 1. When the\n\ncontroller has finished the restore process, it sets DSTS.RSS to 0.\n\nNote: When read, this field always returns 0."]
pub type CrsR = crate::BitReader;
#[doc = "Field `CRS` writer - Controller Restore State\n\nThis command is similar to the USBCMD.CRS bit in host mode\n\nand initiates the restore process. When software sets this bit to 1,\n\nthe controller immediately sets DSTS.RSS to 1. When the\n\ncontroller has finished the restore process, it sets DSTS.RSS to 0.\n\nNote: When read, this field always returns 0."]
pub type CrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1HIBERNATIONEN` reader - L1HibernationEn\n\nWhen this bit is set along with KeepConnect, the device core\n\ngenerates a Hibernation Request Event if L1 is enabled and the\n\nHIRD value in the LPM token is larger than the threshold\n\nprogrammed in DCTL.HIRD_Thres.\n\nThe core does not exit the LPM L1 state until software writes\n\nRecovery into the DCTL.ULStChngReq field.\n\nThis prevents corner cases where the device is entering\n\nhibernation at the same time the host is attempting to exit L1."]
pub type L1hibernationenR = crate::BitReader;
#[doc = "Field `L1HIBERNATIONEN` writer - L1HibernationEn\n\nWhen this bit is set along with KeepConnect, the device core\n\ngenerates a Hibernation Request Event if L1 is enabled and the\n\nHIRD value in the LPM token is larger than the threshold\n\nprogrammed in DCTL.HIRD_Thres.\n\nThe core does not exit the LPM L1 state until software writes\n\nRecovery into the DCTL.ULStChngReq field.\n\nThis prevents corner cases where the device is entering\n\nhibernation at the same time the host is attempting to exit L1."]
pub type L1hibernationenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEEPCONNECT` reader - KeepConnect\n\nWhen 1, this bit enables the save and restore programming\n\nmodel by preventing the core from disconnecting from the host\n\nwhen DCTL.RunStop is set to 0. It also enables the Hibernation\n\nRequest Event to be generated when the link goes to U3 or L2.\n\nThe device core disconnects from the host when DCTL.RunStop is\n\nset to 0.\n\nThis bit indicates whether to preserve this behavior (0), or if the\n\ncore must not disconnect when RunStop is set to 0 (1).\n\nThis bit also prevents the LTSSM from automatically going to\n\nU0/L0 when the host requests resume from U3/L2."]
pub type KeepconnectR = crate::BitReader;
#[doc = "Field `KEEPCONNECT` writer - KeepConnect\n\nWhen 1, this bit enables the save and restore programming\n\nmodel by preventing the core from disconnecting from the host\n\nwhen DCTL.RunStop is set to 0. It also enables the Hibernation\n\nRequest Event to be generated when the link goes to U3 or L2.\n\nThe device core disconnects from the host when DCTL.RunStop is\n\nset to 0.\n\nThis bit indicates whether to preserve this behavior (0), or if the\n\ncore must not disconnect when RunStop is set to 0 (1).\n\nThis bit also prevents the LTSSM from automatically going to\n\nU0/L0 when the host requests resume from U3/L2."]
pub type KeepconnectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM_NYET_THRES` reader - LPM NYET Threshold\n\nHandshake response to LPM token specified by device application.\n\nResponse depends on DCFG.LPMCap.\n\nDCFG.LPMCap is 1'b0 - The core always responds with Timeout\n\n(that is, no response).\n\nDCFG.LPMCap is 1'b1 - The core responds with an ACK on\n\nsuccessful LPM transaction, which requires that all of the\n\nfollowing are satisfied:\n\n1. There are no PID or CRC5 errors in both the EXT token and the\n\nLPM token (if not true, inactivity results in a timeout ERROR).\n\n2. No data is pending in the Transmit FIFO and OUT endpoints\n\nnot in flow controlled state (else NYET).\n\n3. The BESL value in the LPM token is less than or equal to\n\nLPM_NYET_thres\\[3:0\\]"]
pub type LpmNyetThresR = crate::FieldReader;
#[doc = "Field `LPM_NYET_THRES` writer - LPM NYET Threshold\n\nHandshake response to LPM token specified by device application.\n\nResponse depends on DCFG.LPMCap.\n\nDCFG.LPMCap is 1'b0 - The core always responds with Timeout\n\n(that is, no response).\n\nDCFG.LPMCap is 1'b1 - The core responds with an ACK on\n\nsuccessful LPM transaction, which requires that all of the\n\nfollowing are satisfied:\n\n1. There are no PID or CRC5 errors in both the EXT token and the\n\nLPM token (if not true, inactivity results in a timeout ERROR).\n\n2. No data is pending in the Transmit FIFO and OUT endpoints\n\nnot in flow controlled state (else NYET).\n\n3. The BESL value in the LPM token is less than or equal to\n\nLPM_NYET_thres\\[3:0\\]"]
pub type LpmNyetThresW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HIRDTHRES` reader - HIRD Threshold\n\nThe core asserts output signals utmi_l1_suspend_n and\n\nutmi_sleep_n on the basis of this signal:\n\nThe core asserts utmi_l1_suspend_n to put the PHY into Deep\n\nLow-Power mode in L1 when both of the following are true:\n\n1. HIRD value is greater than or equal to the value in\n\nDCTL.HIRD_Thres\\[3:0\\]\n\n2. HIRD_Thres\\[4\\]
is set to 1'b1.\n\nThe core asserts utmi_sleep_n on L1 when one of the following is\n\ntrue:\n\n1. If the HIRD value is less than HIRD_Thres\\[3:0\\]
or\n\n2. HIRD_Thres\\[4\\]
is set to 1'b0.\n\nNote: This field must be set to '0' during SuperSpeed mode of\n\noperation."]
pub type HirdthresR = crate::FieldReader;
#[doc = "Field `HIRDTHRES` writer - HIRD Threshold\n\nThe core asserts output signals utmi_l1_suspend_n and\n\nutmi_sleep_n on the basis of this signal:\n\nThe core asserts utmi_l1_suspend_n to put the PHY into Deep\n\nLow-Power mode in L1 when both of the following are true:\n\n1. HIRD value is greater than or equal to the value in\n\nDCTL.HIRD_Thres\\[3:0\\]\n\n2. HIRD_Thres\\[4\\]
is set to 1'b1.\n\nThe core asserts utmi_sleep_n on L1 when one of the following is\n\ntrue:\n\n1. If the HIRD value is less than HIRD_Thres\\[3:0\\]
or\n\n2. HIRD_Thres\\[4\\]
is set to 1'b0.\n\nNote: This field must be set to '0' during SuperSpeed mode of\n\noperation."]
pub type HirdthresW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CSFTRST` reader - Core Soft Reset\n\nReset all clock domains as follows:\n\n1. This bit clears the interrupts and all the CSRs except GSTS,\n\nGSNPSID, GGPIO, GUID, GUSB2PHYCFGn registers,\n\nGUSB3PIPECTLn registers, DCFG, DCTL, DEVTEN, and DSTS\n\nregisters.\n\n2. All module state machines (except the SoC Bus Slave Unit) are\n\nreset to the IDLE state, and all the TxFIFOs and the RxFIFO are\n\nflushed.\n\n3. Any transactions on the SoC bus Master are terminated as\n\nsoon as possible, after gracefully completing the last data phase\n\nof a SoC bus transfer. Any transactions on the USB are\n\nterminated immediately.\n\nThe application can write this bit at any time to reset the core.\n\nThis is a self-clearing bit; the core clears this bit after all\n\nnecessary logic is reset in the core, which may take several\n\nclocks depending on the core's current state. Once this bit is\n\ncleared, the software must wait at least 3 PHY clocks before\n\naccessing the PHY domain (synchronization delay).\n\nTypically, software reset is used during software development\n\nand also when you dynamically change the PHY selection bits in\n\nthe USB configuration registers listed above. When you change\n\nthe PHY, the corresponding clock for the PHY is selected and used\n\nin the PHY domain. Once a new clock is selected, the PHY domain\n\nmust be reset for proper operation."]
pub type CsftrstR = crate::BitReader;
#[doc = "Field `CSFTRST` writer - Core Soft Reset\n\nReset all clock domains as follows:\n\n1. This bit clears the interrupts and all the CSRs except GSTS,\n\nGSNPSID, GGPIO, GUID, GUSB2PHYCFGn registers,\n\nGUSB3PIPECTLn registers, DCFG, DCTL, DEVTEN, and DSTS\n\nregisters.\n\n2. All module state machines (except the SoC Bus Slave Unit) are\n\nreset to the IDLE state, and all the TxFIFOs and the RxFIFO are\n\nflushed.\n\n3. Any transactions on the SoC bus Master are terminated as\n\nsoon as possible, after gracefully completing the last data phase\n\nof a SoC bus transfer. Any transactions on the USB are\n\nterminated immediately.\n\nThe application can write this bit at any time to reset the core.\n\nThis is a self-clearing bit; the core clears this bit after all\n\nnecessary logic is reset in the core, which may take several\n\nclocks depending on the core's current state. Once this bit is\n\ncleared, the software must wait at least 3 PHY clocks before\n\naccessing the PHY domain (synchronization delay).\n\nTypically, software reset is used during software development\n\nand also when you dynamically change the PHY selection bits in\n\nthe USB configuration registers listed above. When you change\n\nthe PHY, the corresponding clock for the PHY is selected and used\n\nin the PHY domain. Once a new clock is selected, the PHY domain\n\nmust be reset for proper operation."]
pub type CsftrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUN_STOP` reader - Run/Stop\n\nThe software writes 1 to this bit to start the device controller\n\noperation.\n\nTo stop the device controller operation, the software must\n\nremove any active transfers and write 0 to this bit. When the\n\ncontroller is stopped, it sets the DSTS.DevCtrlHlt bit when the\n\ncore is idle and the lower layer finishes the disconnect process.\n\nThe Run/Stop bit must be used in following cases as specified:\n\n1. After power-on reset and CSR initialization, the software must\n\nwrite 1 to this bit to start the device controller. The controller\n\ndoes not signal connect to the host until this bit is set.\n\n2. The software uses this bit to control the device controller to\n\nperform a soft disconnect. When the software writes 0 to this bit,\n\nthe host does not see that the device is connected. The device\n\ncontroller stays in the disconnected state until the software writes\n\n1 to this bit.\n\nThe minimum duration of keeping this bit cleared is specified in\n\nthe Note below. If the software attempts a connect after the soft\n\ndisconnect or detects a disconnect event, it must set DCTL\\[8:5\\]\n\nto 5 before reasserting the Run/Stop bit.\n\n3. When the USB or Link is in a lower power state and the Two\n\nPower Rails configuration is selected, software writes 0 to this bit\n\nto indicate that it is going to turn off the Core Power Rail. After\n\nthe software turns on the Core Power Rail again and re-initializes\n\nthe device controller, it must set this bit to start the device\n\ncontroller.\n\nNote: The following is the minimum duration under various\n\nconditions for which the soft disconnect (SftDiscon) bit must be\n\nset for the USB host to detect a device disconnect:\n\n30ms: For SuperSpeed, when the device state is Suspended,\n\nIdle, Transmit, or Receive.\n\n10ms: For high-speed, when the device state is Suspended, Idle,\n\nor not Idle/Suspended (performing transactions); For full-\n\nspeed/low-speed, when the device state is Suspended, Idle, or\n\nnot Idle/Supended (performing transactions)\n\nTo accommodate clock jitter, it is recommended that the\n\napplication add extra delay to the specified minimum duration."]
pub type RunStopR = crate::BitReader;
#[doc = "Field `RUN_STOP` writer - Run/Stop\n\nThe software writes 1 to this bit to start the device controller\n\noperation.\n\nTo stop the device controller operation, the software must\n\nremove any active transfers and write 0 to this bit. When the\n\ncontroller is stopped, it sets the DSTS.DevCtrlHlt bit when the\n\ncore is idle and the lower layer finishes the disconnect process.\n\nThe Run/Stop bit must be used in following cases as specified:\n\n1. After power-on reset and CSR initialization, the software must\n\nwrite 1 to this bit to start the device controller. The controller\n\ndoes not signal connect to the host until this bit is set.\n\n2. The software uses this bit to control the device controller to\n\nperform a soft disconnect. When the software writes 0 to this bit,\n\nthe host does not see that the device is connected. The device\n\ncontroller stays in the disconnected state until the software writes\n\n1 to this bit.\n\nThe minimum duration of keeping this bit cleared is specified in\n\nthe Note below. If the software attempts a connect after the soft\n\ndisconnect or detects a disconnect event, it must set DCTL\\[8:5\\]\n\nto 5 before reasserting the Run/Stop bit.\n\n3. When the USB or Link is in a lower power state and the Two\n\nPower Rails configuration is selected, software writes 0 to this bit\n\nto indicate that it is going to turn off the Core Power Rail. After\n\nthe software turns on the Core Power Rail again and re-initializes\n\nthe device controller, it must set this bit to start the device\n\ncontroller.\n\nNote: The following is the minimum duration under various\n\nconditions for which the soft disconnect (SftDiscon) bit must be\n\nset for the USB host to detect a device disconnect:\n\n30ms: For SuperSpeed, when the device state is Suspended,\n\nIdle, Transmit, or Receive.\n\n10ms: For high-speed, when the device state is Suspended, Idle,\n\nor not Idle/Suspended (performing transactions); For full-\n\nspeed/low-speed, when the device state is Suspended, Idle, or\n\nnot Idle/Supended (performing transactions)\n\nTo accommodate clock jitter, it is recommended that the\n\napplication add extra delay to the specified minimum duration."]
pub type RunStopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 1:4 - Test Control"]
    #[inline(always)]
    pub fn tstctl(&self) -> TstctlR {
        TstctlR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:8 - ULSTCHNGREQ\n\nSoftware writes this field to issue a USB/Link state change\n\nrequest. A change in this field indicates a new request to the\n\ncore.\n\nIf software wants to issue the same request back-to-back, it must\n\nwrite a 0 to this field between the two requests. The result of the\n\nstate change request is reflected in the USB/Link State in DSTS.\n\nThese bits are self-cleared on the MAC Layer exiting suspended\n\nstate.\n\nIf software is updating other fields of the DCTL register and not\n\nintending to force any link state change, then it must write a 0 to\n\nthis field.\n\nSS Compliance mode is normally entered and controlled by the\n\nremote link partner. Refer to the USB 3.0 specification.\n\nAlternatively, you can force the local link directly into compliance\n\nmode, by resetting the SS link with the RUN/STOP bit set to zero.\n\nIf you then write 10 to the USB/Link State Change field and 1 to\n\nRUN/STOP, the link goes to compliance mode.\n\nOnce you are in compliance, you may alternately write zero and\n\n10 to this field to advance the compliance pattern.\n\nIn SS mode:\n\nValue: Requested Link State Transition/Action\n\n0: No Action\n\n4: SS.Disabled\n\n5: Rx.Detect\n\n6: SS.Inactive\n\n8: Recovery\n\n10: Compliance\n\nOthers: Reserved\n\nIn HS/FS/LS mode:\n\nValue:Requested USB state transition\n\n8: Remote wakeup request\n\nOthers: Reserved\n\nThe Remote wakeup request must be issued 2us after the device\n\ngoes into suspend state (DSTS\\[21:18\\]
is 3 ).\n\nNote: After coming out of hibernation, software must write 8\n\n(Recovery) into this field to confirm exit from the suspended\n\nstate."]
    #[inline(always)]
    pub fn ulstchngreq(&self) -> UlstchngreqR {
        UlstchngreqR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Accept U1 Enable"]
    #[inline(always)]
    pub fn acceptu1ena(&self) -> Acceptu1enaR {
        Acceptu1enaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Initiate U1 Enable"]
    #[inline(always)]
    pub fn initu1ena(&self) -> Initu1enaR {
        Initu1enaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Accept U2 Enable"]
    #[inline(always)]
    pub fn acceptu2ena(&self) -> Acceptu2enaR {
        Acceptu2enaR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Initiate U2 Enable"]
    #[inline(always)]
    pub fn initu2ena(&self) -> Initu2enaR {
        Initu2enaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Controller Save State\n\nThis command is similar to the USBCMD.CSS bit in host mode\n\nand initiates the save process. When software sets this bit to 1,\n\nthe controller immediately sets DSTS.SSS to 1. When the\n\ncontroller has finished the save process, it sets DSTS.SSS to 0.\n\nNote: When read, this field always returns 0."]
    #[inline(always)]
    pub fn css(&self) -> CssR {
        CssR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Controller Restore State\n\nThis command is similar to the USBCMD.CRS bit in host mode\n\nand initiates the restore process. When software sets this bit to 1,\n\nthe controller immediately sets DSTS.RSS to 1. When the\n\ncontroller has finished the restore process, it sets DSTS.RSS to 0.\n\nNote: When read, this field always returns 0."]
    #[inline(always)]
    pub fn crs(&self) -> CrsR {
        CrsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - L1HibernationEn\n\nWhen this bit is set along with KeepConnect, the device core\n\ngenerates a Hibernation Request Event if L1 is enabled and the\n\nHIRD value in the LPM token is larger than the threshold\n\nprogrammed in DCTL.HIRD_Thres.\n\nThe core does not exit the LPM L1 state until software writes\n\nRecovery into the DCTL.ULStChngReq field.\n\nThis prevents corner cases where the device is entering\n\nhibernation at the same time the host is attempting to exit L1."]
    #[inline(always)]
    pub fn l1hibernationen(&self) -> L1hibernationenR {
        L1hibernationenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - KeepConnect\n\nWhen 1, this bit enables the save and restore programming\n\nmodel by preventing the core from disconnecting from the host\n\nwhen DCTL.RunStop is set to 0. It also enables the Hibernation\n\nRequest Event to be generated when the link goes to U3 or L2.\n\nThe device core disconnects from the host when DCTL.RunStop is\n\nset to 0.\n\nThis bit indicates whether to preserve this behavior (0), or if the\n\ncore must not disconnect when RunStop is set to 0 (1).\n\nThis bit also prevents the LTSSM from automatically going to\n\nU0/L0 when the host requests resume from U3/L2."]
    #[inline(always)]
    pub fn keepconnect(&self) -> KeepconnectR {
        KeepconnectR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - LPM NYET Threshold\n\nHandshake response to LPM token specified by device application.\n\nResponse depends on DCFG.LPMCap.\n\nDCFG.LPMCap is 1'b0 - The core always responds with Timeout\n\n(that is, no response).\n\nDCFG.LPMCap is 1'b1 - The core responds with an ACK on\n\nsuccessful LPM transaction, which requires that all of the\n\nfollowing are satisfied:\n\n1. There are no PID or CRC5 errors in both the EXT token and the\n\nLPM token (if not true, inactivity results in a timeout ERROR).\n\n2. No data is pending in the Transmit FIFO and OUT endpoints\n\nnot in flow controlled state (else NYET).\n\n3. The BESL value in the LPM token is less than or equal to\n\nLPM_NYET_thres\\[3:0\\]"]
    #[inline(always)]
    pub fn lpm_nyet_thres(&self) -> LpmNyetThresR {
        LpmNyetThresR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:28 - HIRD Threshold\n\nThe core asserts output signals utmi_l1_suspend_n and\n\nutmi_sleep_n on the basis of this signal:\n\nThe core asserts utmi_l1_suspend_n to put the PHY into Deep\n\nLow-Power mode in L1 when both of the following are true:\n\n1. HIRD value is greater than or equal to the value in\n\nDCTL.HIRD_Thres\\[3:0\\]\n\n2. HIRD_Thres\\[4\\]
is set to 1'b1.\n\nThe core asserts utmi_sleep_n on L1 when one of the following is\n\ntrue:\n\n1. If the HIRD value is less than HIRD_Thres\\[3:0\\]
or\n\n2. HIRD_Thres\\[4\\]
is set to 1'b0.\n\nNote: This field must be set to '0' during SuperSpeed mode of\n\noperation."]
    #[inline(always)]
    pub fn hirdthres(&self) -> HirdthresR {
        HirdthresR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - Core Soft Reset\n\nReset all clock domains as follows:\n\n1. This bit clears the interrupts and all the CSRs except GSTS,\n\nGSNPSID, GGPIO, GUID, GUSB2PHYCFGn registers,\n\nGUSB3PIPECTLn registers, DCFG, DCTL, DEVTEN, and DSTS\n\nregisters.\n\n2. All module state machines (except the SoC Bus Slave Unit) are\n\nreset to the IDLE state, and all the TxFIFOs and the RxFIFO are\n\nflushed.\n\n3. Any transactions on the SoC bus Master are terminated as\n\nsoon as possible, after gracefully completing the last data phase\n\nof a SoC bus transfer. Any transactions on the USB are\n\nterminated immediately.\n\nThe application can write this bit at any time to reset the core.\n\nThis is a self-clearing bit; the core clears this bit after all\n\nnecessary logic is reset in the core, which may take several\n\nclocks depending on the core's current state. Once this bit is\n\ncleared, the software must wait at least 3 PHY clocks before\n\naccessing the PHY domain (synchronization delay).\n\nTypically, software reset is used during software development\n\nand also when you dynamically change the PHY selection bits in\n\nthe USB configuration registers listed above. When you change\n\nthe PHY, the corresponding clock for the PHY is selected and used\n\nin the PHY domain. Once a new clock is selected, the PHY domain\n\nmust be reset for proper operation."]
    #[inline(always)]
    pub fn csftrst(&self) -> CsftrstR {
        CsftrstR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Run/Stop\n\nThe software writes 1 to this bit to start the device controller\n\noperation.\n\nTo stop the device controller operation, the software must\n\nremove any active transfers and write 0 to this bit. When the\n\ncontroller is stopped, it sets the DSTS.DevCtrlHlt bit when the\n\ncore is idle and the lower layer finishes the disconnect process.\n\nThe Run/Stop bit must be used in following cases as specified:\n\n1. After power-on reset and CSR initialization, the software must\n\nwrite 1 to this bit to start the device controller. The controller\n\ndoes not signal connect to the host until this bit is set.\n\n2. The software uses this bit to control the device controller to\n\nperform a soft disconnect. When the software writes 0 to this bit,\n\nthe host does not see that the device is connected. The device\n\ncontroller stays in the disconnected state until the software writes\n\n1 to this bit.\n\nThe minimum duration of keeping this bit cleared is specified in\n\nthe Note below. If the software attempts a connect after the soft\n\ndisconnect or detects a disconnect event, it must set DCTL\\[8:5\\]\n\nto 5 before reasserting the Run/Stop bit.\n\n3. When the USB or Link is in a lower power state and the Two\n\nPower Rails configuration is selected, software writes 0 to this bit\n\nto indicate that it is going to turn off the Core Power Rail. After\n\nthe software turns on the Core Power Rail again and re-initializes\n\nthe device controller, it must set this bit to start the device\n\ncontroller.\n\nNote: The following is the minimum duration under various\n\nconditions for which the soft disconnect (SftDiscon) bit must be\n\nset for the USB host to detect a device disconnect:\n\n30ms: For SuperSpeed, when the device state is Suspended,\n\nIdle, Transmit, or Receive.\n\n10ms: For high-speed, when the device state is Suspended, Idle,\n\nor not Idle/Suspended (performing transactions); For full-\n\nspeed/low-speed, when the device state is Suspended, Idle, or\n\nnot Idle/Supended (performing transactions)\n\nTo accommodate clock jitter, it is recommended that the\n\napplication add extra delay to the specified minimum duration."]
    #[inline(always)]
    pub fn run_stop(&self) -> RunStopR {
        RunStopR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:4 - Test Control"]
    #[inline(always)]
    #[must_use]
    pub fn tstctl(&mut self) -> TstctlW<DctlSpec> {
        TstctlW::new(self, 1)
    }
    #[doc = "Bits 5:8 - ULSTCHNGREQ\n\nSoftware writes this field to issue a USB/Link state change\n\nrequest. A change in this field indicates a new request to the\n\ncore.\n\nIf software wants to issue the same request back-to-back, it must\n\nwrite a 0 to this field between the two requests. The result of the\n\nstate change request is reflected in the USB/Link State in DSTS.\n\nThese bits are self-cleared on the MAC Layer exiting suspended\n\nstate.\n\nIf software is updating other fields of the DCTL register and not\n\nintending to force any link state change, then it must write a 0 to\n\nthis field.\n\nSS Compliance mode is normally entered and controlled by the\n\nremote link partner. Refer to the USB 3.0 specification.\n\nAlternatively, you can force the local link directly into compliance\n\nmode, by resetting the SS link with the RUN/STOP bit set to zero.\n\nIf you then write 10 to the USB/Link State Change field and 1 to\n\nRUN/STOP, the link goes to compliance mode.\n\nOnce you are in compliance, you may alternately write zero and\n\n10 to this field to advance the compliance pattern.\n\nIn SS mode:\n\nValue: Requested Link State Transition/Action\n\n0: No Action\n\n4: SS.Disabled\n\n5: Rx.Detect\n\n6: SS.Inactive\n\n8: Recovery\n\n10: Compliance\n\nOthers: Reserved\n\nIn HS/FS/LS mode:\n\nValue:Requested USB state transition\n\n8: Remote wakeup request\n\nOthers: Reserved\n\nThe Remote wakeup request must be issued 2us after the device\n\ngoes into suspend state (DSTS\\[21:18\\]
is 3 ).\n\nNote: After coming out of hibernation, software must write 8\n\n(Recovery) into this field to confirm exit from the suspended\n\nstate."]
    #[inline(always)]
    #[must_use]
    pub fn ulstchngreq(&mut self) -> UlstchngreqW<DctlSpec> {
        UlstchngreqW::new(self, 5)
    }
    #[doc = "Bit 9 - Accept U1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acceptu1ena(&mut self) -> Acceptu1enaW<DctlSpec> {
        Acceptu1enaW::new(self, 9)
    }
    #[doc = "Bit 10 - Initiate U1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn initu1ena(&mut self) -> Initu1enaW<DctlSpec> {
        Initu1enaW::new(self, 10)
    }
    #[doc = "Bit 11 - Accept U2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acceptu2ena(&mut self) -> Acceptu2enaW<DctlSpec> {
        Acceptu2enaW::new(self, 11)
    }
    #[doc = "Bit 12 - Initiate U2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn initu2ena(&mut self) -> Initu2enaW<DctlSpec> {
        Initu2enaW::new(self, 12)
    }
    #[doc = "Bit 16 - Controller Save State\n\nThis command is similar to the USBCMD.CSS bit in host mode\n\nand initiates the save process. When software sets this bit to 1,\n\nthe controller immediately sets DSTS.SSS to 1. When the\n\ncontroller has finished the save process, it sets DSTS.SSS to 0.\n\nNote: When read, this field always returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn css(&mut self) -> CssW<DctlSpec> {
        CssW::new(self, 16)
    }
    #[doc = "Bit 17 - Controller Restore State\n\nThis command is similar to the USBCMD.CRS bit in host mode\n\nand initiates the restore process. When software sets this bit to 1,\n\nthe controller immediately sets DSTS.RSS to 1. When the\n\ncontroller has finished the restore process, it sets DSTS.RSS to 0.\n\nNote: When read, this field always returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn crs(&mut self) -> CrsW<DctlSpec> {
        CrsW::new(self, 17)
    }
    #[doc = "Bit 18 - L1HibernationEn\n\nWhen this bit is set along with KeepConnect, the device core\n\ngenerates a Hibernation Request Event if L1 is enabled and the\n\nHIRD value in the LPM token is larger than the threshold\n\nprogrammed in DCTL.HIRD_Thres.\n\nThe core does not exit the LPM L1 state until software writes\n\nRecovery into the DCTL.ULStChngReq field.\n\nThis prevents corner cases where the device is entering\n\nhibernation at the same time the host is attempting to exit L1."]
    #[inline(always)]
    #[must_use]
    pub fn l1hibernationen(&mut self) -> L1hibernationenW<DctlSpec> {
        L1hibernationenW::new(self, 18)
    }
    #[doc = "Bit 19 - KeepConnect\n\nWhen 1, this bit enables the save and restore programming\n\nmodel by preventing the core from disconnecting from the host\n\nwhen DCTL.RunStop is set to 0. It also enables the Hibernation\n\nRequest Event to be generated when the link goes to U3 or L2.\n\nThe device core disconnects from the host when DCTL.RunStop is\n\nset to 0.\n\nThis bit indicates whether to preserve this behavior (0), or if the\n\ncore must not disconnect when RunStop is set to 0 (1).\n\nThis bit also prevents the LTSSM from automatically going to\n\nU0/L0 when the host requests resume from U3/L2."]
    #[inline(always)]
    #[must_use]
    pub fn keepconnect(&mut self) -> KeepconnectW<DctlSpec> {
        KeepconnectW::new(self, 19)
    }
    #[doc = "Bits 20:23 - LPM NYET Threshold\n\nHandshake response to LPM token specified by device application.\n\nResponse depends on DCFG.LPMCap.\n\nDCFG.LPMCap is 1'b0 - The core always responds with Timeout\n\n(that is, no response).\n\nDCFG.LPMCap is 1'b1 - The core responds with an ACK on\n\nsuccessful LPM transaction, which requires that all of the\n\nfollowing are satisfied:\n\n1. There are no PID or CRC5 errors in both the EXT token and the\n\nLPM token (if not true, inactivity results in a timeout ERROR).\n\n2. No data is pending in the Transmit FIFO and OUT endpoints\n\nnot in flow controlled state (else NYET).\n\n3. The BESL value in the LPM token is less than or equal to\n\nLPM_NYET_thres\\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn lpm_nyet_thres(&mut self) -> LpmNyetThresW<DctlSpec> {
        LpmNyetThresW::new(self, 20)
    }
    #[doc = "Bits 24:28 - HIRD Threshold\n\nThe core asserts output signals utmi_l1_suspend_n and\n\nutmi_sleep_n on the basis of this signal:\n\nThe core asserts utmi_l1_suspend_n to put the PHY into Deep\n\nLow-Power mode in L1 when both of the following are true:\n\n1. HIRD value is greater than or equal to the value in\n\nDCTL.HIRD_Thres\\[3:0\\]\n\n2. HIRD_Thres\\[4\\]
is set to 1'b1.\n\nThe core asserts utmi_sleep_n on L1 when one of the following is\n\ntrue:\n\n1. If the HIRD value is less than HIRD_Thres\\[3:0\\]
or\n\n2. HIRD_Thres\\[4\\]
is set to 1'b0.\n\nNote: This field must be set to '0' during SuperSpeed mode of\n\noperation."]
    #[inline(always)]
    #[must_use]
    pub fn hirdthres(&mut self) -> HirdthresW<DctlSpec> {
        HirdthresW::new(self, 24)
    }
    #[doc = "Bit 30 - Core Soft Reset\n\nReset all clock domains as follows:\n\n1. This bit clears the interrupts and all the CSRs except GSTS,\n\nGSNPSID, GGPIO, GUID, GUSB2PHYCFGn registers,\n\nGUSB3PIPECTLn registers, DCFG, DCTL, DEVTEN, and DSTS\n\nregisters.\n\n2. All module state machines (except the SoC Bus Slave Unit) are\n\nreset to the IDLE state, and all the TxFIFOs and the RxFIFO are\n\nflushed.\n\n3. Any transactions on the SoC bus Master are terminated as\n\nsoon as possible, after gracefully completing the last data phase\n\nof a SoC bus transfer. Any transactions on the USB are\n\nterminated immediately.\n\nThe application can write this bit at any time to reset the core.\n\nThis is a self-clearing bit; the core clears this bit after all\n\nnecessary logic is reset in the core, which may take several\n\nclocks depending on the core's current state. Once this bit is\n\ncleared, the software must wait at least 3 PHY clocks before\n\naccessing the PHY domain (synchronization delay).\n\nTypically, software reset is used during software development\n\nand also when you dynamically change the PHY selection bits in\n\nthe USB configuration registers listed above. When you change\n\nthe PHY, the corresponding clock for the PHY is selected and used\n\nin the PHY domain. Once a new clock is selected, the PHY domain\n\nmust be reset for proper operation."]
    #[inline(always)]
    #[must_use]
    pub fn csftrst(&mut self) -> CsftrstW<DctlSpec> {
        CsftrstW::new(self, 30)
    }
    #[doc = "Bit 31 - Run/Stop\n\nThe software writes 1 to this bit to start the device controller\n\noperation.\n\nTo stop the device controller operation, the software must\n\nremove any active transfers and write 0 to this bit. When the\n\ncontroller is stopped, it sets the DSTS.DevCtrlHlt bit when the\n\ncore is idle and the lower layer finishes the disconnect process.\n\nThe Run/Stop bit must be used in following cases as specified:\n\n1. After power-on reset and CSR initialization, the software must\n\nwrite 1 to this bit to start the device controller. The controller\n\ndoes not signal connect to the host until this bit is set.\n\n2. The software uses this bit to control the device controller to\n\nperform a soft disconnect. When the software writes 0 to this bit,\n\nthe host does not see that the device is connected. The device\n\ncontroller stays in the disconnected state until the software writes\n\n1 to this bit.\n\nThe minimum duration of keeping this bit cleared is specified in\n\nthe Note below. If the software attempts a connect after the soft\n\ndisconnect or detects a disconnect event, it must set DCTL\\[8:5\\]\n\nto 5 before reasserting the Run/Stop bit.\n\n3. When the USB or Link is in a lower power state and the Two\n\nPower Rails configuration is selected, software writes 0 to this bit\n\nto indicate that it is going to turn off the Core Power Rail. After\n\nthe software turns on the Core Power Rail again and re-initializes\n\nthe device controller, it must set this bit to start the device\n\ncontroller.\n\nNote: The following is the minimum duration under various\n\nconditions for which the soft disconnect (SftDiscon) bit must be\n\nset for the USB host to detect a device disconnect:\n\n30ms: For SuperSpeed, when the device state is Suspended,\n\nIdle, Transmit, or Receive.\n\n10ms: For high-speed, when the device state is Suspended, Idle,\n\nor not Idle/Suspended (performing transactions); For full-\n\nspeed/low-speed, when the device state is Suspended, Idle, or\n\nnot Idle/Supended (performing transactions)\n\nTo accommodate clock jitter, it is recommended that the\n\napplication add extra delay to the specified minimum duration."]
    #[inline(always)]
    #[must_use]
    pub fn run_stop(&mut self) -> RunStopW<DctlSpec> {
        RunStopW::new(self, 31)
    }
}
#[doc = "Device Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DctlSpec;
impl crate::RegisterSpec for DctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dctl::R`](R) reader structure"]
impl crate::Readable for DctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dctl::W`](W) writer structure"]
impl crate::Writable for DctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCTL to value 0x00f0_0000"]
impl crate::Resettable for DctlSpec {
    const RESET_VALUE: u32 = 0x00f0_0000;
}
