#[doc = "Register `DPA_LATENCY_INDICATOR` reader"]
pub type R = crate::R<DpaLatencyIndicatorSpec>;
#[doc = "Field `TLIN` reader - Transition Latency Indicator Bits \\[TLIN\\]
Bit i of this register indicates the choice of the transition latency value for substate i. A setting of 0 indicates that Transition Latency Value 0 from the DPA Capability Register applies to this substate; a setting of 1 indicates that Transition Latency Value 1 applies."]
pub type TlinR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transition Latency Indicator Bits \\[TLIN\\]
Bit i of this register indicates the choice of the transition latency value for substate i. A setting of 0 indicates that Transition Latency Value 0 from the DPA Capability Register applies to this substate; a setting of 1 indicates that Transition Latency Value 1 applies."]
    #[inline(always)]
    pub fn tlin(&self) -> TlinR {
        TlinR::new(self.bits)
    }
}
#[doc = "DPA Latency Indicator Register Bit i of this register indicates the choice of the transition latency value for substate i. A setting of 0 indicates that Transition Latency Value 0 from the DPA Capability Register applies to this substate; a setting of 1 indicates that Transition Latency Value 1 applies.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpa_latency_indicator::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpaLatencyIndicatorSpec;
impl crate::RegisterSpec for DpaLatencyIndicatorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpa_latency_indicator::R`](R) reader structure"]
impl crate::Readable for DpaLatencyIndicatorSpec {}
#[doc = "`reset()` method sets DPA_LATENCY_INDICATOR to value 0"]
impl crate::Resettable for DpaLatencyIndicatorSpec {
    const RESET_VALUE: u32 = 0;
}
