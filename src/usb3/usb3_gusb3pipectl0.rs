#[doc = "Register `USB3_GUSB3PIPECTL0` reader"]
pub type R = crate::R<Usb3Gusb3pipectl0Spec>;
#[doc = "Register `USB3_GUSB3PIPECTL0` writer"]
pub type W = crate::W<Usb3Gusb3pipectl0Spec>;
#[doc = "Field `ELASTIC_BUFFER_MODE` reader - Elastic Buffer Mode\n\nDrive the setting value to the pipe interface of PHY."]
pub type ElasticBufferModeR = crate::BitReader;
#[doc = "Field `ELASTIC_BUFFER_MODE` writer - Elastic Buffer Mode\n\nDrive the setting value to the pipe interface of PHY."]
pub type ElasticBufferModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DE_EPPHASIS` reader - Tx Deemphasis\n\nThe value driven to the PHY is controlled by the LTSSM during\n\nUSB3 Compliance mode."]
pub type TxDeEpphasisR = crate::FieldReader;
#[doc = "Field `TX_DE_EPPHASIS` writer - Tx Deemphasis\n\nThe value driven to the PHY is controlled by the LTSSM during\n\nUSB3 Compliance mode."]
pub type TxDeEpphasisW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX_MARGIN` reader - Tx Margin\\[2:0\\]\n\nDrive the setting value to the pipe interface of PHY."]
pub type TxMarginR = crate::FieldReader;
#[doc = "Field `TX_MARGIN` writer - Tx Margin\\[2:0\\]\n\nDrive the setting value to the pipe interface of PHY."]
pub type TxMarginW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TX_SWING` reader - Tx Swing\n\nDrive the setting value to the pipe interface of PHY."]
pub type TxSwingR = crate::BitReader;
#[doc = "Field `TX_SWING` writer - Tx Swing\n\nDrive the setting value to the pipe interface of PHY."]
pub type TxSwingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DETECT_TO_POLLING_L` reader - RX_DETECT to Polling.LFPS Control\n\n1'b0 (Default): Enables a 400us delay to start Polling LFPS after\n\nRX_DETECT. This allows VCM offset to settle to a proper level.\n\n1'b1: Disables the 400us delay to start Polling LFPS after\n\nRX_DETECT.\n\nDuring controller certification with third party PHY it is observed\n\nthat the PHY is not able to meet the Tx AC common mode voltage\n\nactive (VTX-CM-ACPP_ACTIVE &lt;100mv) if the link starts polling\n\nwithin 80us from the time rx.detect is performed.\n\nTo meet this VTX-CM-ACPP_ACTIVE specification, the polling\n\nmust be delayed further. If the PHY does not have issue then\n\nthey can set this bit to 1 which allows polling to start within 80us."]
pub type RxDetectToPollingLR = crate::BitReader;
#[doc = "Field `RX_DETECT_TO_POLLING_L` writer - RX_DETECT to Polling.LFPS Control\n\n1'b0 (Default): Enables a 400us delay to start Polling LFPS after\n\nRX_DETECT. This allows VCM offset to settle to a proper level.\n\n1'b1: Disables the 400us delay to start Polling LFPS after\n\nRX_DETECT.\n\nDuring controller certification with third party PHY it is observed\n\nthat the PHY is not able to meet the Tx AC common mode voltage\n\nactive (VTX-CM-ACPP_ACTIVE &lt;100mv) if the link starts polling\n\nwithin 80us from the time rx.detect is performed.\n\nTo meet this VTX-CM-ACPP_ACTIVE specification, the polling\n\nmust be delayed further. If the PHY does not have issue then\n\nthey can set this bit to 1 which allows polling to start within 80us."]
pub type RxDetectToPollingLW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFPSFILTER` reader - LFPS Filter\n\nWhen set, filter LFPS reception with pipe3_RxValid in PHY power\n\nstate P0, that is, ignore LFPS reception from the PHY unless both\n\npipe3_Rxelecidle and pipe3_RxValid are deasserted."]
pub type LfpsfilterR = crate::BitReader;
#[doc = "Field `LFPSFILTER` writer - LFPS Filter\n\nWhen set, filter LFPS reception with pipe3_RxValid in PHY power\n\nstate P0, that is, ignore LFPS reception from the PHY unless both\n\npipe3_Rxelecidle and pipe3_RxValid are deasserted."]
pub type LfpsfilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3EXSIGP2` reader - P3 Exit Signal in P2\n\nWhen this bit is set, the core always changes the PHY power state\n\nto P2, before attempting a U3 exit handshake."]
pub type P3exsigp2R = crate::BitReader;
#[doc = "Field `P3EXSIGP2` writer - P3 Exit Signal in P2\n\nWhen this bit is set, the core always changes the PHY power state\n\nto P2, before attempting a U3 exit handshake."]
pub type P3exsigp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3P2TRANOK` reader - P3 P2 Transitions OK\n\nWhen set, the core transitions directly from Phy power state P2 to\n\nP3 or from state P3 to P2. When not set, P0 is always entered as\n\nan intermediate state during transitions between P2 and P3, as\n\ndefined in the PIPE3 Specification.\n\nAccording to the PIPE3 Specification, any direct transition\n\nbetween P3 and P2 is illegal."]
pub type P3p2tranokR = crate::BitReader;
#[doc = "Field `P3P2TRANOK` writer - P3 P2 Transitions OK\n\nWhen set, the core transitions directly from Phy power state P2 to\n\nP3 or from state P3 to P2. When not set, P0 is always entered as\n\nan intermediate state during transitions between P2 and P3, as\n\ndefined in the PIPE3 Specification.\n\nAccording to the PIPE3 Specification, any direct transition\n\nbetween P3 and P2 is illegal."]
pub type P3p2tranokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFPSP0ALGN` reader - LFPS P0 Align\n\nWhen set:\n\n1. The core deasserts LFPS transmission on the clock edge that it\n\nrequests Phy power state 0 when exiting U1, U2, or U3 low power\n\nstates. Otherwise, LFPS transmission is asserted one clock\n\nearlier.\n\n2. The core requests symbol transmission two pipe3_rx_pclks\n\nperiods after the PHY asserts PhyStatus as a result of the PHY\n\nswitching from P1 or P2 state to P0 state."]
pub type Lfpsp0algnR = crate::BitReader;
#[doc = "Field `LFPSP0ALGN` writer - LFPS P0 Align\n\nWhen set:\n\n1. The core deasserts LFPS transmission on the clock edge that it\n\nrequests Phy power state 0 when exiting U1, U2, or U3 low power\n\nstates. Otherwise, LFPS transmission is asserted one clock\n\nearlier.\n\n2. The core requests symbol transmission two pipe3_rx_pclks\n\nperiods after the PHY asserts PhyStatus as a result of the PHY\n\nswitching from P1 or P2 state to P0 state."]
pub type Lfpsp0algnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SKIPRXDET` reader - Skip Rx Detect\n\nWhen set, the core skips Rx Detection if pipe3_RxElecIdle is low.\n\nSkip is defined as waiting for the appropriate timeout, then\n\nrepeating the operation."]
pub type SkiprxdetR = crate::BitReader;
#[doc = "Field `SKIPRXDET` writer - Skip Rx Detect\n\nWhen set, the core skips Rx Detection if pipe3_RxElecIdle is low.\n\nSkip is defined as waiting for the appropriate timeout, then\n\nrepeating the operation."]
pub type SkiprxdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORTRXDETINU2` reader - Abort Rx Detect in U2\n\nWhen set and the link state is U2, then the core will abort\n\nreceiver detection if it receives U2 exit LFPS from the remote link\n\npartner. This bit is for the downstream port only."]
pub type Abortrxdetinu2R = crate::BitReader;
#[doc = "Field `ABORTRXDETINU2` writer - Abort Rx Detect in U2\n\nWhen set and the link state is U2, then the core will abort\n\nreceiver detection if it receives U2 exit LFPS from the remote link\n\npartner. This bit is for the downstream port only."]
pub type Abortrxdetinu2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "PIPE Data Width\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datwidth {
    #[doc = "0: 32 bits"]
    B00 = 0,
    #[doc = "1: 16 bits"]
    B01 = 1,
    #[doc = "2: 8 bits Note: USB3 controller only support 32-bit width pipe interface."]
    B10 = 2,
}
impl From<Datwidth> for u8 {
    #[inline(always)]
    fn from(variant: Datwidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datwidth {
    type Ux = u8;
}
#[doc = "Field `DATWIDTH` reader - PIPE Data Width"]
pub type DatwidthR = crate::FieldReader<Datwidth>;
impl DatwidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Datwidth> {
        match self.bits {
            0 => Some(Datwidth::B00),
            1 => Some(Datwidth::B01),
            2 => Some(Datwidth::B10),
            _ => None,
        }
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Datwidth::B00
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Datwidth::B01
    }
    #[doc = "8 bits Note: USB3 controller only support 32-bit width pipe interface."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Datwidth::B10
    }
}
#[doc = "Field `DATWIDTH` writer - PIPE Data Width"]
pub type DatwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, Datwidth>;
impl<'a, REG> DatwidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Datwidth::B00)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Datwidth::B01)
    }
    #[doc = "8 bits Note: USB3 controller only support 32-bit width pipe interface."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Datwidth::B10)
    }
}
#[doc = "Field `SUSPENDENABLE` reader - Suspend USB3.0 SS PHY\n\nWhen set, and if Suspend conditions are valid, the USB 3.0 PHY\n\nenters Suspend mode.\n\nFor DRD/OTG configurations, it is recommended that this bit is\n\nset to '0' during coreConsultant configuration. If it is set to '1',\n\nthen the application must clear this bit after power-on reset.\n\nApplication needs to set it to '1' after the core initialization is\n\ncompleted.\n\nFor all other configurations, this bit can be set to '1' during core\n\nconfiguration."]
pub type SuspendenableR = crate::BitReader;
#[doc = "Field `SUSPENDENABLE` writer - Suspend USB3.0 SS PHY\n\nWhen set, and if Suspend conditions are valid, the USB 3.0 PHY\n\nenters Suspend mode.\n\nFor DRD/OTG configurations, it is recommended that this bit is\n\nset to '0' during coreConsultant configuration. If it is set to '1',\n\nthen the application must clear this bit after power-on reset.\n\nApplication needs to set it to '1' after the core initialization is\n\ncompleted.\n\nFor all other configurations, this bit can be set to '1' during core\n\nconfiguration."]
pub type SuspendenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DELAYP1TRANS\n\nDelay PHY power change from P0 to P1/P2/P3 when link state\n\nchanging from U0 to U1/U2/U3 respectively.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Delayp1trans {
    #[doc = "1: When entering U1/U2/U3, delay the transition to P1/P2/P3 until the pipe3 signals, Pipe3_RxElecIlde is 1 and pipe3_RxValid is 0"]
    B1 = 1,
    #[doc = "0: When entering U1/U2/U3, transition to P1/P2/P3 without checking for Pipe3_RxElecIlde and pipe3_RxValid."]
    B0 = 0,
}
impl From<Delayp1trans> for bool {
    #[inline(always)]
    fn from(variant: Delayp1trans) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DELAYP1TRANS` reader - DELAYP1TRANS\n\nDelay PHY power change from P0 to P1/P2/P3 when link state\n\nchanging from U0 to U1/U2/U3 respectively."]
pub type Delayp1transR = crate::BitReader<Delayp1trans>;
impl Delayp1transR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Delayp1trans {
        match self.bits {
            true => Delayp1trans::B1,
            false => Delayp1trans::B0,
        }
    }
    #[doc = "When entering U1/U2/U3, delay the transition to P1/P2/P3 until the pipe3 signals, Pipe3_RxElecIlde is 1 and pipe3_RxValid is 0"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Delayp1trans::B1
    }
    #[doc = "When entering U1/U2/U3, transition to P1/P2/P3 without checking for Pipe3_RxElecIlde and pipe3_RxValid."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Delayp1trans::B0
    }
}
#[doc = "Field `DELAYP1TRANS` writer - DELAYP1TRANS\n\nDelay PHY power change from P0 to P1/P2/P3 when link state\n\nchanging from U0 to U1/U2/U3 respectively."]
pub type Delayp1transW<'a, REG> = crate::BitWriter<'a, REG, Delayp1trans>;
impl<'a, REG> Delayp1transW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When entering U1/U2/U3, delay the transition to P1/P2/P3 until the pipe3 signals, Pipe3_RxElecIlde is 1 and pipe3_RxValid is 0"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Delayp1trans::B1)
    }
    #[doc = "When entering U1/U2/U3, transition to P1/P2/P3 without checking for Pipe3_RxElecIlde and pipe3_RxValid."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Delayp1trans::B0)
    }
}
#[doc = "Field `DELAYP1P2P3` reader - Delay P1P2P3\n\nDelay P0 to P1/P2/P3 request when entering U1/U2/U3 until\n\n(DWC_USB3_GUSB3PIPECTL_INIT\\[21:19\\]*8) 8B10B error\n\noccurs, or Pipe3_RxValid drops to 0.\n\nDWC_USB3_GUSB3PIPECTL_INIT\\[18\\]
must be 1 to enable this\n\nfunctionality."]
pub type Delayp1p2p3R = crate::FieldReader;
#[doc = "Field `DELAYP1P2P3` writer - Delay P1P2P3\n\nDelay P0 to P1/P2/P3 request when entering U1/U2/U3 until\n\n(DWC_USB3_GUSB3PIPECTL_INIT\\[21:19\\]*8) 8B10B error\n\noccurs, or Pipe3_RxValid drops to 0.\n\nDWC_USB3_GUSB3PIPECTL_INIT\\[18\\]
must be 1 to enable this\n\nfunctionality."]
pub type Delayp1p2p3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DISRXDETU3RXDET` reader - Disable Receiver Detection in U3/Rx.Det\n\nWhen set, the core does not handle receiver detection in either\n\nU3 or Rx.Detect states. DWC_USB3_GUSB3PIPECTL_INIT\\[23\\]\n\nmust be used to start receiver detection manually. This bit can\n\nonly be used for the downstream port. This bit must be set to 0\n\nfor Upstream ports."]
pub type Disrxdetu3rxdetR = crate::BitReader;
#[doc = "Field `DISRXDETU3RXDET` writer - Disable Receiver Detection in U3/Rx.Det\n\nWhen set, the core does not handle receiver detection in either\n\nU3 or Rx.Detect states. DWC_USB3_GUSB3PIPECTL_INIT\\[23\\]\n\nmust be used to start receiver detection manually. This bit can\n\nonly be used for the downstream port. This bit must be set to 0\n\nfor Upstream ports."]
pub type Disrxdetu3rxdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTRXDETU3RXDET` reader - Start Receiver Detection in U3/Rx.Detect\n\nIf DWC_USB3_GUSB3PIPECTL_INIT\\[22\\]
is set, and the link is in\n\neither U3 or Rx.Detect state, the core starts receiver detection on\n\nthe rising edge of this bit. This can only be used for Downstream\n\nports. This bit must be set to 0 for Upstream ports."]
pub type Startrxdetu3rxdetR = crate::BitReader;
#[doc = "Field `STARTRXDETU3RXDET` writer - Start Receiver Detection in U3/Rx.Detect\n\nIf DWC_USB3_GUSB3PIPECTL_INIT\\[22\\]
is set, and the link is in\n\neither U3 or Rx.Detect state, the core starts receiver detection on\n\nthe rising edge of this bit. This can only be used for Downstream\n\nports. This bit must be set to 0 for Upstream ports."]
pub type Startrxdetu3rxdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REQUEST_P1P2P3` reader - Always Request P1/P2/P3 for U1/U2/U3\n\nWhen set, the core always requests PHY power change from P0 to\n\nP1/P2/P3 during U0 to U1/U2/U3 transition.\n\nIf this bit is 0, and immediate Ux exit (remotely initiated, or\n\nlocally initiated) happens, the core does not request P1/P2/P3\n\npower state change.\n\nNote: This bit must be set to 1 for Synopsys PHY. For third-party\n\nSS PHY, check with your PHY vendor."]
pub type RequestP1p2p3R = crate::BitReader;
#[doc = "Field `REQUEST_P1P2P3` writer - Always Request P1/P2/P3 for U1/U2/U3\n\nWhen set, the core always requests PHY power change from P0 to\n\nP1/P2/P3 during U0 to U1/U2/U3 transition.\n\nIf this bit is 0, and immediate Ux exit (remotely initiated, or\n\nlocally initiated) happens, the core does not request P1/P2/P3\n\npower state change.\n\nNote: This bit must be set to 1 for Synopsys PHY. For third-party\n\nSS PHY, check with your PHY vendor."]
pub type RequestP1p2p3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `U1U2EXITFAIL_TO_RECOV` reader - U1U2exitfail to Recovery\n\nWhen set, and U1/U2 LFPS handshake fails, the LTSSM\n\ntransitions from U1/U2 to Recovery instead of SS Inactive. If\n\nRecovery fails, then the LTSSM can enter SS.Inactive. This is an\n\nenhancement only. It prevents interoperability issue if the remote\n\nlink does not do proper handshake."]
pub type U1u2exitfailToRecovR = crate::BitReader;
#[doc = "Field `U1U2EXITFAIL_TO_RECOV` writer - U1U2exitfail to Recovery\n\nWhen set, and U1/U2 LFPS handshake fails, the LTSSM\n\ntransitions from U1/U2 to Recovery instead of SS Inactive. If\n\nRecovery fails, then the LTSSM can enter SS.Inactive. This is an\n\nenhancement only. It prevents interoperability issue if the remote\n\nlink does not do proper handshake."]
pub type U1u2exitfailToRecovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PING_ENHANCEMENT_EN` reader - Ping Enhancement Enable\n\nWhen set, the Downstream port U1 ping receive timeout becomes\n\n500 ms instead of 300 ms. Minimum Ping.LFPS receive duration\n\nis 8 ns (one mac3_clk). This field is valid for the downstream port\n\nonly."]
pub type PingEnhancementEnR = crate::BitReader;
#[doc = "Field `PING_ENHANCEMENT_EN` writer - Ping Enhancement Enable\n\nWhen set, the Downstream port U1 ping receive timeout becomes\n\n500 ms instead of 300 ms. Minimum Ping.LFPS receive duration\n\nis 8 ns (one mac3_clk). This field is valid for the downstream port\n\nonly."]
pub type PingEnhancementEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Ux Exit in Px\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UxExitInPx {
    #[doc = "0: The core does U1/U2/U3 exit in PHY power state P0 (default behavior)."]
    B0 = 0,
    #[doc = "1: The core does U1/U2/U3 exit in PHY power state P1/P2/P3 respectively. This bit is added for SS PHY workaround where SS PHY injects a glitch on pipe3_RxElecIdle while receiving Ux exit LFPS, and pipe3_PowerDown change is in progress."]
    B1 = 1,
}
impl From<UxExitInPx> for bool {
    #[inline(always)]
    fn from(variant: UxExitInPx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UX_EXIT_IN_PX` reader - Ux Exit in Px"]
pub type UxExitInPxR = crate::BitReader<UxExitInPx>;
impl UxExitInPxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UxExitInPx {
        match self.bits {
            false => UxExitInPx::B0,
            true => UxExitInPx::B1,
        }
    }
    #[doc = "The core does U1/U2/U3 exit in PHY power state P0 (default behavior)."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == UxExitInPx::B0
    }
    #[doc = "The core does U1/U2/U3 exit in PHY power state P1/P2/P3 respectively. This bit is added for SS PHY workaround where SS PHY injects a glitch on pipe3_RxElecIdle while receiving Ux exit LFPS, and pipe3_PowerDown change is in progress."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == UxExitInPx::B1
    }
}
#[doc = "Field `UX_EXIT_IN_PX` writer - Ux Exit in Px"]
pub type UxExitInPxW<'a, REG> = crate::BitWriter<'a, REG, UxExitInPx>;
impl<'a, REG> UxExitInPxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The core does U1/U2/U3 exit in PHY power state P0 (default behavior)."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(UxExitInPx::B0)
    }
    #[doc = "The core does U1/U2/U3 exit in PHY power state P1/P2/P3 respectively. This bit is added for SS PHY workaround where SS PHY injects a glitch on pipe3_RxElecIdle while receiving Ux exit LFPS, and pipe3_PowerDown change is in progress."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(UxExitInPx::B1)
    }
}
#[doc = "Disabled receiver detection in P3\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disrxdetp3 {
    #[doc = "0: If PHY is in P3 and Core needs to perform receiver detection, The core performs receiver detection in P3. (Default)"]
    B0 = 0,
    #[doc = "1: If PHY is in P3 and Core needs to perform receiver detection, The core changes the PHY power state to P2 and then performs receiver detection. After receiver detection, the cores changes PHY power state to P3."]
    B1 = 1,
}
impl From<Disrxdetp3> for bool {
    #[inline(always)]
    fn from(variant: Disrxdetp3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISRXDETP3` reader - Disabled receiver detection in P3"]
pub type Disrxdetp3R = crate::BitReader<Disrxdetp3>;
impl Disrxdetp3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Disrxdetp3 {
        match self.bits {
            false => Disrxdetp3::B0,
            true => Disrxdetp3::B1,
        }
    }
    #[doc = "If PHY is in P3 and Core needs to perform receiver detection, The core performs receiver detection in P3. (Default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Disrxdetp3::B0
    }
    #[doc = "If PHY is in P3 and Core needs to perform receiver detection, The core changes the PHY power state to P2 and then performs receiver detection. After receiver detection, the cores changes PHY power state to P3."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Disrxdetp3::B1
    }
}
#[doc = "Field `DISRXDETP3` writer - Disabled receiver detection in P3"]
pub type Disrxdetp3W<'a, REG> = crate::BitWriter<'a, REG, Disrxdetp3>;
impl<'a, REG> Disrxdetp3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If PHY is in P3 and Core needs to perform receiver detection, The core performs receiver detection in P3. (Default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Disrxdetp3::B0)
    }
    #[doc = "If PHY is in P3 and Core needs to perform receiver detection, The core changes the PHY power state to P2 and then performs receiver detection. After receiver detection, the cores changes PHY power state to P3."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Disrxdetp3::B1)
    }
}
#[doc = "P3 OK for U2/SSInactive\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U2ssinactp3ok {
    #[doc = "0: During link state U2/SS.Inactive, put PHY in P2 (Default)"]
    B0 = 0,
    #[doc = "1: During link state U2/SS.Inactive, put PHY in P3. Note: For a port, if GUSB3PIPECTL\\[7\\]=1 and GUSB3PIPECTL\\[29\\]=1, set GUSB3PIPECTL\\[11\\]
to 1."]
    B1 = 1,
}
impl From<U2ssinactp3ok> for bool {
    #[inline(always)]
    fn from(variant: U2ssinactp3ok) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `U2SSINACTP3OK` reader - P3 OK for U2/SSInactive"]
pub type U2ssinactp3okR = crate::BitReader<U2ssinactp3ok>;
impl U2ssinactp3okR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U2ssinactp3ok {
        match self.bits {
            false => U2ssinactp3ok::B0,
            true => U2ssinactp3ok::B1,
        }
    }
    #[doc = "During link state U2/SS.Inactive, put PHY in P2 (Default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == U2ssinactp3ok::B0
    }
    #[doc = "During link state U2/SS.Inactive, put PHY in P3. Note: For a port, if GUSB3PIPECTL\\[7\\]=1 and GUSB3PIPECTL\\[29\\]=1, set GUSB3PIPECTL\\[11\\]
to 1."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == U2ssinactp3ok::B1
    }
}
#[doc = "Field `U2SSINACTP3OK` writer - P3 OK for U2/SSInactive"]
pub type U2ssinactp3okW<'a, REG> = crate::BitWriter<'a, REG, U2ssinactp3ok>;
impl<'a, REG> U2ssinactp3okW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "During link state U2/SS.Inactive, put PHY in P2 (Default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(U2ssinactp3ok::B0)
    }
    #[doc = "During link state U2/SS.Inactive, put PHY in P3. Note: For a port, if GUSB3PIPECTL\\[7\\]=1 and GUSB3PIPECTL\\[29\\]=1, set GUSB3PIPECTL\\[11\\]
to 1."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(U2ssinactp3ok::B1)
    }
}
#[doc = "Field `HSTPRTCMPL` reader - HstPrtCmpl\n\nThis feature tests the PIPE PHY compliance patterns without\n\nhaving to have a test fixture on the USB 3.0 cable.\n\nThis bit enables placing the SS port link into a compliance state.\n\nBy default, this bit must be set to 1'b0.\n\nIn compliance lab testing, the SS port link enters compliance\n\nafter failing the first polling sequence after power on. Set this bit\n\nto 0, when you run compliance tests.\n\nThe sequence for using this functionality is as follows:\n\n1. Disconnect any plugged in devices.\n\n2. Perform USBCMD.HCRST or power-on-chip reset.\n\n3. Set PORTSC.PP=0.\n\n4. Set GUSB3PIPECTL. HstPrtCmpl=1. This places the link into\n\ncompliance state.\n\nTo advance the compliance pattern, follow this sequence (toggle\n\nthe set GUSB3PIPECTL. HstPrtCmpl):\n\n1. Set GUSB3PIPECTL.HstPrtCmpl=0.\n\n2. Set GUSB3PIPECTL.HstPrtCmpl=1. This advances the link to\n\nthe next compliance pattern.\n\nTo exit from the compliance state perform USBCMD.HCRST or\n\npower-on-chip reset."]
pub type HstprtcmplR = crate::BitReader;
#[doc = "Field `HSTPRTCMPL` writer - HstPrtCmpl\n\nThis feature tests the PIPE PHY compliance patterns without\n\nhaving to have a test fixture on the USB 3.0 cable.\n\nThis bit enables placing the SS port link into a compliance state.\n\nBy default, this bit must be set to 1'b0.\n\nIn compliance lab testing, the SS port link enters compliance\n\nafter failing the first polling sequence after power on. Set this bit\n\nto 0, when you run compliance tests.\n\nThe sequence for using this functionality is as follows:\n\n1. Disconnect any plugged in devices.\n\n2. Perform USBCMD.HCRST or power-on-chip reset.\n\n3. Set PORTSC.PP=0.\n\n4. Set GUSB3PIPECTL. HstPrtCmpl=1. This places the link into\n\ncompliance state.\n\nTo advance the compliance pattern, follow this sequence (toggle\n\nthe set GUSB3PIPECTL. HstPrtCmpl):\n\n1. Set GUSB3PIPECTL.HstPrtCmpl=0.\n\n2. Set GUSB3PIPECTL.HstPrtCmpl=1. This advances the link to\n\nthe next compliance pattern.\n\nTo exit from the compliance state perform USBCMD.HCRST or\n\npower-on-chip reset."]
pub type HstprtcmplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYSOFTRST` reader - USB3 PHY Soft Reset\n\nAfter setting this bit to 1, the software needs to clear this bit."]
pub type PhysoftrstR = crate::BitReader;
#[doc = "Field `PHYSOFTRST` writer - USB3 PHY Soft Reset\n\nAfter setting this bit to 1, the software needs to clear this bit."]
pub type PhysoftrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Elastic Buffer Mode\n\nDrive the setting value to the pipe interface of PHY."]
    #[inline(always)]
    pub fn elastic_buffer_mode(&self) -> ElasticBufferModeR {
        ElasticBufferModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Tx Deemphasis\n\nThe value driven to the PHY is controlled by the LTSSM during\n\nUSB3 Compliance mode."]
    #[inline(always)]
    pub fn tx_de_epphasis(&self) -> TxDeEpphasisR {
        TxDeEpphasisR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:5 - Tx Margin\\[2:0\\]\n\nDrive the setting value to the pipe interface of PHY."]
    #[inline(always)]
    pub fn tx_margin(&self) -> TxMarginR {
        TxMarginR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Tx Swing\n\nDrive the setting value to the pipe interface of PHY."]
    #[inline(always)]
    pub fn tx_swing(&self) -> TxSwingR {
        TxSwingR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - RX_DETECT to Polling.LFPS Control\n\n1'b0 (Default): Enables a 400us delay to start Polling LFPS after\n\nRX_DETECT. This allows VCM offset to settle to a proper level.\n\n1'b1: Disables the 400us delay to start Polling LFPS after\n\nRX_DETECT.\n\nDuring controller certification with third party PHY it is observed\n\nthat the PHY is not able to meet the Tx AC common mode voltage\n\nactive (VTX-CM-ACPP_ACTIVE &lt;100mv) if the link starts polling\n\nwithin 80us from the time rx.detect is performed.\n\nTo meet this VTX-CM-ACPP_ACTIVE specification, the polling\n\nmust be delayed further. If the PHY does not have issue then\n\nthey can set this bit to 1 which allows polling to start within 80us."]
    #[inline(always)]
    pub fn rx_detect_to_polling_l(&self) -> RxDetectToPollingLR {
        RxDetectToPollingLR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LFPS Filter\n\nWhen set, filter LFPS reception with pipe3_RxValid in PHY power\n\nstate P0, that is, ignore LFPS reception from the PHY unless both\n\npipe3_Rxelecidle and pipe3_RxValid are deasserted."]
    #[inline(always)]
    pub fn lfpsfilter(&self) -> LfpsfilterR {
        LfpsfilterR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - P3 Exit Signal in P2\n\nWhen this bit is set, the core always changes the PHY power state\n\nto P2, before attempting a U3 exit handshake."]
    #[inline(always)]
    pub fn p3exsigp2(&self) -> P3exsigp2R {
        P3exsigp2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - P3 P2 Transitions OK\n\nWhen set, the core transitions directly from Phy power state P2 to\n\nP3 or from state P3 to P2. When not set, P0 is always entered as\n\nan intermediate state during transitions between P2 and P3, as\n\ndefined in the PIPE3 Specification.\n\nAccording to the PIPE3 Specification, any direct transition\n\nbetween P3 and P2 is illegal."]
    #[inline(always)]
    pub fn p3p2tranok(&self) -> P3p2tranokR {
        P3p2tranokR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LFPS P0 Align\n\nWhen set:\n\n1. The core deasserts LFPS transmission on the clock edge that it\n\nrequests Phy power state 0 when exiting U1, U2, or U3 low power\n\nstates. Otherwise, LFPS transmission is asserted one clock\n\nearlier.\n\n2. The core requests symbol transmission two pipe3_rx_pclks\n\nperiods after the PHY asserts PhyStatus as a result of the PHY\n\nswitching from P1 or P2 state to P0 state."]
    #[inline(always)]
    pub fn lfpsp0algn(&self) -> Lfpsp0algnR {
        Lfpsp0algnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Skip Rx Detect\n\nWhen set, the core skips Rx Detection if pipe3_RxElecIdle is low.\n\nSkip is defined as waiting for the appropriate timeout, then\n\nrepeating the operation."]
    #[inline(always)]
    pub fn skiprxdet(&self) -> SkiprxdetR {
        SkiprxdetR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Abort Rx Detect in U2\n\nWhen set and the link state is U2, then the core will abort\n\nreceiver detection if it receives U2 exit LFPS from the remote link\n\npartner. This bit is for the downstream port only."]
    #[inline(always)]
    pub fn abortrxdetinu2(&self) -> Abortrxdetinu2R {
        Abortrxdetinu2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - PIPE Data Width"]
    #[inline(always)]
    pub fn datwidth(&self) -> DatwidthR {
        DatwidthR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - Suspend USB3.0 SS PHY\n\nWhen set, and if Suspend conditions are valid, the USB 3.0 PHY\n\nenters Suspend mode.\n\nFor DRD/OTG configurations, it is recommended that this bit is\n\nset to '0' during coreConsultant configuration. If it is set to '1',\n\nthen the application must clear this bit after power-on reset.\n\nApplication needs to set it to '1' after the core initialization is\n\ncompleted.\n\nFor all other configurations, this bit can be set to '1' during core\n\nconfiguration."]
    #[inline(always)]
    pub fn suspendenable(&self) -> SuspendenableR {
        SuspendenableR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DELAYP1TRANS\n\nDelay PHY power change from P0 to P1/P2/P3 when link state\n\nchanging from U0 to U1/U2/U3 respectively."]
    #[inline(always)]
    pub fn delayp1trans(&self) -> Delayp1transR {
        Delayp1transR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - Delay P1P2P3\n\nDelay P0 to P1/P2/P3 request when entering U1/U2/U3 until\n\n(DWC_USB3_GUSB3PIPECTL_INIT\\[21:19\\]*8) 8B10B error\n\noccurs, or Pipe3_RxValid drops to 0.\n\nDWC_USB3_GUSB3PIPECTL_INIT\\[18\\]
must be 1 to enable this\n\nfunctionality."]
    #[inline(always)]
    pub fn delayp1p2p3(&self) -> Delayp1p2p3R {
        Delayp1p2p3R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bit 22 - Disable Receiver Detection in U3/Rx.Det\n\nWhen set, the core does not handle receiver detection in either\n\nU3 or Rx.Detect states. DWC_USB3_GUSB3PIPECTL_INIT\\[23\\]\n\nmust be used to start receiver detection manually. This bit can\n\nonly be used for the downstream port. This bit must be set to 0\n\nfor Upstream ports."]
    #[inline(always)]
    pub fn disrxdetu3rxdet(&self) -> Disrxdetu3rxdetR {
        Disrxdetu3rxdetR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Start Receiver Detection in U3/Rx.Detect\n\nIf DWC_USB3_GUSB3PIPECTL_INIT\\[22\\]
is set, and the link is in\n\neither U3 or Rx.Detect state, the core starts receiver detection on\n\nthe rising edge of this bit. This can only be used for Downstream\n\nports. This bit must be set to 0 for Upstream ports."]
    #[inline(always)]
    pub fn startrxdetu3rxdet(&self) -> Startrxdetu3rxdetR {
        Startrxdetu3rxdetR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Always Request P1/P2/P3 for U1/U2/U3\n\nWhen set, the core always requests PHY power change from P0 to\n\nP1/P2/P3 during U0 to U1/U2/U3 transition.\n\nIf this bit is 0, and immediate Ux exit (remotely initiated, or\n\nlocally initiated) happens, the core does not request P1/P2/P3\n\npower state change.\n\nNote: This bit must be set to 1 for Synopsys PHY. For third-party\n\nSS PHY, check with your PHY vendor."]
    #[inline(always)]
    pub fn request_p1p2p3(&self) -> RequestP1p2p3R {
        RequestP1p2p3R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - U1U2exitfail to Recovery\n\nWhen set, and U1/U2 LFPS handshake fails, the LTSSM\n\ntransitions from U1/U2 to Recovery instead of SS Inactive. If\n\nRecovery fails, then the LTSSM can enter SS.Inactive. This is an\n\nenhancement only. It prevents interoperability issue if the remote\n\nlink does not do proper handshake."]
    #[inline(always)]
    pub fn u1u2exitfail_to_recov(&self) -> U1u2exitfailToRecovR {
        U1u2exitfailToRecovR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Ping Enhancement Enable\n\nWhen set, the Downstream port U1 ping receive timeout becomes\n\n500 ms instead of 300 ms. Minimum Ping.LFPS receive duration\n\nis 8 ns (one mac3_clk). This field is valid for the downstream port\n\nonly."]
    #[inline(always)]
    pub fn ping_enhancement_en(&self) -> PingEnhancementEnR {
        PingEnhancementEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Ux Exit in Px"]
    #[inline(always)]
    pub fn ux_exit_in_px(&self) -> UxExitInPxR {
        UxExitInPxR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Disabled receiver detection in P3"]
    #[inline(always)]
    pub fn disrxdetp3(&self) -> Disrxdetp3R {
        Disrxdetp3R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - P3 OK for U2/SSInactive"]
    #[inline(always)]
    pub fn u2ssinactp3ok(&self) -> U2ssinactp3okR {
        U2ssinactp3okR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - HstPrtCmpl\n\nThis feature tests the PIPE PHY compliance patterns without\n\nhaving to have a test fixture on the USB 3.0 cable.\n\nThis bit enables placing the SS port link into a compliance state.\n\nBy default, this bit must be set to 1'b0.\n\nIn compliance lab testing, the SS port link enters compliance\n\nafter failing the first polling sequence after power on. Set this bit\n\nto 0, when you run compliance tests.\n\nThe sequence for using this functionality is as follows:\n\n1. Disconnect any plugged in devices.\n\n2. Perform USBCMD.HCRST or power-on-chip reset.\n\n3. Set PORTSC.PP=0.\n\n4. Set GUSB3PIPECTL. HstPrtCmpl=1. This places the link into\n\ncompliance state.\n\nTo advance the compliance pattern, follow this sequence (toggle\n\nthe set GUSB3PIPECTL. HstPrtCmpl):\n\n1. Set GUSB3PIPECTL.HstPrtCmpl=0.\n\n2. Set GUSB3PIPECTL.HstPrtCmpl=1. This advances the link to\n\nthe next compliance pattern.\n\nTo exit from the compliance state perform USBCMD.HCRST or\n\npower-on-chip reset."]
    #[inline(always)]
    pub fn hstprtcmpl(&self) -> HstprtcmplR {
        HstprtcmplR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - USB3 PHY Soft Reset\n\nAfter setting this bit to 1, the software needs to clear this bit."]
    #[inline(always)]
    pub fn physoftrst(&self) -> PhysoftrstR {
        PhysoftrstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Elastic Buffer Mode\n\nDrive the setting value to the pipe interface of PHY."]
    #[inline(always)]
    #[must_use]
    pub fn elastic_buffer_mode(&mut self) -> ElasticBufferModeW<Usb3Gusb3pipectl0Spec> {
        ElasticBufferModeW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Tx Deemphasis\n\nThe value driven to the PHY is controlled by the LTSSM during\n\nUSB3 Compliance mode."]
    #[inline(always)]
    #[must_use]
    pub fn tx_de_epphasis(&mut self) -> TxDeEpphasisW<Usb3Gusb3pipectl0Spec> {
        TxDeEpphasisW::new(self, 1)
    }
    #[doc = "Bits 3:5 - Tx Margin\\[2:0\\]\n\nDrive the setting value to the pipe interface of PHY."]
    #[inline(always)]
    #[must_use]
    pub fn tx_margin(&mut self) -> TxMarginW<Usb3Gusb3pipectl0Spec> {
        TxMarginW::new(self, 3)
    }
    #[doc = "Bit 6 - Tx Swing\n\nDrive the setting value to the pipe interface of PHY."]
    #[inline(always)]
    #[must_use]
    pub fn tx_swing(&mut self) -> TxSwingW<Usb3Gusb3pipectl0Spec> {
        TxSwingW::new(self, 6)
    }
    #[doc = "Bit 8 - RX_DETECT to Polling.LFPS Control\n\n1'b0 (Default): Enables a 400us delay to start Polling LFPS after\n\nRX_DETECT. This allows VCM offset to settle to a proper level.\n\n1'b1: Disables the 400us delay to start Polling LFPS after\n\nRX_DETECT.\n\nDuring controller certification with third party PHY it is observed\n\nthat the PHY is not able to meet the Tx AC common mode voltage\n\nactive (VTX-CM-ACPP_ACTIVE &lt;100mv) if the link starts polling\n\nwithin 80us from the time rx.detect is performed.\n\nTo meet this VTX-CM-ACPP_ACTIVE specification, the polling\n\nmust be delayed further. If the PHY does not have issue then\n\nthey can set this bit to 1 which allows polling to start within 80us."]
    #[inline(always)]
    #[must_use]
    pub fn rx_detect_to_polling_l(&mut self) -> RxDetectToPollingLW<Usb3Gusb3pipectl0Spec> {
        RxDetectToPollingLW::new(self, 8)
    }
    #[doc = "Bit 9 - LFPS Filter\n\nWhen set, filter LFPS reception with pipe3_RxValid in PHY power\n\nstate P0, that is, ignore LFPS reception from the PHY unless both\n\npipe3_Rxelecidle and pipe3_RxValid are deasserted."]
    #[inline(always)]
    #[must_use]
    pub fn lfpsfilter(&mut self) -> LfpsfilterW<Usb3Gusb3pipectl0Spec> {
        LfpsfilterW::new(self, 9)
    }
    #[doc = "Bit 10 - P3 Exit Signal in P2\n\nWhen this bit is set, the core always changes the PHY power state\n\nto P2, before attempting a U3 exit handshake."]
    #[inline(always)]
    #[must_use]
    pub fn p3exsigp2(&mut self) -> P3exsigp2W<Usb3Gusb3pipectl0Spec> {
        P3exsigp2W::new(self, 10)
    }
    #[doc = "Bit 11 - P3 P2 Transitions OK\n\nWhen set, the core transitions directly from Phy power state P2 to\n\nP3 or from state P3 to P2. When not set, P0 is always entered as\n\nan intermediate state during transitions between P2 and P3, as\n\ndefined in the PIPE3 Specification.\n\nAccording to the PIPE3 Specification, any direct transition\n\nbetween P3 and P2 is illegal."]
    #[inline(always)]
    #[must_use]
    pub fn p3p2tranok(&mut self) -> P3p2tranokW<Usb3Gusb3pipectl0Spec> {
        P3p2tranokW::new(self, 11)
    }
    #[doc = "Bit 12 - LFPS P0 Align\n\nWhen set:\n\n1. The core deasserts LFPS transmission on the clock edge that it\n\nrequests Phy power state 0 when exiting U1, U2, or U3 low power\n\nstates. Otherwise, LFPS transmission is asserted one clock\n\nearlier.\n\n2. The core requests symbol transmission two pipe3_rx_pclks\n\nperiods after the PHY asserts PhyStatus as a result of the PHY\n\nswitching from P1 or P2 state to P0 state."]
    #[inline(always)]
    #[must_use]
    pub fn lfpsp0algn(&mut self) -> Lfpsp0algnW<Usb3Gusb3pipectl0Spec> {
        Lfpsp0algnW::new(self, 12)
    }
    #[doc = "Bit 13 - Skip Rx Detect\n\nWhen set, the core skips Rx Detection if pipe3_RxElecIdle is low.\n\nSkip is defined as waiting for the appropriate timeout, then\n\nrepeating the operation."]
    #[inline(always)]
    #[must_use]
    pub fn skiprxdet(&mut self) -> SkiprxdetW<Usb3Gusb3pipectl0Spec> {
        SkiprxdetW::new(self, 13)
    }
    #[doc = "Bit 14 - Abort Rx Detect in U2\n\nWhen set and the link state is U2, then the core will abort\n\nreceiver detection if it receives U2 exit LFPS from the remote link\n\npartner. This bit is for the downstream port only."]
    #[inline(always)]
    #[must_use]
    pub fn abortrxdetinu2(&mut self) -> Abortrxdetinu2W<Usb3Gusb3pipectl0Spec> {
        Abortrxdetinu2W::new(self, 14)
    }
    #[doc = "Bits 15:16 - PIPE Data Width"]
    #[inline(always)]
    #[must_use]
    pub fn datwidth(&mut self) -> DatwidthW<Usb3Gusb3pipectl0Spec> {
        DatwidthW::new(self, 15)
    }
    #[doc = "Bit 17 - Suspend USB3.0 SS PHY\n\nWhen set, and if Suspend conditions are valid, the USB 3.0 PHY\n\nenters Suspend mode.\n\nFor DRD/OTG configurations, it is recommended that this bit is\n\nset to '0' during coreConsultant configuration. If it is set to '1',\n\nthen the application must clear this bit after power-on reset.\n\nApplication needs to set it to '1' after the core initialization is\n\ncompleted.\n\nFor all other configurations, this bit can be set to '1' during core\n\nconfiguration."]
    #[inline(always)]
    #[must_use]
    pub fn suspendenable(&mut self) -> SuspendenableW<Usb3Gusb3pipectl0Spec> {
        SuspendenableW::new(self, 17)
    }
    #[doc = "Bit 18 - DELAYP1TRANS\n\nDelay PHY power change from P0 to P1/P2/P3 when link state\n\nchanging from U0 to U1/U2/U3 respectively."]
    #[inline(always)]
    #[must_use]
    pub fn delayp1trans(&mut self) -> Delayp1transW<Usb3Gusb3pipectl0Spec> {
        Delayp1transW::new(self, 18)
    }
    #[doc = "Bits 19:21 - Delay P1P2P3\n\nDelay P0 to P1/P2/P3 request when entering U1/U2/U3 until\n\n(DWC_USB3_GUSB3PIPECTL_INIT\\[21:19\\]*8) 8B10B error\n\noccurs, or Pipe3_RxValid drops to 0.\n\nDWC_USB3_GUSB3PIPECTL_INIT\\[18\\]
must be 1 to enable this\n\nfunctionality."]
    #[inline(always)]
    #[must_use]
    pub fn delayp1p2p3(&mut self) -> Delayp1p2p3W<Usb3Gusb3pipectl0Spec> {
        Delayp1p2p3W::new(self, 19)
    }
    #[doc = "Bit 22 - Disable Receiver Detection in U3/Rx.Det\n\nWhen set, the core does not handle receiver detection in either\n\nU3 or Rx.Detect states. DWC_USB3_GUSB3PIPECTL_INIT\\[23\\]\n\nmust be used to start receiver detection manually. This bit can\n\nonly be used for the downstream port. This bit must be set to 0\n\nfor Upstream ports."]
    #[inline(always)]
    #[must_use]
    pub fn disrxdetu3rxdet(&mut self) -> Disrxdetu3rxdetW<Usb3Gusb3pipectl0Spec> {
        Disrxdetu3rxdetW::new(self, 22)
    }
    #[doc = "Bit 23 - Start Receiver Detection in U3/Rx.Detect\n\nIf DWC_USB3_GUSB3PIPECTL_INIT\\[22\\]
is set, and the link is in\n\neither U3 or Rx.Detect state, the core starts receiver detection on\n\nthe rising edge of this bit. This can only be used for Downstream\n\nports. This bit must be set to 0 for Upstream ports."]
    #[inline(always)]
    #[must_use]
    pub fn startrxdetu3rxdet(&mut self) -> Startrxdetu3rxdetW<Usb3Gusb3pipectl0Spec> {
        Startrxdetu3rxdetW::new(self, 23)
    }
    #[doc = "Bit 24 - Always Request P1/P2/P3 for U1/U2/U3\n\nWhen set, the core always requests PHY power change from P0 to\n\nP1/P2/P3 during U0 to U1/U2/U3 transition.\n\nIf this bit is 0, and immediate Ux exit (remotely initiated, or\n\nlocally initiated) happens, the core does not request P1/P2/P3\n\npower state change.\n\nNote: This bit must be set to 1 for Synopsys PHY. For third-party\n\nSS PHY, check with your PHY vendor."]
    #[inline(always)]
    #[must_use]
    pub fn request_p1p2p3(&mut self) -> RequestP1p2p3W<Usb3Gusb3pipectl0Spec> {
        RequestP1p2p3W::new(self, 24)
    }
    #[doc = "Bit 25 - U1U2exitfail to Recovery\n\nWhen set, and U1/U2 LFPS handshake fails, the LTSSM\n\ntransitions from U1/U2 to Recovery instead of SS Inactive. If\n\nRecovery fails, then the LTSSM can enter SS.Inactive. This is an\n\nenhancement only. It prevents interoperability issue if the remote\n\nlink does not do proper handshake."]
    #[inline(always)]
    #[must_use]
    pub fn u1u2exitfail_to_recov(&mut self) -> U1u2exitfailToRecovW<Usb3Gusb3pipectl0Spec> {
        U1u2exitfailToRecovW::new(self, 25)
    }
    #[doc = "Bit 26 - Ping Enhancement Enable\n\nWhen set, the Downstream port U1 ping receive timeout becomes\n\n500 ms instead of 300 ms. Minimum Ping.LFPS receive duration\n\nis 8 ns (one mac3_clk). This field is valid for the downstream port\n\nonly."]
    #[inline(always)]
    #[must_use]
    pub fn ping_enhancement_en(&mut self) -> PingEnhancementEnW<Usb3Gusb3pipectl0Spec> {
        PingEnhancementEnW::new(self, 26)
    }
    #[doc = "Bit 27 - Ux Exit in Px"]
    #[inline(always)]
    #[must_use]
    pub fn ux_exit_in_px(&mut self) -> UxExitInPxW<Usb3Gusb3pipectl0Spec> {
        UxExitInPxW::new(self, 27)
    }
    #[doc = "Bit 28 - Disabled receiver detection in P3"]
    #[inline(always)]
    #[must_use]
    pub fn disrxdetp3(&mut self) -> Disrxdetp3W<Usb3Gusb3pipectl0Spec> {
        Disrxdetp3W::new(self, 28)
    }
    #[doc = "Bit 29 - P3 OK for U2/SSInactive"]
    #[inline(always)]
    #[must_use]
    pub fn u2ssinactp3ok(&mut self) -> U2ssinactp3okW<Usb3Gusb3pipectl0Spec> {
        U2ssinactp3okW::new(self, 29)
    }
    #[doc = "Bit 30 - HstPrtCmpl\n\nThis feature tests the PIPE PHY compliance patterns without\n\nhaving to have a test fixture on the USB 3.0 cable.\n\nThis bit enables placing the SS port link into a compliance state.\n\nBy default, this bit must be set to 1'b0.\n\nIn compliance lab testing, the SS port link enters compliance\n\nafter failing the first polling sequence after power on. Set this bit\n\nto 0, when you run compliance tests.\n\nThe sequence for using this functionality is as follows:\n\n1. Disconnect any plugged in devices.\n\n2. Perform USBCMD.HCRST or power-on-chip reset.\n\n3. Set PORTSC.PP=0.\n\n4. Set GUSB3PIPECTL. HstPrtCmpl=1. This places the link into\n\ncompliance state.\n\nTo advance the compliance pattern, follow this sequence (toggle\n\nthe set GUSB3PIPECTL. HstPrtCmpl):\n\n1. Set GUSB3PIPECTL.HstPrtCmpl=0.\n\n2. Set GUSB3PIPECTL.HstPrtCmpl=1. This advances the link to\n\nthe next compliance pattern.\n\nTo exit from the compliance state perform USBCMD.HCRST or\n\npower-on-chip reset."]
    #[inline(always)]
    #[must_use]
    pub fn hstprtcmpl(&mut self) -> HstprtcmplW<Usb3Gusb3pipectl0Spec> {
        HstprtcmplW::new(self, 30)
    }
    #[doc = "Bit 31 - USB3 PHY Soft Reset\n\nAfter setting this bit to 1, the software needs to clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn physoftrst(&mut self) -> PhysoftrstW<Usb3Gusb3pipectl0Spec> {
        PhysoftrstW::new(self, 31)
    }
}
#[doc = "Global USB3 PIPE Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gusb3pipectl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gusb3pipectl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3Gusb3pipectl0Spec;
impl crate::RegisterSpec for Usb3Gusb3pipectl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_gusb3pipectl0::R`](R) reader structure"]
impl crate::Readable for Usb3Gusb3pipectl0Spec {}
#[doc = "`write(|w| ..)` method takes [`usb3_gusb3pipectl0::W`](W) writer structure"]
impl crate::Writable for Usb3Gusb3pipectl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_GUSB3PIPECTL0 to value 0x010c_0002"]
impl crate::Resettable for Usb3Gusb3pipectl0Spec {
    const RESET_VALUE: u32 = 0x010c_0002;
}
