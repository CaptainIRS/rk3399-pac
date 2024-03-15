#[doc = "Register `GRF_A72_PERF_WR_MON_END` reader"]
pub type R = crate::R<GrfA72PerfWrMonEndSpec>;
#[doc = "Register `GRF_A72_PERF_WR_MON_END` writer"]
pub type W = crate::W<GrfA72PerfWrMonEndSpec>;
#[doc = "Field `WR_END_ADDR` reader - monitor write end address"]
pub type WrEndAddrR = crate::FieldReader<u32>;
#[doc = "Field `WR_END_ADDR` writer - monitor write end address"]
pub type WrEndAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - monitor write end address"]
    #[inline(always)]
    pub fn wr_end_addr(&self) -> WrEndAddrR {
        WrEndAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - monitor write end address"]
    #[inline(always)]
    #[must_use]
    pub fn wr_end_addr(&mut self) -> WrEndAddrW<GrfA72PerfWrMonEndSpec> {
        WrEndAddrW::new(self, 0)
    }
}
#[doc = "performance monitor write end address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a72_perf_wr_mon_end::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a72_perf_wr_mon_end::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfA72PerfWrMonEndSpec;
impl crate::RegisterSpec for GrfA72PerfWrMonEndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_a72_perf_wr_mon_end::R`](R) reader structure"]
impl crate::Readable for GrfA72PerfWrMonEndSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_a72_perf_wr_mon_end::W`](W) writer structure"]
impl crate::Writable for GrfA72PerfWrMonEndSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_A72_PERF_WR_MON_END to value 0"]
impl crate::Resettable for GrfA72PerfWrMonEndSpec {
    const RESET_VALUE: u32 = 0;
}
