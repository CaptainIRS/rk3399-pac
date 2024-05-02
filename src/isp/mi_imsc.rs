#[doc = "Register `MI_IMSC` reader"]
pub type R = crate::R<MiImscSpec>;
#[doc = "Register `MI_IMSC` writer"]
pub type W = crate::W<MiImscSpec>;
#[doc = "Field `mp_frame_end` reader - Mask main picture end of frame interrupt\n\n"]
pub type MpFrameEndR = crate::BitReader;
#[doc = "Field `mp_frame_end` writer - Mask main picture end of frame interrupt\n\n"]
pub type MpFrameEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_frame_end` reader - Mask self picture end of frame interrupt"]
pub type SpFrameEndR = crate::BitReader;
#[doc = "Field `sp_frame_end` writer - Mask self picture end of frame interrupt"]
pub type SpFrameEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mblk_line` reader - Mask bit for makroblock line interrupt of main picture\n\n(16 lines of Y, 8 lines of Cb and 8 lines of Cr are written\n\ninto RAM)"]
pub type MblkLineR = crate::BitReader;
#[doc = "Field `mblk_line` writer - Mask bit for makroblock line interrupt of main picture\n\n(16 lines of Y, 8 lines of Cb and 8 lines of Cr are written\n\ninto RAM)"]
pub type MblkLineW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fill_mp_y` reader - Mask bit for fill level interrupt of main picture Y,\n\nJPEG or raw data"]
pub type FillMpYR = crate::BitReader;
#[doc = "Field `fill_mp_y` writer - Mask bit for fill level interrupt of main picture Y,\n\nJPEG or raw data"]
pub type FillMpYW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wrap_mp_y` reader - Mask bit for main picture Y address wrap interrupt"]
pub type WrapMpYR = crate::BitReader;
#[doc = "Field `wrap_mp_y` writer - Mask bit for main picture Y address wrap interrupt"]
pub type WrapMpYW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wrap_mp_cb` reader - Mask bit for main picture Cb address wrap interrupt"]
pub type WrapMpCbR = crate::BitReader;
#[doc = "Field `wrap_mp_cb` writer - Mask bit for main picture Cb address wrap interrupt"]
pub type WrapMpCbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wrap_mp_cr` reader - Mask bit for main picture Cr address wrap interrupt"]
pub type WrapMpCrR = crate::BitReader;
#[doc = "Field `wrap_mp_cr` writer - Mask bit for main picture Cr address wrap interrupt"]
pub type WrapMpCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wrap_sp_y` reader - Mask bit for self picture Y address wrap interrupt"]
pub type WrapSpYR = crate::BitReader;
#[doc = "Field `wrap_sp_y` writer - Mask bit for self picture Y address wrap interrupt"]
pub type WrapSpYW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wrap_sp_cb` reader - Mask bit for self picture Cb address wrap interrupt"]
pub type WrapSpCbR = crate::BitReader;
#[doc = "Field `wrap_sp_cb` writer - Mask bit for self picture Cb address wrap interrupt"]
pub type WrapSpCbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wrap_sp_cr` reader - Mask bit for self picture Cr address wrap interrupt"]
pub type WrapSpCrR = crate::BitReader;
#[doc = "Field `wrap_sp_cr` writer - Mask bit for self picture Cr address wrap interrupt"]
pub type WrapSpCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma_ready` reader - Mask bit for dma ready interrupt"]
pub type DmaReadyR = crate::BitReader;
#[doc = "Field `dma_ready` writer - Mask bit for dma ready interrupt"]
pub type DmaReadyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask main picture end of frame interrupt\n\n"]
    #[inline(always)]
    pub fn mp_frame_end(&self) -> MpFrameEndR {
        MpFrameEndR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask self picture end of frame interrupt"]
    #[inline(always)]
    pub fn sp_frame_end(&self) -> SpFrameEndR {
        SpFrameEndR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask bit for makroblock line interrupt of main picture\n\n(16 lines of Y, 8 lines of Cb and 8 lines of Cr are written\n\ninto RAM)"]
    #[inline(always)]
    pub fn mblk_line(&self) -> MblkLineR {
        MblkLineR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask bit for fill level interrupt of main picture Y,\n\nJPEG or raw data"]
    #[inline(always)]
    pub fn fill_mp_y(&self) -> FillMpYR {
        FillMpYR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask bit for main picture Y address wrap interrupt"]
    #[inline(always)]
    pub fn wrap_mp_y(&self) -> WrapMpYR {
        WrapMpYR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask bit for main picture Cb address wrap interrupt"]
    #[inline(always)]
    pub fn wrap_mp_cb(&self) -> WrapMpCbR {
        WrapMpCbR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask bit for main picture Cr address wrap interrupt"]
    #[inline(always)]
    pub fn wrap_mp_cr(&self) -> WrapMpCrR {
        WrapMpCrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask bit for self picture Y address wrap interrupt"]
    #[inline(always)]
    pub fn wrap_sp_y(&self) -> WrapSpYR {
        WrapSpYR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask bit for self picture Cb address wrap interrupt"]
    #[inline(always)]
    pub fn wrap_sp_cb(&self) -> WrapSpCbR {
        WrapSpCbR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask bit for self picture Cr address wrap interrupt"]
    #[inline(always)]
    pub fn wrap_sp_cr(&self) -> WrapSpCrR {
        WrapSpCrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Mask bit for dma ready interrupt"]
    #[inline(always)]
    pub fn dma_ready(&self) -> DmaReadyR {
        DmaReadyR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask main picture end of frame interrupt\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn mp_frame_end(&mut self) -> MpFrameEndW<MiImscSpec> {
        MpFrameEndW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask self picture end of frame interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sp_frame_end(&mut self) -> SpFrameEndW<MiImscSpec> {
        SpFrameEndW::new(self, 1)
    }
    #[doc = "Bit 2 - Mask bit for makroblock line interrupt of main picture\n\n(16 lines of Y, 8 lines of Cb and 8 lines of Cr are written\n\ninto RAM)"]
    #[inline(always)]
    #[must_use]
    pub fn mblk_line(&mut self) -> MblkLineW<MiImscSpec> {
        MblkLineW::new(self, 2)
    }
    #[doc = "Bit 3 - Mask bit for fill level interrupt of main picture Y,\n\nJPEG or raw data"]
    #[inline(always)]
    #[must_use]
    pub fn fill_mp_y(&mut self) -> FillMpYW<MiImscSpec> {
        FillMpYW::new(self, 3)
    }
    #[doc = "Bit 4 - Mask bit for main picture Y address wrap interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wrap_mp_y(&mut self) -> WrapMpYW<MiImscSpec> {
        WrapMpYW::new(self, 4)
    }
    #[doc = "Bit 5 - Mask bit for main picture Cb address wrap interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wrap_mp_cb(&mut self) -> WrapMpCbW<MiImscSpec> {
        WrapMpCbW::new(self, 5)
    }
    #[doc = "Bit 6 - Mask bit for main picture Cr address wrap interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wrap_mp_cr(&mut self) -> WrapMpCrW<MiImscSpec> {
        WrapMpCrW::new(self, 6)
    }
    #[doc = "Bit 7 - Mask bit for self picture Y address wrap interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wrap_sp_y(&mut self) -> WrapSpYW<MiImscSpec> {
        WrapSpYW::new(self, 7)
    }
    #[doc = "Bit 8 - Mask bit for self picture Cb address wrap interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wrap_sp_cb(&mut self) -> WrapSpCbW<MiImscSpec> {
        WrapSpCbW::new(self, 8)
    }
    #[doc = "Bit 9 - Mask bit for self picture Cr address wrap interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wrap_sp_cr(&mut self) -> WrapSpCrW<MiImscSpec> {
        WrapSpCrW::new(self, 9)
    }
    #[doc = "Bit 11 - Mask bit for dma ready interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dma_ready(&mut self) -> DmaReadyW<MiImscSpec> {
        DmaReadyW::new(self, 11)
    }
}
#[doc = "Interrupt Mask („1‟: interrupt active, „0‟: interrupt masked)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_imsc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_imsc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiImscSpec;
impl crate::RegisterSpec for MiImscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_imsc::R`](R) reader structure"]
impl crate::Readable for MiImscSpec {}
#[doc = "`write(|w| ..)` method takes [`mi_imsc::W`](W) writer structure"]
impl crate::Writable for MiImscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_IMSC to value 0"]
impl crate::Resettable for MiImscSpec {
    const RESET_VALUE: u32 = 0;
}
