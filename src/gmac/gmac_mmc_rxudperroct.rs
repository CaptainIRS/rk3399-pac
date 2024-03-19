#[doc = "Register `GMAC_MMC_RXUDPERROCT` reader"]
pub type R = crate::R<GmacMmcRxudperroctSpec>;
#[doc = "Register `GMAC_MMC_RXUDPERROCT` writer"]
pub type W = crate::W<GmacMmcRxudperroctSpec>;
#[doc = "Field `RXUDP_ERR_OCTETS` reader - Number of bytes received in a UDP segment that had checksum\n\nerrors."]
pub type RxudpErrOctetsR = crate::FieldReader<u32>;
#[doc = "Field `RXUDP_ERR_OCTETS` writer - Number of bytes received in a UDP segment that had checksum\n\nerrors."]
pub type RxudpErrOctetsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes received in a UDP segment that had checksum\n\nerrors."]
    #[inline(always)]
    pub fn rxudp_err_octets(&self) -> RxudpErrOctetsR {
        RxudpErrOctetsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes received in a UDP segment that had checksum\n\nerrors."]
    #[inline(always)]
    #[must_use]
    pub fn rxudp_err_octets(&mut self) -> RxudpErrOctetsW<GmacMmcRxudperroctSpec> {
        RxudpErrOctetsW::new(self, 0)
    }
}
#[doc = "MMC RX OCTET UDP Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_mmc_rxudperroct::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_mmc_rxudperroct::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacMmcRxudperroctSpec;
impl crate::RegisterSpec for GmacMmcRxudperroctSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_mmc_rxudperroct::R`](R) reader structure"]
impl crate::Readable for GmacMmcRxudperroctSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_mmc_rxudperroct::W`](W) writer structure"]
impl crate::Writable for GmacMmcRxudperroctSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_MMC_RXUDPERROCT to value 0"]
impl crate::Resettable for GmacMmcRxudperroctSpec {
    const RESET_VALUE: u32 = 0;
}
