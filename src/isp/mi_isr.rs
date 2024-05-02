#[doc = "Register `MI_ISR` writer"]
pub type W = crate::W<MiIsrSpec>;
#[doc = "Field `mp_frame_end` writer - Set main picture end of frame interrupt\n\n"]
pub type MpFrameEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_frame_end` writer - Set self picture end of frame interrupt"]
pub type SpFrameEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mblk_line` writer - Set makroblock line interrupt"]
pub type MblkLineW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fill_mp_y` writer - Set fill level interrupt"]
pub type FillMpYW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wrap_mp_y` writer - Set main picture Y address wrap interrupt"]
pub type WrapMpYW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wrap_mp_cb` writer - Set main picture Cb address wrap interrupt\n\n"]
pub type WrapMpCbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wrap_mp_cr` writer - Set main picture Cr address wrap interrupt"]
pub type WrapMpCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wrap_sp_y` writer - Set self picture Y address wrap interrupt"]
pub type WrapSpYW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wrap_sp_cb` writer - Set self picture Cb address wrap interrupt"]
pub type WrapSpCbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wrap_sp_cr` writer - Set self picture Cr address wrap interrupt"]
pub type WrapSpCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma_ready` writer - Set dma ready interrupt"]
pub type DmaReadyW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set main picture end of frame interrupt\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn mp_frame_end(&mut self) -> MpFrameEndW<MiIsrSpec> {
        MpFrameEndW::new(self, 0)
    }
    #[doc = "Bit 1 - Set self picture end of frame interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sp_frame_end(&mut self) -> SpFrameEndW<MiIsrSpec> {
        SpFrameEndW::new(self, 1)
    }
    #[doc = "Bit 2 - Set makroblock line interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mblk_line(&mut self) -> MblkLineW<MiIsrSpec> {
        MblkLineW::new(self, 2)
    }
    #[doc = "Bit 3 - Set fill level interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fill_mp_y(&mut self) -> FillMpYW<MiIsrSpec> {
        FillMpYW::new(self, 3)
    }
    #[doc = "Bit 4 - Set main picture Y address wrap interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wrap_mp_y(&mut self) -> WrapMpYW<MiIsrSpec> {
        WrapMpYW::new(self, 4)
    }
    #[doc = "Bit 5 - Set main picture Cb address wrap interrupt\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn wrap_mp_cb(&mut self) -> WrapMpCbW<MiIsrSpec> {
        WrapMpCbW::new(self, 5)
    }
    #[doc = "Bit 6 - Set main picture Cr address wrap interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wrap_mp_cr(&mut self) -> WrapMpCrW<MiIsrSpec> {
        WrapMpCrW::new(self, 6)
    }
    #[doc = "Bit 7 - Set self picture Y address wrap interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wrap_sp_y(&mut self) -> WrapSpYW<MiIsrSpec> {
        WrapSpYW::new(self, 7)
    }
    #[doc = "Bit 8 - Set self picture Cb address wrap interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wrap_sp_cb(&mut self) -> WrapSpCbW<MiIsrSpec> {
        WrapSpCbW::new(self, 8)
    }
    #[doc = "Bit 9 - Set self picture Cr address wrap interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wrap_sp_cr(&mut self) -> WrapSpCrW<MiIsrSpec> {
        WrapSpCrW::new(self, 9)
    }
    #[doc = "Bit 11 - Set dma ready interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dma_ready(&mut self) -> DmaReadyW<MiIsrSpec> {
        DmaReadyW::new(self, 11)
    }
}
#[doc = "Interrupt Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_isr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiIsrSpec;
impl crate::RegisterSpec for MiIsrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mi_isr::W`](W) writer structure"]
impl crate::Writable for MiIsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_ISR to value 0"]
impl crate::Resettable for MiIsrSpec {
    const RESET_VALUE: u32 = 0;
}
