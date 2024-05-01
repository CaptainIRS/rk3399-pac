#[doc = "Register `BCSH_COLOR_BAR` reader"]
pub type R = crate::R<BcshColorBarSpec>;
#[doc = "Register `BCSH_COLOR_BAR` writer"]
pub type W = crate::W<BcshColorBarSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BcshEn {
    #[doc = "0: bcsh bypass"]
    B0 = 0,
    #[doc = "1: bcsh enable"]
    B1 = 1,
}
impl From<BcshEn> for bool {
    #[inline(always)]
    fn from(variant: BcshEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCSH_EN` reader - "]
pub type BcshEnR = crate::BitReader<BcshEn>;
impl BcshEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BcshEn {
        match self.bits {
            false => BcshEn::B0,
            true => BcshEn::B1,
        }
    }
    #[doc = "bcsh bypass"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BcshEn::B0
    }
    #[doc = "bcsh enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BcshEn::B1
    }
}
#[doc = "Field `BCSH_EN` writer - "]
pub type BcshEnW<'a, REG> = crate::BitWriter<'a, REG, BcshEn>;
impl<'a, REG> BcshEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bcsh bypass"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BcshEn::B0)
    }
    #[doc = "bcsh enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BcshEn::B1)
    }
}
#[doc = "Field `COLOR_BAR_Y` reader - y color value"]
pub type ColorBarYR = crate::FieldReader;
#[doc = "Field `COLOR_BAR_Y` writer - y color value"]
pub type ColorBarYW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLOR_BAR_U` reader - u color value"]
pub type ColorBarUR = crate::FieldReader;
#[doc = "Field `COLOR_BAR_U` writer - u color value"]
pub type ColorBarUW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLOR_BAR_V` reader - v color value"]
pub type ColorBarVR = crate::FieldReader;
#[doc = "Field `COLOR_BAR_V` writer - v color value"]
pub type ColorBarVW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bcsh_en(&self) -> BcshEnR {
        BcshEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - y color value"]
    #[inline(always)]
    pub fn color_bar_y(&self) -> ColorBarYR {
        ColorBarYR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - u color value"]
    #[inline(always)]
    pub fn color_bar_u(&self) -> ColorBarUR {
        ColorBarUR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - v color value"]
    #[inline(always)]
    pub fn color_bar_v(&self) -> ColorBarVR {
        ColorBarVR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn bcsh_en(&mut self) -> BcshEnW<BcshColorBarSpec> {
        BcshEnW::new(self, 0)
    }
    #[doc = "Bits 8:15 - y color value"]
    #[inline(always)]
    #[must_use]
    pub fn color_bar_y(&mut self) -> ColorBarYW<BcshColorBarSpec> {
        ColorBarYW::new(self, 8)
    }
    #[doc = "Bits 16:23 - u color value"]
    #[inline(always)]
    #[must_use]
    pub fn color_bar_u(&mut self) -> ColorBarUW<BcshColorBarSpec> {
        ColorBarUW::new(self, 16)
    }
    #[doc = "Bits 24:31 - v color value"]
    #[inline(always)]
    #[must_use]
    pub fn color_bar_v(&mut self) -> ColorBarVW<BcshColorBarSpec> {
        ColorBarVW::new(self, 24)
    }
}
#[doc = "Color bar config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcsh_color_bar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcsh_color_bar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcshColorBarSpec;
impl crate::RegisterSpec for BcshColorBarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcsh_color_bar::R`](R) reader structure"]
impl crate::Readable for BcshColorBarSpec {}
#[doc = "`write(|w| ..)` method takes [`bcsh_color_bar::W`](W) writer structure"]
impl crate::Writable for BcshColorBarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCSH_COLOR_BAR to value 0"]
impl crate::Resettable for BcshColorBarSpec {
    const RESET_VALUE: u32 = 0;
}
