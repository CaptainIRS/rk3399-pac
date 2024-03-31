#[doc = "Register `MMC_RXFIFOOVRFLW` reader"]
pub type R = crate::R<MmcRxfifoovrflwSpec>;
#[doc = "Register `MMC_RXFIFOOVRFLW` writer"]
pub type W = crate::W<MmcRxfifoovrflwSpec>;
#[doc = "Field `RXFIFOOVERFLOW` reader - Number of missed received frames due to FIFO overflow."]
pub type RxfifooverflowR = crate::FieldReader<u32>;
#[doc = "Field `RXFIFOOVERFLOW` writer - Number of missed received frames due to FIFO overflow."]
pub type RxfifooverflowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of missed received frames due to FIFO overflow."]
    #[inline(always)]
    pub fn rxfifooverflow(&self) -> RxfifooverflowR {
        RxfifooverflowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of missed received frames due to FIFO overflow."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifooverflow(&mut self) -> RxfifooverflowW<MmcRxfifoovrflwSpec> {
        RxfifooverflowW::new(self, 0)
    }
}
#[doc = "MMC RX FIFO Overflow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_rxfifoovrflw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_rxfifoovrflw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcRxfifoovrflwSpec;
impl crate::RegisterSpec for MmcRxfifoovrflwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_rxfifoovrflw::R`](R) reader structure"]
impl crate::Readable for MmcRxfifoovrflwSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_rxfifoovrflw::W`](W) writer structure"]
impl crate::Writable for MmcRxfifoovrflwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMC_RXFIFOOVRFLW to value 0"]
impl crate::Resettable for MmcRxfifoovrflwSpec {
    const RESET_VALUE: u32 = 0;
}
