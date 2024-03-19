#[doc = "Register `GMAC_MMC_RXIPV6HDERRFRM` reader"]
pub type R = crate::R<GmacMmcRxipv6hderrfrmSpec>;
#[doc = "Register `GMAC_MMC_RXIPV6HDERRFRM` writer"]
pub type W = crate::W<GmacMmcRxipv6hderrfrmSpec>;
#[doc = "Field `RXIPV6_HDRERR_FRMS` reader - Number of IPv6 datagrams received with header errors (length or\n\nversion mismatch)."]
pub type Rxipv6HdrerrFrmsR = crate::FieldReader<u32>;
#[doc = "Field `RXIPV6_HDRERR_FRMS` writer - Number of IPv6 datagrams received with header errors (length or\n\nversion mismatch)."]
pub type Rxipv6HdrerrFrmsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of IPv6 datagrams received with header errors (length or\n\nversion mismatch)."]
    #[inline(always)]
    pub fn rxipv6_hdrerr_frms(&self) -> Rxipv6HdrerrFrmsR {
        Rxipv6HdrerrFrmsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of IPv6 datagrams received with header errors (length or\n\nversion mismatch)."]
    #[inline(always)]
    #[must_use]
    pub fn rxipv6_hdrerr_frms(&mut self) -> Rxipv6HdrerrFrmsW<GmacMmcRxipv6hderrfrmSpec> {
        Rxipv6HdrerrFrmsW::new(self, 0)
    }
}
#[doc = "MMC RX IPV6 Head Error Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxipv6hderrfrm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxipv6hderrfrm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacMmcRxipv6hderrfrmSpec;
impl crate::RegisterSpec for GmacMmcRxipv6hderrfrmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_mmc_rxipv6hderrfrm::R`](R) reader structure"]
impl crate::Readable for GmacMmcRxipv6hderrfrmSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_mmc_rxipv6hderrfrm::W`](W) writer structure"]
impl crate::Writable for GmacMmcRxipv6hderrfrmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_MMC_RXIPV6HDERRFRM to value 0"]
impl crate::Resettable for GmacMmcRxipv6hderrfrmSpec {
    const RESET_VALUE: u32 = 0;
}
