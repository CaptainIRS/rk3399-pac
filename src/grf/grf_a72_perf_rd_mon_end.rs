#[doc = "Register `GRF_A72_PERF_RD_MON_END` reader"]
pub type R = crate::R<GrfA72PerfRdMonEndSpec>;
#[doc = "Register `GRF_A72_PERF_RD_MON_END` writer"]
pub type W = crate::W<GrfA72PerfRdMonEndSpec>;
#[doc = "Field `RD_END_ADDR` reader - monitor read end address"]
pub type RdEndAddrR = crate::FieldReader<u32>;
#[doc = "Field `RD_END_ADDR` writer - monitor read end address"]
pub type RdEndAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - monitor read end address"]
    #[inline(always)]
    pub fn rd_end_addr(&self) -> RdEndAddrR {
        RdEndAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - monitor read end address"]
    #[inline(always)]
    #[must_use]
    pub fn rd_end_addr(&mut self) -> RdEndAddrW<GrfA72PerfRdMonEndSpec> {
        RdEndAddrW::new(self, 0)
    }
}
#[doc = "performance monitor end address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_a72_perf_rd_mon_end::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_a72_perf_rd_mon_end::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfA72PerfRdMonEndSpec;
impl crate::RegisterSpec for GrfA72PerfRdMonEndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_a72_perf_rd_mon_end::R`](R) reader structure"]
impl crate::Readable for GrfA72PerfRdMonEndSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_a72_perf_rd_mon_end::W`](W) writer structure"]
impl crate::Writable for GrfA72PerfRdMonEndSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_A72_PERF_RD_MON_END to value 0"]
impl crate::Resettable for GrfA72PerfRdMonEndSpec {
    const RESET_VALUE: u32 = 0;
}
