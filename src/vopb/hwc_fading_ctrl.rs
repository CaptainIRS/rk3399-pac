#[doc = "Register `HWC_FADING_CTRL` reader"]
pub type R = crate::R<HwcFadingCtrlSpec>;
#[doc = "Register `HWC_FADING_CTRL` writer"]
pub type W = crate::W<HwcFadingCtrlSpec>;
#[doc = "Field `HWC_FADING_OFFSET_R` reader - fading offset red"]
pub type HwcFadingOffsetRR = crate::FieldReader;
#[doc = "Field `HWC_FADING_OFFSET_R` writer - fading offset red"]
pub type HwcFadingOffsetRW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HWC_FADING_OFFSET_G` reader - fading offset green"]
pub type HwcFadingOffsetGR = crate::FieldReader;
#[doc = "Field `HWC_FADING_OFFSET_G` writer - fading offset green"]
pub type HwcFadingOffsetGW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HWC_FADING_OFFSET_B` reader - fading offset blue"]
pub type HwcFadingOffsetBR = crate::FieldReader;
#[doc = "Field `HWC_FADING_OFFSET_B` writer - fading offset blue"]
pub type HwcFadingOffsetBW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HwcFadingEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<HwcFadingEn> for bool {
    #[inline(always)]
    fn from(variant: HwcFadingEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWC_FADING_EN` reader - "]
pub type HwcFadingEnR = crate::BitReader<HwcFadingEn>;
impl HwcFadingEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HwcFadingEn {
        match self.bits {
            false => HwcFadingEn::B0,
            true => HwcFadingEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HwcFadingEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HwcFadingEn::B1
    }
}
#[doc = "Field `HWC_FADING_EN` writer - "]
pub type HwcFadingEnW<'a, REG> = crate::BitWriter<'a, REG, HwcFadingEn>;
impl<'a, REG> HwcFadingEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HwcFadingEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HwcFadingEn::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - fading offset red"]
    #[inline(always)]
    pub fn hwc_fading_offset_r(&self) -> HwcFadingOffsetRR {
        HwcFadingOffsetRR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - fading offset green"]
    #[inline(always)]
    pub fn hwc_fading_offset_g(&self) -> HwcFadingOffsetGR {
        HwcFadingOffsetGR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - fading offset blue"]
    #[inline(always)]
    pub fn hwc_fading_offset_b(&self) -> HwcFadingOffsetBR {
        HwcFadingOffsetBR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn hwc_fading_en(&self) -> HwcFadingEnR {
        HwcFadingEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - fading offset red"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_fading_offset_r(&mut self) -> HwcFadingOffsetRW<HwcFadingCtrlSpec> {
        HwcFadingOffsetRW::new(self, 0)
    }
    #[doc = "Bits 8:15 - fading offset green"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_fading_offset_g(&mut self) -> HwcFadingOffsetGW<HwcFadingCtrlSpec> {
        HwcFadingOffsetGW::new(self, 8)
    }
    #[doc = "Bits 16:23 - fading offset blue"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_fading_offset_b(&mut self) -> HwcFadingOffsetBW<HwcFadingCtrlSpec> {
        HwcFadingOffsetBW::new(self, 16)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_fading_en(&mut self) -> HwcFadingEnW<HwcFadingCtrlSpec> {
        HwcFadingEnW::new(self, 24)
    }
}
#[doc = "Hwc fading contrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwc_fading_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwc_fading_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwcFadingCtrlSpec;
impl crate::RegisterSpec for HwcFadingCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwc_fading_ctrl::R`](R) reader structure"]
impl crate::Readable for HwcFadingCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`hwc_fading_ctrl::W`](W) writer structure"]
impl crate::Writable for HwcFadingCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWC_FADING_CTRL to value 0"]
impl crate::Resettable for HwcFadingCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
