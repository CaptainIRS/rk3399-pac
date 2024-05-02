#[doc = "Register `MI_ICR` writer"]
pub type W = crate::W<MiIcrSpec>;
#[doc = "Field `mp_frame_end` writer - Clear main picture end of frame interrupt\n\n"]
pub type MpFrameEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sp_frame_end` writer - Clear self picture end of frame interrupt"]
pub type SpFrameEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mblk_line` writer - Clear makroblock line interrupt"]
pub type MblkLineW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fill_mp_y` writer - Clear fill level interrupt"]
pub type FillMpYW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wrap_mp_y` writer - Clear main picture Y address wrap interrupt"]
pub type WrapMpYW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wrap_mp_cb` writer - Clear main picture Cb address wrap interrupt"]
pub type WrapMpCbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wrap_mp_cr` writer - Clear main picture Cr address wrap interrupt"]
pub type WrapMpCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wrap_sp_y` writer - Clear self picture Y address wrap interrupt"]
pub type WrapSpYW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wrap_sp_cb` writer - Clear self picture Cb address wrap interrupt"]
pub type WrapSpCbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wrap_sp_cr` writer - Clear self picture Cr address wrap interrupt"]
pub type WrapSpCrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dma_ready` writer - Clear dma ready interrupt"]
pub type DmaReadyW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear main picture end of frame interrupt\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn mp_frame_end(&mut self) -> MpFrameEndW<MiIcrSpec> {
        MpFrameEndW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear self picture end of frame interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sp_frame_end(&mut self) -> SpFrameEndW<MiIcrSpec> {
        SpFrameEndW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear makroblock line interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mblk_line(&mut self) -> MblkLineW<MiIcrSpec> {
        MblkLineW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear fill level interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fill_mp_y(&mut self) -> FillMpYW<MiIcrSpec> {
        FillMpYW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear main picture Y address wrap interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wrap_mp_y(&mut self) -> WrapMpYW<MiIcrSpec> {
        WrapMpYW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear main picture Cb address wrap interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wrap_mp_cb(&mut self) -> WrapMpCbW<MiIcrSpec> {
        WrapMpCbW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear main picture Cr address wrap interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wrap_mp_cr(&mut self) -> WrapMpCrW<MiIcrSpec> {
        WrapMpCrW::new(self, 6)
    }
    #[doc = "Bit 7 - Clear self picture Y address wrap interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wrap_sp_y(&mut self) -> WrapSpYW<MiIcrSpec> {
        WrapSpYW::new(self, 7)
    }
    #[doc = "Bit 8 - Clear self picture Cb address wrap interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wrap_sp_cb(&mut self) -> WrapSpCbW<MiIcrSpec> {
        WrapSpCbW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear self picture Cr address wrap interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wrap_sp_cr(&mut self) -> WrapSpCrW<MiIcrSpec> {
        WrapSpCrW::new(self, 9)
    }
    #[doc = "Bit 11 - Clear dma ready interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dma_ready(&mut self) -> DmaReadyW<MiIcrSpec> {
        DmaReadyW::new(self, 11)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiIcrSpec;
impl crate::RegisterSpec for MiIcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mi_icr::W`](W) writer structure"]
impl crate::Writable for MiIcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_ICR to value 0"]
impl crate::Resettable for MiIcrSpec {
    const RESET_VALUE: u32 = 0;
}
