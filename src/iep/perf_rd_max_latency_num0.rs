#[doc = "Register `PERF_RD_MAX_LATENCY_NUM0` reader"]
pub type R = crate::R<PerfRdMaxLatencyNum0Spec>;
#[doc = "Field `RD_MAX_LATENCY_NUM_CH0` reader - read max latency value of channel 0"]
pub type RdMaxLatencyNumCh0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - read max latency value of channel 0"]
    #[inline(always)]
    pub fn rd_max_latency_num_ch0(&self) -> RdMaxLatencyNumCh0R {
        RdMaxLatencyNumCh0R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Read max latency number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perf_rd_max_latency_num0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PerfRdMaxLatencyNum0Spec;
impl crate::RegisterSpec for PerfRdMaxLatencyNum0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perf_rd_max_latency_num0::R`](R) reader structure"]
impl crate::Readable for PerfRdMaxLatencyNum0Spec {}
#[doc = "`reset()` method sets PERF_RD_MAX_LATENCY_NUM0 to value 0"]
impl crate::Resettable for PerfRdMaxLatencyNum0Spec {
    const RESET_VALUE: u32 = 0;
}
