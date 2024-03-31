#[doc = "Register `MMC_TXFRMCNT_GB` reader"]
pub type R = crate::R<MmcTxfrmcntGbSpec>;
#[doc = "Register `MMC_TXFRMCNT_GB` writer"]
pub type W = crate::W<MmcTxfrmcntGbSpec>;
#[doc = "Field `TXFRAMECOUNT_GB` reader - Number of good and bad frames transmitted, exclusive of retried\n\nframes."]
pub type TxframecountGbR = crate::FieldReader<u32>;
#[doc = "Field `TXFRAMECOUNT_GB` writer - Number of good and bad frames transmitted, exclusive of retried\n\nframes."]
pub type TxframecountGbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of good and bad frames transmitted, exclusive of retried\n\nframes."]
    #[inline(always)]
    pub fn txframecount_gb(&self) -> TxframecountGbR {
        TxframecountGbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of good and bad frames transmitted, exclusive of retried\n\nframes."]
    #[inline(always)]
    #[must_use]
    pub fn txframecount_gb(&mut self) -> TxframecountGbW<MmcTxfrmcntGbSpec> {
        TxframecountGbW::new(self, 0)
    }
}
#[doc = "MMC TX Frame Good and Bad Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_txfrmcnt_gb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_txfrmcnt_gb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcTxfrmcntGbSpec;
impl crate::RegisterSpec for MmcTxfrmcntGbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_txfrmcnt_gb::R`](R) reader structure"]
impl crate::Readable for MmcTxfrmcntGbSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_txfrmcnt_gb::W`](W) writer structure"]
impl crate::Writable for MmcTxfrmcntGbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_TXFRMCNT_GB to value 0"]
impl crate::Resettable for MmcTxfrmcntGbSpec {
    const RESET_VALUE: u32 = 0;
}
