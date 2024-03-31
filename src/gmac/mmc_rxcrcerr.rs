#[doc = "Register `MMC_RXCRCERR` reader"]
pub type R = crate::R<MmcRxcrcerrSpec>;
#[doc = "Register `MMC_RXCRCERR` writer"]
pub type W = crate::W<MmcRxcrcerrSpec>;
#[doc = "Field `RXCRCERROR` reader - Number of frames received with CRC error."]
pub type RxcrcerrorR = crate::FieldReader<u32>;
#[doc = "Field `RXCRCERROR` writer - Number of frames received with CRC error."]
pub type RxcrcerrorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames received with CRC error."]
    #[inline(always)]
    pub fn rxcrcerror(&self) -> RxcrcerrorR {
        RxcrcerrorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of frames received with CRC error."]
    #[inline(always)]
    #[must_use]
    pub fn rxcrcerror(&mut self) -> RxcrcerrorW<MmcRxcrcerrSpec> {
        RxcrcerrorW::new(self, 0)
    }
}
#[doc = "MMC RX Carrier\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxcrcerr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxcrcerr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcRxcrcerrSpec;
impl crate::RegisterSpec for MmcRxcrcerrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_rxcrcerr::R`](R) reader structure"]
impl crate::Readable for MmcRxcrcerrSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_rxcrcerr::W`](W) writer structure"]
impl crate::Writable for MmcRxcrcerrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_RXCRCERR to value 0"]
impl crate::Resettable for MmcRxcrcerrSpec {
    const RESET_VALUE: u32 = 0;
}
