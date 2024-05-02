#[doc = "Register `MRSZ_CTRL_SHD` reader"]
pub type R = crate::R<MrszCtrlShdSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScaleHyEnableShd {
    #[doc = "0: bypass horizontal luminance scaling unit"]
    B0 = 0,
    #[doc = "1: enable horizontal luminance scaling unit"]
    B1 = 1,
}
impl From<ScaleHyEnableShd> for bool {
    #[inline(always)]
    fn from(variant: ScaleHyEnableShd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `scale_hy_enable_shd` reader - "]
pub type ScaleHyEnableShdR = crate::BitReader<ScaleHyEnableShd>;
impl ScaleHyEnableShdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ScaleHyEnableShd {
        match self.bits {
            false => ScaleHyEnableShd::B0,
            true => ScaleHyEnableShd::B1,
        }
    }
    #[doc = "bypass horizontal luminance scaling unit"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ScaleHyEnableShd::B0
    }
    #[doc = "enable horizontal luminance scaling unit"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ScaleHyEnableShd::B1
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScaleHcEnableShd {
    #[doc = "0: bypass horizontal chrominance scaling unit"]
    B0 = 0,
    #[doc = "1: enable horizontal chrominance scaling unit"]
    B1 = 1,
}
impl From<ScaleHcEnableShd> for bool {
    #[inline(always)]
    fn from(variant: ScaleHcEnableShd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `scale_hc_enable_shd` reader - "]
pub type ScaleHcEnableShdR = crate::BitReader<ScaleHcEnableShd>;
impl ScaleHcEnableShdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ScaleHcEnableShd {
        match self.bits {
            false => ScaleHcEnableShd::B0,
            true => ScaleHcEnableShd::B1,
        }
    }
    #[doc = "bypass horizontal chrominance scaling unit"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ScaleHcEnableShd::B0
    }
    #[doc = "enable horizontal chrominance scaling unit"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ScaleHcEnableShd::B1
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScaleVyEnableShd {
    #[doc = "0: bypass vertical luminance scaling unit"]
    B0 = 0,
    #[doc = "1: enable vertical luminance scaling unit"]
    B1 = 1,
}
impl From<ScaleVyEnableShd> for bool {
    #[inline(always)]
    fn from(variant: ScaleVyEnableShd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `scale_vy_enable_shd` reader - "]
pub type ScaleVyEnableShdR = crate::BitReader<ScaleVyEnableShd>;
impl ScaleVyEnableShdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ScaleVyEnableShd {
        match self.bits {
            false => ScaleVyEnableShd::B0,
            true => ScaleVyEnableShd::B1,
        }
    }
    #[doc = "bypass vertical luminance scaling unit"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ScaleVyEnableShd::B0
    }
    #[doc = "enable vertical luminance scaling unit"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ScaleVyEnableShd::B1
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScaleVcEnableShd {
    #[doc = "0: bypass vertical chrominance scaling unit"]
    B0 = 0,
    #[doc = "1: enable vertical chrominance scaling unit"]
    B1 = 1,
}
impl From<ScaleVcEnableShd> for bool {
    #[inline(always)]
    fn from(variant: ScaleVcEnableShd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `scale_vc_enable_shd` reader - "]
pub type ScaleVcEnableShdR = crate::BitReader<ScaleVcEnableShd>;
impl ScaleVcEnableShdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ScaleVcEnableShd {
        match self.bits {
            false => ScaleVcEnableShd::B0,
            true => ScaleVcEnableShd::B1,
        }
    }
    #[doc = "bypass vertical chrominance scaling unit"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ScaleVcEnableShd::B0
    }
    #[doc = "enable vertical chrominance scaling unit"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ScaleVcEnableShd::B1
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScaleHyUpShd {
    #[doc = "1: horizontal luminance upscaling selected"]
    B1 = 1,
    #[doc = "0: horizontal luminance downscaling selected"]
    B0 = 0,
}
impl From<ScaleHyUpShd> for bool {
    #[inline(always)]
    fn from(variant: ScaleHyUpShd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `scale_hy_up_shd` reader - "]
pub type ScaleHyUpShdR = crate::BitReader<ScaleHyUpShd>;
impl ScaleHyUpShdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ScaleHyUpShd {
        match self.bits {
            true => ScaleHyUpShd::B1,
            false => ScaleHyUpShd::B0,
        }
    }
    #[doc = "horizontal luminance upscaling selected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ScaleHyUpShd::B1
    }
    #[doc = "horizontal luminance downscaling selected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ScaleHyUpShd::B0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScaleHcUpShd {
    #[doc = "1: horizontal chrominance upscaling selected"]
    B1 = 1,
    #[doc = "0: horizontal chrominance downscaling selected"]
    B0 = 0,
}
impl From<ScaleHcUpShd> for bool {
    #[inline(always)]
    fn from(variant: ScaleHcUpShd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `scale_hc_up_shd` reader - "]
pub type ScaleHcUpShdR = crate::BitReader<ScaleHcUpShd>;
impl ScaleHcUpShdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ScaleHcUpShd {
        match self.bits {
            true => ScaleHcUpShd::B1,
            false => ScaleHcUpShd::B0,
        }
    }
    #[doc = "horizontal chrominance upscaling selected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ScaleHcUpShd::B1
    }
    #[doc = "horizontal chrominance downscaling selected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ScaleHcUpShd::B0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScaleVyUpShd {
    #[doc = "1: vertical luminance upscaling selected"]
    B1 = 1,
    #[doc = "0: vertical luminance downscaling selected"]
    B0 = 0,
}
impl From<ScaleVyUpShd> for bool {
    #[inline(always)]
    fn from(variant: ScaleVyUpShd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `scale_vy_up_shd` reader - "]
pub type ScaleVyUpShdR = crate::BitReader<ScaleVyUpShd>;
impl ScaleVyUpShdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ScaleVyUpShd {
        match self.bits {
            true => ScaleVyUpShd::B1,
            false => ScaleVyUpShd::B0,
        }
    }
    #[doc = "vertical luminance upscaling selected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ScaleVyUpShd::B1
    }
    #[doc = "vertical luminance downscaling selected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ScaleVyUpShd::B0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScaleVcUpShd {
    #[doc = "1: vertical chrominance upscaling selected"]
    B1 = 1,
    #[doc = "0: vertical chrominance downscaling selected"]
    B0 = 0,
}
impl From<ScaleVcUpShd> for bool {
    #[inline(always)]
    fn from(variant: ScaleVcUpShd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `scale_vc_up_shd` reader - "]
pub type ScaleVcUpShdR = crate::BitReader<ScaleVcUpShd>;
impl ScaleVcUpShdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ScaleVcUpShd {
        match self.bits {
            true => ScaleVcUpShd::B1,
            false => ScaleVcUpShd::B0,
        }
    }
    #[doc = "vertical chrominance upscaling selected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ScaleVcUpShd::B1
    }
    #[doc = "vertical chrominance downscaling selected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ScaleVcUpShd::B0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn scale_hy_enable_shd(&self) -> ScaleHyEnableShdR {
        ScaleHyEnableShdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn scale_hc_enable_shd(&self) -> ScaleHcEnableShdR {
        ScaleHcEnableShdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn scale_vy_enable_shd(&self) -> ScaleVyEnableShdR {
        ScaleVyEnableShdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn scale_vc_enable_shd(&self) -> ScaleVcEnableShdR {
        ScaleVcEnableShdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn scale_hy_up_shd(&self) -> ScaleHyUpShdR {
        ScaleHyUpShdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn scale_hc_up_shd(&self) -> ScaleHcUpShdR {
        ScaleHcUpShdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn scale_vy_up_shd(&self) -> ScaleVyUpShdR {
        ScaleVyUpShdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn scale_vc_up_shd(&self) -> ScaleVcUpShdR {
        ScaleVcUpShdR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "global control shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mrsz_ctrl_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrszCtrlShdSpec;
impl crate::RegisterSpec for MrszCtrlShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrsz_ctrl_shd::R`](R) reader structure"]
impl crate::Readable for MrszCtrlShdSpec {}
#[doc = "`reset()` method sets MRSZ_CTRL_SHD to value 0"]
impl crate::Resettable for MrszCtrlShdSpec {
    const RESET_VALUE: u32 = 0;
}
