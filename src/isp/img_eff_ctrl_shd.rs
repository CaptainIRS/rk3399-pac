#[doc = "Register `IMG_EFF_CTRL_SHD` reader"]
pub type R = crate::R<ImgEffCtrlShdSpec>;
#[doc = "effect mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EffectModeShd {
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
impl From<EffectModeShd> for u8 {
    #[inline(always)]
    fn from(variant: EffectModeShd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EffectModeShd {
    type Ux = u8;
}
#[doc = "Field `effect_mode_shd` reader - effect mode"]
pub type EffectModeShdR = crate::FieldReader<EffectModeShd>;
impl EffectModeShdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EffectModeShd> {
        match self.bits {
            0 => Some(EffectModeShd::B000),
            2 => Some(EffectModeShd::B010),
            3 => Some(EffectModeShd::B011),
            5 => Some(EffectModeShd::B101),
            6 => Some(EffectModeShd::B110),
            _ => None,
        }
    }
    #[doc = "black &amp; white effect (grayscale) 001: negative"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == EffectModeShd::B000
    }
    #[doc = "sepia effect"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == EffectModeShd::B010
    }
    #[doc = "color selection effect 100: emboss effect"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == EffectModeShd::B011
    }
    #[doc = "sketch effect"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == EffectModeShd::B101
    }
    #[doc = "sharpen effect"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == EffectModeShd::B110
    }
}
impl R {
    #[doc = "Bits 1:3 - effect mode"]
    #[inline(always)]
    pub fn effect_mode_shd(&self) -> EffectModeShdR {
        EffectModeShdR::new(((self.bits >> 1) & 7) as u8)
    }
}
#[doc = "Shadow register for control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`img_eff_ctrl_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImgEffCtrlShdSpec;
impl crate::RegisterSpec for ImgEffCtrlShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`img_eff_ctrl_shd::R`](R) reader structure"]
impl crate::Readable for ImgEffCtrlShdSpec {}
#[doc = "`reset()` method sets IMG_EFF_CTRL_SHD to value 0"]
impl crate::Resettable for ImgEffCtrlShdSpec {
    const RESET_VALUE: u32 = 0;
}
