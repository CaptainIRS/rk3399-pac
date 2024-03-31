#[doc = "Register `MMC_RXMCFRMCNT_G` reader"]
pub type R = crate::R<MmcRxmcfrmcntGSpec>;
#[doc = "Register `MMC_RXMCFRMCNT_G` writer"]
pub type W = crate::W<MmcRxmcfrmcntGSpec>;
#[doc = "Field `RXMULTICASTFRAMES_G` reader - Number of good multicast frames received."]
pub type RxmulticastframesGR = crate::FieldReader<u32>;
#[doc = "Field `RXMULTICASTFRAMES_G` writer - Number of good multicast frames received."]
pub type RxmulticastframesGW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of good multicast frames received."]
    #[inline(always)]
    pub fn rxmulticastframes_g(&self) -> RxmulticastframesGR {
        RxmulticastframesGR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of good multicast frames received."]
    #[inline(always)]
    #[must_use]
    pub fn rxmulticastframes_g(&mut self) -> RxmulticastframesGW<MmcRxmcfrmcntGSpec> {
        RxmulticastframesGW::new(self, 0)
    }
}
#[doc = "MMC RX Multicast Frame Good Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxmcfrmcnt_g::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxmcfrmcnt_g::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcRxmcfrmcntGSpec;
impl crate::RegisterSpec for MmcRxmcfrmcntGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_rxmcfrmcnt_g::R`](R) reader structure"]
impl crate::Readable for MmcRxmcfrmcntGSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_rxmcfrmcnt_g::W`](W) writer structure"]
impl crate::Writable for MmcRxmcfrmcntGSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_RXMCFRMCNT_G to value 0"]
impl crate::Resettable for MmcRxmcfrmcntGSpec {
    const RESET_VALUE: u32 = 0;
}
