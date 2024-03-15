#[doc = "Register `GMAC_MMC_RXICMPERROCT` reader"]
pub type R = crate::R<GmacMmcRxicmperroctSpec>;
#[doc = "Register `GMAC_MMC_RXICMPERROCT` writer"]
pub type W = crate::W<GmacMmcRxicmperroctSpec>;
#[doc = "Field `RXICMP_ERR_OCTETS` reader - Number of bytes received in an ICMP segment with checksum errors."]
pub type RxicmpErrOctetsR = crate::FieldReader<u32>;
#[doc = "Field `RXICMP_ERR_OCTETS` writer - Number of bytes received in an ICMP segment with checksum errors."]
pub type RxicmpErrOctetsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes received in an ICMP segment with checksum errors."]
    #[inline(always)]
    pub fn rxicmp_err_octets(&self) -> RxicmpErrOctetsR {
        RxicmpErrOctetsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes received in an ICMP segment with checksum errors."]
    #[inline(always)]
    #[must_use]
    pub fn rxicmp_err_octets(&mut self) -> RxicmpErrOctetsW<GmacMmcRxicmperroctSpec> {
        RxicmpErrOctetsW::new(self, 0)
    }
}
#[doc = "MMC RX OCTET ICMP Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxicmperroct::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxicmperroct::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacMmcRxicmperroctSpec;
impl crate::RegisterSpec for GmacMmcRxicmperroctSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_mmc_rxicmperroct::R`](R) reader structure"]
impl crate::Readable for GmacMmcRxicmperroctSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_mmc_rxicmperroct::W`](W) writer structure"]
impl crate::Writable for GmacMmcRxicmperroctSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_MMC_RXICMPERROCT to value 0"]
impl crate::Resettable for GmacMmcRxicmperroctSpec {
    const RESET_VALUE: u32 = 0;
}
