#[doc = "Register `GMAC_MMC_TXOCTETCNT_G` reader"]
pub type R = crate::R<GmacMmcTxoctetcntGSpec>;
#[doc = "Register `GMAC_MMC_TXOCTETCNT_G` writer"]
pub type W = crate::W<GmacMmcTxoctetcntGSpec>;
#[doc = "Field `TXOCTETCOUNT_G` reader - Number of bytes transmitted, exclusive of preamble, in good frames only."]
pub type TxoctetcountGR = crate::FieldReader<u32>;
#[doc = "Field `TXOCTETCOUNT_G` writer - Number of bytes transmitted, exclusive of preamble, in good frames only."]
pub type TxoctetcountGW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes transmitted, exclusive of preamble, in good frames only."]
    #[inline(always)]
    pub fn txoctetcount_g(&self) -> TxoctetcountGR {
        TxoctetcountGR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes transmitted, exclusive of preamble, in good frames only."]
    #[inline(always)]
    #[must_use]
    pub fn txoctetcount_g(&mut self) -> TxoctetcountGW<GmacMmcTxoctetcntGSpec> {
        TxoctetcountGW::new(self, 0)
    }
}
#[doc = "MMC TX OCTET Good Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_txoctetcnt_g::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_txoctetcnt_g::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacMmcTxoctetcntGSpec;
impl crate::RegisterSpec for GmacMmcTxoctetcntGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_mmc_txoctetcnt_g::R`](R) reader structure"]
impl crate::Readable for GmacMmcTxoctetcntGSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_mmc_txoctetcnt_g::W`](W) writer structure"]
impl crate::Writable for GmacMmcTxoctetcntGSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_MMC_TXOCTETCNT_G to value 0"]
impl crate::Resettable for GmacMmcTxoctetcntGSpec {
    const RESET_VALUE: u32 = 0;
}
