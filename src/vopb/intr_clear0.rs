#[doc = "Register `INTR_CLEAR0` reader"]
pub type R = crate::R<IntrClear0Spec>;
#[doc = "Register `INTR_CLEAR0` writer"]
pub type W = crate::W<IntrClear0Spec>;
#[doc = "Field `INT_CLR_FS` reader - Frame start interrupt clear (Auto clear)"]
pub type IntClrFsR = crate::BitReader;
#[doc = "Field `INT_CLR_FS` writer - Frame start interrupt clear (Auto clear)"]
pub type IntClrFsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INT_CLR_FS_NEW` reader - Frame new start interrupt clear (Auto clear)"]
pub type IntClrFsNewR = crate::BitReader;
#[doc = "Field `INT_CLR_FS_NEW` writer - Frame new start interrupt clear (Auto clear)"]
pub type IntClrFsNewW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INT_CLR_ADDR_SAME` reader - memory start addr same interruption clear(Auto clear)"]
pub type IntClrAddrSameR = crate::BitReader;
#[doc = "Field `INT_CLR_ADDR_SAME` writer - memory start addr same interruption clear(Auto clear)"]
pub type IntClrAddrSameW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INT_CLR_LINE_FLAG0` reader - Line flag 0 Interrupt clear(Auto clear)"]
pub type IntClrLineFlag0R = crate::BitReader;
#[doc = "Field `INT_CLR_LINE_FLAG0` writer - Line flag 0 Interrupt clear(Auto clear)"]
pub type IntClrLineFlag0W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INT_CLR_LINE_FLAG1` reader - Line flag 1 Interrupt clear(Auto clear)"]
pub type IntClrLineFlag1R = crate::BitReader;
#[doc = "Field `INT_CLR_LINE_FLAG1` writer - Line flag 1 Interrupt clear(Auto clear)"]
pub type IntClrLineFlag1W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INT_CLR_BUS_ERROR` reader - Bus error Interrupt clear(Auto clear)"]
pub type IntClrBusErrorR = crate::BitReader;
#[doc = "Field `INT_CLR_BUS_ERROR` writer - Bus error Interrupt clear(Auto clear)"]
pub type IntClrBusErrorW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INT_CLR_WIN0_EMPTY` reader - win0 data empty interrupt clear(Auto clear)"]
pub type IntClrWin0EmptyR = crate::BitReader;
#[doc = "Field `INT_CLR_WIN0_EMPTY` writer - win0 data empty interrupt clear(Auto clear)"]
pub type IntClrWin0EmptyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INT_CLR_WIN1_EMPTY` reader - win1 data empty interrupt clear(Auto clear)"]
pub type IntClrWin1EmptyR = crate::BitReader;
#[doc = "Field `INT_CLR_WIN1_EMPTY` writer - win1 data empty interrupt clear(Auto clear)"]
pub type IntClrWin1EmptyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INT_CLR_WIN2_EMPTY` reader - win2 data empty interrupt clear(Auto clear)"]
pub type IntClrWin2EmptyR = crate::BitReader;
#[doc = "Field `INT_CLR_WIN2_EMPTY` writer - win2 data empty interrupt clear(Auto clear)"]
pub type IntClrWin2EmptyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INT_CLR_WIN3_EMPTY` reader - win3 data empty interrupt clear(Auto clear)"]
pub type IntClrWin3EmptyR = crate::BitReader;
#[doc = "Field `INT_CLR_WIN3_EMPTY` writer - win3 data empty interrupt clear(Auto clear)"]
pub type IntClrWin3EmptyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INT_CLR_HWC_EMPTY` reader - hwc data empty interrupt clear(Auto clear)"]
pub type IntClrHwcEmptyR = crate::BitReader;
#[doc = "Field `INT_CLR_HWC_EMPTY` writer - hwc data empty interrupt clear(Auto clear)"]
pub type IntClrHwcEmptyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INT_CLR_POST_BUF_EMPTY` reader - post buffer empty interrupt clear(Auto clear)"]
pub type IntClrPostBufEmptyR = crate::BitReader;
#[doc = "Field `INT_CLR_POST_BUF_EMPTY` writer - post buffer empty interrupt clear(Auto clear)"]
pub type IntClrPostBufEmptyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INT_CLR_FS_FIELD` reader - field start interrupt clear(Auto clear)\n\nField start interrupt clear (Auto clear)"]
pub type IntClrFsFieldR = crate::BitReader;
#[doc = "Field `INT_CLR_FS_FIELD` writer - field start interrupt clear(Auto clear)\n\nField start interrupt clear (Auto clear)"]
pub type IntClrFsFieldW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INT_CLR_DSP_HOLD_VALID` reader - display hold valid interrupt clear(Auto clear)"]
pub type IntClrDspHoldValidR = crate::BitReader;
#[doc = "Field `INT_CLR_DSP_HOLD_VALID` writer - display hold valid interrupt clear(Auto clear)"]
pub type IntClrDspHoldValidW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `INT_CLR_MMU` reader - mmu interrupt clear(Auto clear)"]
pub type IntClrMmuR = crate::BitReader;
#[doc = "Field `INT_CLR_MMU` writer - mmu interrupt clear(Auto clear)"]
pub type IntClrMmuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_CLR_DMA_FINISH` reader - dma finish interrupt clear(Auto clear)"]
pub type IntClrDmaFinishR = crate::BitReader;
#[doc = "Field `INT_CLR_DMA_FINISH` writer - dma finish interrupt clear(Auto clear)"]
pub type IntClrDmaFinishW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` reader - When every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_MASK` writer - When every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Frame start interrupt clear (Auto clear)"]
    #[inline(always)]
    pub fn int_clr_fs(&self) -> IntClrFsR {
        IntClrFsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame new start interrupt clear (Auto clear)"]
    #[inline(always)]
    pub fn int_clr_fs_new(&self) -> IntClrFsNewR {
        IntClrFsNewR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - memory start addr same interruption clear(Auto clear)"]
    #[inline(always)]
    pub fn int_clr_addr_same(&self) -> IntClrAddrSameR {
        IntClrAddrSameR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Line flag 0 Interrupt clear(Auto clear)"]
    #[inline(always)]
    pub fn int_clr_line_flag0(&self) -> IntClrLineFlag0R {
        IntClrLineFlag0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line flag 1 Interrupt clear(Auto clear)"]
    #[inline(always)]
    pub fn int_clr_line_flag1(&self) -> IntClrLineFlag1R {
        IntClrLineFlag1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bus error Interrupt clear(Auto clear)"]
    #[inline(always)]
    pub fn int_clr_bus_error(&self) -> IntClrBusErrorR {
        IntClrBusErrorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - win0 data empty interrupt clear(Auto clear)"]
    #[inline(always)]
    pub fn int_clr_win0_empty(&self) -> IntClrWin0EmptyR {
        IntClrWin0EmptyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - win1 data empty interrupt clear(Auto clear)"]
    #[inline(always)]
    pub fn int_clr_win1_empty(&self) -> IntClrWin1EmptyR {
        IntClrWin1EmptyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - win2 data empty interrupt clear(Auto clear)"]
    #[inline(always)]
    pub fn int_clr_win2_empty(&self) -> IntClrWin2EmptyR {
        IntClrWin2EmptyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - win3 data empty interrupt clear(Auto clear)"]
    #[inline(always)]
    pub fn int_clr_win3_empty(&self) -> IntClrWin3EmptyR {
        IntClrWin3EmptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - hwc data empty interrupt clear(Auto clear)"]
    #[inline(always)]
    pub fn int_clr_hwc_empty(&self) -> IntClrHwcEmptyR {
        IntClrHwcEmptyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - post buffer empty interrupt clear(Auto clear)"]
    #[inline(always)]
    pub fn int_clr_post_buf_empty(&self) -> IntClrPostBufEmptyR {
        IntClrPostBufEmptyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - field start interrupt clear(Auto clear)\n\nField start interrupt clear (Auto clear)"]
    #[inline(always)]
    pub fn int_clr_fs_field(&self) -> IntClrFsFieldR {
        IntClrFsFieldR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - display hold valid interrupt clear(Auto clear)"]
    #[inline(always)]
    pub fn int_clr_dsp_hold_valid(&self) -> IntClrDspHoldValidR {
        IntClrDspHoldValidR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - mmu interrupt clear(Auto clear)"]
    #[inline(always)]
    pub fn int_clr_mmu(&self) -> IntClrMmuR {
        IntClrMmuR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - dma finish interrupt clear(Auto clear)"]
    #[inline(always)]
    pub fn int_clr_dma_finish(&self) -> IntClrDmaFinishR {
        IntClrDmaFinishR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - When every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    pub fn write_mask(&self) -> WriteMaskR {
        WriteMaskR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Frame start interrupt clear (Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_fs(&mut self) -> IntClrFsW<IntrClear0Spec> {
        IntClrFsW::new(self, 0)
    }
    #[doc = "Bit 1 - Frame new start interrupt clear (Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_fs_new(&mut self) -> IntClrFsNewW<IntrClear0Spec> {
        IntClrFsNewW::new(self, 1)
    }
    #[doc = "Bit 2 - memory start addr same interruption clear(Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_addr_same(&mut self) -> IntClrAddrSameW<IntrClear0Spec> {
        IntClrAddrSameW::new(self, 2)
    }
    #[doc = "Bit 3 - Line flag 0 Interrupt clear(Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_line_flag0(&mut self) -> IntClrLineFlag0W<IntrClear0Spec> {
        IntClrLineFlag0W::new(self, 3)
    }
    #[doc = "Bit 4 - Line flag 1 Interrupt clear(Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_line_flag1(&mut self) -> IntClrLineFlag1W<IntrClear0Spec> {
        IntClrLineFlag1W::new(self, 4)
    }
    #[doc = "Bit 5 - Bus error Interrupt clear(Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_bus_error(&mut self) -> IntClrBusErrorW<IntrClear0Spec> {
        IntClrBusErrorW::new(self, 5)
    }
    #[doc = "Bit 6 - win0 data empty interrupt clear(Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_win0_empty(&mut self) -> IntClrWin0EmptyW<IntrClear0Spec> {
        IntClrWin0EmptyW::new(self, 6)
    }
    #[doc = "Bit 7 - win1 data empty interrupt clear(Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_win1_empty(&mut self) -> IntClrWin1EmptyW<IntrClear0Spec> {
        IntClrWin1EmptyW::new(self, 7)
    }
    #[doc = "Bit 8 - win2 data empty interrupt clear(Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_win2_empty(&mut self) -> IntClrWin2EmptyW<IntrClear0Spec> {
        IntClrWin2EmptyW::new(self, 8)
    }
    #[doc = "Bit 9 - win3 data empty interrupt clear(Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_win3_empty(&mut self) -> IntClrWin3EmptyW<IntrClear0Spec> {
        IntClrWin3EmptyW::new(self, 9)
    }
    #[doc = "Bit 10 - hwc data empty interrupt clear(Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_hwc_empty(&mut self) -> IntClrHwcEmptyW<IntrClear0Spec> {
        IntClrHwcEmptyW::new(self, 10)
    }
    #[doc = "Bit 11 - post buffer empty interrupt clear(Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_post_buf_empty(&mut self) -> IntClrPostBufEmptyW<IntrClear0Spec> {
        IntClrPostBufEmptyW::new(self, 11)
    }
    #[doc = "Bit 12 - field start interrupt clear(Auto clear)\n\nField start interrupt clear (Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_fs_field(&mut self) -> IntClrFsFieldW<IntrClear0Spec> {
        IntClrFsFieldW::new(self, 12)
    }
    #[doc = "Bit 13 - display hold valid interrupt clear(Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_dsp_hold_valid(&mut self) -> IntClrDspHoldValidW<IntrClear0Spec> {
        IntClrDspHoldValidW::new(self, 13)
    }
    #[doc = "Bit 14 - mmu interrupt clear(Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_mmu(&mut self) -> IntClrMmuW<IntrClear0Spec> {
        IntClrMmuW::new(self, 14)
    }
    #[doc = "Bit 15 - dma finish interrupt clear(Auto clear)"]
    #[inline(always)]
    #[must_use]
    pub fn int_clr_dma_finish(&mut self) -> IntClrDmaFinishW<IntrClear0Spec> {
        IntClrDmaFinishW::new(self, 15)
    }
    #[doc = "Bits 16:31 - When every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<IntrClear0Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_clear0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_clear0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrClear0Spec;
impl crate::RegisterSpec for IntrClear0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_clear0::R`](R) reader structure"]
impl crate::Readable for IntrClear0Spec {}
#[doc = "`write(|w| ..)` method takes [`intr_clear0::W`](W) writer structure"]
impl crate::Writable for IntrClear0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3fff;
}
#[doc = "`reset()` method sets INTR_CLEAR0 to value 0"]
impl crate::Resettable for IntrClear0Spec {
    const RESET_VALUE: u32 = 0;
}
