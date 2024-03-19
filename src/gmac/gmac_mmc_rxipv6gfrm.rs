#[doc = "Register `GMAC_MMC_RXIPV6GFRM` reader"]
pub type R = crate::R<GmacMmcRxipv6gfrmSpec>;
#[doc = "Register `GMAC_MMC_RXIPV6GFRM` writer"]
pub type W = crate::W<GmacMmcRxipv6gfrmSpec>;
#[doc = "Field `RXIPV6_GD_FRMS` reader - Number of good IPv6 datagrams received with TCP, UDP, or ICMP\n\npayloads."]
pub type Rxipv6GdFrmsR = crate::FieldReader<u32>;
#[doc = "Field `RXIPV6_GD_FRMS` writer - Number of good IPv6 datagrams received with TCP, UDP, or ICMP\n\npayloads."]
pub type Rxipv6GdFrmsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of good IPv6 datagrams received with TCP, UDP, or ICMP\n\npayloads."]
    #[inline(always)]
    pub fn rxipv6_gd_frms(&self) -> Rxipv6GdFrmsR {
        Rxipv6GdFrmsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of good IPv6 datagrams received with TCP, UDP, or ICMP\n\npayloads."]
    #[inline(always)]
    #[must_use]
    pub fn rxipv6_gd_frms(&mut self) -> Rxipv6GdFrmsW<GmacMmcRxipv6gfrmSpec> {
        Rxipv6GdFrmsW::new(self, 0)
    }
}
#[doc = "MMC RX IPV6 Good Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxipv6gfrm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxipv6gfrm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacMmcRxipv6gfrmSpec;
impl crate::RegisterSpec for GmacMmcRxipv6gfrmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_mmc_rxipv6gfrm::R`](R) reader structure"]
impl crate::Readable for GmacMmcRxipv6gfrmSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_mmc_rxipv6gfrm::W`](W) writer structure"]
impl crate::Writable for GmacMmcRxipv6gfrmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_MMC_RXIPV6GFRM to value 0"]
impl crate::Resettable for GmacMmcRxipv6gfrmSpec {
    const RESET_VALUE: u32 = 0;
}
