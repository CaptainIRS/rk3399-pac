#[doc = "Register `A72_PERF_RD_MON_ST` reader"]
pub type R = crate::R<A72PerfRdMonStSpec>;
#[doc = "Register `A72_PERF_RD_MON_ST` writer"]
pub type W = crate::W<A72PerfRdMonStSpec>;
#[doc = "Field `RD_START_ADDR` reader - monitor read start address"]
pub type RdStartAddrR = crate::FieldReader<u32>;
#[doc = "Field `RD_START_ADDR` writer - monitor read start address"]
pub type RdStartAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - monitor read start address"]
    #[inline(always)]
    pub fn rd_start_addr(&self) -> RdStartAddrR {
        RdStartAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - monitor read start address"]
    #[inline(always)]
    #[must_use]
    pub fn rd_start_addr(&mut self) -> RdStartAddrW<A72PerfRdMonStSpec> {
        RdStartAddrW::new(self, 0)
    }
}
#[doc = "performance monitor read start address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a72_perf_rd_mon_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a72_perf_rd_mon_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A72PerfRdMonStSpec;
impl crate::RegisterSpec for A72PerfRdMonStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a72_perf_rd_mon_st::R`](R) reader structure"]
impl crate::Readable for A72PerfRdMonStSpec {}
#[doc = "`write(|w| ..)` method takes [`a72_perf_rd_mon_st::W`](W) writer structure"]
impl crate::Writable for A72PerfRdMonStSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets A72_PERF_RD_MON_ST to value 0"]
impl crate::Resettable for A72PerfRdMonStSpec {
    const RESET_VALUE: u32 = 0;
}
