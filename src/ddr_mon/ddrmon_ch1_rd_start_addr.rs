#[doc = "Register `DDRMON_CH1_RD_START_ADDR` reader"]
pub type R = crate::R<DdrmonCh1RdStartAddrSpec>;
#[doc = "Register `DDRMON_CH1_RD_START_ADDR` writer"]
pub type W = crate::W<DdrmonCh1RdStartAddrSpec>;
#[doc = "Field `CH1_RD_START_ADDR` reader - Channel 1 read start address for address comparison"]
pub type Ch1RdStartAddrR = crate::FieldReader<u32>;
#[doc = "Field `CH1_RD_START_ADDR` writer - Channel 1 read start address for address comparison"]
pub type Ch1RdStartAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel 1 read start address for address comparison"]
    #[inline(always)]
    pub fn ch1_rd_start_addr(&self) -> Ch1RdStartAddrR {
        Ch1RdStartAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel 1 read start address for address comparison"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_rd_start_addr(&mut self) -> Ch1RdStartAddrW<DdrmonCh1RdStartAddrSpec> {
        Ch1RdStartAddrW::new(self, 0)
    }
}
#[doc = "Channel 1 Read Start Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrmon_ch1_rd_start_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrmon_ch1_rd_start_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrmonCh1RdStartAddrSpec;
impl crate::RegisterSpec for DdrmonCh1RdStartAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrmon_ch1_rd_start_addr::R`](R) reader structure"]
impl crate::Readable for DdrmonCh1RdStartAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`ddrmon_ch1_rd_start_addr::W`](W) writer structure"]
impl crate::Writable for DdrmonCh1RdStartAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRMON_CH1_RD_START_ADDR to value 0"]
impl crate::Resettable for DdrmonCh1RdStartAddrSpec {
    const RESET_VALUE: u32 = 0;
}
