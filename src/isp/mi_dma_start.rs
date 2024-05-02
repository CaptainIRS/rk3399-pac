#[doc = "Register `MI_DMA_START` writer"]
pub type W = crate::W<MiDmaStartSpec>;
#[doc = "Field `dma_start` writer - Enables DMA access. Additionally main or self path\n\nhas to be enabled separately.\n\n"]
pub type DmaStartW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Enables DMA access. Additionally main or self path\n\nhas to be enabled separately.\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn dma_start(&mut self) -> DmaStartW<MiDmaStartSpec> {
        DmaStartW::new(self, 0)
    }
}
#[doc = "DMA start register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_dma_start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiDmaStartSpec;
impl crate::RegisterSpec for MiDmaStartSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mi_dma_start::W`](W) writer structure"]
impl crate::Writable for MiDmaStartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_DMA_START to value 0"]
impl crate::Resettable for MiDmaStartSpec {
    const RESET_VALUE: u32 = 0;
}
