#[doc = "Register `IMG_EFF_CTRL` reader"]
pub type R = crate::R<ImgEffCtrlSpec>;
#[doc = "Register `IMG_EFF_CTRL` writer"]
pub type W = crate::W<ImgEffCtrlSpec>;
#[doc = "bypass mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BypassMode {
    #[doc = "1: processing is activated"]
    B1 = 1,
    #[doc = "0: processing is deactivated, bypass mode is selected"]
    B0 = 0,
}
impl From<BypassMode> for bool {
    #[inline(always)]
    fn from(variant: BypassMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `bypass_mode` reader - bypass mode"]
pub type BypassModeR = crate::BitReader<BypassMode>;
impl BypassModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BypassMode {
        match self.bits {
            true => BypassMode::B1,
            false => BypassMode::B0,
        }
    }
    #[doc = "processing is activated"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BypassMode::B1
    }
    #[doc = "processing is deactivated, bypass mode is selected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BypassMode::B0
    }
}
#[doc = "Field `bypass_mode` writer - bypass mode"]
pub type BypassModeW<'a, REG> = crate::BitWriter<'a, REG, BypassMode>;
impl<'a, REG> BypassModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "processing is activated"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BypassMode::B1)
    }
    #[doc = "processing is deactivated, bypass mode is selected"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BypassMode::B0)
    }
}
#[doc = "effect mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EffectMode {
    #[doc = "0: black &amp; white effect (grayscale) 001: negative"]
    B000 = 0,
    #[doc = "2: sepia effect"]
    B010 = 2,
    #[doc = "3: color selection effect 100: emboss effect"]
    B011 = 3,
    #[doc = "5: sketch effect"]
    B101 = 5,
    #[doc = "6: sharpen effect"]
    B110 = 6,
}
impl From<EffectMode> for u8 {
    #[inline(always)]
    fn from(variant: EffectMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EffectMode {
    type Ux = u8;
}
#[doc = "Field `effect_mode` reader - effect mode"]
pub type EffectModeR = crate::FieldReader<EffectMode>;
impl EffectModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EffectMode> {
        match self.bits {
            0 => Some(EffectMode::B000),
            2 => Some(EffectMode::B010),
            3 => Some(EffectMode::B011),
            5 => Some(EffectMode::B101),
            6 => Some(EffectMode::B110),
            _ => None,
        }
    }
    #[doc = "black &amp; white effect (grayscale) 001: negative"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == EffectMode::B000
    }
    #[doc = "sepia effect"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == EffectMode::B010
    }
    #[doc = "color selection effect 100: emboss effect"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == EffectMode::B011
    }
    #[doc = "sketch effect"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == EffectMode::B101
    }
    #[doc = "sharpen effect"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == EffectMode::B110
    }
}
#[doc = "Field `effect_mode` writer - effect mode"]
pub type EffectModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, EffectMode>;
impl<'a, REG> EffectModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "black &amp; white effect (grayscale) 001: negative"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(EffectMode::B000)
    }
    #[doc = "sepia effect"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(EffectMode::B010)
    }
    #[doc = "color selection effect 100: emboss effect"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(EffectMode::B011)
    }
    #[doc = "sketch effect"]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(EffectMode::B101)
    }
    #[doc = "sharpen effect"]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(EffectMode::B110)
    }
}
#[doc = "Field `cfg_upd` reader - write '0': nothing happens\n\nwrite '1': update shadow registers read: always '0'"]
pub type CfgUpdR = crate::BitReader;
#[doc = "Field `cfg_upd` writer - write '0': nothing happens\n\nwrite '1': update shadow registers read: always '0'"]
pub type CfgUpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `full_range` reader - '0': pixel value range according to BT.601\n\n'1': YCbCr full range 0...255\n\n"]
pub type FullRangeR = crate::BitReader;
#[doc = "Field `full_range` writer - '0': pixel value range according to BT.601\n\n'1': YCbCr full range 0...255\n\n"]
pub type FullRangeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - bypass mode"]
    #[inline(always)]
    pub fn bypass_mode(&self) -> BypassModeR {
        BypassModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - effect mode"]
    #[inline(always)]
    pub fn effect_mode(&self) -> EffectModeR {
        EffectModeR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - write '0': nothing happens\n\nwrite '1': update shadow registers read: always '0'"]
    #[inline(always)]
    pub fn cfg_upd(&self) -> CfgUpdR {
        CfgUpdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - '0': pixel value range according to BT.601\n\n'1': YCbCr full range 0...255\n\n"]
    #[inline(always)]
    pub fn full_range(&self) -> FullRangeR {
        FullRangeR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - bypass mode"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_mode(&mut self) -> BypassModeW<ImgEffCtrlSpec> {
        BypassModeW::new(self, 0)
    }
    #[doc = "Bits 1:3 - effect mode"]
    #[inline(always)]
    #[must_use]
    pub fn effect_mode(&mut self) -> EffectModeW<ImgEffCtrlSpec> {
        EffectModeW::new(self, 1)
    }
    #[doc = "Bit 4 - write '0': nothing happens\n\nwrite '1': update shadow registers read: always '0'"]
    #[inline(always)]
    #[must_use]
    pub fn cfg_upd(&mut self) -> CfgUpdW<ImgEffCtrlSpec> {
        CfgUpdW::new(self, 4)
    }
    #[doc = "Bit 5 - '0': pixel value range according to BT.601\n\n'1': YCbCr full range 0...255\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn full_range(&mut self) -> FullRangeW<ImgEffCtrlSpec> {
        FullRangeW::new(self, 5)
    }
}
#[doc = "Global control register\n\nNote: full_range for image effects is supported in ISP M5_v6, M5_v7 only \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`img_eff_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`img_eff_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImgEffCtrlSpec;
impl crate::RegisterSpec for ImgEffCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`img_eff_ctrl::R`](R) reader structure"]
impl crate::Readable for ImgEffCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`img_eff_ctrl::W`](W) writer structure"]
impl crate::Writable for ImgEffCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMG_EFF_CTRL to value 0"]
impl crate::Resettable for ImgEffCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
