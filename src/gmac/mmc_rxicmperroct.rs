#[doc = "Register `MMC_RXICMPERROCT` reader"]
pub type R = crate::R<MmcRxicmperroctSpec>;
#[doc = "Register `MMC_RXICMPERROCT` writer"]
pub type W = crate::W<MmcRxicmperroctSpec>;
#[doc = "Field `RXICMP_ERR_OCTETS` reader - Number of bytes received in an ICMP segment with checksum\n\nerrors."]
pub type RxicmpErrOctetsR = crate::FieldReader<u32>;
#[doc = "Field `RXICMP_ERR_OCTETS` writer - Number of bytes received in an ICMP segment with checksum\n\nerrors."]
pub type RxicmpErrOctetsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes received in an ICMP segment with checksum\n\nerrors."]
    #[inline(always)]
    pub fn rxicmp_err_octets(&self) -> RxicmpErrOctetsR {
        RxicmpErrOctetsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes received in an ICMP segment with checksum\n\nerrors."]
    #[inline(always)]
    #[must_use]
    pub fn rxicmp_err_octets(&mut self) -> RxicmpErrOctetsW<MmcRxicmperroctSpec> {
        RxicmpErrOctetsW::new(self, 0)
    }
}
#[doc = "MMC RX OCTET ICMP Error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxicmperroct::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxicmperroct::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcRxicmperroctSpec;
impl crate::RegisterSpec for MmcRxicmperroctSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_rxicmperroct::R`](R) reader structure"]
impl crate::Readable for MmcRxicmperroctSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_rxicmperroct::W`](W) writer structure"]
impl crate::Writable for MmcRxicmperroctSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_RXICMPERROCT to value 0"]
impl crate::Resettable for MmcRxicmperroctSpec {
    const RESET_VALUE: u32 = 0;
}
