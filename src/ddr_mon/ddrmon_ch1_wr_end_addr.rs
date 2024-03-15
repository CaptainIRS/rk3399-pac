#[doc = "Register `DDRMON_CH1_WR_END_ADDR` reader"]
pub type R = crate::R<DdrmonCh1WrEndAddrSpec>;
#[doc = "Register `DDRMON_CH1_WR_END_ADDR` writer"]
pub type W = crate::W<DdrmonCh1WrEndAddrSpec>;
#[doc = "Field `CH1_WR_END_ADDR` reader - Channel 1 write end address for address comparison"]
pub type Ch1WrEndAddrR = crate::FieldReader<u32>;
#[doc = "Field `CH1_WR_END_ADDR` writer - Channel 1 write end address for address comparison"]
pub type Ch1WrEndAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel 1 write end address for address comparison"]
    #[inline(always)]
    pub fn ch1_wr_end_addr(&self) -> Ch1WrEndAddrR {
        Ch1WrEndAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel 1 write end address for address comparison"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_wr_end_addr(&mut self) -> Ch1WrEndAddrW<DdrmonCh1WrEndAddrSpec> {
        Ch1WrEndAddrW::new(self, 0)
    }
}
#[doc = "Channel 1 Write End Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch1_wr_end_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrmon_ch1_wr_end_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrmonCh1WrEndAddrSpec;
impl crate::RegisterSpec for DdrmonCh1WrEndAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrmon_ch1_wr_end_addr::R`](R) reader structure"]
impl crate::Readable for DdrmonCh1WrEndAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`ddrmon_ch1_wr_end_addr::W`](W) writer structure"]
impl crate::Writable for DdrmonCh1WrEndAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRMON_CH1_WR_END_ADDR to value 0"]
impl crate::Resettable for DdrmonCh1WrEndAddrSpec {
    const RESET_VALUE: u32 = 0;
}
