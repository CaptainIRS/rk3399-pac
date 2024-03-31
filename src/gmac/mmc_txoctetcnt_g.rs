#[doc = "Register `MMC_TXOCTETCNT_G` reader"]
pub type R = crate::R<MmcTxoctetcntGSpec>;
#[doc = "Register `MMC_TXOCTETCNT_G` writer"]
pub type W = crate::W<MmcTxoctetcntGSpec>;
#[doc = "Field `TXOCTETCOUNT_G` reader - Number of bytes transmitted, exclusive of preamble, in good\n\nframes only."]
pub type TxoctetcountGR = crate::FieldReader<u32>;
#[doc = "Field `TXOCTETCOUNT_G` writer - Number of bytes transmitted, exclusive of preamble, in good\n\nframes only."]
pub type TxoctetcountGW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes transmitted, exclusive of preamble, in good\n\nframes only."]
    #[inline(always)]
    pub fn txoctetcount_g(&self) -> TxoctetcountGR {
        TxoctetcountGR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes transmitted, exclusive of preamble, in good\n\nframes only."]
    #[inline(always)]
    #[must_use]
    pub fn txoctetcount_g(&mut self) -> TxoctetcountGW<MmcTxoctetcntGSpec> {
        TxoctetcountGW::new(self, 0)
    }
}
#[doc = "MMC TX OCTET Good Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_txoctetcnt_g::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_txoctetcnt_g::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcTxoctetcntGSpec;
impl crate::RegisterSpec for MmcTxoctetcntGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_txoctetcnt_g::R`](R) reader structure"]
impl crate::Readable for MmcTxoctetcntGSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_txoctetcnt_g::W`](W) writer structure"]
impl crate::Writable for MmcTxoctetcntGSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_TXOCTETCNT_G to value 0"]
impl crate::Resettable for MmcTxoctetcntGSpec {
    const RESET_VALUE: u32 = 0;
}
