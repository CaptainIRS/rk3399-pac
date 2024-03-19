#[doc = "Register `GMAC_MMC_RXOCTETCNT_GB` reader"]
pub type R = crate::R<GmacMmcRxoctetcntGbSpec>;
#[doc = "Register `GMAC_MMC_RXOCTETCNT_GB` writer"]
pub type W = crate::W<GmacMmcRxoctetcntGbSpec>;
#[doc = "Field `RXOCTETCOUNT_GB` reader - Number of bytes received, exclusive of preamble, in good and\n\nbad frames."]
pub type RxoctetcountGbR = crate::FieldReader<u32>;
#[doc = "Field `RXOCTETCOUNT_GB` writer - Number of bytes received, exclusive of preamble, in good and\n\nbad frames."]
pub type RxoctetcountGbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes received, exclusive of preamble, in good and\n\nbad frames."]
    #[inline(always)]
    pub fn rxoctetcount_gb(&self) -> RxoctetcountGbR {
        RxoctetcountGbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes received, exclusive of preamble, in good and\n\nbad frames."]
    #[inline(always)]
    #[must_use]
    pub fn rxoctetcount_gb(&mut self) -> RxoctetcountGbW<GmacMmcRxoctetcntGbSpec> {
        RxoctetcountGbW::new(self, 0)
    }
}
#[doc = "MMC RX OCTET Good and Bad Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxoctetcnt_gb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxoctetcnt_gb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacMmcRxoctetcntGbSpec;
impl crate::RegisterSpec for GmacMmcRxoctetcntGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_mmc_rxoctetcnt_gb::R`](R) reader structure"]
impl crate::Readable for GmacMmcRxoctetcntGbSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_mmc_rxoctetcnt_gb::W`](W) writer structure"]
impl crate::Writable for GmacMmcRxoctetcntGbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_MMC_RXOCTETCNT_GB to value 0"]
impl crate::Resettable for GmacMmcRxoctetcntGbSpec {
    const RESET_VALUE: u32 = 0;
}
