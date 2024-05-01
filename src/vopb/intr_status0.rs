#[doc = "Register `INTR_STATUS0` reader"]
pub type R = crate::R<IntrStatus0Spec>;
#[doc = "Register `INTR_STATUS0` writer"]
pub type W = crate::W<IntrStatus0Spec>;
#[doc = "Field `INT_STATUS_FS` reader - Frame start interrupt status"]
pub type IntStatusFsR = crate::BitReader;
#[doc = "Field `INT_STATUS_FS_NEW` reader - Frame start interrupt status(when memory start addr are\n\nsame,no interruption)"]
pub type IntStatusFsNewR = crate::BitReader;
#[doc = "Field `INT_STATUS_ADDR_SAME` reader - memory start addr same interruption status"]
pub type IntStatusAddrSameR = crate::BitReader;
#[doc = "Field `INT_STATUS_ADDR_SAME` writer - memory start addr same interruption status"]
pub type IntStatusAddrSameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_STATUS_LINE_FLAG0` reader - Line flag 0 Interrupt status"]
pub type IntStatusLineFlag0R = crate::BitReader;
#[doc = "Field `INT_STATUS_LINE_FLAG1` reader - Line flag 1 Interrupt status"]
pub type IntStatusLineFlag1R = crate::BitReader;
#[doc = "Field `INT_STATUS_BUS_ERROR` reader - Bus error Interrupt status"]
pub type IntStatusBusErrorR = crate::BitReader;
#[doc = "Field `INT_STATUS_WIN0_EMPTY` reader - win0 data empty interrupt status"]
pub type IntStatusWin0EmptyR = crate::BitReader;
#[doc = "Field `INT_STATUS_WIN1_EMPTY` reader - win1 data empty interrupt status"]
pub type IntStatusWin1EmptyR = crate::BitReader;
#[doc = "Field `INT_STATUS_WIN2_EMPTY` reader - win2 data empty interrupt status"]
pub type IntStatusWin2EmptyR = crate::BitReader;
#[doc = "Field `INT_STATUS_WIN3_EMPTY` reader - win3 data empty interrupt status"]
pub type IntStatusWin3EmptyR = crate::BitReader;
#[doc = "Field `INT_STATUS_HWC_EMPTY` reader - hwc data empty interrupt status"]
pub type IntStatusHwcEmptyR = crate::BitReader;
#[doc = "Field `INT_STATUS_POST_BUF_EMPTY` reader - post buffer empty interrupt status"]
pub type IntStatusPostBufEmptyR = crate::BitReader;
#[doc = "Field `INT_STATUS_FS_FIELD` reader - Field start interrupt status"]
pub type IntStatusFsFieldR = crate::BitReader;
#[doc = "Field `INT_STATUS_DSP_HOLD_VALID` reader - display hold valid interrupt status"]
pub type IntStatusDspHoldValidR = crate::BitReader;
#[doc = "Field `INT_STATUS_MMU` reader - mmu interrupt status"]
pub type IntStatusMmuR = crate::BitReader;
#[doc = "Field `INT_STATUS_MMU` writer - mmu interrupt status"]
pub type IntStatusMmuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_STATUS_DMA_FINISH` reader - dma finish interrupt status"]
pub type IntStatusDmaFinishR = crate::BitReader;
#[doc = "Field `INT_STATUS_DMA_FINISH` writer - dma finish interrupt status"]
pub type IntStatusDmaFinishW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Frame start interrupt status"]
    #[inline(always)]
    pub fn int_status_fs(&self) -> IntStatusFsR {
        IntStatusFsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame start interrupt status(when memory start addr are\n\nsame,no interruption)"]
    #[inline(always)]
    pub fn int_status_fs_new(&self) -> IntStatusFsNewR {
        IntStatusFsNewR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - memory start addr same interruption status"]
    #[inline(always)]
    pub fn int_status_addr_same(&self) -> IntStatusAddrSameR {
        IntStatusAddrSameR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Line flag 0 Interrupt status"]
    #[inline(always)]
    pub fn int_status_line_flag0(&self) -> IntStatusLineFlag0R {
        IntStatusLineFlag0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line flag 1 Interrupt status"]
    #[inline(always)]
    pub fn int_status_line_flag1(&self) -> IntStatusLineFlag1R {
        IntStatusLineFlag1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bus error Interrupt status"]
    #[inline(always)]
    pub fn int_status_bus_error(&self) -> IntStatusBusErrorR {
        IntStatusBusErrorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - win0 data empty interrupt status"]
    #[inline(always)]
    pub fn int_status_win0_empty(&self) -> IntStatusWin0EmptyR {
        IntStatusWin0EmptyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - win1 data empty interrupt status"]
    #[inline(always)]
    pub fn int_status_win1_empty(&self) -> IntStatusWin1EmptyR {
        IntStatusWin1EmptyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - win2 data empty interrupt status"]
    #[inline(always)]
    pub fn int_status_win2_empty(&self) -> IntStatusWin2EmptyR {
        IntStatusWin2EmptyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - win3 data empty interrupt status"]
    #[inline(always)]
    pub fn int_status_win3_empty(&self) -> IntStatusWin3EmptyR {
        IntStatusWin3EmptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - hwc data empty interrupt status"]
    #[inline(always)]
    pub fn int_status_hwc_empty(&self) -> IntStatusHwcEmptyR {
        IntStatusHwcEmptyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - post buffer empty interrupt status"]
    #[inline(always)]
    pub fn int_status_post_buf_empty(&self) -> IntStatusPostBufEmptyR {
        IntStatusPostBufEmptyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Field start interrupt status"]
    #[inline(always)]
    pub fn int_status_fs_field(&self) -> IntStatusFsFieldR {
        IntStatusFsFieldR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - display hold valid interrupt status"]
    #[inline(always)]
    pub fn int_status_dsp_hold_valid(&self) -> IntStatusDspHoldValidR {
        IntStatusDspHoldValidR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - mmu interrupt status"]
    #[inline(always)]
    pub fn int_status_mmu(&self) -> IntStatusMmuR {
        IntStatusMmuR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - dma finish interrupt status"]
    #[inline(always)]
    pub fn int_status_dma_finish(&self) -> IntStatusDmaFinishR {
        IntStatusDmaFinishR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - memory start addr same interruption status"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_addr_same(&mut self) -> IntStatusAddrSameW<IntrStatus0Spec> {
        IntStatusAddrSameW::new(self, 2)
    }
    #[doc = "Bit 14 - mmu interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_mmu(&mut self) -> IntStatusMmuW<IntrStatus0Spec> {
        IntStatusMmuW::new(self, 14)
    }
    #[doc = "Bit 15 - dma finish interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn int_status_dma_finish(&mut self) -> IntStatusDmaFinishW<IntrStatus0Spec> {
        IntStatusDmaFinishW::new(self, 15)
    }
}
#[doc = "interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_status0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_status0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrStatus0Spec;
impl crate::RegisterSpec for IntrStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_status0::R`](R) reader structure"]
impl crate::Readable for IntrStatus0Spec {}
#[doc = "`write(|w| ..)` method takes [`intr_status0::W`](W) writer structure"]
impl crate::Writable for IntrStatus0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_STATUS0 to value 0"]
impl crate::Resettable for IntrStatus0Spec {
    const RESET_VALUE: u32 = 0;
}
