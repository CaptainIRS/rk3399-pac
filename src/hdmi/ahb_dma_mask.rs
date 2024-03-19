#[doc = "Register `AHB_DMA_MASK` reader"]
pub type R = crate::R<AhbDmaMaskSpec>;
#[doc = "Register `AHB_DMA_MASK` writer"]
pub type W = crate::W<AhbDmaMaskSpec>;
#[doc = "Field `FIFO_EMPTY_MASK` reader - Audio FIFO empty interrupt mask."]
pub type FifoEmptyMaskR = crate::BitReader;
#[doc = "Field `FIFO_EMPTY_MASK` writer - Audio FIFO empty interrupt mask."]
pub type FifoEmptyMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_FULL_MASK` reader - Audio FIFO full interrupt mask."]
pub type FifoFullMaskR = crate::BitReader;
#[doc = "Field `FIFO_FULL_MASK` writer - Audio FIFO full interrupt mask."]
pub type FifoFullMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_THREMPTY_MASK` reader - Audio FIFO empty interrupt mask when audio FIFO has\n\nless than the number of enabled audio channels."]
pub type FifoThremptyMaskR = crate::BitReader;
#[doc = "Field `FIFO_THREMPTY_MASK` writer - Audio FIFO empty interrupt mask when audio FIFO has\n\nless than the number of enabled audio channels."]
pub type FifoThremptyMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR_MASK` reader - Error interrupt mask. Active when slave indicates error\n\nthrough the isresp\\[1:0\\]."]
pub type ErrorMaskR = crate::BitReader;
#[doc = "Field `ERROR_MASK` writer - Error interrupt mask. Active when slave indicates error\n\nthrough the isresp\\[1:0\\]."]
pub type ErrorMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOSTOWNERSHIP_MASK` reader - Master lost ownership interrupt mask when in burst\n\ntransfer. Active when AHB master loses BUS ownership\n\nwithin the course of a burst transfer."]
pub type LostownershipMaskR = crate::BitReader;
#[doc = "Field `LOSTOWNERSHIP_MASK` writer - Master lost ownership interrupt mask when in burst\n\ntransfer. Active when AHB master loses BUS ownership\n\nwithin the course of a burst transfer."]
pub type LostownershipMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETRYSPLIT_MASK` reader - Retry/split interrupt mask. Active when AHB master\n\nreceives a RETRY or SPLIT response from slave."]
pub type RetrysplitMaskR = crate::BitReader;
#[doc = "Field `RETRYSPLIT_MASK` writer - Retry/split interrupt mask. Active when AHB master\n\nreceives a RETRY or SPLIT response from slave."]
pub type RetrysplitMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE_MASK` reader - DMA end of operation interrupt mask. Active when DMA\n\nengine reaches final_addr\\[15:0\\]
or when stop DMA\n\noperation is activated."]
pub type DoneMaskR = crate::BitReader;
#[doc = "Field `DONE_MASK` writer - DMA end of operation interrupt mask. Active when DMA\n\nengine reaches final_addr\\[15:0\\]
or when stop DMA\n\noperation is activated."]
pub type DoneMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Audio FIFO empty interrupt mask."]
    #[inline(always)]
    pub fn fifo_empty_mask(&self) -> FifoEmptyMaskR {
        FifoEmptyMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Audio FIFO full interrupt mask."]
    #[inline(always)]
    pub fn fifo_full_mask(&self) -> FifoFullMaskR {
        FifoFullMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Audio FIFO empty interrupt mask when audio FIFO has\n\nless than the number of enabled audio channels."]
    #[inline(always)]
    pub fn fifo_thrempty_mask(&self) -> FifoThremptyMaskR {
        FifoThremptyMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Error interrupt mask. Active when slave indicates error\n\nthrough the isresp\\[1:0\\]."]
    #[inline(always)]
    pub fn error_mask(&self) -> ErrorMaskR {
        ErrorMaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master lost ownership interrupt mask when in burst\n\ntransfer. Active when AHB master loses BUS ownership\n\nwithin the course of a burst transfer."]
    #[inline(always)]
    pub fn lostownership_mask(&self) -> LostownershipMaskR {
        LostownershipMaskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Retry/split interrupt mask. Active when AHB master\n\nreceives a RETRY or SPLIT response from slave."]
    #[inline(always)]
    pub fn retrysplit_mask(&self) -> RetrysplitMaskR {
        RetrysplitMaskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA end of operation interrupt mask. Active when DMA\n\nengine reaches final_addr\\[15:0\\]
or when stop DMA\n\noperation is activated."]
    #[inline(always)]
    pub fn done_mask(&self) -> DoneMaskR {
        DoneMaskR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Audio FIFO empty interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_empty_mask(&mut self) -> FifoEmptyMaskW<AhbDmaMaskSpec> {
        FifoEmptyMaskW::new(self, 0)
    }
    #[doc = "Bit 1 - Audio FIFO full interrupt mask."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_full_mask(&mut self) -> FifoFullMaskW<AhbDmaMaskSpec> {
        FifoFullMaskW::new(self, 1)
    }
    #[doc = "Bit 2 - Audio FIFO empty interrupt mask when audio FIFO has\n\nless than the number of enabled audio channels."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_thrempty_mask(&mut self) -> FifoThremptyMaskW<AhbDmaMaskSpec> {
        FifoThremptyMaskW::new(self, 2)
    }
    #[doc = "Bit 4 - Error interrupt mask. Active when slave indicates error\n\nthrough the isresp\\[1:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn error_mask(&mut self) -> ErrorMaskW<AhbDmaMaskSpec> {
        ErrorMaskW::new(self, 4)
    }
    #[doc = "Bit 5 - Master lost ownership interrupt mask when in burst\n\ntransfer. Active when AHB master loses BUS ownership\n\nwithin the course of a burst transfer."]
    #[inline(always)]
    #[must_use]
    pub fn lostownership_mask(&mut self) -> LostownershipMaskW<AhbDmaMaskSpec> {
        LostownershipMaskW::new(self, 5)
    }
    #[doc = "Bit 6 - Retry/split interrupt mask. Active when AHB master\n\nreceives a RETRY or SPLIT response from slave."]
    #[inline(always)]
    #[must_use]
    pub fn retrysplit_mask(&mut self) -> RetrysplitMaskW<AhbDmaMaskSpec> {
        RetrysplitMaskW::new(self, 6)
    }
    #[doc = "Bit 7 - DMA end of operation interrupt mask. Active when DMA\n\nengine reaches final_addr\\[15:0\\]
or when stop DMA\n\noperation is activated."]
    #[inline(always)]
    #[must_use]
    pub fn done_mask(&mut self) -> DoneMaskW<AhbDmaMaskSpec> {
        DoneMaskW::new(self, 7)
    }
}
#[doc = "Audio DMA Mask Interrupt Register\n\nThis register masks each of the interrupts present in the AHB audio DMA module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbDmaMaskSpec;
impl crate::RegisterSpec for AhbDmaMaskSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ahb_dma_mask::R`](R) reader structure"]
impl crate::Readable for AhbDmaMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_mask::W`](W) writer structure"]
impl crate::Writable for AhbDmaMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AHB_DMA_MASK to value 0xf7"]
impl crate::Resettable for AhbDmaMaskSpec {
    const RESET_VALUE: u8 = 0xf7;
}
