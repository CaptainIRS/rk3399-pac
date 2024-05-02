#[doc = "Register `MI_MIS` reader"]
pub type R = crate::R<MiMisSpec>;
#[doc = "Field `mp_frame_end` reader - Masked status of main picture end of frame interrupt"]
pub type MpFrameEndR = crate::BitReader;
#[doc = "Field `sp_frame_end` reader - Masked status of self picture end of frame interrupt"]
pub type SpFrameEndR = crate::BitReader;
#[doc = "Field `mblk_line` reader - Masked status of makroblock line interrupt of main\n\npicture (16 lines of Y, 8 lines of Cb and 8 lines of Cr are\n\nwritten into RAM, valid only for planar and semi-planar\n\nmode)"]
pub type MblkLineR = crate::BitReader;
#[doc = "Field `fill_mp_y` reader - Masked status of fill level interrupt of main picture\n\nY, JPEG or raw data"]
pub type FillMpYR = crate::BitReader;
#[doc = "Field `wrap_mp_y` reader - Masked status of main picture Y address wrap\n\ninterrupt"]
pub type WrapMpYR = crate::BitReader;
#[doc = "Field `wrap_mp_cb` reader - Masked status of main picture Cb address wrap\n\ninterrupt"]
pub type WrapMpCbR = crate::BitReader;
#[doc = "Field `wrap_mp_cr` reader - Masked status of main picture Cr address wrap\n\ninterrupt"]
pub type WrapMpCrR = crate::BitReader;
#[doc = "Field `wrap_sp_y` reader - Masked status of self picture Y address wrap interrupt"]
pub type WrapSpYR = crate::BitReader;
#[doc = "Field `wrap_sp_cb` reader - Masked status of self picture Cb address wrap\n\ninterrupt"]
pub type WrapSpCbR = crate::BitReader;
#[doc = "Field `wrap_sp_cr` reader - Masked status of self picture Cr address wrap interrupt"]
pub type WrapSpCrR = crate::BitReader;
#[doc = "Field `dma_ready` reader - Masked status of dma ready interrupt"]
pub type DmaReadyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Masked status of main picture end of frame interrupt"]
    #[inline(always)]
    pub fn mp_frame_end(&self) -> MpFrameEndR {
        MpFrameEndR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masked status of self picture end of frame interrupt"]
    #[inline(always)]
    pub fn sp_frame_end(&self) -> SpFrameEndR {
        SpFrameEndR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masked status of makroblock line interrupt of main\n\npicture (16 lines of Y, 8 lines of Cb and 8 lines of Cr are\n\nwritten into RAM, valid only for planar and semi-planar\n\nmode)"]
    #[inline(always)]
    pub fn mblk_line(&self) -> MblkLineR {
        MblkLineR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masked status of fill level interrupt of main picture\n\nY, JPEG or raw data"]
    #[inline(always)]
    pub fn fill_mp_y(&self) -> FillMpYR {
        FillMpYR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Masked status of main picture Y address wrap\n\ninterrupt"]
    #[inline(always)]
    pub fn wrap_mp_y(&self) -> WrapMpYR {
        WrapMpYR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Masked status of main picture Cb address wrap\n\ninterrupt"]
    #[inline(always)]
    pub fn wrap_mp_cb(&self) -> WrapMpCbR {
        WrapMpCbR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Masked status of main picture Cr address wrap\n\ninterrupt"]
    #[inline(always)]
    pub fn wrap_mp_cr(&self) -> WrapMpCrR {
        WrapMpCrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Masked status of self picture Y address wrap interrupt"]
    #[inline(always)]
    pub fn wrap_sp_y(&self) -> WrapSpYR {
        WrapSpYR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Masked status of self picture Cb address wrap\n\ninterrupt"]
    #[inline(always)]
    pub fn wrap_sp_cb(&self) -> WrapSpCbR {
        WrapSpCbR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Masked status of self picture Cr address wrap interrupt"]
    #[inline(always)]
    pub fn wrap_sp_cr(&self) -> WrapSpCrR {
        WrapSpCrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Masked status of dma ready interrupt"]
    #[inline(always)]
    pub fn dma_ready(&self) -> DmaReadyR {
        DmaReadyR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiMisSpec;
impl crate::RegisterSpec for MiMisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_mis::R`](R) reader structure"]
impl crate::Readable for MiMisSpec {}
#[doc = "`reset()` method sets MI_MIS to value 0"]
impl crate::Resettable for MiMisSpec {
    const RESET_VALUE: u32 = 0;
}
