#[doc = "Register `SWREG168_PERF_RD_LATENCY_ACC_SUM` reader"]
pub type R = crate::R<Swreg168PerfRdLatencyAccSumSpec>;
#[doc = "Field `RD_LATENCY_ACC_SUM` reader - rd_latency_acc_sum"]
pub type RdLatencyAccSumR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - rd_latency_acc_sum"]
    #[inline(always)]
    pub fn rd_latency_acc_sum(&self) -> RdLatencyAccSumR {
        RdLatencyAccSumR::new(self.bits)
    }
}
#[doc = "Total sample number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg168_perf_rd_latency_acc_sum::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg168PerfRdLatencyAccSumSpec;
impl crate::RegisterSpec for Swreg168PerfRdLatencyAccSumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg168_perf_rd_latency_acc_sum::R`](R) reader structure"]
impl crate::Readable for Swreg168PerfRdLatencyAccSumSpec {}
#[doc = "`reset()` method sets SWREG168_PERF_RD_LATENCY_ACC_SUM to value 0"]
impl crate::Resettable for Swreg168PerfRdLatencyAccSumSpec {
    const RESET_VALUE: u32 = 0;
}
