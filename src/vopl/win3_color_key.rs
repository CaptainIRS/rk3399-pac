#[doc = "Register `WIN3_COLOR_KEY` reader"]
pub type R = crate::R<Win3ColorKeySpec>;
#[doc = "Register `WIN3_COLOR_KEY` writer"]
pub type W = crate::W<Win3ColorKeySpec>;
#[doc = "Field `WIN3_KEY_COLOR` reader - Win3 key color"]
pub type Win3KeyColorR = crate::FieldReader<u32>;
#[doc = "Field `WIN3_KEY_COLOR` writer - Win3 key color"]
pub type Win3KeyColorW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Win3 transparency color key enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3KeyEn {
    #[doc = "0: disable;"]
    B0 = 0,
    #[doc = "1: enable;"]
    B1 = 1,
}
impl From<Win3KeyEn> for bool {
    #[inline(always)]
    fn from(variant: Win3KeyEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_KEY_EN` reader - Win3 transparency color key enable"]
pub type Win3KeyEnR = crate::BitReader<Win3KeyEn>;
impl Win3KeyEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3KeyEn {
        match self.bits {
            false => Win3KeyEn::B0,
            true => Win3KeyEn::B1,
        }
    }
    #[doc = "disable;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3KeyEn::B0
    }
    #[doc = "enable;"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3KeyEn::B1
    }
}
#[doc = "Field `WIN3_KEY_EN` writer - Win3 transparency color key enable"]
pub type Win3KeyEnW<'a, REG> = crate::BitWriter<'a, REG, Win3KeyEn>;
impl<'a, REG> Win3KeyEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3KeyEn::B0)
    }
    #[doc = "enable;"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3KeyEn::B1)
    }
}
impl R {
    #[doc = "Bits 0:23 - Win3 key color"]
    #[inline(always)]
    pub fn win3_key_color(&self) -> Win3KeyColorR {
        Win3KeyColorR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Win3 transparency color key enable"]
    #[inline(always)]
    pub fn win3_key_en(&self) -> Win3KeyEnR {
        Win3KeyEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Win3 key color"]
    #[inline(always)]
    #[must_use]
    pub fn win3_key_color(&mut self) -> Win3KeyColorW<Win3ColorKeySpec> {
        Win3KeyColorW::new(self, 0)
    }
    #[doc = "Bit 24 - Win3 transparency color key enable"]
    #[inline(always)]
    #[must_use]
    pub fn win3_key_en(&mut self) -> Win3KeyEnW<Win3ColorKeySpec> {
        Win3KeyEnW::new(self, 24)
    }
}
#[doc = "Win3 color key register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_color_key::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_color_key::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win3ColorKeySpec;
impl crate::RegisterSpec for Win3ColorKeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win3_color_key::R`](R) reader structure"]
impl crate::Readable for Win3ColorKeySpec {}
#[doc = "`write(|w| ..)` method takes [`win3_color_key::W`](W) writer structure"]
impl crate::Writable for Win3ColorKeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN3_COLOR_KEY to value 0"]
impl crate::Resettable for Win3ColorKeySpec {
    const RESET_VALUE: u32 = 0;
}
