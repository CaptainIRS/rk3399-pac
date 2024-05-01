#[doc = "Register `WIN1_FADING_CTRL` reader"]
pub type R = crate::R<Win1FadingCtrlSpec>;
#[doc = "Register `WIN1_FADING_CTRL` writer"]
pub type W = crate::W<Win1FadingCtrlSpec>;
#[doc = "Field `WIN1_FADING_OFFSET_R` reader - fading offset red value"]
pub type Win1FadingOffsetRR = crate::FieldReader;
#[doc = "Field `WIN1_FADING_OFFSET_R` writer - fading offset red value"]
pub type Win1FadingOffsetRW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN1_FADING_OFFSET_G` reader - fading offset green value"]
pub type Win1FadingOffsetGR = crate::FieldReader;
#[doc = "Field `WIN1_FADING_OFFSET_G` writer - fading offset green value"]
pub type Win1FadingOffsetGW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN1_FADING_OFFSET_B` reader - fading offset blue value"]
pub type Win1FadingOffsetBR = crate::FieldReader;
#[doc = "Field `WIN1_FADING_OFFSET_B` writer - fading offset blue value"]
pub type Win1FadingOffsetBW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN1_FADING_EN` reader - fading enable"]
pub type Win1FadingEnR = crate::BitReader;
#[doc = "Field `WIN1_FADING_EN` writer - fading enable"]
pub type Win1FadingEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - fading offset red value"]
    #[inline(always)]
    pub fn win1_fading_offset_r(&self) -> Win1FadingOffsetRR {
        Win1FadingOffsetRR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - fading offset green value"]
    #[inline(always)]
    pub fn win1_fading_offset_g(&self) -> Win1FadingOffsetGR {
        Win1FadingOffsetGR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - fading offset blue value"]
    #[inline(always)]
    pub fn win1_fading_offset_b(&self) -> Win1FadingOffsetBR {
        Win1FadingOffsetBR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - fading enable"]
    #[inline(always)]
    pub fn win1_fading_en(&self) -> Win1FadingEnR {
        Win1FadingEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - fading offset red value"]
    #[inline(always)]
    #[must_use]
    pub fn win1_fading_offset_r(&mut self) -> Win1FadingOffsetRW<Win1FadingCtrlSpec> {
        Win1FadingOffsetRW::new(self, 0)
    }
    #[doc = "Bits 8:15 - fading offset green value"]
    #[inline(always)]
    #[must_use]
    pub fn win1_fading_offset_g(&mut self) -> Win1FadingOffsetGW<Win1FadingCtrlSpec> {
        Win1FadingOffsetGW::new(self, 8)
    }
    #[doc = "Bits 16:23 - fading offset blue value"]
    #[inline(always)]
    #[must_use]
    pub fn win1_fading_offset_b(&mut self) -> Win1FadingOffsetBW<Win1FadingCtrlSpec> {
        Win1FadingOffsetBW::new(self, 16)
    }
    #[doc = "Bit 24 - fading enable"]
    #[inline(always)]
    #[must_use]
    pub fn win1_fading_en(&mut self) -> Win1FadingEnW<Win1FadingCtrlSpec> {
        Win1FadingEnW::new(self, 24)
    }
}
#[doc = "Win1 fading contrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_fading_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_fading_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win1FadingCtrlSpec;
impl crate::RegisterSpec for Win1FadingCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win1_fading_ctrl::R`](R) reader structure"]
impl crate::Readable for Win1FadingCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`win1_fading_ctrl::W`](W) writer structure"]
impl crate::Writable for Win1FadingCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN1_FADING_CTRL to value 0"]
impl crate::Resettable for Win1FadingCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
