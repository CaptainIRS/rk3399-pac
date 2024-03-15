#[doc = "Register `DDRMON_CH0_RD_END_ADDR` reader"]
pub type R = crate::R<DdrmonCh0RdEndAddrSpec>;
#[doc = "Register `DDRMON_CH0_RD_END_ADDR` writer"]
pub type W = crate::W<DdrmonCh0RdEndAddrSpec>;
#[doc = "Field `CH0_RD_END_ADDR` reader - Channel 0 read end address for address comparison"]
pub type Ch0RdEndAddrR = crate::FieldReader<u32>;
#[doc = "Field `CH0_RD_END_ADDR` writer - Channel 0 read end address for address comparison"]
pub type Ch0RdEndAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel 0 read end address for address comparison"]
    #[inline(always)]
    pub fn ch0_rd_end_addr(&self) -> Ch0RdEndAddrR {
        Ch0RdEndAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel 0 read end address for address comparison"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_rd_end_addr(&mut self) -> Ch0RdEndAddrW<DdrmonCh0RdEndAddrSpec> {
        Ch0RdEndAddrW::new(self, 0)
    }
}
#[doc = "Channel 0 Read End Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch0_rd_end_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrmon_ch0_rd_end_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrmonCh0RdEndAddrSpec;
impl crate::RegisterSpec for DdrmonCh0RdEndAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrmon_ch0_rd_end_addr::R`](R) reader structure"]
impl crate::Readable for DdrmonCh0RdEndAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`ddrmon_ch0_rd_end_addr::W`](W) writer structure"]
impl crate::Writable for DdrmonCh0RdEndAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRMON_CH0_RD_END_ADDR to value 0"]
impl crate::Resettable for DdrmonCh0RdEndAddrSpec {
    const RESET_VALUE: u32 = 0;
}
