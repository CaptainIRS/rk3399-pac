#[doc = "Register `AHB_DMA_MASK1` reader"]
pub type R = crate::R<AhbDmaMask1Spec>;
#[doc = "Register `AHB_DMA_MASK1` writer"]
pub type W = crate::W<AhbDmaMask1Spec>;
#[doc = "Field `FIFO_OVERRUN_MASK` reader - AHB DMA FIFO overrun mask"]
pub type FifoOverrunMaskR = crate::BitReader;
#[doc = "Field `FIFO_OVERRUN_MASK` writer - AHB DMA FIFO overrun mask"]
pub type FifoOverrunMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_UNDERRUN_MASK` reader - AHB DMA FIFO underrun mask"]
pub type FifoUnderrunMaskR = crate::BitReader;
#[doc = "Field `FIFO_UNDERRUN_MASK` writer - AHB DMA FIFO underrun mask"]
pub type FifoUnderrunMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AHB DMA FIFO overrun mask"]
    #[inline(always)]
    pub fn fifo_overrun_mask(&self) -> FifoOverrunMaskR {
        FifoOverrunMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AHB DMA FIFO underrun mask"]
    #[inline(always)]
    pub fn fifo_underrun_mask(&self) -> FifoUnderrunMaskR {
        FifoUnderrunMaskR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AHB DMA FIFO overrun mask"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_overrun_mask(&mut self) -> FifoOverrunMaskW<AhbDmaMask1Spec> {
        FifoOverrunMaskW::new(self, 0)
    }
    #[doc = "Bit 1 - AHB DMA FIFO underrun mask"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_underrun_mask(&mut self) -> FifoUnderrunMaskW<AhbDmaMask1Spec> {
        FifoUnderrunMaskW::new(self, 1)
    }
}
#[doc = "AHB DMA FIFO overrun mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_mask1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_mask1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbDmaMask1Spec;
impl crate::RegisterSpec for AhbDmaMask1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ahb_dma_mask1::R`](R) reader structure"]
impl crate::Readable for AhbDmaMask1Spec {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_mask1::W`](W) writer structure"]
impl crate::Writable for AhbDmaMask1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AHB_DMA_MASK1 to value 0x03"]
impl crate::Resettable for AhbDmaMask1Spec {
    const RESET_VALUE: u8 = 0x03;
}
