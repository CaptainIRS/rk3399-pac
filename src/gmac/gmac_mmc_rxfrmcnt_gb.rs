#[doc = "Register `GMAC_MMC_RXFRMCNT_GB` reader"]
pub type R = crate::R<GmacMmcRxfrmcntGbSpec>;
#[doc = "Register `GMAC_MMC_RXFRMCNT_GB` writer"]
pub type W = crate::W<GmacMmcRxfrmcntGbSpec>;
#[doc = "Field `RXFRAMECOUNT_GB` reader - Number of good and bad frames received."]
pub type RxframecountGbR = crate::FieldReader<u32>;
#[doc = "Field `RXFRAMECOUNT_GB` writer - Number of good and bad frames received."]
pub type RxframecountGbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of good and bad frames received."]
    #[inline(always)]
    pub fn rxframecount_gb(&self) -> RxframecountGbR {
        RxframecountGbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of good and bad frames received."]
    #[inline(always)]
    #[must_use]
    pub fn rxframecount_gb(&mut self) -> RxframecountGbW<GmacMmcRxfrmcntGbSpec> {
        RxframecountGbW::new(self, 0)
    }
}
#[doc = "MMC RX Frame Good and Bad Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxfrmcnt_gb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxfrmcnt_gb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacMmcRxfrmcntGbSpec;
impl crate::RegisterSpec for GmacMmcRxfrmcntGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_mmc_rxfrmcnt_gb::R`](R) reader structure"]
impl crate::Readable for GmacMmcRxfrmcntGbSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_mmc_rxfrmcnt_gb::W`](W) writer structure"]
impl crate::Writable for GmacMmcRxfrmcntGbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_MMC_RXFRMCNT_GB to value 0"]
impl crate::Resettable for GmacMmcRxfrmcntGbSpec {
    const RESET_VALUE: u32 = 0;
}
