#[doc = "Register `AHB_DMA_BUFFMASK` reader"]
pub type R = crate::R<AhbDmaBuffmaskSpec>;
#[doc = "Register `AHB_DMA_BUFFMASK` writer"]
pub type W = crate::W<AhbDmaBuffmaskSpec>;
#[doc = "Field `MASK_BUFF_EMPTY` reader - Buffer empty flag mask"]
pub type MaskBuffEmptyR = crate::BitReader;
#[doc = "Field `MASK_BUFF_EMPTY` writer - Buffer empty flag mask"]
pub type MaskBuffEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_BUFF_FULL` reader - Buffer full flag mask"]
pub type MaskBuffFullR = crate::BitReader;
#[doc = "Field `MASK_BUFF_FULL` writer - Buffer full flag mask"]
pub type MaskBuffFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_FIFO_OVERRUN` reader - Buffer overrun flag mask"]
pub type MaskFifoOverrunR = crate::BitReader;
#[doc = "Field `MASK_FIFO_OVERRUN` writer - Buffer overrun flag mask"]
pub type MaskFifoOverrunW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Buffer empty flag mask"]
    #[inline(always)]
    pub fn mask_buff_empty(&self) -> MaskBuffEmptyR {
        MaskBuffEmptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Buffer full flag mask"]
    #[inline(always)]
    pub fn mask_buff_full(&self) -> MaskBuffFullR {
        MaskBuffFullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer overrun flag mask"]
    #[inline(always)]
    pub fn mask_fifo_overrun(&self) -> MaskFifoOverrunR {
        MaskFifoOverrunR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer empty flag mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask_buff_empty(&mut self) -> MaskBuffEmptyW<AhbDmaBuffmaskSpec> {
        MaskBuffEmptyW::new(self, 0)
    }
    #[doc = "Bit 1 - Buffer full flag mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask_buff_full(&mut self) -> MaskBuffFullW<AhbDmaBuffmaskSpec> {
        MaskBuffFullW::new(self, 1)
    }
    #[doc = "Bit 4 - Buffer overrun flag mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask_fifo_overrun(&mut self) -> MaskFifoOverrunW<AhbDmaBuffmaskSpec> {
        MaskFifoOverrunW::new(self, 4)
    }
}
#[doc = "Audio DMA Buffer Mask Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_buffmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_buffmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbDmaBuffmaskSpec;
impl crate::RegisterSpec for AhbDmaBuffmaskSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ahb_dma_buffmask::R`](R) reader structure"]
impl crate::Readable for AhbDmaBuffmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_buffmask::W`](W) writer structure"]
impl crate::Writable for AhbDmaBuffmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AHB_DMA_BUFFMASK to value 0x13"]
impl crate::Resettable for AhbDmaBuffmaskSpec {
    const RESET_VALUE: u8 = 0x13;
}
