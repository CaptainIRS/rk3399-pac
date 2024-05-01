#[doc = "Register `WIN0_FADING_CTRL` reader"]
pub type R = crate::R<Win0FadingCtrlSpec>;
#[doc = "Register `WIN0_FADING_CTRL` writer"]
pub type W = crate::W<Win0FadingCtrlSpec>;
#[doc = "Field `LAYER0_FADING_OFFSET_R` reader - fading offset red value"]
pub type Layer0FadingOffsetRR = crate::FieldReader;
#[doc = "Field `LAYER0_FADING_OFFSET_R` writer - fading offset red value"]
pub type Layer0FadingOffsetRW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LAYER0_FADING_OFFSET_G` reader - fading offset green value"]
pub type Layer0FadingOffsetGR = crate::FieldReader;
#[doc = "Field `LAYER0_FADING_OFFSET_G` writer - fading offset green value"]
pub type Layer0FadingOffsetGW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LAYER0_FADING_OFFSET_B` reader - fading offset blue value"]
pub type Layer0FadingOffsetBR = crate::FieldReader;
#[doc = "Field `LAYER0_FADING_OFFSET_B` writer - fading offset blue value"]
pub type Layer0FadingOffsetBW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LAYER0_FADING_EN` reader - fading enable"]
pub type Layer0FadingEnR = crate::BitReader;
#[doc = "Field `LAYER0_FADING_EN` writer - fading enable"]
pub type Layer0FadingEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - fading offset red value"]
    #[inline(always)]
    pub fn layer0_fading_offset_r(&self) -> Layer0FadingOffsetRR {
        Layer0FadingOffsetRR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - fading offset green value"]
    #[inline(always)]
    pub fn layer0_fading_offset_g(&self) -> Layer0FadingOffsetGR {
        Layer0FadingOffsetGR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - fading offset blue value"]
    #[inline(always)]
    pub fn layer0_fading_offset_b(&self) -> Layer0FadingOffsetBR {
        Layer0FadingOffsetBR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - fading enable"]
    #[inline(always)]
    pub fn layer0_fading_en(&self) -> Layer0FadingEnR {
        Layer0FadingEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - fading offset red value"]
    #[inline(always)]
    #[must_use]
    pub fn layer0_fading_offset_r(&mut self) -> Layer0FadingOffsetRW<Win0FadingCtrlSpec> {
        Layer0FadingOffsetRW::new(self, 0)
    }
    #[doc = "Bits 8:15 - fading offset green value"]
    #[inline(always)]
    #[must_use]
    pub fn layer0_fading_offset_g(&mut self) -> Layer0FadingOffsetGW<Win0FadingCtrlSpec> {
        Layer0FadingOffsetGW::new(self, 8)
    }
    #[doc = "Bits 16:23 - fading offset blue value"]
    #[inline(always)]
    #[must_use]
    pub fn layer0_fading_offset_b(&mut self) -> Layer0FadingOffsetBW<Win0FadingCtrlSpec> {
        Layer0FadingOffsetBW::new(self, 16)
    }
    #[doc = "Bit 24 - fading enable"]
    #[inline(always)]
    #[must_use]
    pub fn layer0_fading_en(&mut self) -> Layer0FadingEnW<Win0FadingCtrlSpec> {
        Layer0FadingEnW::new(self, 24)
    }
}
#[doc = "Win0 fading contrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_fading_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_fading_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win0FadingCtrlSpec;
impl crate::RegisterSpec for Win0FadingCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win0_fading_ctrl::R`](R) reader structure"]
impl crate::Readable for Win0FadingCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`win0_fading_ctrl::W`](W) writer structure"]
impl crate::Writable for Win0FadingCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN0_FADING_CTRL to value 0"]
impl crate::Resettable for Win0FadingCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
