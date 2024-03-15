#[doc = "Register `GRF_A53_PERF_WR_MON_ST` reader"]
pub type R = crate::R<GrfA53PerfWrMonStSpec>;
#[doc = "Register `GRF_A53_PERF_WR_MON_ST` writer"]
pub type W = crate::W<GrfA53PerfWrMonStSpec>;
#[doc = "Field `WR_START_ADDR` reader - monitor write start address"]
pub type WrStartAddrR = crate::FieldReader<u32>;
#[doc = "Field `WR_START_ADDR` writer - monitor write start address"]
pub type WrStartAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - monitor write start address"]
    #[inline(always)]
    pub fn wr_start_addr(&self) -> WrStartAddrR {
        WrStartAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - monitor write start address"]
    #[inline(always)]
    #[must_use]
    pub fn wr_start_addr(&mut self) -> WrStartAddrW<GrfA53PerfWrMonStSpec> {
        WrStartAddrW::new(self, 0)
    }
}
#[doc = "performance write monitor start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a53_perf_wr_mon_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a53_perf_wr_mon_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfA53PerfWrMonStSpec;
impl crate::RegisterSpec for GrfA53PerfWrMonStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_a53_perf_wr_mon_st::R`](R) reader structure"]
impl crate::Readable for GrfA53PerfWrMonStSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_a53_perf_wr_mon_st::W`](W) writer structure"]
impl crate::Writable for GrfA53PerfWrMonStSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_A53_PERF_WR_MON_ST to value 0"]
impl crate::Resettable for GrfA53PerfWrMonStSpec {
    const RESET_VALUE: u32 = 0;
}
