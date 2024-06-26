#[doc = "Register `MMC_RXIPV4GFRM` reader"]
pub type R = crate::R<MmcRxipv4gfrmSpec>;
#[doc = "Register `MMC_RXIPV4GFRM` writer"]
pub type W = crate::W<MmcRxipv4gfrmSpec>;
#[doc = "Field `RXIPV4_GD_FRMS` reader - Number of good IPv4 datagrams received with the TCP, UDP, or\n\nICMP payload"]
pub type Rxipv4GdFrmsR = crate::FieldReader<u32>;
#[doc = "Field `RXIPV4_GD_FRMS` writer - Number of good IPv4 datagrams received with the TCP, UDP, or\n\nICMP payload"]
pub type Rxipv4GdFrmsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of good IPv4 datagrams received with the TCP, UDP, or\n\nICMP payload"]
    #[inline(always)]
    pub fn rxipv4_gd_frms(&self) -> Rxipv4GdFrmsR {
        Rxipv4GdFrmsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of good IPv4 datagrams received with the TCP, UDP, or\n\nICMP payload"]
    #[inline(always)]
    #[must_use]
    pub fn rxipv4_gd_frms(&mut self) -> Rxipv4GdFrmsW<MmcRxipv4gfrmSpec> {
        Rxipv4GdFrmsW::new(self, 0)
    }
}
#[doc = "MMC RX IPV4 Good Frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxipv4gfrm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxipv4gfrm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcRxipv4gfrmSpec;
impl crate::RegisterSpec for MmcRxipv4gfrmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_rxipv4gfrm::R`](R) reader structure"]
impl crate::Readable for MmcRxipv4gfrmSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_rxipv4gfrm::W`](W) writer structure"]
impl crate::Writable for MmcRxipv4gfrmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_RXIPV4GFRM to value 0"]
impl crate::Resettable for MmcRxipv4gfrmSpec {
    const RESET_VALUE: u32 = 0;
}
