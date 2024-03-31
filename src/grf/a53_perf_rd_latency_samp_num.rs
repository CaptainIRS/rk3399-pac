#[doc = "Register `A53_PERF_RD_LATENCY_SAMP_NUM` reader"]
pub type R = crate::R<A53PerfRdLatencySampNumSpec>;
#[doc = "Register `A53_PERF_RD_LATENCY_SAMP_NUM` writer"]
pub type W = crate::W<A53PerfRdLatencySampNumSpec>;
#[doc = "Field `RD_LATENCY_SAMP_R` reader - AXI read latency total sample number"]
pub type RdLatencySampRR = crate::FieldReader<u32>;
#[doc = "Field `RD_LATENCY_SAMP_R` writer - AXI read latency total sample number"]
pub type RdLatencySampRW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 0:26 - AXI read latency total sample number"]
    #[inline(always)]
    pub fn rd_latency_samp_r(&self) -> RdLatencySampRR {
        RdLatencySampRR::new(self.bits & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:26 - AXI read latency total sample number"]
    #[inline(always)]
    #[must_use]
    pub fn rd_latency_samp_r(&mut self) -> RdLatencySampRW<A53PerfRdLatencySampNumSpec> {
        RdLatencySampRW::new(self, 0)
    }
}
#[doc = "a53 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a53_perf_rd_latency_samp_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a53_perf_rd_latency_samp_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A53PerfRdLatencySampNumSpec;
impl crate::RegisterSpec for A53PerfRdLatencySampNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a53_perf_rd_latency_samp_num::R`](R) reader structure"]
impl crate::Readable for A53PerfRdLatencySampNumSpec {}
#[doc = "`write(|w| ..)` method takes [`a53_perf_rd_latency_samp_num::W`](W) writer structure"]
impl crate::Writable for A53PerfRdLatencySampNumSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets A53_PERF_RD_LATENCY_SAMP_NUM to value 0"]
impl crate::Resettable for A53PerfRdLatencySampNumSpec {
    const RESET_VALUE: u32 = 0;
}
