#[doc = "Register `GMAC_FLOW_CTRL` reader"]
pub type R = crate::R<GmacFlowCtrlSpec>;
#[doc = "Register `GMAC_FLOW_CTRL` writer"]
pub type W = crate::W<GmacFlowCtrlSpec>;
#[doc = "Field `FCB_BPA` reader - Flow Control Busy/Backpressure Activate\n\nThis bit initiates a Pause Control frame in Full-Duplex mode and\n\nactivates the backpressure function in Half-Duplex mode if TFE\n\nbit is set.\n\nIn Full-Duplex mode, this bit should be read as 1'b0 before\n\nwriting to the register GMAC_FLOW_CTRL. To initiate a pause\n\ncontrol frame, the application must set this bit to 1'b1. During a\n\ntransfer of the control frame, this bit will continue to be set to\n\nsignify that a frame transmission is in progress. After the\n\ncompletion of Pause control frame transmission, the GMAC will\n\nreset this bit to 1'b0. The register GMAC_FLOW_CTRL should not\n\nbe written to until this bit is cleared.\n\nIn Half-Duplex mode, when this bit is set (and TFE is set), then\n\nbackpressure is asserted by the GMAC Core. During\n\nbackpressure, when the GMAC receives a new frame, the\n\ntransmitter starts sending a JAM pattern resulting in a collision.\n\nThis control register bit is logically OR'ed with the mti_flowctrl_i\n\ninput signal for the backpressure function."]
pub type FcbBpaR = crate::BitReader;
#[doc = "Field `FCB_BPA` writer - Flow Control Busy/Backpressure Activate\n\nThis bit initiates a Pause Control frame in Full-Duplex mode and\n\nactivates the backpressure function in Half-Duplex mode if TFE\n\nbit is set.\n\nIn Full-Duplex mode, this bit should be read as 1'b0 before\n\nwriting to the register GMAC_FLOW_CTRL. To initiate a pause\n\ncontrol frame, the application must set this bit to 1'b1. During a\n\ntransfer of the control frame, this bit will continue to be set to\n\nsignify that a frame transmission is in progress. After the\n\ncompletion of Pause control frame transmission, the GMAC will\n\nreset this bit to 1'b0. The register GMAC_FLOW_CTRL should not\n\nbe written to until this bit is cleared.\n\nIn Half-Duplex mode, when this bit is set (and TFE is set), then\n\nbackpressure is asserted by the GMAC Core. During\n\nbackpressure, when the GMAC receives a new frame, the\n\ntransmitter starts sending a JAM pattern resulting in a collision.\n\nThis control register bit is logically OR'ed with the mti_flowctrl_i\n\ninput signal for the backpressure function."]
pub type FcbBpaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFE` reader - Transmit Flow Control Enable\n\nIn Full-Duplex mode, when this bit is set, the GMAC enables the\n\nflow control operation to transmit Pause frames. When this bit is\n\nreset, the flow control operation in the GMAC is disabled, and the\n\nGMAC will not transmit any Pause frames.\n\nIn Half-Duplex mode, when this bit is set, the GMAC enables the\n\nback-pressure operation. When this bit is reset, the backpressure\n\nfeature is disabled."]
pub type TfeR = crate::BitReader;
#[doc = "Field `TFE` writer - Transmit Flow Control Enable\n\nIn Full-Duplex mode, when this bit is set, the GMAC enables the\n\nflow control operation to transmit Pause frames. When this bit is\n\nreset, the flow control operation in the GMAC is disabled, and the\n\nGMAC will not transmit any Pause frames.\n\nIn Half-Duplex mode, when this bit is set, the GMAC enables the\n\nback-pressure operation. When this bit is reset, the backpressure\n\nfeature is disabled."]
pub type TfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFE` reader - Receive Flow Control Enable\n\nWhen this bit is set, the GMAC will decode the received Pause\n\nframe and disable its transmitter for a specified (Pause Time)\n\ntime. When this bit is reset, the decode function of the Pause\n\nframe is disabled."]
pub type RfeR = crate::BitReader;
#[doc = "Field `RFE` writer - Receive Flow Control Enable\n\nWhen this bit is set, the GMAC will decode the received Pause\n\nframe and disable its transmitter for a specified (Pause Time)\n\ntime. When this bit is reset, the decode function of the Pause\n\nframe is disabled."]
pub type RfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UP` reader - Unicast Pause Frame Detect\n\nWhen this bit is set, the GMAC will detect the Pause frames with\n\nthe station's unicast address specified in MAC Address0 High\n\nRegister and MAC Address0 Low Register, in addition to the\n\ndetecting Pause frames with the unique multicast address. When\n\nthis bit is reset, the GMAC will detect only a Pause frame with the\n\nunique multicast address specified in the 802.3x standard."]
pub type UpR = crate::BitReader;
#[doc = "Field `UP` writer - Unicast Pause Frame Detect\n\nWhen this bit is set, the GMAC will detect the Pause frames with\n\nthe station's unicast address specified in MAC Address0 High\n\nRegister and MAC Address0 Low Register, in addition to the\n\ndetecting Pause frames with the unique multicast address. When\n\nthis bit is reset, the GMAC will detect only a Pause frame with the\n\nunique multicast address specified in the 802.3x standard."]
pub type UpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLT` reader - Pause Low Threshold\n\nThis field configures the threshold of the PAUSE timer at which\n\nthe input flow control signal mti_flowctrl_i (or sbd_flowctrl_i) is\n\nchecked for automatic retransmission of PAUSE Frame. The\n\nthreshold values should be always less than the Pause Time\n\nconfigured in Bits\\[31:16\\]. For example, if PT = 100H (256 slot-\n\ntimes), and PLT = 01, then a second PAUSE frame is\n\nautomatically transmitted if the mti_flowctrl_i signal is asserted\n\nat 228 (256-28) slot-times after the first PAUSE frame is\n\ntransmitted.\n\nSelection Threshold\n\n00 Pause time minus 4 slot times\n\n01 Pause time minus 28 slot times\n\n10 Pause time minus 144 slot times\n\n11 Pause time minus 256 slot times\n\nSlot time is defined as time taken to transmit 512 bits (64 bytes)\n\non the GMII/MII interface."]
pub type PltR = crate::FieldReader;
#[doc = "Field `PLT` writer - Pause Low Threshold\n\nThis field configures the threshold of the PAUSE timer at which\n\nthe input flow control signal mti_flowctrl_i (or sbd_flowctrl_i) is\n\nchecked for automatic retransmission of PAUSE Frame. The\n\nthreshold values should be always less than the Pause Time\n\nconfigured in Bits\\[31:16\\]. For example, if PT = 100H (256 slot-\n\ntimes), and PLT = 01, then a second PAUSE frame is\n\nautomatically transmitted if the mti_flowctrl_i signal is asserted\n\nat 228 (256-28) slot-times after the first PAUSE frame is\n\ntransmitted.\n\nSelection Threshold\n\n00 Pause time minus 4 slot times\n\n01 Pause time minus 28 slot times\n\n10 Pause time minus 144 slot times\n\n11 Pause time minus 256 slot times\n\nSlot time is defined as time taken to transmit 512 bits (64 bytes)\n\non the GMII/MII interface."]
pub type PltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DZPQ` reader - Disable Zero-Quanta Pause\n\nWhen set, this bit disables the automatic generation of Zero-\n\nQuanta Pause Control frames on the de-assertion of the flow-\n\ncontrol signal from the FIFO layer (MTL or external sideband flow\n\ncontrol signal sbd_flowctrl_i/mti_flowctrl_i).\n\nWhen this bit is reset, normal operation with automatic Zero-\n\nQuanta Pause Control frame generation is enabled."]
pub type DzpqR = crate::BitReader;
#[doc = "Field `DZPQ` writer - Disable Zero-Quanta Pause\n\nWhen set, this bit disables the automatic generation of Zero-\n\nQuanta Pause Control frames on the de-assertion of the flow-\n\ncontrol signal from the FIFO layer (MTL or external sideband flow\n\ncontrol signal sbd_flowctrl_i/mti_flowctrl_i).\n\nWhen this bit is reset, normal operation with automatic Zero-\n\nQuanta Pause Control frame generation is enabled."]
pub type DzpqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PT` reader - Pause Time\n\nThis field holds the value to be used in the Pause Time field in the\n\ntransmit control frame. If the Pause Time bits is configured to be\n\ndouble-synchronized to the (G)MII clock domain, then\n\nconsecutive writes to this register should be performed only after\n\nat least 4 clock cycles in the destination clock domain."]
pub type PtR = crate::FieldReader<u16>;
#[doc = "Field `PT` writer - Pause Time\n\nThis field holds the value to be used in the Pause Time field in the\n\ntransmit control frame. If the Pause Time bits is configured to be\n\ndouble-synchronized to the (G)MII clock domain, then\n\nconsecutive writes to this register should be performed only after\n\nat least 4 clock cycles in the destination clock domain."]
pub type PtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Flow Control Busy/Backpressure Activate\n\nThis bit initiates a Pause Control frame in Full-Duplex mode and\n\nactivates the backpressure function in Half-Duplex mode if TFE\n\nbit is set.\n\nIn Full-Duplex mode, this bit should be read as 1'b0 before\n\nwriting to the register GMAC_FLOW_CTRL. To initiate a pause\n\ncontrol frame, the application must set this bit to 1'b1. During a\n\ntransfer of the control frame, this bit will continue to be set to\n\nsignify that a frame transmission is in progress. After the\n\ncompletion of Pause control frame transmission, the GMAC will\n\nreset this bit to 1'b0. The register GMAC_FLOW_CTRL should not\n\nbe written to until this bit is cleared.\n\nIn Half-Duplex mode, when this bit is set (and TFE is set), then\n\nbackpressure is asserted by the GMAC Core. During\n\nbackpressure, when the GMAC receives a new frame, the\n\ntransmitter starts sending a JAM pattern resulting in a collision.\n\nThis control register bit is logically OR'ed with the mti_flowctrl_i\n\ninput signal for the backpressure function."]
    #[inline(always)]
    pub fn fcb_bpa(&self) -> FcbBpaR {
        FcbBpaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable\n\nIn Full-Duplex mode, when this bit is set, the GMAC enables the\n\nflow control operation to transmit Pause frames. When this bit is\n\nreset, the flow control operation in the GMAC is disabled, and the\n\nGMAC will not transmit any Pause frames.\n\nIn Half-Duplex mode, when this bit is set, the GMAC enables the\n\nback-pressure operation. When this bit is reset, the backpressure\n\nfeature is disabled."]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Flow Control Enable\n\nWhen this bit is set, the GMAC will decode the received Pause\n\nframe and disable its transmitter for a specified (Pause Time)\n\ntime. When this bit is reset, the decode function of the Pause\n\nframe is disabled."]
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect\n\nWhen this bit is set, the GMAC will detect the Pause frames with\n\nthe station's unicast address specified in MAC Address0 High\n\nRegister and MAC Address0 Low Register, in addition to the\n\ndetecting Pause frames with the unique multicast address. When\n\nthis bit is reset, the GMAC will detect only a Pause frame with the\n\nunique multicast address specified in the 802.3x standard."]
    #[inline(always)]
    pub fn up(&self) -> UpR {
        UpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Pause Low Threshold\n\nThis field configures the threshold of the PAUSE timer at which\n\nthe input flow control signal mti_flowctrl_i (or sbd_flowctrl_i) is\n\nchecked for automatic retransmission of PAUSE Frame. The\n\nthreshold values should be always less than the Pause Time\n\nconfigured in Bits\\[31:16\\]. For example, if PT = 100H (256 slot-\n\ntimes), and PLT = 01, then a second PAUSE frame is\n\nautomatically transmitted if the mti_flowctrl_i signal is asserted\n\nat 228 (256-28) slot-times after the first PAUSE frame is\n\ntransmitted.\n\nSelection Threshold\n\n00 Pause time minus 4 slot times\n\n01 Pause time minus 28 slot times\n\n10 Pause time minus 144 slot times\n\n11 Pause time minus 256 slot times\n\nSlot time is defined as time taken to transmit 512 bits (64 bytes)\n\non the GMII/MII interface."]
    #[inline(always)]
    pub fn plt(&self) -> PltR {
        PltR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause\n\nWhen set, this bit disables the automatic generation of Zero-\n\nQuanta Pause Control frames on the de-assertion of the flow-\n\ncontrol signal from the FIFO layer (MTL or external sideband flow\n\ncontrol signal sbd_flowctrl_i/mti_flowctrl_i).\n\nWhen this bit is reset, normal operation with automatic Zero-\n\nQuanta Pause Control frame generation is enabled."]
    #[inline(always)]
    pub fn dzpq(&self) -> DzpqR {
        DzpqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Pause Time\n\nThis field holds the value to be used in the Pause Time field in the\n\ntransmit control frame. If the Pause Time bits is configured to be\n\ndouble-synchronized to the (G)MII clock domain, then\n\nconsecutive writes to this register should be performed only after\n\nat least 4 clock cycles in the destination clock domain."]
    #[inline(always)]
    pub fn pt(&self) -> PtR {
        PtR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flow Control Busy/Backpressure Activate\n\nThis bit initiates a Pause Control frame in Full-Duplex mode and\n\nactivates the backpressure function in Half-Duplex mode if TFE\n\nbit is set.\n\nIn Full-Duplex mode, this bit should be read as 1'b0 before\n\nwriting to the register GMAC_FLOW_CTRL. To initiate a pause\n\ncontrol frame, the application must set this bit to 1'b1. During a\n\ntransfer of the control frame, this bit will continue to be set to\n\nsignify that a frame transmission is in progress. After the\n\ncompletion of Pause control frame transmission, the GMAC will\n\nreset this bit to 1'b0. The register GMAC_FLOW_CTRL should not\n\nbe written to until this bit is cleared.\n\nIn Half-Duplex mode, when this bit is set (and TFE is set), then\n\nbackpressure is asserted by the GMAC Core. During\n\nbackpressure, when the GMAC receives a new frame, the\n\ntransmitter starts sending a JAM pattern resulting in a collision.\n\nThis control register bit is logically OR'ed with the mti_flowctrl_i\n\ninput signal for the backpressure function."]
    #[inline(always)]
    #[must_use]
    pub fn fcb_bpa(&mut self) -> FcbBpaW<GmacFlowCtrlSpec> {
        FcbBpaW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable\n\nIn Full-Duplex mode, when this bit is set, the GMAC enables the\n\nflow control operation to transmit Pause frames. When this bit is\n\nreset, the flow control operation in the GMAC is disabled, and the\n\nGMAC will not transmit any Pause frames.\n\nIn Half-Duplex mode, when this bit is set, the GMAC enables the\n\nback-pressure operation. When this bit is reset, the backpressure\n\nfeature is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TfeW<GmacFlowCtrlSpec> {
        TfeW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive Flow Control Enable\n\nWhen this bit is set, the GMAC will decode the received Pause\n\nframe and disable its transmitter for a specified (Pause Time)\n\ntime. When this bit is reset, the decode function of the Pause\n\nframe is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn rfe(&mut self) -> RfeW<GmacFlowCtrlSpec> {
        RfeW::new(self, 2)
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect\n\nWhen this bit is set, the GMAC will detect the Pause frames with\n\nthe station's unicast address specified in MAC Address0 High\n\nRegister and MAC Address0 Low Register, in addition to the\n\ndetecting Pause frames with the unique multicast address. When\n\nthis bit is reset, the GMAC will detect only a Pause frame with the\n\nunique multicast address specified in the 802.3x standard."]
    #[inline(always)]
    #[must_use]
    pub fn up(&mut self) -> UpW<GmacFlowCtrlSpec> {
        UpW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Pause Low Threshold\n\nThis field configures the threshold of the PAUSE timer at which\n\nthe input flow control signal mti_flowctrl_i (or sbd_flowctrl_i) is\n\nchecked for automatic retransmission of PAUSE Frame. The\n\nthreshold values should be always less than the Pause Time\n\nconfigured in Bits\\[31:16\\]. For example, if PT = 100H (256 slot-\n\ntimes), and PLT = 01, then a second PAUSE frame is\n\nautomatically transmitted if the mti_flowctrl_i signal is asserted\n\nat 228 (256-28) slot-times after the first PAUSE frame is\n\ntransmitted.\n\nSelection Threshold\n\n00 Pause time minus 4 slot times\n\n01 Pause time minus 28 slot times\n\n10 Pause time minus 144 slot times\n\n11 Pause time minus 256 slot times\n\nSlot time is defined as time taken to transmit 512 bits (64 bytes)\n\non the GMII/MII interface."]
    #[inline(always)]
    #[must_use]
    pub fn plt(&mut self) -> PltW<GmacFlowCtrlSpec> {
        PltW::new(self, 4)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause\n\nWhen set, this bit disables the automatic generation of Zero-\n\nQuanta Pause Control frames on the de-assertion of the flow-\n\ncontrol signal from the FIFO layer (MTL or external sideband flow\n\ncontrol signal sbd_flowctrl_i/mti_flowctrl_i).\n\nWhen this bit is reset, normal operation with automatic Zero-\n\nQuanta Pause Control frame generation is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn dzpq(&mut self) -> DzpqW<GmacFlowCtrlSpec> {
        DzpqW::new(self, 7)
    }
    #[doc = "Bits 16:31 - Pause Time\n\nThis field holds the value to be used in the Pause Time field in the\n\ntransmit control frame. If the Pause Time bits is configured to be\n\ndouble-synchronized to the (G)MII clock domain, then\n\nconsecutive writes to this register should be performed only after\n\nat least 4 clock cycles in the destination clock domain."]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PtW<GmacFlowCtrlSpec> {
        PtW::new(self, 16)
    }
}
#[doc = "Flow Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_flow_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_flow_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacFlowCtrlSpec;
impl crate::RegisterSpec for GmacFlowCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_flow_ctrl::R`](R) reader structure"]
impl crate::Readable for GmacFlowCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_flow_ctrl::W`](W) writer structure"]
impl crate::Writable for GmacFlowCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_FLOW_CTRL to value 0"]
impl crate::Resettable for GmacFlowCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
