#[doc = "Register `DMARDLR` reader"]
pub type R = crate::R<DmardlrSpec>;
#[doc = "Register `DMARDLR` writer"]
pub type W = crate::W<DmardlrSpec>;
#[doc = "Field `RDL` reader - Receive Data Level\n\nThis bit field controls the level at which a DMA request is made by\n\nthe receive logic. The watermark level = DMARDL+1; that is,\n\ndma_rx_req is generated when the number of valid data entries\n\nin the receive FIFO is equal to or above this field value + 1, and\n\nReceive DMA Enable(DMACR\\[0\\])=1."]
pub type RdlR = crate::FieldReader;
#[doc = "Field `RDL` writer - Receive Data Level\n\nThis bit field controls the level at which a DMA request is made by\n\nthe receive logic. The watermark level = DMARDL+1; that is,\n\ndma_rx_req is generated when the number of valid data entries\n\nin the receive FIFO is equal to or above this field value + 1, and\n\nReceive DMA Enable(DMACR\\[0\\])=1."]
pub type RdlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Receive Data Level\n\nThis bit field controls the level at which a DMA request is made by\n\nthe receive logic. The watermark level = DMARDL+1; that is,\n\ndma_rx_req is generated when the number of valid data entries\n\nin the receive FIFO is equal to or above this field value + 1, and\n\nReceive DMA Enable(DMACR\\[0\\])=1."]
    #[inline(always)]
    pub fn rdl(&self) -> RdlR {
        RdlR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Receive Data Level\n\nThis bit field controls the level at which a DMA request is made by\n\nthe receive logic. The watermark level = DMARDL+1; that is,\n\ndma_rx_req is generated when the number of valid data entries\n\nin the receive FIFO is equal to or above this field value + 1, and\n\nReceive DMA Enable(DMACR\\[0\\])=1."]
    #[inline(always)]
    #[must_use]
    pub fn rdl(&mut self) -> RdlW<DmardlrSpec> {
        RdlW::new(self, 0)
    }
}
#[doc = "DMA Receive Data Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmardlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmardlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmardlrSpec;
impl crate::RegisterSpec for DmardlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmardlr::R`](R) reader structure"]
impl crate::Readable for DmardlrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmardlr::W`](W) writer structure"]
impl crate::Writable for DmardlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMARDLR to value 0"]
impl crate::Resettable for DmardlrSpec {
    const RESET_VALUE: u32 = 0;
}
