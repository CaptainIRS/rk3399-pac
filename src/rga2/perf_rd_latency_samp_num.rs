#[doc = "Register `PERF_RD_LATENCY_SAMP_NUM` reader"]
pub type R = crate::R<PerfRdLatencySampNumSpec>;
#[doc = "Field `RD_LATENCY_THR_NUM_CH0` reader - read latency thr number channel 0"]
pub type RdLatencyThrNumCh0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - read latency thr number channel 0"]
    #[inline(always)]
    pub fn rd_latency_thr_num_ch0(&self) -> RdLatencyThrNumCh0R {
        RdLatencyThrNumCh0R::new(self.bits)
    }
}
#[doc = "The number of bigger than configed threshold value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perf_rd_latency_samp_num::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerfRdLatencySampNumSpec;
impl crate::RegisterSpec for PerfRdLatencySampNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perf_rd_latency_samp_num::R`](R) reader structure"]
impl crate::Readable for PerfRdLatencySampNumSpec {}
#[doc = "`reset()` method sets PERF_RD_LATENCY_SAMP_NUM to value 0"]
impl crate::Resettable for PerfRdLatencySampNumSpec {
    const RESET_VALUE: u32 = 0;
}
