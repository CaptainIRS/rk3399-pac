#[doc = "Register `WIN2_FADING_CTRL` reader"]
pub type R = crate::R<Win2FadingCtrlSpec>;
#[doc = "Register `WIN2_FADING_CTRL` writer"]
pub type W = crate::W<Win2FadingCtrlSpec>;
#[doc = "Field `WIN2_FADING_OFFSET_R` reader - fading offset red"]
pub type Win2FadingOffsetRR = crate::FieldReader;
#[doc = "Field `WIN2_FADING_OFFSET_R` writer - fading offset red"]
pub type Win2FadingOffsetRW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN2_FADING_OFFSET_G` reader - fading offset green"]
pub type Win2FadingOffsetGR = crate::FieldReader;
#[doc = "Field `WIN2_FADING_OFFSET_G` writer - fading offset green"]
pub type Win2FadingOffsetGW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN2_FADING_OFFSET_B` reader - fading offset blue"]
pub type Win2FadingOffsetBR = crate::FieldReader;
#[doc = "Field `WIN2_FADING_OFFSET_B` writer - fading offset blue"]
pub type Win2FadingOffsetBW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "fading enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2FadingEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win2FadingEn> for bool {
    #[inline(always)]
    fn from(variant: Win2FadingEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_FADING_EN` reader - fading enable"]
pub type Win2FadingEnR = crate::BitReader<Win2FadingEn>;
impl Win2FadingEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2FadingEn {
        match self.bits {
            false => Win2FadingEn::B0,
            true => Win2FadingEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2FadingEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2FadingEn::B1
    }
}
#[doc = "Field `WIN2_FADING_EN` writer - fading enable"]
pub type Win2FadingEnW<'a, REG> = crate::BitWriter<'a, REG, Win2FadingEn>;
impl<'a, REG> Win2FadingEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2FadingEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2FadingEn::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - fading offset red"]
    #[inline(always)]
    pub fn win2_fading_offset_r(&self) -> Win2FadingOffsetRR {
        Win2FadingOffsetRR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - fading offset green"]
    #[inline(always)]
    pub fn win2_fading_offset_g(&self) -> Win2FadingOffsetGR {
        Win2FadingOffsetGR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - fading offset blue"]
    #[inline(always)]
    pub fn win2_fading_offset_b(&self) -> Win2FadingOffsetBR {
        Win2FadingOffsetBR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - fading enable"]
    #[inline(always)]
    pub fn win2_fading_en(&self) -> Win2FadingEnR {
        Win2FadingEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - fading offset red"]
    #[inline(always)]
    #[must_use]
    pub fn win2_fading_offset_r(&mut self) -> Win2FadingOffsetRW<Win2FadingCtrlSpec> {
        Win2FadingOffsetRW::new(self, 0)
    }
    #[doc = "Bits 8:15 - fading offset green"]
    #[inline(always)]
    #[must_use]
    pub fn win2_fading_offset_g(&mut self) -> Win2FadingOffsetGW<Win2FadingCtrlSpec> {
        Win2FadingOffsetGW::new(self, 8)
    }
    #[doc = "Bits 16:23 - fading offset blue"]
    #[inline(always)]
    #[must_use]
    pub fn win2_fading_offset_b(&mut self) -> Win2FadingOffsetBW<Win2FadingCtrlSpec> {
        Win2FadingOffsetBW::new(self, 16)
    }
    #[doc = "Bit 24 - fading enable"]
    #[inline(always)]
    #[must_use]
    pub fn win2_fading_en(&mut self) -> Win2FadingEnW<Win2FadingCtrlSpec> {
        Win2FadingEnW::new(self, 24)
    }
}
#[doc = "Win2 fading contrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_fading_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_fading_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win2FadingCtrlSpec;
impl crate::RegisterSpec for Win2FadingCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win2_fading_ctrl::R`](R) reader structure"]
impl crate::Readable for Win2FadingCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`win2_fading_ctrl::W`](W) writer structure"]
impl crate::Writable for Win2FadingCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN2_FADING_CTRL to value 0"]
impl crate::Resettable for Win2FadingCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
