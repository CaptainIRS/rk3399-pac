#[doc = "Register `GMAC_MMC_RXICMPERRFRM` reader"]
pub type R = crate::R<GmacMmcRxicmperrfrmSpec>;
#[doc = "Register `GMAC_MMC_RXICMPERRFRM` writer"]
pub type W = crate::W<GmacMmcRxicmperrfrmSpec>;
#[doc = "Field `RXICMP_ERR_FRMS` reader - Number of good IP datagrams whose ICMP payload has a\n\nchecksum error."]
pub type RxicmpErrFrmsR = crate::FieldReader<u32>;
#[doc = "Field `RXICMP_ERR_FRMS` writer - Number of good IP datagrams whose ICMP payload has a\n\nchecksum error."]
pub type RxicmpErrFrmsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of good IP datagrams whose ICMP payload has a\n\nchecksum error."]
    #[inline(always)]
    pub fn rxicmp_err_frms(&self) -> RxicmpErrFrmsR {
        RxicmpErrFrmsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of good IP datagrams whose ICMP payload has a\n\nchecksum error."]
    #[inline(always)]
    #[must_use]
    pub fn rxicmp_err_frms(&mut self) -> RxicmpErrFrmsW<GmacMmcRxicmperrfrmSpec> {
        RxicmpErrFrmsW::new(self, 0)
    }
}
#[doc = "MMC RX ICMP Error Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxicmperrfrm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxicmperrfrm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacMmcRxicmperrfrmSpec;
impl crate::RegisterSpec for GmacMmcRxicmperrfrmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_mmc_rxicmperrfrm::R`](R) reader structure"]
impl crate::Readable for GmacMmcRxicmperrfrmSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_mmc_rxicmperrfrm::W`](W) writer structure"]
impl crate::Writable for GmacMmcRxicmperrfrmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_MMC_RXICMPERRFRM to value 0"]
impl crate::Resettable for GmacMmcRxicmperrfrmSpec {
    const RESET_VALUE: u32 = 0;
}
