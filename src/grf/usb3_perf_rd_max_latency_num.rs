#[doc = "Register `USB3_PERF_RD_MAX_LATENCY_NUM` reader"]
pub type R = crate::R<Usb3PerfRdMaxLatencyNumSpec>;
#[doc = "Register `USB3_PERF_RD_MAX_LATENCY_NUM` writer"]
pub type W = crate::W<Usb3PerfRdMaxLatencyNumSpec>;
#[doc = "Field `RD_MAX_LATENCY_R` reader - axi read max latency oaxi read max latency\n\noutpututput"]
pub type RdMaxLatencyRR = crate::FieldReader<u16>;
#[doc = "Field `RD_MAX_LATENCY_R` writer - axi read max latency oaxi read max latency\n\noutpututput"]
pub type RdMaxLatencyRW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - axi read max latency oaxi read max latency\n\noutpututput"]
    #[inline(always)]
    pub fn rd_max_latency_r(&self) -> RdMaxLatencyRR {
        RdMaxLatencyRR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - axi read max latency oaxi read max latency\n\noutpututput"]
    #[inline(always)]
    #[must_use]
    pub fn rd_max_latency_r(&mut self) -> RdMaxLatencyRW<Usb3PerfRdMaxLatencyNumSpec> {
        RdMaxLatencyRW::new(self, 0)
    }
}
#[doc = "usb3 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_perf_rd_max_latency_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_perf_rd_max_latency_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3PerfRdMaxLatencyNumSpec;
impl crate::RegisterSpec for Usb3PerfRdMaxLatencyNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_perf_rd_max_latency_num::R`](R) reader structure"]
impl crate::Readable for Usb3PerfRdMaxLatencyNumSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_perf_rd_max_latency_num::W`](W) writer structure"]
impl crate::Writable for Usb3PerfRdMaxLatencyNumSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_PERF_RD_MAX_LATENCY_NUM to value 0"]
impl crate::Resettable for Usb3PerfRdMaxLatencyNumSpec {
    const RESET_VALUE: u32 = 0;
}
