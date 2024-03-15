#[doc = "Register `GMAC_MMC_RXOCTETCNT_G` reader"]
pub type R = crate::R<GmacMmcRxoctetcntGSpec>;
#[doc = "Register `GMAC_MMC_RXOCTETCNT_G` writer"]
pub type W = crate::W<GmacMmcRxoctetcntGSpec>;
#[doc = "Field `RXOCTETCOUNT_G` reader - Number of bytes received, exclusive of preamble, only in good frames."]
pub type RxoctetcountGR = crate::FieldReader<u32>;
#[doc = "Field `RXOCTETCOUNT_G` writer - Number of bytes received, exclusive of preamble, only in good frames."]
pub type RxoctetcountGW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes received, exclusive of preamble, only in good frames."]
    #[inline(always)]
    pub fn rxoctetcount_g(&self) -> RxoctetcountGR {
        RxoctetcountGR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes received, exclusive of preamble, only in good frames."]
    #[inline(always)]
    #[must_use]
    pub fn rxoctetcount_g(&mut self) -> RxoctetcountGW<GmacMmcRxoctetcntGSpec> {
        RxoctetcountGW::new(self, 0)
    }
}
#[doc = "MMC RX OCTET Good Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxoctetcnt_g::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxoctetcnt_g::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacMmcRxoctetcntGSpec;
impl crate::RegisterSpec for GmacMmcRxoctetcntGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_mmc_rxoctetcnt_g::R`](R) reader structure"]
impl crate::Readable for GmacMmcRxoctetcntGSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_mmc_rxoctetcnt_g::W`](W) writer structure"]
impl crate::Writable for GmacMmcRxoctetcntGSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_MMC_RXOCTETCNT_G to value 0"]
impl crate::Resettable for GmacMmcRxoctetcntGSpec {
    const RESET_VALUE: u32 = 0;
}
