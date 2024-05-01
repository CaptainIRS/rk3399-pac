#[doc = "Register `WIN0_COLOR_KEY` reader"]
pub type R = crate::R<Win0ColorKeySpec>;
#[doc = "Register `WIN0_COLOR_KEY` writer"]
pub type W = crate::W<Win0ColorKeySpec>;
#[doc = "Field `WIN0_KEY_COLOR` reader - Win0 key color\n\n24 bit RGB888"]
pub type Win0KeyColorR = crate::FieldReader<u32>;
#[doc = "Field `WIN0_KEY_COLOR` writer - Win0 key color\n\n24 bit RGB888"]
pub type Win0KeyColorW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Win0 transparency color key enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0KeyEn {
    #[doc = "0: disable;"]
    B0 = 0,
    #[doc = "1: enable;"]
    B1 = 1,
}
impl From<Win0KeyEn> for bool {
    #[inline(always)]
    fn from(variant: Win0KeyEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_KEY_EN` reader - Win0 transparency color key enable"]
pub type Win0KeyEnR = crate::BitReader<Win0KeyEn>;
impl Win0KeyEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0KeyEn {
        match self.bits {
            false => Win0KeyEn::B0,
            true => Win0KeyEn::B1,
        }
    }
    #[doc = "disable;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0KeyEn::B0
    }
    #[doc = "enable;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0KeyEn::B1
    }
}
#[doc = "Field `WIN0_KEY_EN` writer - Win0 transparency color key enable"]
pub type Win0KeyEnW<'a, REG> = crate::BitWriter<'a, REG, Win0KeyEn>;
impl<'a, REG> Win0KeyEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0KeyEn::B0)
    }
    #[doc = "enable;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0KeyEn::B1)
    }
}
impl R {
    #[doc = "Bits 0:23 - Win0 key color\n\n24 bit RGB888"]
    #[inline(always)]
    pub fn win0_key_color(&self) -> Win0KeyColorR {
        Win0KeyColorR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 31 - Win0 transparency color key enable"]
    #[inline(always)]
    pub fn win0_key_en(&self) -> Win0KeyEnR {
        Win0KeyEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Win0 key color\n\n24 bit RGB888"]
    #[inline(always)]
    #[must_use]
    pub fn win0_key_color(&mut self) -> Win0KeyColorW<Win0ColorKeySpec> {
        Win0KeyColorW::new(self, 0)
    }
    #[doc = "Bit 31 - Win0 transparency color key enable"]
    #[inline(always)]
    #[must_use]
    pub fn win0_key_en(&mut self) -> Win0KeyEnW<Win0ColorKeySpec> {
        Win0KeyEnW::new(self, 31)
    }
}
#[doc = "Win0 color key register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_color_key::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_color_key::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win0ColorKeySpec;
impl crate::RegisterSpec for Win0ColorKeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win0_color_key::R`](R) reader structure"]
impl crate::Readable for Win0ColorKeySpec {}
#[doc = "`write(|w| ..)` method takes [`win0_color_key::W`](W) writer structure"]
impl crate::Writable for Win0ColorKeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN0_COLOR_KEY to value 0"]
impl crate::Resettable for Win0ColorKeySpec {
    const RESET_VALUE: u32 = 0;
}
