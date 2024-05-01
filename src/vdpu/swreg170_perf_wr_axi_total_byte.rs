#[doc = "Register `SWREG170_PERF_WR_AXI_TOTAL_BYTE` reader"]
pub type R = crate::R<Swreg170PerfWrAxiTotalByteSpec>;
#[doc = "Register `SWREG170_PERF_WR_AXI_TOTAL_BYTE` writer"]
pub type W = crate::W<Swreg170PerfWrAxiTotalByteSpec>;
#[doc = "Field `PERF_WR_AXI_TOTAL_BYTE` reader - perf_wr_axi_total_byte\n\nperf_wr_axi_total_byte"]
pub type PerfWrAxiTotalByteR = crate::FieldReader<u32>;
#[doc = "Field `PERF_WR_AXI_TOTAL_BYTE` writer - perf_wr_axi_total_byte\n\nperf_wr_axi_total_byte"]
pub type PerfWrAxiTotalByteW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - perf_wr_axi_total_byte\n\nperf_wr_axi_total_byte"]
    #[inline(always)]
    pub fn perf_wr_axi_total_byte(&self) -> PerfWrAxiTotalByteR {
        PerfWrAxiTotalByteR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - perf_wr_axi_total_byte\n\nperf_wr_axi_total_byte"]
    #[inline(always)]
    #[must_use]
    pub fn perf_wr_axi_total_byte(
        &mut self,
    ) -> PerfWrAxiTotalByteW<Swreg170PerfWrAxiTotalByteSpec> {
        PerfWrAxiTotalByteW::new(self, 0)
    }
}
#[doc = "perf_wr_axi_total_byte\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg170_perf_wr_axi_total_byte::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg170_perf_wr_axi_total_byte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg170PerfWrAxiTotalByteSpec;
impl crate::RegisterSpec for Swreg170PerfWrAxiTotalByteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg170_perf_wr_axi_total_byte::R`](R) reader structure"]
impl crate::Readable for Swreg170PerfWrAxiTotalByteSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg170_perf_wr_axi_total_byte::W`](W) writer structure"]
impl crate::Writable for Swreg170PerfWrAxiTotalByteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG170_PERF_WR_AXI_TOTAL_BYTE to value 0"]
impl crate::Resettable for Swreg170PerfWrAxiTotalByteSpec {
    const RESET_VALUE: u32 = 0;
}
