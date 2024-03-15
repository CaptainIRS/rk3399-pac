#[doc = "Register `DDRMON_CH0_RD_START_ADDR` reader"]
pub type R = crate::R<DdrmonCh0RdStartAddrSpec>;
#[doc = "Register `DDRMON_CH0_RD_START_ADDR` writer"]
pub type W = crate::W<DdrmonCh0RdStartAddrSpec>;
#[doc = "Field `CH0_RD_START_ADDR` reader - Channel 0 read start address for address comparison"]
pub type Ch0RdStartAddrR = crate::FieldReader<u32>;
#[doc = "Field `CH0_RD_START_ADDR` writer - Channel 0 read start address for address comparison"]
pub type Ch0RdStartAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel 0 read start address for address comparison"]
    #[inline(always)]
    pub fn ch0_rd_start_addr(&self) -> Ch0RdStartAddrR {
        Ch0RdStartAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel 0 read start address for address comparison"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_rd_start_addr(&mut self) -> Ch0RdStartAddrW<DdrmonCh0RdStartAddrSpec> {
        Ch0RdStartAddrW::new(self, 0)
    }
}
#[doc = "Channel 0 Read Start Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch0_rd_start_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrmon_ch0_rd_start_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrmonCh0RdStartAddrSpec;
impl crate::RegisterSpec for DdrmonCh0RdStartAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrmon_ch0_rd_start_addr::R`](R) reader structure"]
impl crate::Readable for DdrmonCh0RdStartAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`ddrmon_ch0_rd_start_addr::W`](W) writer structure"]
impl crate::Writable for DdrmonCh0RdStartAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRMON_CH0_RD_START_ADDR to value 0"]
impl crate::Resettable for DdrmonCh0RdStartAddrSpec {
    const RESET_VALUE: u32 = 0;
}
