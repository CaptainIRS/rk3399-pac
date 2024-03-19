#[doc = "Register `AHB_DMA_THRSLD` reader"]
pub type R = crate::R<AhbDmaThrsldSpec>;
#[doc = "Register `AHB_DMA_THRSLD` writer"]
pub type W = crate::W<AhbDmaThrsldSpec>;
#[doc = "Field `FIFO_THRESHOLD` reader - FIFO medium threshold occupation value"]
pub type FifoThresholdR = crate::FieldReader;
#[doc = "Field `FIFO_THRESHOLD` writer - FIFO medium threshold occupation value"]
pub type FifoThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - FIFO medium threshold occupation value"]
    #[inline(always)]
    pub fn fifo_threshold(&self) -> FifoThresholdR {
        FifoThresholdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - FIFO medium threshold occupation value"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_threshold(&mut self) -> FifoThresholdW<AhbDmaThrsldSpec> {
        FifoThresholdW::new(self, 0)
    }
}
#[doc = "Audio DMA FIFO Threshold Register\n\nThis register defines the FIFO medium threshold occupation value.\n\nAfter the AHB master completes a burst transaction successfully, the FIFO may remain full\n\ntill the data fetch interface requests samples. Each data fetch operation reduces the\n\nnumber of samples stored in the FIFO by the number of channels enabled.\n\nTherefore, the fifo_threshold\\[7:0\\]
is the medium number of samples that should be\n\navailable in the audio FIFO across the DMA operation.\n\nAs soon as the number of samples in the FIFO drops lower than the fifo_threshold\\[7:0\\], the\n\nDMA engine requests a new burst of samples for the AHB master. The length is constrained\n\nby the size of buffer provided, the instantiated FIFO depth minus fifo_threshold\\[7:0\\],\n\nand/or the number of words up to the next 1 kbyte boundary.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_thrsld::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_thrsld::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbDmaThrsldSpec;
impl crate::RegisterSpec for AhbDmaThrsldSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ahb_dma_thrsld::R`](R) reader structure"]
impl crate::Readable for AhbDmaThrsldSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_thrsld::W`](W) writer structure"]
impl crate::Writable for AhbDmaThrsldSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AHB_DMA_THRSLD to value 0"]
impl crate::Resettable for AhbDmaThrsldSpec {
    const RESET_VALUE: u8 = 0;
}
