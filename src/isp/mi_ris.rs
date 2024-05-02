#[doc = "Register `MI_RIS` reader"]
pub type R = crate::R<MiRisSpec>;
#[doc = "Field `mp_frame_end` reader - Raw status of main picture end of frame interrupt\n\n"]
pub type MpFrameEndR = crate::BitReader;
#[doc = "Field `sp_frame_end` reader - Raw status of self picture end of frame interrupt"]
pub type SpFrameEndR = crate::BitReader;
#[doc = "Field `mblk_line` reader - Raw status of makroblock line interrupt of main\n\npicture (16 lines of Y, 8 lines of Cb and 8 lines of Cr are\n\nwritten into RAM, valid only for planar and semi-planar\n\nmode)"]
pub type MblkLineR = crate::BitReader;
#[doc = "Field `fill_mp_y` reader - Raw status of fill level interrupt of main picture Y,\n\nJPEG or raw data"]
pub type FillMpYR = crate::BitReader;
#[doc = "Field `wrap_mp_y` reader - Raw status of main picture Y address wrap interrupt\n\n"]
pub type WrapMpYR = crate::BitReader;
#[doc = "Field `wrap_mp_cb` reader - Raw status of main picture Cb address wrap interrupt"]
pub type WrapMpCbR = crate::BitReader;
#[doc = "Field `wrap_mp_cr` reader - Raw status of main picture Cr address wrap interrupt"]
pub type WrapMpCrR = crate::BitReader;
#[doc = "Field `wrap_sp_y` reader - Raw status of self picture Y address wrap interrupt"]
pub type WrapSpYR = crate::BitReader;
#[doc = "Field `wrap_sp_cb` reader - Raw status of self picture Cb address wrap interrupt"]
pub type WrapSpCbR = crate::BitReader;
#[doc = "Field `wrap_sp_cr` reader - Raw status of self picture Cr address wrap interrupt"]
pub type WrapSpCrR = crate::BitReader;
#[doc = "Field `dma_ready` reader - Raw status of dma ready interrupt"]
pub type DmaReadyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Raw status of main picture end of frame interrupt\n\n"]
    #[inline(always)]
    pub fn mp_frame_end(&self) -> MpFrameEndR {
        MpFrameEndR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw status of self picture end of frame interrupt"]
    #[inline(always)]
    pub fn sp_frame_end(&self) -> SpFrameEndR {
        SpFrameEndR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw status of makroblock line interrupt of main\n\npicture (16 lines of Y, 8 lines of Cb and 8 lines of Cr are\n\nwritten into RAM, valid only for planar and semi-planar\n\nmode)"]
    #[inline(always)]
    pub fn mblk_line(&self) -> MblkLineR {
        MblkLineR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw status of fill level interrupt of main picture Y,\n\nJPEG or raw data"]
    #[inline(always)]
    pub fn fill_mp_y(&self) -> FillMpYR {
        FillMpYR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Raw status of main picture Y address wrap interrupt\n\n"]
    #[inline(always)]
    pub fn wrap_mp_y(&self) -> WrapMpYR {
        WrapMpYR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Raw status of main picture Cb address wrap interrupt"]
    #[inline(always)]
    pub fn wrap_mp_cb(&self) -> WrapMpCbR {
        WrapMpCbR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Raw status of main picture Cr address wrap interrupt"]
    #[inline(always)]
    pub fn wrap_mp_cr(&self) -> WrapMpCrR {
        WrapMpCrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Raw status of self picture Y address wrap interrupt"]
    #[inline(always)]
    pub fn wrap_sp_y(&self) -> WrapSpYR {
        WrapSpYR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Raw status of self picture Cb address wrap interrupt"]
    #[inline(always)]
    pub fn wrap_sp_cb(&self) -> WrapSpCbR {
        WrapSpCbR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Raw status of self picture Cr address wrap interrupt"]
    #[inline(always)]
    pub fn wrap_sp_cr(&self) -> WrapSpCrR {
        WrapSpCrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Raw status of dma ready interrupt"]
    #[inline(always)]
    pub fn dma_ready(&self) -> DmaReadyR {
        DmaReadyR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiRisSpec;
impl crate::RegisterSpec for MiRisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_ris::R`](R) reader structure"]
impl crate::Readable for MiRisSpec {}
#[doc = "`reset()` method sets MI_RIS to value 0"]
impl crate::Resettable for MiRisSpec {
    const RESET_VALUE: u32 = 0;
}
