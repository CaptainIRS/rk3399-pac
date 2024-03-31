#[doc = "Register `A53_PERF_RD_MON_END` reader"]
pub type R = crate::R<A53PerfRdMonEndSpec>;
#[doc = "Register `A53_PERF_RD_MON_END` writer"]
pub type W = crate::W<A53PerfRdMonEndSpec>;
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
    pub fn rd_end_addr(&mut self) -> RdEndAddrW<A53PerfRdMonEndSpec> {
        RdEndAddrW::new(self, 0)
    }
}
#[doc = "performance monitor end address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a53_perf_rd_mon_end::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a53_perf_rd_mon_end::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A53PerfRdMonEndSpec;
impl crate::RegisterSpec for A53PerfRdMonEndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a53_perf_rd_mon_end::R`](R) reader structure"]
impl crate::Readable for A53PerfRdMonEndSpec {}
#[doc = "`write(|w| ..)` method takes [`a53_perf_rd_mon_end::W`](W) writer structure"]
impl crate::Writable for A53PerfRdMonEndSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets A53_PERF_RD_MON_END to value 0"]
impl crate::Resettable for A53PerfRdMonEndSpec {
    const RESET_VALUE: u32 = 0;
}
