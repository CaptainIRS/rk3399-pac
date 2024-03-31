#[doc = "Register `MMC_RXICMPERRFRM` reader"]
pub type R = crate::R<MmcRxicmperrfrmSpec>;
#[doc = "Register `MMC_RXICMPERRFRM` writer"]
pub type W = crate::W<MmcRxicmperrfrmSpec>;
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
    pub fn rxicmp_err_frms(&mut self) -> RxicmpErrFrmsW<MmcRxicmperrfrmSpec> {
        RxicmpErrFrmsW::new(self, 0)
    }
}
#[doc = "MMC RX ICMP Error Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxicmperrfrm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxicmperrfrm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcRxicmperrfrmSpec;
impl crate::RegisterSpec for MmcRxicmperrfrmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_rxicmperrfrm::R`](R) reader structure"]
impl crate::Readable for MmcRxicmperrfrmSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_rxicmperrfrm::W`](W) writer structure"]
impl crate::Writable for MmcRxicmperrfrmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_RXICMPERRFRM to value 0"]
impl crate::Resettable for MmcRxicmperrfrmSpec {
    const RESET_VALUE: u32 = 0;
}
