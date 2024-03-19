#[doc = "Register `AHB_DMA_START` reader"]
pub type R = crate::R<AhbDmaStartSpec>;
#[doc = "Register `AHB_DMA_START` writer"]
pub type W = crate::W<AhbDmaStartSpec>;
#[doc = "Field `START_DMA_TRANSACTION` reader - Start DMA transaction\n\nThis register is auto-cleared when the transfer operation\n\nis completed (done)."]
pub type StartDmaTransactionR = crate::BitReader;
#[doc = "Field `START_DMA_TRANSACTION` writer - Start DMA transaction\n\nThis register is auto-cleared when the transfer operation\n\nis completed (done)."]
pub type StartDmaTransactionW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start DMA transaction\n\nThis register is auto-cleared when the transfer operation\n\nis completed (done)."]
    #[inline(always)]
    pub fn start_dma_transaction(&self) -> StartDmaTransactionR {
        StartDmaTransactionR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start DMA transaction\n\nThis register is auto-cleared when the transfer operation\n\nis completed (done)."]
    #[inline(always)]
    #[must_use]
    pub fn start_dma_transaction(&mut self) -> StartDmaTransactionW<AhbDmaStartSpec> {
        StartDmaTransactionW::new(self, 0)
    }
}
#[doc = "Audio DMA Start Register\n\nThe start_dma_transaction bit field signals the AHB audio DMA to start accessing system\n\nmemory in order to fetch data samples to store in the FIFO. After the operation starts, a\n\nnew request for a DMA start is ignored until the DMA is stopped or it reaches the end\n\naddress. Only in one of these situations is a new start request acknowledged.\n\nThe first DMA burst request after start_dma_transaction configuration uses\n\ninitial_addr\\[31:0\\]
as ohaddr\\[31:0\\]
value; mburstlength\\[8:0\\]
is set to the maximum\n\nadmissible value. This maximum value is constrained by the size of buffer provided, the\n\ninstantiated FIFO depth, or/and the number of words up to the next 1 Kbyte boundary.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_start::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_start::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbDmaStartSpec;
impl crate::RegisterSpec for AhbDmaStartSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ahb_dma_start::R`](R) reader structure"]
impl crate::Readable for AhbDmaStartSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_start::W`](W) writer structure"]
impl crate::Writable for AhbDmaStartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AHB_DMA_START to value 0"]
impl crate::Resettable for AhbDmaStartSpec {
    const RESET_VALUE: u8 = 0;
}
