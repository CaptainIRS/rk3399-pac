#[doc = "Register `MMC_TXCARERR` reader"]
pub type R = crate::R<MmcTxcarerrSpec>;
#[doc = "Register `MMC_TXCARERR` writer"]
pub type W = crate::W<MmcTxcarerrSpec>;
#[doc = "Field `TXCARRIERERROR` reader - Number of frames aborted due to carrier sense error (no carrier\n\nor loss of carrier)."]
pub type TxcarriererrorR = crate::FieldReader<u32>;
#[doc = "Field `TXCARRIERERROR` writer - Number of frames aborted due to carrier sense error (no carrier\n\nor loss of carrier)."]
pub type TxcarriererrorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames aborted due to carrier sense error (no carrier\n\nor loss of carrier)."]
    #[inline(always)]
    pub fn txcarriererror(&self) -> TxcarriererrorR {
        TxcarriererrorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of frames aborted due to carrier sense error (no carrier\n\nor loss of carrier)."]
    #[inline(always)]
    #[must_use]
    pub fn txcarriererror(&mut self) -> TxcarriererrorW<MmcTxcarerrSpec> {
        TxcarriererrorW::new(self, 0)
    }
}
#[doc = "MMC TX Carrier Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_txcarerr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_txcarerr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcTxcarerrSpec;
impl crate::RegisterSpec for MmcTxcarerrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_txcarerr::R`](R) reader structure"]
impl crate::Readable for MmcTxcarerrSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_txcarerr::W`](W) writer structure"]
impl crate::Writable for MmcTxcarerrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_TXCARERR to value 0"]
impl crate::Resettable for MmcTxcarerrSpec {
    const RESET_VALUE: u32 = 0;
}
