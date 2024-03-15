#[doc = "Register `GMAC_MMC_TXFRMCNT_G` reader"]
pub type R = crate::R<GmacMmcTxfrmcntGSpec>;
#[doc = "Register `GMAC_MMC_TXFRMCNT_G` writer"]
pub type W = crate::W<GmacMmcTxfrmcntGSpec>;
#[doc = "Field `TXFRAMECOUNT_G` reader - Number of good frames transmitted."]
pub type TxframecountGR = crate::FieldReader<u32>;
#[doc = "Field `TXFRAMECOUNT_G` writer - Number of good frames transmitted."]
pub type TxframecountGW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of good frames transmitted."]
    #[inline(always)]
    pub fn txframecount_g(&self) -> TxframecountGR {
        TxframecountGR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of good frames transmitted."]
    #[inline(always)]
    #[must_use]
    pub fn txframecount_g(&mut self) -> TxframecountGW<GmacMmcTxfrmcntGSpec> {
        TxframecountGW::new(self, 0)
    }
}
#[doc = "MMC TX Frame Good Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_txfrmcnt_g::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_txfrmcnt_g::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacMmcTxfrmcntGSpec;
impl crate::RegisterSpec for GmacMmcTxfrmcntGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_mmc_txfrmcnt_g::R`](R) reader structure"]
impl crate::Readable for GmacMmcTxfrmcntGSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_mmc_txfrmcnt_g::W`](W) writer structure"]
impl crate::Writable for GmacMmcTxfrmcntGSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_MMC_TXFRMCNT_G to value 0"]
impl crate::Resettable for GmacMmcTxfrmcntGSpec {
    const RESET_VALUE: u32 = 0;
}
