#[doc = "Register `GMAC_MMC_RXLENERR` reader"]
pub type R = crate::R<GmacMmcRxlenerrSpec>;
#[doc = "Register `GMAC_MMC_RXLENERR` writer"]
pub type W = crate::W<GmacMmcRxlenerrSpec>;
#[doc = "Field `RXLENGTHERROR` reader - Number of frames received with length error (Length type field ≠frame size), for all frames with valid length field."]
pub type RxlengtherrorR = crate::FieldReader<u32>;
#[doc = "Field `RXLENGTHERROR` writer - Number of frames received with length error (Length type field ≠frame size), for all frames with valid length field."]
pub type RxlengtherrorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of frames received with length error (Length type field ≠frame size), for all frames with valid length field."]
    #[inline(always)]
    pub fn rxlengtherror(&self) -> RxlengtherrorR {
        RxlengtherrorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of frames received with length error (Length type field ≠frame size), for all frames with valid length field."]
    #[inline(always)]
    #[must_use]
    pub fn rxlengtherror(&mut self) -> RxlengtherrorW<GmacMmcRxlenerrSpec> {
        RxlengtherrorW::new(self, 0)
    }
}
#[doc = "MMC RX Length Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxlenerr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxlenerr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacMmcRxlenerrSpec;
impl crate::RegisterSpec for GmacMmcRxlenerrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_mmc_rxlenerr::R`](R) reader structure"]
impl crate::Readable for GmacMmcRxlenerrSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_mmc_rxlenerr::W`](W) writer structure"]
impl crate::Writable for GmacMmcRxlenerrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_MMC_RXLENERR to value 0"]
impl crate::Resettable for GmacMmcRxlenerrSpec {
    const RESET_VALUE: u32 = 0;
}
