#[doc = "Register `SWREG169_PERF_RD_AXI_TOTAL_BYTE` reader"]
pub type R = crate::R<Swreg169PerfRdAxiTotalByteSpec>;
#[doc = "Register `SWREG169_PERF_RD_AXI_TOTAL_BYTE` writer"]
pub type W = crate::W<Swreg169PerfRdAxiTotalByteSpec>;
#[doc = "Field `PERF_RD_AXI_TOTAL_BYTE` reader - perf_rd_axi_total_byte\n\nperf_rd_axi_total_byte"]
pub type PerfRdAxiTotalByteR = crate::FieldReader<u32>;
#[doc = "Field `PERF_RD_AXI_TOTAL_BYTE` writer - perf_rd_axi_total_byte\n\nperf_rd_axi_total_byte"]
pub type PerfRdAxiTotalByteW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - perf_rd_axi_total_byte\n\nperf_rd_axi_total_byte"]
    #[inline(always)]
    pub fn perf_rd_axi_total_byte(&self) -> PerfRdAxiTotalByteR {
        PerfRdAxiTotalByteR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - perf_rd_axi_total_byte\n\nperf_rd_axi_total_byte"]
    #[inline(always)]
    #[must_use]
    pub fn perf_rd_axi_total_byte(
        &mut self,
    ) -> PerfRdAxiTotalByteW<Swreg169PerfRdAxiTotalByteSpec> {
        PerfRdAxiTotalByteW::new(self, 0)
    }
}
#[doc = "perf_rd_axi_total_byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg169_perf_rd_axi_total_byte::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg169_perf_rd_axi_total_byte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg169PerfRdAxiTotalByteSpec;
impl crate::RegisterSpec for Swreg169PerfRdAxiTotalByteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg169_perf_rd_axi_total_byte::R`](R) reader structure"]
impl crate::Readable for Swreg169PerfRdAxiTotalByteSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg169_perf_rd_axi_total_byte::W`](W) writer structure"]
impl crate::Writable for Swreg169PerfRdAxiTotalByteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG169_PERF_RD_AXI_TOTAL_BYTE to value 0"]
impl crate::Resettable for Swreg169PerfRdAxiTotalByteSpec {
    const RESET_VALUE: u32 = 0;
}
