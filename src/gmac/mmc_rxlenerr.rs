#[doc = "Register `MMC_RXLENERR` reader"]
pub type R = crate::R<MmcRxlenerrSpec>;
#[doc = "Register `MMC_RXLENERR` writer"]
pub type W = crate::W<MmcRxlenerrSpec>;
#[doc = "Field `RXLENGTHERROR` reader - Number of frames received with length error (Length type field\n\n≠frame size), for all frames with valid length field."]
pub type RxlengtherrorR = crate::FieldReader<u32>;
#[doc = "Field `RXLENGTHERROR` writer - Number of frames received with length error (Length type field\n\n≠frame size), for all frames with valid length field."]
pub type RxlengtherrorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames received with length error (Length type field\n\n≠frame size), for all frames with valid length field."]
    #[inline(always)]
    pub fn rxlengtherror(&self) -> RxlengtherrorR {
        RxlengtherrorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of frames received with length error (Length type field\n\n≠frame size), for all frames with valid length field."]
    #[inline(always)]
    #[must_use]
    pub fn rxlengtherror(&mut self) -> RxlengtherrorW<MmcRxlenerrSpec> {
        RxlengtherrorW::new(self, 0)
    }
}
#[doc = "MMC RX Length Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxlenerr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxlenerr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcRxlenerrSpec;
impl crate::RegisterSpec for MmcRxlenerrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_rxlenerr::R`](R) reader structure"]
impl crate::Readable for MmcRxlenerrSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_rxlenerr::W`](W) writer structure"]
impl crate::Writable for MmcRxlenerrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_RXLENERR to value 0"]
impl crate::Resettable for MmcRxlenerrSpec {
    const RESET_VALUE: u32 = 0;
}
