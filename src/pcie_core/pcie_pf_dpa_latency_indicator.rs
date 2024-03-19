#[doc = "Register `PCIE_PF_DPA_LATENCY_INDICATOR` reader"]
pub type R = crate::R<PciePfDpaLatencyIndicatorSpec>;
#[doc = "Field `TLIN` reader - Transition Latency Indicator Bits \\[TLIN\\]\n\nBit i of this register indicates the\n\nchoice of the transition latency value\n\nfor substate i. A setting of 0\n\nindicates that Transition Latency\n\nValue 0 from the DPA Capability\n\nRegister applies to this substate; a\n\nsetting of 1 indicates that Transition\n\nLatency Value 1 applies."]
pub type TlinR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transition Latency Indicator Bits \\[TLIN\\]\n\nBit i of this register indicates the\n\nchoice of the transition latency value\n\nfor substate i. A setting of 0\n\nindicates that Transition Latency\n\nValue 0 from the DPA Capability\n\nRegister applies to this substate; a\n\nsetting of 1 indicates that Transition\n\nLatency Value 1 applies."]
    #[inline(always)]
    pub fn tlin(&self) -> TlinR {
        TlinR::new(self.bits)
    }
}
#[doc = "DPA Latency Indicator Register\n\nBit i of this register indicates the\n\nchoice of the transition latency value\n\nfor substate i. A setting of 0\n\nindicates that Transition Latency\n\nValue 0 from the DPA Capability\n\nRegister applies to this substate; a\n\nsetting of 1 indicates that Transition\n\nLatency Value 1 applies.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_dpa_latency_indicator::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfDpaLatencyIndicatorSpec;
impl crate::RegisterSpec for PciePfDpaLatencyIndicatorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_dpa_latency_indicator::R`](R) reader structure"]
impl crate::Readable for PciePfDpaLatencyIndicatorSpec {}
#[doc = "`reset()` method sets PCIE_PF_DPA_LATENCY_INDICATOR to value 0"]
impl crate::Resettable for PciePfDpaLatencyIndicatorSpec {
    const RESET_VALUE: u32 = 0;
}
