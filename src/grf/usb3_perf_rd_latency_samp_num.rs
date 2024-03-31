#[doc = "Register `USB3_PERF_RD_LATENCY_SAMP_NUM` reader"]
pub type R = crate::R<Usb3PerfRdLatencySampNumSpec>;
#[doc = "Register `USB3_PERF_RD_LATENCY_SAMP_NUM` writer"]
pub type W = crate::W<Usb3PerfRdLatencySampNumSpec>;
#[doc = "Field `RD_LATENCY_SAMP_R` reader - AXI read latency total sample number"]
pub type RdLatencySampRR = crate::FieldReader<u32>;
#[doc = "Field `RD_LATENCY_SAMP_R` writer - AXI read latency total sample number"]
pub type RdLatencySampRW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AXI read latency total sample number"]
    #[inline(always)]
    pub fn rd_latency_samp_r(&self) -> RdLatencySampRR {
        RdLatencySampRR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AXI read latency total sample number"]
    #[inline(always)]
    #[must_use]
    pub fn rd_latency_samp_r(&mut self) -> RdLatencySampRW<Usb3PerfRdLatencySampNumSpec> {
        RdLatencySampRW::new(self, 0)
    }
}
#[doc = "usb3 performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_perf_rd_latency_samp_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_perf_rd_latency_samp_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3PerfRdLatencySampNumSpec;
impl crate::RegisterSpec for Usb3PerfRdLatencySampNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_perf_rd_latency_samp_num::R`](R) reader structure"]
impl crate::Readable for Usb3PerfRdLatencySampNumSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_perf_rd_latency_samp_num::W`](W) writer structure"]
impl crate::Writable for Usb3PerfRdLatencySampNumSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_PERF_RD_LATENCY_SAMP_NUM to value 0"]
impl crate::Resettable for Usb3PerfRdLatencySampNumSpec {
    const RESET_VALUE: u32 = 0;
}
