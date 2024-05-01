#[doc = "Register `INTR_RAW_STATUS0` reader"]
pub type R = crate::R<IntrRawStatus0Spec>;
#[doc = "Field `INT_RAW_STATUS_FS` reader - Frame start raw interrupt status\n\nFrame start raw interrupt status"]
pub type IntRawStatusFsR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_FS_NEW` reader - Frame start interrupt raw status(when memory start addr are\n\nsame)"]
pub type IntRawStatusFsNewR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_ADDR_SAME` reader - memory start addr same interruption raw status"]
pub type IntRawStatusAddrSameR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_LINE_FRAG0` reader - Line flag 0 Interrupt raw status"]
pub type IntRawStatusLineFrag0R = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_LINE_FRAG1` reader - Line flag 1 Interrupt raw status"]
pub type IntRawStatusLineFrag1R = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_BUS_ERROR` reader - Bus error Interrupt raw status"]
pub type IntRawStatusBusErrorR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_WIN0_EMPTY` reader - win0 data empty interrupt raw status"]
pub type IntRawStatusWin0EmptyR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_WIN1_EMPTY` reader - win1 data empty interrupt raw status"]
pub type IntRawStatusWin1EmptyR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_WIN2_EMPTY` reader - win2 data empty interrupt raw status"]
pub type IntRawStatusWin2EmptyR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_WIN3_EMPTY` reader - win3 data empty interrupt raw status"]
pub type IntRawStatusWin3EmptyR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_HWC_EMPTY` reader - hwc data empty interrupt raw status"]
pub type IntRawStatusHwcEmptyR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_POST_BUF_EMPTY` reader - post buffer empty interrupt raw status"]
pub type IntRawStatusPostBufEmptyR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_FS_FIELD` reader - Field start raw interrupt status"]
pub type IntRawStatusFsFieldR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_DSP_HOLD_VALID` reader - display hold valid interrupt raw status"]
pub type IntRawStatusDspHoldValidR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_MMU` reader - mmu interrupt raw status"]
pub type IntRawStatusMmuR = crate::BitReader;
#[doc = "Field `INT_RAW_STATUS_DMA_FINISH` reader - dma finish interrupt raw status"]
pub type IntRawStatusDmaFinishR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Frame start raw interrupt status\n\nFrame start raw interrupt status"]
    #[inline(always)]
    pub fn int_raw_status_fs(&self) -> IntRawStatusFsR {
        IntRawStatusFsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame start interrupt raw status(when memory start addr are\n\nsame)"]
    #[inline(always)]
    pub fn int_raw_status_fs_new(&self) -> IntRawStatusFsNewR {
        IntRawStatusFsNewR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - memory start addr same interruption raw status"]
    #[inline(always)]
    pub fn int_raw_status_addr_same(&self) -> IntRawStatusAddrSameR {
        IntRawStatusAddrSameR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Line flag 0 Interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_line_frag0(&self) -> IntRawStatusLineFrag0R {
        IntRawStatusLineFrag0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line flag 1 Interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_line_frag1(&self) -> IntRawStatusLineFrag1R {
        IntRawStatusLineFrag1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bus error Interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_bus_error(&self) -> IntRawStatusBusErrorR {
        IntRawStatusBusErrorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - win0 data empty interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_win0_empty(&self) -> IntRawStatusWin0EmptyR {
        IntRawStatusWin0EmptyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - win1 data empty interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_win1_empty(&self) -> IntRawStatusWin1EmptyR {
        IntRawStatusWin1EmptyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - win2 data empty interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_win2_empty(&self) -> IntRawStatusWin2EmptyR {
        IntRawStatusWin2EmptyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - win3 data empty interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_win3_empty(&self) -> IntRawStatusWin3EmptyR {
        IntRawStatusWin3EmptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - hwc data empty interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_hwc_empty(&self) -> IntRawStatusHwcEmptyR {
        IntRawStatusHwcEmptyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - post buffer empty interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_post_buf_empty(&self) -> IntRawStatusPostBufEmptyR {
        IntRawStatusPostBufEmptyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Field start raw interrupt status"]
    #[inline(always)]
    pub fn int_raw_status_fs_field(&self) -> IntRawStatusFsFieldR {
        IntRawStatusFsFieldR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - display hold valid interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_dsp_hold_valid(&self) -> IntRawStatusDspHoldValidR {
        IntRawStatusDspHoldValidR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - mmu interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_mmu(&self) -> IntRawStatusMmuR {
        IntRawStatusMmuR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - dma finish interrupt raw status"]
    #[inline(always)]
    pub fn int_raw_status_dma_finish(&self) -> IntRawStatusDmaFinishR {
        IntRawStatusDmaFinishR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_raw_status0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrRawStatus0Spec;
impl crate::RegisterSpec for IntrRawStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_raw_status0::R`](R) reader structure"]
impl crate::Readable for IntrRawStatus0Spec {}
#[doc = "`reset()` method sets INTR_RAW_STATUS0 to value 0"]
impl crate::Resettable for IntrRawStatus0Spec {
    const RESET_VALUE: u32 = 0;
}
