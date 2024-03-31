#[doc = "Register `MMC_RXIPV4HDERRFRM` reader"]
pub type R = crate::R<MmcRxipv4hderrfrmSpec>;
#[doc = "Register `MMC_RXIPV4HDERRFRM` writer"]
pub type W = crate::W<MmcRxipv4hderrfrmSpec>;
#[doc = "Field `RXIPV4_HDRERR_FRMS` reader - Number of IPv4 datagrams received with header (checksum,\n\nlength, or version mismatch) errors"]
pub type Rxipv4HdrerrFrmsR = crate::FieldReader<u32>;
#[doc = "Field `RXIPV4_HDRERR_FRMS` writer - Number of IPv4 datagrams received with header (checksum,\n\nlength, or version mismatch) errors"]
pub type Rxipv4HdrerrFrmsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of IPv4 datagrams received with header (checksum,\n\nlength, or version mismatch) errors"]
    #[inline(always)]
    pub fn rxipv4_hdrerr_frms(&self) -> Rxipv4HdrerrFrmsR {
        Rxipv4HdrerrFrmsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of IPv4 datagrams received with header (checksum,\n\nlength, or version mismatch) errors"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv4_hdrerr_frms(&mut self) -> Rxipv4HdrerrFrmsW<MmcRxipv4hderrfrmSpec> {
        Rxipv4HdrerrFrmsW::new(self, 0)
    }
}
#[doc = "MMC RX IPV4 Head Error Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxipv4hderrfrm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxipv4hderrfrm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcRxipv4hderrfrmSpec;
impl crate::RegisterSpec for MmcRxipv4hderrfrmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_rxipv4hderrfrm::R`](R) reader structure"]
impl crate::Readable for MmcRxipv4hderrfrmSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_rxipv4hderrfrm::W`](W) writer structure"]
impl crate::Writable for MmcRxipv4hderrfrmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_RXIPV4HDERRFRM to value 0"]
impl crate::Resettable for MmcRxipv4hderrfrmSpec {
    const RESET_VALUE: u32 = 0;
}
