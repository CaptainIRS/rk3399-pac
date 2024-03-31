#[doc = "Register `MMC_TXUNDFLWERR` reader"]
pub type R = crate::R<MmcTxundflwerrSpec>;
#[doc = "Register `MMC_TXUNDFLWERR` writer"]
pub type W = crate::W<MmcTxundflwerrSpec>;
#[doc = "Field `TXUNDERFLOWERROR` reader - Number of frames aborted due to frame underflow error."]
pub type TxunderflowerrorR = crate::FieldReader<u32>;
#[doc = "Field `TXUNDERFLOWERROR` writer - Number of frames aborted due to frame underflow error."]
pub type TxunderflowerrorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames aborted due to frame underflow error."]
    #[inline(always)]
    pub fn txunderflowerror(&self) -> TxunderflowerrorR {
        TxunderflowerrorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of frames aborted due to frame underflow error."]
    #[inline(always)]
    #[must_use]
    pub fn txunderflowerror(&mut self) -> TxunderflowerrorW<MmcTxundflwerrSpec> {
        TxunderflowerrorW::new(self, 0)
    }
}
#[doc = "MMC TX Underflow Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_txundflwerr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_txundflwerr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcTxundflwerrSpec;
impl crate::RegisterSpec for MmcTxundflwerrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_txundflwerr::R`](R) reader structure"]
impl crate::Readable for MmcTxundflwerrSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_txundflwerr::W`](W) writer structure"]
impl crate::Writable for MmcTxundflwerrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_TXUNDFLWERR to value 0"]
impl crate::Resettable for MmcTxundflwerrSpec {
    const RESET_VALUE: u32 = 0;
}
