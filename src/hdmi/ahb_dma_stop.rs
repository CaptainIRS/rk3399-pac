#[doc = "Register `AHB_DMA_STOP` reader"]
pub type R = crate::R<AhbDmaStopSpec>;
#[doc = "Register `AHB_DMA_STOP` writer"]
pub type W = crate::W<AhbDmaStopSpec>;
#[doc = "Field `STOP_DMA_TRANSACTION` reader - Stop DMA transaction\n\nThis register is auto-cleared when the transfer operation\n\nis stopped (done)."]
pub type StopDmaTransactionR = crate::BitReader;
#[doc = "Field `STOP_DMA_TRANSACTION` writer - Stop DMA transaction\n\nThis register is auto-cleared when the transfer operation\n\nis stopped (done)."]
pub type StopDmaTransactionW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stop DMA transaction\n\nThis register is auto-cleared when the transfer operation\n\nis stopped (done)."]
    #[inline(always)]
    pub fn stop_dma_transaction(&self) -> StopDmaTransactionR {
        StopDmaTransactionR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop DMA transaction\n\nThis register is auto-cleared when the transfer operation\n\nis stopped (done)."]
    #[inline(always)]
    #[must_use]
    pub fn stop_dma_transaction(&mut self) -> StopDmaTransactionW<AhbDmaStopSpec> {
        StopDmaTransactionW::new(self, 0)
    }
}
#[doc = "Audio DMA Stop Register\n\nThe stop_dma_transaction bit field signals the AHB audio DMA to stop current Attr. After it\n\nstops, if a new start DMA operation is requested, the DMA engine restarts the Attr using\n\nthe initial_addr\\[31:0\\], which is programmed at ahb_dma_straddr0 to ahb_dma_straddr3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_stop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_stop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbDmaStopSpec;
impl crate::RegisterSpec for AhbDmaStopSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ahb_dma_stop::R`](R) reader structure"]
impl crate::Readable for AhbDmaStopSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_stop::W`](W) writer structure"]
impl crate::Writable for AhbDmaStopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AHB_DMA_STOP to value 0"]
impl crate::Resettable for AhbDmaStopSpec {
    const RESET_VALUE: u8 = 0;
}
