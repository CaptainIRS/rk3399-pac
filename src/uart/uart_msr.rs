#[doc = "Register `UART_MSR` reader"]
pub type R = crate::R<UartMsrSpec>;
#[doc = "Field `DELTA_CLEAR_TO_SEND` reader - Delta Clear to Send.\n\nThis is used to indicate that the modem control line cts_n has\n\nchanged since the last time the MSR was read."]
pub type DeltaClearToSendR = crate::BitReader;
#[doc = "Field `DELTA_DATA_SET_READY` reader - Delta Data Set Ready.\n\nThis is used to indicate that the modem control line dsr_n has\n\nchanged since the last time the MSR was read."]
pub type DeltaDataSetReadyR = crate::BitReader;
#[doc = "Field `TRAILING_EDGE_RING_INDICATOR` reader - Trailing Edge of Ring Indicator.\n\nTrailing Edge of Ring Indicator. This is used to indicate that a\n\nchange on the input ri_n (from an active-low to an inactive-high\n\nstate) has occurred since the last time the MSR was read."]
pub type TrailingEdgeRingIndicatorR = crate::BitReader;
#[doc = "Field `DELTA_DATA_CARRIER_DETECT` reader - Delta Data Carrier Detect.\n\nThis is used to indicate that the modem control line dcd_n has\n\nchanged since the last time the MSR was read."]
pub type DeltaDataCarrierDetectR = crate::BitReader;
#[doc = "Field `CLEAR_TO_SEND` reader - Clear to Send.\n\nThis is used to indicate the current state of the modem control\n\nline cts_n."]
pub type ClearToSendR = crate::BitReader;
#[doc = "Field `DATA_SET_READY` reader - Data Set Ready.\n\nThis is used to indicate the current state of the modem control\n\nline dsr_n."]
pub type DataSetReadyR = crate::BitReader;
#[doc = "Field `RING_INDICATOR` reader - Ring Indicator.\n\nThis is used to indicate the current state of the modem control\n\nline ri_n."]
pub type RingIndicatorR = crate::BitReader;
#[doc = "Field `DATA_CARRIOR_DETECT` reader - Data Carrier Detect.\n\nThis is used to indicate the current state of the modem control\n\nline dcd_n."]
pub type DataCarriorDetectR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Delta Clear to Send.\n\nThis is used to indicate that the modem control line cts_n has\n\nchanged since the last time the MSR was read."]
    #[inline(always)]
    pub fn delta_clear_to_send(&self) -> DeltaClearToSendR {
        DeltaClearToSendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Delta Data Set Ready.\n\nThis is used to indicate that the modem control line dsr_n has\n\nchanged since the last time the MSR was read."]
    #[inline(always)]
    pub fn delta_data_set_ready(&self) -> DeltaDataSetReadyR {
        DeltaDataSetReadyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trailing Edge of Ring Indicator.\n\nTrailing Edge of Ring Indicator. This is used to indicate that a\n\nchange on the input ri_n (from an active-low to an inactive-high\n\nstate) has occurred since the last time the MSR was read."]
    #[inline(always)]
    pub fn trailing_edge_ring_indicator(&self) -> TrailingEdgeRingIndicatorR {
        TrailingEdgeRingIndicatorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Delta Data Carrier Detect.\n\nThis is used to indicate that the modem control line dcd_n has\n\nchanged since the last time the MSR was read."]
    #[inline(always)]
    pub fn delta_data_carrier_detect(&self) -> DeltaDataCarrierDetectR {
        DeltaDataCarrierDetectR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear to Send.\n\nThis is used to indicate the current state of the modem control\n\nline cts_n."]
    #[inline(always)]
    pub fn clear_to_send(&self) -> ClearToSendR {
        ClearToSendR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Set Ready.\n\nThis is used to indicate the current state of the modem control\n\nline dsr_n."]
    #[inline(always)]
    pub fn data_set_ready(&self) -> DataSetReadyR {
        DataSetReadyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Ring Indicator.\n\nThis is used to indicate the current state of the modem control\n\nline ri_n."]
    #[inline(always)]
    pub fn ring_indicator(&self) -> RingIndicatorR {
        RingIndicatorR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data Carrier Detect.\n\nThis is used to indicate the current state of the modem control\n\nline dcd_n."]
    #[inline(always)]
    pub fn data_carrior_detect(&self) -> DataCarriorDetectR {
        DataCarriorDetectR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Modem Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_msr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartMsrSpec;
impl crate::RegisterSpec for UartMsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_msr::R`](R) reader structure"]
impl crate::Readable for UartMsrSpec {}
#[doc = "`reset()` method sets UART_MSR to value 0"]
impl crate::Resettable for UartMsrSpec {
    const RESET_VALUE: u32 = 0;
}
