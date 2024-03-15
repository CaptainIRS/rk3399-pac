#[doc = "Register `PCIE_CLIENT_DEBUG_OUT_1` reader"]
pub type R = crate::R<PcieClientDebugOut1Spec>;
#[doc = "Field `PERF_DATA_OUT` reader - Performance data out Each of the bits of this vector is explained below: Bit\\[17\\]:Pulse appears when event happens,described in Appendix B"]
pub type PerfDataOutR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:17 - Performance data out Each of the bits of this vector is explained below: Bit\\[17\\]:Pulse appears when event happens,described in Appendix B"]
    #[inline(always)]
    pub fn perf_data_out(&self) -> PerfDataOutR {
        PerfDataOutR::new(self.bits & 0x0003_ffff)
    }
}
#[doc = "Debug information 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_debug_out_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientDebugOut1Spec;
impl crate::RegisterSpec for PcieClientDebugOut1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_debug_out_1::R`](R) reader structure"]
impl crate::Readable for PcieClientDebugOut1Spec {}
#[doc = "`reset()` method sets PCIE_CLIENT_DEBUG_OUT_1 to value 0"]
impl crate::Resettable for PcieClientDebugOut1Spec {
    const RESET_VALUE: u32 = 0;
}
