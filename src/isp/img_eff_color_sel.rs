#[doc = "Register `IMG_EFF_COLOR_SEL` reader"]
pub type R = crate::R<ImgEffColorSelSpec>;
#[doc = "Register `IMG_EFF_COLOR_SEL` writer"]
pub type W = crate::W<ImgEffColorSelSpec>;
#[doc = "Defining the maintained color: 000: red green and\n\nblue\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ColorSelection {
    #[doc = "1: blue"]
    B001 = 1,
    #[doc = "2: green"]
    B010 = 2,
    #[doc = "3: green and blue 100: red"]
    B011 = 3,
    #[doc = "5: red and blue 110: red and green"]
    B101 = 5,
    #[doc = "7: red green and blue"]
    B111 = 7,
}
impl From<ColorSelection> for u8 {
    #[inline(always)]
    fn from(variant: ColorSelection) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ColorSelection {
    type Ux = u8;
}
#[doc = "Field `color_selection` reader - Defining the maintained color: 000: red green and\n\nblue"]
pub type ColorSelectionR = crate::FieldReader<ColorSelection>;
impl ColorSelectionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ColorSelection> {
        match self.bits {
            1 => Some(ColorSelection::B001),
            2 => Some(ColorSelection::B010),
            3 => Some(ColorSelection::B011),
            5 => Some(ColorSelection::B101),
            7 => Some(ColorSelection::B111),
            _ => None,
        }
    }
    #[doc = "blue"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == ColorSelection::B001
    }
    #[doc = "green"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == ColorSelection::B010
    }
    #[doc = "green and blue 100: red"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == ColorSelection::B011
    }
    #[doc = "red and blue 110: red and green"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == ColorSelection::B101
    }
    #[doc = "red green and blue"]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == ColorSelection::B111
    }
}
#[doc = "Field `color_selection` writer - Defining the maintained color: 000: red green and\n\nblue"]
pub type ColorSelectionW<'a, REG> = crate::FieldWriter<'a, REG, 3, ColorSelection>;
impl<'a, REG> ColorSelectionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "blue"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(ColorSelection::B001)
    }
    #[doc = "green"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(ColorSelection::B010)
    }
    #[doc = "green and blue 100: red"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(ColorSelection::B011)
    }
    #[doc = "red and blue 110: red and green"]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(ColorSelection::B101)
    }
    #[doc = "red green and blue"]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(ColorSelection::B111)
    }
}
#[doc = "Field `color_threshold` reader - Threshold value of the RGB colors for the color\n\nselection effect."]
pub type ColorThresholdR = crate::FieldReader;
#[doc = "Field `color_threshold` writer - Threshold value of the RGB colors for the color\n\nselection effect."]
pub type ColorThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - Defining the maintained color: 000: red green and\n\nblue"]
    #[inline(always)]
    pub fn color_selection(&self) -> ColorSelectionR {
        ColorSelectionR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:15 - Threshold value of the RGB colors for the color\n\nselection effect."]
    #[inline(always)]
    pub fn color_threshold(&self) -> ColorThresholdR {
        ColorThresholdR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Defining the maintained color: 000: red green and\n\nblue"]
    #[inline(always)]
    #[must_use]
    pub fn color_selection(&mut self) -> ColorSelectionW<ImgEffColorSelSpec> {
        ColorSelectionW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Threshold value of the RGB colors for the color\n\nselection effect."]
    #[inline(always)]
    #[must_use]
    pub fn color_threshold(&mut self) -> ColorThresholdW<ImgEffColorSelSpec> {
        ColorThresholdW::new(self, 8)
    }
}
#[doc = "Color selection register (for color selection effect)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`img_eff_color_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`img_eff_color_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImgEffColorSelSpec;
impl crate::RegisterSpec for ImgEffColorSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`img_eff_color_sel::R`](R) reader structure"]
impl crate::Readable for ImgEffColorSelSpec {}
#[doc = "`write(|w| ..)` method takes [`img_eff_color_sel::W`](W) writer structure"]
impl crate::Writable for ImgEffColorSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMG_EFF_COLOR_SEL to value 0"]
impl crate::Resettable for ImgEffColorSelSpec {
    const RESET_VALUE: u32 = 0;
}
