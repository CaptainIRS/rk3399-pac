#[doc = "Register `GMAC_MMC_TXFRMCNT_GB` reader"]
pub type R = crate::R<GmacMmcTxfrmcntGbSpec>;
#[doc = "Register `GMAC_MMC_TXFRMCNT_GB` writer"]
pub type W = crate::W<GmacMmcTxfrmcntGbSpec>;
#[doc = "Field `TXFRAMECOUNT_GB` reader - Number of good and bad frames transmitted, exclusive of retried frames."]
pub type TxframecountGbR = crate::FieldReader<u32>;
#[doc = "Field `TXFRAMECOUNT_GB` writer - Number of good and bad frames transmitted, exclusive of retried frames."]
pub type TxframecountGbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of good and bad frames transmitted, exclusive of retried frames."]
    #[inline(always)]
    pub fn txframecount_gb(&self) -> TxframecountGbR {
        TxframecountGbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of good and bad frames transmitted, exclusive of retried frames."]
    #[inline(always)]
    #[must_use]
    pub fn txframecount_gb(&mut self) -> TxframecountGbW<GmacMmcTxfrmcntGbSpec> {
        TxframecountGbW::new(self, 0)
    }
}
#[doc = "MMC TX Frame Good and Bad Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_txfrmcnt_gb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_txfrmcnt_gb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacMmcTxfrmcntGbSpec;
impl crate::RegisterSpec for GmacMmcTxfrmcntGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_mmc_txfrmcnt_gb::R`](R) reader structure"]
impl crate::Readable for GmacMmcTxfrmcntGbSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_mmc_txfrmcnt_gb::W`](W) writer structure"]
impl crate::Writable for GmacMmcTxfrmcntGbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_MMC_TXFRMCNT_GB to value 0"]
impl crate::Resettable for GmacMmcTxfrmcntGbSpec {
    const RESET_VALUE: u32 = 0;
}
