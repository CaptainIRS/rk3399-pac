#[doc = "Register `GMAC_MMC_RXUDPERRFRM` reader"]
pub type R = crate::R<GmacMmcRxudperrfrmSpec>;
#[doc = "Register `GMAC_MMC_RXUDPERRFRM` writer"]
pub type W = crate::W<GmacMmcRxudperrfrmSpec>;
#[doc = "Field `RXUDP_ERR_FRMS` reader - Number of good IP datagrams whose UDP payload has a\n\nchecksum error."]
pub type RxudpErrFrmsR = crate::FieldReader<u32>;
#[doc = "Field `RXUDP_ERR_FRMS` writer - Number of good IP datagrams whose UDP payload has a\n\nchecksum error."]
pub type RxudpErrFrmsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of good IP datagrams whose UDP payload has a\n\nchecksum error."]
    #[inline(always)]
    pub fn rxudp_err_frms(&self) -> RxudpErrFrmsR {
        RxudpErrFrmsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of good IP datagrams whose UDP payload has a\n\nchecksum error."]
    #[inline(always)]
    #[must_use]
    pub fn rxudp_err_frms(&mut self) -> RxudpErrFrmsW<GmacMmcRxudperrfrmSpec> {
        RxudpErrFrmsW::new(self, 0)
    }
}
#[doc = "MMC RX UDP Error Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxudperrfrm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxudperrfrm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacMmcRxudperrfrmSpec;
impl crate::RegisterSpec for GmacMmcRxudperrfrmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_mmc_rxudperrfrm::R`](R) reader structure"]
impl crate::Readable for GmacMmcRxudperrfrmSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_mmc_rxudperrfrm::W`](W) writer structure"]
impl crate::Writable for GmacMmcRxudperrfrmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_MMC_RXUDPERRFRM to value 0"]
impl crate::Resettable for GmacMmcRxudperrfrmSpec {
    const RESET_VALUE: u32 = 0;
}
