#[doc = "Register `WIN3_FADING_CTRL` reader"]
pub type R = crate::R<Win3FadingCtrlSpec>;
#[doc = "Register `WIN3_FADING_CTRL` writer"]
pub type W = crate::W<Win3FadingCtrlSpec>;
#[doc = "Field `WIN3_FADING_OFFSET_R` reader - fading offset red"]
pub type Win3FadingOffsetRR = crate::FieldReader;
#[doc = "Field `WIN3_FADING_OFFSET_R` writer - fading offset red"]
pub type Win3FadingOffsetRW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN3_FADING_OFFSET_G` reader - fading offset green"]
pub type Win3FadingOffsetGR = crate::FieldReader;
#[doc = "Field `WIN3_FADING_OFFSET_G` writer - fading offset green"]
pub type Win3FadingOffsetGW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN3_FADING_OFFSET_B` reader - fading offset blue"]
pub type Win3FadingOffsetBR = crate::FieldReader;
#[doc = "Field `WIN3_FADING_OFFSET_B` writer - fading offset blue"]
pub type Win3FadingOffsetBW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "fadfing enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3FadingEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win3FadingEn> for bool {
    #[inline(always)]
    fn from(variant: Win3FadingEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_FADING_EN` reader - fadfing enable"]
pub type Win3FadingEnR = crate::BitReader<Win3FadingEn>;
impl Win3FadingEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3FadingEn {
        match self.bits {
            false => Win3FadingEn::B0,
            true => Win3FadingEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3FadingEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3FadingEn::B1
    }
}
#[doc = "Field `WIN3_FADING_EN` writer - fadfing enable"]
pub type Win3FadingEnW<'a, REG> = crate::BitWriter<'a, REG, Win3FadingEn>;
impl<'a, REG> Win3FadingEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3FadingEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3FadingEn::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - fading offset red"]
    #[inline(always)]
    pub fn win3_fading_offset_r(&self) -> Win3FadingOffsetRR {
        Win3FadingOffsetRR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - fading offset green"]
    #[inline(always)]
    pub fn win3_fading_offset_g(&self) -> Win3FadingOffsetGR {
        Win3FadingOffsetGR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - fading offset blue"]
    #[inline(always)]
    pub fn win3_fading_offset_b(&self) -> Win3FadingOffsetBR {
        Win3FadingOffsetBR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - fadfing enable"]
    #[inline(always)]
    pub fn win3_fading_en(&self) -> Win3FadingEnR {
        Win3FadingEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - fading offset red"]
    #[inline(always)]
    #[must_use]
    pub fn win3_fading_offset_r(&mut self) -> Win3FadingOffsetRW<Win3FadingCtrlSpec> {
        Win3FadingOffsetRW::new(self, 0)
    }
    #[doc = "Bits 8:15 - fading offset green"]
    #[inline(always)]
    #[must_use]
    pub fn win3_fading_offset_g(&mut self) -> Win3FadingOffsetGW<Win3FadingCtrlSpec> {
        Win3FadingOffsetGW::new(self, 8)
    }
    #[doc = "Bits 16:23 - fading offset blue"]
    #[inline(always)]
    #[must_use]
    pub fn win3_fading_offset_b(&mut self) -> Win3FadingOffsetBW<Win3FadingCtrlSpec> {
        Win3FadingOffsetBW::new(self, 16)
    }
    #[doc = "Bit 24 - fadfing enable"]
    #[inline(always)]
    #[must_use]
    pub fn win3_fading_en(&mut self) -> Win3FadingEnW<Win3FadingCtrlSpec> {
        Win3FadingEnW::new(self, 24)
    }
}
#[doc = "Win3 fading contrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_fading_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_fading_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win3FadingCtrlSpec;
impl crate::RegisterSpec for Win3FadingCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win3_fading_ctrl::R`](R) reader structure"]
impl crate::Readable for Win3FadingCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`win3_fading_ctrl::W`](W) writer structure"]
impl crate::Writable for Win3FadingCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN3_FADING_CTRL to value 0"]
impl crate::Resettable for Win3FadingCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
