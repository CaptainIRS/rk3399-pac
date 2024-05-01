#[doc = "Register `FADING_CTRL` reader"]
pub type R = crate::R<FadingCtrlSpec>;
#[doc = "Register `FADING_CTRL` writer"]
pub type W = crate::W<FadingCtrlSpec>;
#[doc = "Field `SW_FADING_OFFSET_R` reader - Fading offset R value\n\n(Start point of pattern ram in pattern mode)"]
pub type SwFadingOffsetRR = crate::FieldReader;
#[doc = "Field `SW_FADING_OFFSET_R` writer - Fading offset R value\n\n(Start point of pattern ram in pattern mode)"]
pub type SwFadingOffsetRW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_FADING_OFFSET_G` reader - Fading offset G value\n\n(Pattern total number when pattern loading)"]
pub type SwFadingOffsetGR = crate::FieldReader;
#[doc = "Field `SW_FADING_OFFSET_G` writer - Fading offset G value\n\n(Pattern total number when pattern loading)"]
pub type SwFadingOffsetGW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_FADING_OFFSET_B` reader - Fading offset B value"]
pub type SwFadingOffsetBR = crate::FieldReader;
#[doc = "Field `SW_FADING_OFFSET_B` writer - Fading offset B value"]
pub type SwFadingOffsetBW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_FADING_EN` reader - Fading enable"]
pub type SwFadingEnR = crate::BitReader;
#[doc = "Field `SW_FADING_EN` writer - Fading enable"]
pub type SwFadingEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Fading offset R value\n\n(Start point of pattern ram in pattern mode)"]
    #[inline(always)]
    pub fn sw_fading_offset_r(&self) -> SwFadingOffsetRR {
        SwFadingOffsetRR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fading offset G value\n\n(Pattern total number when pattern loading)"]
    #[inline(always)]
    pub fn sw_fading_offset_g(&self) -> SwFadingOffsetGR {
        SwFadingOffsetGR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Fading offset B value"]
    #[inline(always)]
    pub fn sw_fading_offset_b(&self) -> SwFadingOffsetBR {
        SwFadingOffsetBR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Fading enable"]
    #[inline(always)]
    pub fn sw_fading_en(&self) -> SwFadingEnR {
        SwFadingEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fading offset R value\n\n(Start point of pattern ram in pattern mode)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_fading_offset_r(&mut self) -> SwFadingOffsetRW<FadingCtrlSpec> {
        SwFadingOffsetRW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Fading offset G value\n\n(Pattern total number when pattern loading)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_fading_offset_g(&mut self) -> SwFadingOffsetGW<FadingCtrlSpec> {
        SwFadingOffsetGW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Fading offset B value"]
    #[inline(always)]
    #[must_use]
    pub fn sw_fading_offset_b(&mut self) -> SwFadingOffsetBW<FadingCtrlSpec> {
        SwFadingOffsetBW::new(self, 16)
    }
    #[doc = "Bit 24 - Fading enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_fading_en(&mut self) -> SwFadingEnW<FadingCtrlSpec> {
        SwFadingEnW::new(self, 24)
    }
}
#[doc = "Fading control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fading_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fading_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FadingCtrlSpec;
impl crate::RegisterSpec for FadingCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fading_ctrl::R`](R) reader structure"]
impl crate::Readable for FadingCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fading_ctrl::W`](W) writer structure"]
impl crate::Writable for FadingCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FADING_CTRL to value 0"]
impl crate::Resettable for FadingCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
