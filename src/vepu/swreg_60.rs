#[doc = "Register `SWREG_60` reader"]
pub type R = crate::R<Swreg60Spec>;
#[doc = "Register `SWREG_60` writer"]
pub type W = crate::W<Swreg60Spec>;
#[doc = "Field `BOT_SPILL` reader - the bottom edge of image for spill pixels\n\nthe bottom edge of image for spill pixels"]
pub type BotSpillR = crate::FieldReader;
#[doc = "Field `BOT_SPILL` writer - the bottom edge of image for spill pixels\n\nthe bottom edge of image for spill pixels"]
pub type BotSpillW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RIGHT_SPILL` reader - the right edge of image for spill pixels\n\ndiv4 value\n\nrange:0~3"]
pub type RightSpillR = crate::FieldReader;
#[doc = "Field `RIGHT_SPILL` writer - the right edge of image for spill pixels\n\ndiv4 value\n\nrange:0~3"]
pub type RightSpillW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SKIP_MB_MODE` reader - H.264:SKIP macroblock mode"]
pub type SkipMbModeR = crate::FieldReader;
#[doc = "Field `SKIP_MB_MODE` writer - H.264:SKIP macroblock mode"]
pub type SkipMbModeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `STRM_ST_OFFSET` reader - the start offset for stream"]
pub type StrmStOffsetR = crate::FieldReader;
#[doc = "Field `STRM_ST_OFFSET` writer - the start offset for stream"]
pub type StrmStOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:3 - the bottom edge of image for spill pixels\n\nthe bottom edge of image for spill pixels"]
    #[inline(always)]
    pub fn bot_spill(&self) -> BotSpillR {
        BotSpillR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - the right edge of image for spill pixels\n\ndiv4 value\n\nrange:0~3"]
    #[inline(always)]
    pub fn right_spill(&self) -> RightSpillR {
        RightSpillR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:15 - H.264:SKIP macroblock mode"]
    #[inline(always)]
    pub fn skip_mb_mode(&self) -> SkipMbModeR {
        SkipMbModeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:21 - the start offset for stream"]
    #[inline(always)]
    pub fn strm_st_offset(&self) -> StrmStOffsetR {
        StrmStOffsetR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - the bottom edge of image for spill pixels\n\nthe bottom edge of image for spill pixels"]
    #[inline(always)]
    #[must_use]
    pub fn bot_spill(&mut self) -> BotSpillW<Swreg60Spec> {
        BotSpillW::new(self, 0)
    }
    #[doc = "Bits 4:5 - the right edge of image for spill pixels\n\ndiv4 value\n\nrange:0~3"]
    #[inline(always)]
    #[must_use]
    pub fn right_spill(&mut self) -> RightSpillW<Swreg60Spec> {
        RightSpillW::new(self, 4)
    }
    #[doc = "Bits 8:15 - H.264:SKIP macroblock mode"]
    #[inline(always)]
    #[must_use]
    pub fn skip_mb_mode(&mut self) -> SkipMbModeW<Swreg60Spec> {
        SkipMbModeW::new(self, 8)
    }
    #[doc = "Bits 16:21 - the start offset for stream"]
    #[inline(always)]
    #[must_use]
    pub fn strm_st_offset(&mut self) -> StrmStOffsetW<Swreg60Spec> {
        StrmStOffsetW::new(self, 16)
    }
}
#[doc = "Register0001 Abstract\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_60::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_60::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg60Spec;
impl crate::RegisterSpec for Swreg60Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_60::R`](R) reader structure"]
impl crate::Readable for Swreg60Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_60::W`](W) writer structure"]
impl crate::Writable for Swreg60Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_60 to value 0"]
impl crate::Resettable for Swreg60Spec {
    const RESET_VALUE: u32 = 0;
}
