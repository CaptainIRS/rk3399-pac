#[doc = "Register `DMATDLR` reader"]
pub type R = crate::R<DmatdlrSpec>;
#[doc = "Register `DMATDLR` writer"]
pub type W = crate::W<DmatdlrSpec>;
#[doc = "Field `TDL` reader - Transmit Data Level\n\nThis bit field controls the level at which a DMA request is made by\n\nthe transmit logic. It is equal to the watermark level; that is, the\n\ndma_tx_req signal is generated when the number of valid data\n\nentries in the transmit FIFO is equal to or below this field value,\n\nand Transmit DMA Enable (DMACR\\[1\\]) = 1."]
pub type TdlR = crate::FieldReader;
#[doc = "Field `TDL` writer - Transmit Data Level\n\nThis bit field controls the level at which a DMA request is made by\n\nthe transmit logic. It is equal to the watermark level; that is, the\n\ndma_tx_req signal is generated when the number of valid data\n\nentries in the transmit FIFO is equal to or below this field value,\n\nand Transmit DMA Enable (DMACR\\[1\\]) = 1."]
pub type TdlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Transmit Data Level\n\nThis bit field controls the level at which a DMA request is made by\n\nthe transmit logic. It is equal to the watermark level; that is, the\n\ndma_tx_req signal is generated when the number of valid data\n\nentries in the transmit FIFO is equal to or below this field value,\n\nand Transmit DMA Enable (DMACR\\[1\\]) = 1."]
    #[inline(always)]
    pub fn tdl(&self) -> TdlR {
        TdlR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Transmit Data Level\n\nThis bit field controls the level at which a DMA request is made by\n\nthe transmit logic. It is equal to the watermark level; that is, the\n\ndma_tx_req signal is generated when the number of valid data\n\nentries in the transmit FIFO is equal to or below this field value,\n\nand Transmit DMA Enable (DMACR\\[1\\]) = 1."]
    #[inline(always)]
    #[must_use]
    pub fn tdl(&mut self) -> TdlW<DmatdlrSpec> {
        TdlW::new(self, 0)
    }
}
#[doc = "DMA Transmit Data Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatdlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatdlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmatdlrSpec;
impl crate::RegisterSpec for DmatdlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatdlr::R`](R) reader structure"]
impl crate::Readable for DmatdlrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmatdlr::W`](W) writer structure"]
impl crate::Writable for DmatdlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMATDLR to value 0"]
impl crate::Resettable for DmatdlrSpec {
    const RESET_VALUE: u32 = 0;
}
