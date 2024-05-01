#[doc = "Register `SWREG167_PERF_RD_LATENCY_SAMP_NUM` reader"]
pub type R = crate::R<Swreg167PerfRdLatencySampNumSpec>;
#[doc = "Field `RD_LATENCY_THR_NUM_CH0` reader - read latency thr number channel 0"]
pub type RdLatencyThrNumCh0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - read latency thr number channel 0"]
    #[inline(always)]
    pub fn rd_latency_thr_num_ch0(&self) -> RdLatencyThrNumCh0R {
        RdLatencyThrNumCh0R::new(self.bits)
    }
}
#[doc = "The number of bigger than configed threshold value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg167_perf_rd_latency_samp_num::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg167PerfRdLatencySampNumSpec;
impl crate::RegisterSpec for Swreg167PerfRdLatencySampNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg167_perf_rd_latency_samp_num::R`](R) reader structure"]
impl crate::Readable for Swreg167PerfRdLatencySampNumSpec {}
#[doc = "`reset()` method sets SWREG167_PERF_RD_LATENCY_SAMP_NUM to value 0"]
impl crate::Resettable for Swreg167PerfRdLatencySampNumSpec {
    const RESET_VALUE: u32 = 0;
}
