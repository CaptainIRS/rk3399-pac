#[doc = "Register `GRF_GMAC_PERF_WR_AXI_TOTAL_BYTE` reader"]
pub type R = crate::R<GrfGmacPerfWrAxiTotalByteSpec>;
#[doc = "Register `GRF_GMAC_PERF_WR_AXI_TOTAL_BYTE` writer"]
pub type W = crate::W<GrfGmacPerfWrAxiTotalByteSpec>;
#[doc = "Field `WR_AXI_TOTAL_BYTE` reader - AXI active total write bytes/ddr align write\n\nbytes"]
pub type WrAxiTotalByteR = crate::FieldReader<u32>;
#[doc = "Field `WR_AXI_TOTAL_BYTE` writer - AXI active total write bytes/ddr align write\n\nbytes"]
pub type WrAxiTotalByteW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AXI active total write bytes/ddr align write\n\nbytes"]
    #[inline(always)]
    pub fn wr_axi_total_byte(&self) -> WrAxiTotalByteR {
        WrAxiTotalByteR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AXI active total write bytes/ddr align write\n\nbytes"]
    #[inline(always)]
    #[must_use]
    pub fn wr_axi_total_byte(&mut self) -> WrAxiTotalByteW<GrfGmacPerfWrAxiTotalByteSpec> {
        WrAxiTotalByteW::new(self, 0)
    }
}
#[doc = "gmac performance monitor status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gmac_perf_wr_axi_total_byte::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gmac_perf_wr_axi_total_byte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGmacPerfWrAxiTotalByteSpec;
impl crate::RegisterSpec for GrfGmacPerfWrAxiTotalByteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gmac_perf_wr_axi_total_byte::R`](R) reader structure"]
impl crate::Readable for GrfGmacPerfWrAxiTotalByteSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gmac_perf_wr_axi_total_byte::W`](W) writer structure"]
impl crate::Writable for GrfGmacPerfWrAxiTotalByteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GMAC_PERF_WR_AXI_TOTAL_BYTE to value 0"]
impl crate::Resettable for GrfGmacPerfWrAxiTotalByteSpec {
    const RESET_VALUE: u32 = 0;
}
