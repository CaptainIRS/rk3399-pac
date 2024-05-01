#[doc = "Register `PERF_RD_LATENCY_ACC_SUM` reader"]
pub type R = crate::R<PerfRdLatencyAccSumSpec>;
#[doc = "Field `RD_LATENCY_ACC_SUM` reader - rd_latency_acc_sum"]
pub type RdLatencyAccSumR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - rd_latency_acc_sum"]
    #[inline(always)]
    pub fn rd_latency_acc_sum(&self) -> RdLatencyAccSumR {
        RdLatencyAccSumR::new(self.bits)
    }
}
#[doc = "Total sample number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perf_rd_latency_acc_sum::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerfRdLatencyAccSumSpec;
impl crate::RegisterSpec for PerfRdLatencyAccSumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perf_rd_latency_acc_sum::R`](R) reader structure"]
impl crate::Readable for PerfRdLatencyAccSumSpec {}
#[doc = "`reset()` method sets PERF_RD_LATENCY_ACC_SUM to value 0"]
impl crate::Resettable for PerfRdLatencyAccSumSpec {
    const RESET_VALUE: u32 = 0;
}
