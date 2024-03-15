#[doc = "Register `GMAC_MMC_RXIPV6HDERROCT` reader"]
pub type R = crate::R<GmacMmcRxipv6hderroctSpec>;
#[doc = "Register `GMAC_MMC_RXIPV6HDERROCT` writer"]
pub type W = crate::W<GmacMmcRxipv6hderroctSpec>;
#[doc = "Field `RXIPV6_HDRERR_OCTETS` reader - Number of bytes received in IPv6 datagrams with header errors (length, version mismatch). The value in the IPv6 header's Length field is used to update this counter."]
pub type Rxipv6HdrerrOctetsR = crate::FieldReader<u32>;
#[doc = "Field `RXIPV6_HDRERR_OCTETS` writer - Number of bytes received in IPv6 datagrams with header errors (length, version mismatch). The value in the IPv6 header's Length field is used to update this counter."]
pub type Rxipv6HdrerrOctetsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes received in IPv6 datagrams with header errors (length, version mismatch). The value in the IPv6 header's Length field is used to update this counter."]
    #[inline(always)]
    pub fn rxipv6_hdrerr_octets(&self) -> Rxipv6HdrerrOctetsR {
        Rxipv6HdrerrOctetsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes received in IPv6 datagrams with header errors (length, version mismatch). The value in the IPv6 header's Length field is used to update this counter."]
    #[inline(always)]
    #[must_use]
    pub fn rxipv6_hdrerr_octets(&mut self) -> Rxipv6HdrerrOctetsW<GmacMmcRxipv6hderroctSpec> {
        Rxipv6HdrerrOctetsW::new(self, 0)
    }
}
#[doc = "MMC RX OCTET IPV6 Head Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxipv6hderroct::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxipv6hderroct::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacMmcRxipv6hderroctSpec;
impl crate::RegisterSpec for GmacMmcRxipv6hderroctSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_mmc_rxipv6hderroct::R`](R) reader structure"]
impl crate::Readable for GmacMmcRxipv6hderroctSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_mmc_rxipv6hderroct::W`](W) writer structure"]
impl crate::Writable for GmacMmcRxipv6hderroctSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_MMC_RXIPV6HDERROCT to value 0"]
impl crate::Resettable for GmacMmcRxipv6hderroctSpec {
    const RESET_VALUE: u32 = 0;
}
