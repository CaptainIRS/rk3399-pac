#[doc = "Register `MMC_RXOCTETCNT_G` reader"]
pub type R = crate::R<MmcRxoctetcntGSpec>;
#[doc = "Register `MMC_RXOCTETCNT_G` writer"]
pub type W = crate::W<MmcRxoctetcntGSpec>;
#[doc = "Field `RXOCTETCOUNT_G` reader - Number of bytes received, exclusive of preamble, only in good\n\nframes."]
pub type RxoctetcountGR = crate::FieldReader<u32>;
#[doc = "Field `RXOCTETCOUNT_G` writer - Number of bytes received, exclusive of preamble, only in good\n\nframes."]
pub type RxoctetcountGW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes received, exclusive of preamble, only in good\n\nframes."]
    #[inline(always)]
    pub fn rxoctetcount_g(&self) -> RxoctetcountGR {
        RxoctetcountGR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes received, exclusive of preamble, only in good\n\nframes."]
    #[inline(always)]
    #[must_use]
    pub fn rxoctetcount_g(&mut self) -> RxoctetcountGW<MmcRxoctetcntGSpec> {
        RxoctetcountGW::new(self, 0)
    }
}
#[doc = "MMC RX OCTET Good Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxoctetcnt_g::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxoctetcnt_g::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcRxoctetcntGSpec;
impl crate::RegisterSpec for MmcRxoctetcntGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_rxoctetcnt_g::R`](R) reader structure"]
impl crate::Readable for MmcRxoctetcntGSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_rxoctetcnt_g::W`](W) writer structure"]
impl crate::Writable for MmcRxoctetcntGSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_RXOCTETCNT_G to value 0"]
impl crate::Resettable for MmcRxoctetcntGSpec {
    const RESET_VALUE: u32 = 0;
}
