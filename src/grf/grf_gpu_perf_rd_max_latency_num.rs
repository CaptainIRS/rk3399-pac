#[doc = "Register `GRF_GPU_PERF_RD_MAX_LATENCY_NUM` reader"]
pub type R = crate::R<GrfGpuPerfRdMaxLatencyNumSpec>;
#[doc = "Register `GRF_GPU_PERF_RD_MAX_LATENCY_NUM` writer"]
pub type W = crate::W<GrfGpuPerfRdMaxLatencyNumSpec>;
#[doc = "Field `RD_MAX_LATENCY_R` reader - axi read max latency output"]
pub type RdMaxLatencyRR = crate::FieldReader<u16>;
#[doc = "Field `RD_MAX_LATENCY_R` writer - axi read max latency output"]
pub type RdMaxLatencyRW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - axi read max latency output"]
    #[inline(always)]
    pub fn rd_max_latency_r(&self) -> RdMaxLatencyRR {
        RdMaxLatencyRR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - axi read max latency output"]
    #[inline(always)]
    #[must_use]
    pub fn rd_max_latency_r(&mut self) -> RdMaxLatencyRW<GrfGpuPerfRdMaxLatencyNumSpec> {
        RdMaxLatencyRW::new(self, 0)
    }
}
#[doc = "gpu performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpu_perf_rd_max_latency_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpu_perf_rd_max_latency_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpuPerfRdMaxLatencyNumSpec;
impl crate::RegisterSpec for GrfGpuPerfRdMaxLatencyNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpu_perf_rd_max_latency_num::R`](R) reader structure"]
impl crate::Readable for GrfGpuPerfRdMaxLatencyNumSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpu_perf_rd_max_latency_num::W`](W) writer structure"]
impl crate::Writable for GrfGpuPerfRdMaxLatencyNumSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPU_PERF_RD_MAX_LATENCY_NUM to value 0"]
impl crate::Resettable for GrfGpuPerfRdMaxLatencyNumSpec {
    const RESET_VALUE: u32 = 0;
}
