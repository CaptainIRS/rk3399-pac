#[doc = "Register `GMAC_MMC_TXOCTETCNT_GB` reader"]
pub type R = crate::R<GmacMmcTxoctetcntGbSpec>;
#[doc = "Register `GMAC_MMC_TXOCTETCNT_GB` writer"]
pub type W = crate::W<GmacMmcTxoctetcntGbSpec>;
#[doc = "Field `TXOCTETCOUNT_GB` reader - Number of bytes transmitted, exclusive of preamble and retried bytes, in good and bad frames."]
pub type TxoctetcountGbR = crate::FieldReader<u32>;
#[doc = "Field `TXOCTETCOUNT_GB` writer - Number of bytes transmitted, exclusive of preamble and retried bytes, in good and bad frames."]
pub type TxoctetcountGbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes transmitted, exclusive of preamble and retried bytes, in good and bad frames."]
    #[inline(always)]
    pub fn txoctetcount_gb(&self) -> TxoctetcountGbR {
        TxoctetcountGbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes transmitted, exclusive of preamble and retried bytes, in good and bad frames."]
    #[inline(always)]
    #[must_use]
    pub fn txoctetcount_gb(&mut self) -> TxoctetcountGbW<GmacMmcTxoctetcntGbSpec> {
        TxoctetcountGbW::new(self, 0)
    }
}
#[doc = "MMC TX OCTET Good and Bad Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_txoctetcnt_gb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_txoctetcnt_gb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacMmcTxoctetcntGbSpec;
impl crate::RegisterSpec for GmacMmcTxoctetcntGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_mmc_txoctetcnt_gb::R`](R) reader structure"]
impl crate::Readable for GmacMmcTxoctetcntGbSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_mmc_txoctetcnt_gb::W`](W) writer structure"]
impl crate::Writable for GmacMmcTxoctetcntGbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_MMC_TXOCTETCNT_GB to value 0"]
impl crate::Resettable for GmacMmcTxoctetcntGbSpec {
    const RESET_VALUE: u32 = 0;
}
