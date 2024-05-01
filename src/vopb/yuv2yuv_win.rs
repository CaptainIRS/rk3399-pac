#[doc = "Register `YUV2YUV_WIN` reader"]
pub type R = crate::R<Yuv2yuvWinSpec>;
#[doc = "Register `YUV2YUV_WIN` writer"]
pub type W = crate::W<Yuv2yuvWinSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0Yuv2yuvEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win0Yuv2yuvEn> for bool {
    #[inline(always)]
    fn from(variant: Win0Yuv2yuvEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_YUV2YUV_EN` reader - "]
pub type Win0Yuv2yuvEnR = crate::BitReader<Win0Yuv2yuvEn>;
impl Win0Yuv2yuvEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0Yuv2yuvEn {
        match self.bits {
            false => Win0Yuv2yuvEn::B0,
            true => Win0Yuv2yuvEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0Yuv2yuvEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0Yuv2yuvEn::B1
    }
}
#[doc = "Field `WIN0_YUV2YUV_EN` writer - "]
pub type Win0Yuv2yuvEnW<'a, REG> = crate::BitWriter<'a, REG, Win0Yuv2yuvEn>;
impl<'a, REG> Win0Yuv2yuvEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0Yuv2yuvEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0Yuv2yuvEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0Yuv2yuvY2rEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win0Yuv2yuvY2rEn> for bool {
    #[inline(always)]
    fn from(variant: Win0Yuv2yuvY2rEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_YUV2YUV_Y2R_EN` reader - "]
pub type Win0Yuv2yuvY2rEnR = crate::BitReader<Win0Yuv2yuvY2rEn>;
impl Win0Yuv2yuvY2rEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0Yuv2yuvY2rEn {
        match self.bits {
            false => Win0Yuv2yuvY2rEn::B0,
            true => Win0Yuv2yuvY2rEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0Yuv2yuvY2rEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0Yuv2yuvY2rEn::B1
    }
}
#[doc = "Field `WIN0_YUV2YUV_Y2R_EN` writer - "]
pub type Win0Yuv2yuvY2rEnW<'a, REG> = crate::BitWriter<'a, REG, Win0Yuv2yuvY2rEn>;
impl<'a, REG> Win0Yuv2yuvY2rEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0Yuv2yuvY2rEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0Yuv2yuvY2rEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0Yuv2yuvR2yEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win0Yuv2yuvR2yEn> for bool {
    #[inline(always)]
    fn from(variant: Win0Yuv2yuvR2yEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_YUV2YUV_R2Y_EN` reader - "]
pub type Win0Yuv2yuvR2yEnR = crate::BitReader<Win0Yuv2yuvR2yEn>;
impl Win0Yuv2yuvR2yEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0Yuv2yuvR2yEn {
        match self.bits {
            false => Win0Yuv2yuvR2yEn::B0,
            true => Win0Yuv2yuvR2yEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0Yuv2yuvR2yEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0Yuv2yuvR2yEn::B1
    }
}
#[doc = "Field `WIN0_YUV2YUV_R2Y_EN` writer - "]
pub type Win0Yuv2yuvR2yEnW<'a, REG> = crate::BitWriter<'a, REG, Win0Yuv2yuvR2yEn>;
impl<'a, REG> Win0Yuv2yuvR2yEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0Yuv2yuvR2yEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0Yuv2yuvR2yEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0Yuv2yuvGammaMode {
    #[doc = "0: bt2020 to bt709 or bt709 to bt2020"]
    B0 = 0,
    #[doc = "1: bt2020 to srgb or srgb to bt2020"]
    B1 = 1,
}
impl From<Win0Yuv2yuvGammaMode> for bool {
    #[inline(always)]
    fn from(variant: Win0Yuv2yuvGammaMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_YUV2YUV_GAMMA_MODE` reader - "]
pub type Win0Yuv2yuvGammaModeR = crate::BitReader<Win0Yuv2yuvGammaMode>;
impl Win0Yuv2yuvGammaModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0Yuv2yuvGammaMode {
        match self.bits {
            false => Win0Yuv2yuvGammaMode::B0,
            true => Win0Yuv2yuvGammaMode::B1,
        }
    }
    #[doc = "bt2020 to bt709 or bt709 to bt2020"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0Yuv2yuvGammaMode::B0
    }
    #[doc = "bt2020 to srgb or srgb to bt2020"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0Yuv2yuvGammaMode::B1
    }
}
#[doc = "Field `WIN0_YUV2YUV_GAMMA_MODE` writer - "]
pub type Win0Yuv2yuvGammaModeW<'a, REG> = crate::BitWriter<'a, REG, Win0Yuv2yuvGammaMode>;
impl<'a, REG> Win0Yuv2yuvGammaModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bt2020 to bt709 or bt709 to bt2020"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0Yuv2yuvGammaMode::B0)
    }
    #[doc = "bt2020 to srgb or srgb to bt2020"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0Yuv2yuvGammaMode::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win0Yuv2yuvY2rMode {
    #[doc = "0: bt601_l"]
    B00 = 0,
    #[doc = "1: bt709_l"]
    B01 = 1,
    #[doc = "2: bt601_f"]
    B10 = 2,
    #[doc = "3: bt2020"]
    B11 = 3,
}
impl From<Win0Yuv2yuvY2rMode> for u8 {
    #[inline(always)]
    fn from(variant: Win0Yuv2yuvY2rMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win0Yuv2yuvY2rMode {
    type Ux = u8;
}
#[doc = "Field `WIN0_YUV2YUV_Y2R_MODE` reader - "]
pub type Win0Yuv2yuvY2rModeR = crate::FieldReader<Win0Yuv2yuvY2rMode>;
impl Win0Yuv2yuvY2rModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0Yuv2yuvY2rMode {
        match self.bits {
            0 => Win0Yuv2yuvY2rMode::B00,
            1 => Win0Yuv2yuvY2rMode::B01,
            2 => Win0Yuv2yuvY2rMode::B10,
            3 => Win0Yuv2yuvY2rMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "bt601_l"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win0Yuv2yuvY2rMode::B00
    }
    #[doc = "bt709_l"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win0Yuv2yuvY2rMode::B01
    }
    #[doc = "bt601_f"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win0Yuv2yuvY2rMode::B10
    }
    #[doc = "bt2020"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win0Yuv2yuvY2rMode::B11
    }
}
#[doc = "Field `WIN0_YUV2YUV_Y2R_MODE` writer - "]
pub type Win0Yuv2yuvY2rModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win0Yuv2yuvY2rMode>;
impl<'a, REG> Win0Yuv2yuvY2rModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "bt601_l"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win0Yuv2yuvY2rMode::B00)
    }
    #[doc = "bt709_l"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win0Yuv2yuvY2rMode::B01)
    }
    #[doc = "bt601_f"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win0Yuv2yuvY2rMode::B10)
    }
    #[doc = "bt2020"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win0Yuv2yuvY2rMode::B11)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win0Yuv2yuvR2yMode {
    #[doc = "0: bt601_l"]
    B00 = 0,
    #[doc = "1: bt709_l"]
    B01 = 1,
    #[doc = "2: bt601_f"]
    B10 = 2,
    #[doc = "3: bt2020"]
    B11 = 3,
}
impl From<Win0Yuv2yuvR2yMode> for u8 {
    #[inline(always)]
    fn from(variant: Win0Yuv2yuvR2yMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win0Yuv2yuvR2yMode {
    type Ux = u8;
}
#[doc = "Field `WIN0_YUV2YUV_R2Y_MODE` reader - "]
pub type Win0Yuv2yuvR2yModeR = crate::FieldReader<Win0Yuv2yuvR2yMode>;
impl Win0Yuv2yuvR2yModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0Yuv2yuvR2yMode {
        match self.bits {
            0 => Win0Yuv2yuvR2yMode::B00,
            1 => Win0Yuv2yuvR2yMode::B01,
            2 => Win0Yuv2yuvR2yMode::B10,
            3 => Win0Yuv2yuvR2yMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "bt601_l"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win0Yuv2yuvR2yMode::B00
    }
    #[doc = "bt709_l"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win0Yuv2yuvR2yMode::B01
    }
    #[doc = "bt601_f"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win0Yuv2yuvR2yMode::B10
    }
    #[doc = "bt2020"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win0Yuv2yuvR2yMode::B11
    }
}
#[doc = "Field `WIN0_YUV2YUV_R2Y_MODE` writer - "]
pub type Win0Yuv2yuvR2yModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win0Yuv2yuvR2yMode>;
impl<'a, REG> Win0Yuv2yuvR2yModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "bt601_l"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win0Yuv2yuvR2yMode::B00)
    }
    #[doc = "bt709_l"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win0Yuv2yuvR2yMode::B01)
    }
    #[doc = "bt601_f"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win0Yuv2yuvR2yMode::B10)
    }
    #[doc = "bt2020"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win0Yuv2yuvR2yMode::B11)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1Yuv2yuvEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win1Yuv2yuvEn> for bool {
    #[inline(always)]
    fn from(variant: Win1Yuv2yuvEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_YUV2YUV_EN` reader - "]
pub type Win1Yuv2yuvEnR = crate::BitReader<Win1Yuv2yuvEn>;
impl Win1Yuv2yuvEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1Yuv2yuvEn {
        match self.bits {
            false => Win1Yuv2yuvEn::B0,
            true => Win1Yuv2yuvEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1Yuv2yuvEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1Yuv2yuvEn::B1
    }
}
#[doc = "Field `WIN1_YUV2YUV_EN` writer - "]
pub type Win1Yuv2yuvEnW<'a, REG> = crate::BitWriter<'a, REG, Win1Yuv2yuvEn>;
impl<'a, REG> Win1Yuv2yuvEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1Yuv2yuvEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1Yuv2yuvEn::B1)
    }
}
#[doc = "Field `WIN1_YUV2YUV_Y2R_EN` reader - win1_yuv2yuv_y2r_en"]
pub type Win1Yuv2yuvY2rEnR = crate::BitReader;
#[doc = "Field `WIN1_YUV2YUV_Y2R_EN` writer - win1_yuv2yuv_y2r_en"]
pub type Win1Yuv2yuvY2rEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1Yuv2yuvR2yEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win1Yuv2yuvR2yEn> for bool {
    #[inline(always)]
    fn from(variant: Win1Yuv2yuvR2yEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_YUV2YUV_R2Y_EN` reader - "]
pub type Win1Yuv2yuvR2yEnR = crate::BitReader<Win1Yuv2yuvR2yEn>;
impl Win1Yuv2yuvR2yEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1Yuv2yuvR2yEn {
        match self.bits {
            false => Win1Yuv2yuvR2yEn::B0,
            true => Win1Yuv2yuvR2yEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1Yuv2yuvR2yEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1Yuv2yuvR2yEn::B1
    }
}
#[doc = "Field `WIN1_YUV2YUV_R2Y_EN` writer - "]
pub type Win1Yuv2yuvR2yEnW<'a, REG> = crate::BitWriter<'a, REG, Win1Yuv2yuvR2yEn>;
impl<'a, REG> Win1Yuv2yuvR2yEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1Yuv2yuvR2yEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1Yuv2yuvR2yEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1Yuv2yuvGammaMode {
    #[doc = "0: bt2020 to bt709 or bt709 to bt2020"]
    B0 = 0,
    #[doc = "1: bt2020 to srgb or srgb to bt2020"]
    B1 = 1,
}
impl From<Win1Yuv2yuvGammaMode> for bool {
    #[inline(always)]
    fn from(variant: Win1Yuv2yuvGammaMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_YUV2YUV_GAMMA_MODE` reader - "]
pub type Win1Yuv2yuvGammaModeR = crate::BitReader<Win1Yuv2yuvGammaMode>;
impl Win1Yuv2yuvGammaModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1Yuv2yuvGammaMode {
        match self.bits {
            false => Win1Yuv2yuvGammaMode::B0,
            true => Win1Yuv2yuvGammaMode::B1,
        }
    }
    #[doc = "bt2020 to bt709 or bt709 to bt2020"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1Yuv2yuvGammaMode::B0
    }
    #[doc = "bt2020 to srgb or srgb to bt2020"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1Yuv2yuvGammaMode::B1
    }
}
#[doc = "Field `WIN1_YUV2YUV_GAMMA_MODE` writer - "]
pub type Win1Yuv2yuvGammaModeW<'a, REG> = crate::BitWriter<'a, REG, Win1Yuv2yuvGammaMode>;
impl<'a, REG> Win1Yuv2yuvGammaModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bt2020 to bt709 or bt709 to bt2020"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1Yuv2yuvGammaMode::B0)
    }
    #[doc = "bt2020 to srgb or srgb to bt2020"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1Yuv2yuvGammaMode::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win1Yuv2yuvY2rMode {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win1Yuv2yuvY2rMode> for u8 {
    #[inline(always)]
    fn from(variant: Win1Yuv2yuvY2rMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win1Yuv2yuvY2rMode {
    type Ux = u8;
}
#[doc = "Field `WIN1_YUV2YUV_Y2R_MODE` reader - "]
pub type Win1Yuv2yuvY2rModeR = crate::FieldReader<Win1Yuv2yuvY2rMode>;
impl Win1Yuv2yuvY2rModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Win1Yuv2yuvY2rMode> {
        match self.bits {
            0 => Some(Win1Yuv2yuvY2rMode::B0),
            1 => Some(Win1Yuv2yuvY2rMode::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1Yuv2yuvY2rMode::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1Yuv2yuvY2rMode::B1
    }
}
#[doc = "Field `WIN1_YUV2YUV_Y2R_MODE` writer - "]
pub type Win1Yuv2yuvY2rModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Win1Yuv2yuvY2rMode>;
impl<'a, REG> Win1Yuv2yuvY2rModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1Yuv2yuvY2rMode::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1Yuv2yuvY2rMode::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win1Yuv2yuvR2yMode {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win1Yuv2yuvR2yMode> for u8 {
    #[inline(always)]
    fn from(variant: Win1Yuv2yuvR2yMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win1Yuv2yuvR2yMode {
    type Ux = u8;
}
#[doc = "Field `WIN1_YUV2YUV_R2Y_MODE` reader - "]
pub type Win1Yuv2yuvR2yModeR = crate::FieldReader<Win1Yuv2yuvR2yMode>;
impl Win1Yuv2yuvR2yModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Win1Yuv2yuvR2yMode> {
        match self.bits {
            0 => Some(Win1Yuv2yuvR2yMode::B0),
            1 => Some(Win1Yuv2yuvR2yMode::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1Yuv2yuvR2yMode::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1Yuv2yuvR2yMode::B1
    }
}
#[doc = "Field `WIN1_YUV2YUV_R2Y_MODE` writer - "]
pub type Win1Yuv2yuvR2yModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Win1Yuv2yuvR2yMode>;
impl<'a, REG> Win1Yuv2yuvR2yModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1Yuv2yuvR2yMode::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1Yuv2yuvR2yMode::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2Yuv2yuvEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win2Yuv2yuvEn> for bool {
    #[inline(always)]
    fn from(variant: Win2Yuv2yuvEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_YUV2YUV_EN` reader - "]
pub type Win2Yuv2yuvEnR = crate::BitReader<Win2Yuv2yuvEn>;
impl Win2Yuv2yuvEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2Yuv2yuvEn {
        match self.bits {
            false => Win2Yuv2yuvEn::B0,
            true => Win2Yuv2yuvEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2Yuv2yuvEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2Yuv2yuvEn::B1
    }
}
#[doc = "Field `WIN2_YUV2YUV_EN` writer - "]
pub type Win2Yuv2yuvEnW<'a, REG> = crate::BitWriter<'a, REG, Win2Yuv2yuvEn>;
impl<'a, REG> Win2Yuv2yuvEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2Yuv2yuvEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2Yuv2yuvEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2Yuv2yuvR2yEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win2Yuv2yuvR2yEn> for bool {
    #[inline(always)]
    fn from(variant: Win2Yuv2yuvR2yEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_YUV2YUV_R2Y_EN` reader - "]
pub type Win2Yuv2yuvR2yEnR = crate::BitReader<Win2Yuv2yuvR2yEn>;
impl Win2Yuv2yuvR2yEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2Yuv2yuvR2yEn {
        match self.bits {
            false => Win2Yuv2yuvR2yEn::B0,
            true => Win2Yuv2yuvR2yEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2Yuv2yuvR2yEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2Yuv2yuvR2yEn::B1
    }
}
#[doc = "Field `WIN2_YUV2YUV_R2Y_EN` writer - "]
pub type Win2Yuv2yuvR2yEnW<'a, REG> = crate::BitWriter<'a, REG, Win2Yuv2yuvR2yEn>;
impl<'a, REG> Win2Yuv2yuvR2yEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2Yuv2yuvR2yEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2Yuv2yuvR2yEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2Yuv2yuvGammaMode {
    #[doc = "0: bt2020 to bt709 or bt709 to bt2020"]
    B0 = 0,
    #[doc = "1: bt2020 to srgb or srgb to bt2020"]
    B1 = 1,
}
impl From<Win2Yuv2yuvGammaMode> for bool {
    #[inline(always)]
    fn from(variant: Win2Yuv2yuvGammaMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_YUV2YUV_GAMMA_MODE` reader - "]
pub type Win2Yuv2yuvGammaModeR = crate::BitReader<Win2Yuv2yuvGammaMode>;
impl Win2Yuv2yuvGammaModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2Yuv2yuvGammaMode {
        match self.bits {
            false => Win2Yuv2yuvGammaMode::B0,
            true => Win2Yuv2yuvGammaMode::B1,
        }
    }
    #[doc = "bt2020 to bt709 or bt709 to bt2020"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2Yuv2yuvGammaMode::B0
    }
    #[doc = "bt2020 to srgb or srgb to bt2020"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2Yuv2yuvGammaMode::B1
    }
}
#[doc = "Field `WIN2_YUV2YUV_GAMMA_MODE` writer - "]
pub type Win2Yuv2yuvGammaModeW<'a, REG> = crate::BitWriter<'a, REG, Win2Yuv2yuvGammaMode>;
impl<'a, REG> Win2Yuv2yuvGammaModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bt2020 to bt709 or bt709 to bt2020"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2Yuv2yuvGammaMode::B0)
    }
    #[doc = "bt2020 to srgb or srgb to bt2020"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2Yuv2yuvGammaMode::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win2Yuv2yuvR2yMode {
    #[doc = "0: bt601_l"]
    B00 = 0,
    #[doc = "1: bt709_l"]
    B01 = 1,
    #[doc = "2: bt601_f"]
    B10 = 2,
    #[doc = "3: bt2020"]
    B11 = 3,
}
impl From<Win2Yuv2yuvR2yMode> for u8 {
    #[inline(always)]
    fn from(variant: Win2Yuv2yuvR2yMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win2Yuv2yuvR2yMode {
    type Ux = u8;
}
#[doc = "Field `WIN2_YUV2YUV_R2Y_MODE` reader - "]
pub type Win2Yuv2yuvR2yModeR = crate::FieldReader<Win2Yuv2yuvR2yMode>;
impl Win2Yuv2yuvR2yModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2Yuv2yuvR2yMode {
        match self.bits {
            0 => Win2Yuv2yuvR2yMode::B00,
            1 => Win2Yuv2yuvR2yMode::B01,
            2 => Win2Yuv2yuvR2yMode::B10,
            3 => Win2Yuv2yuvR2yMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "bt601_l"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win2Yuv2yuvR2yMode::B00
    }
    #[doc = "bt709_l"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win2Yuv2yuvR2yMode::B01
    }
    #[doc = "bt601_f"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win2Yuv2yuvR2yMode::B10
    }
    #[doc = "bt2020"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win2Yuv2yuvR2yMode::B11
    }
}
#[doc = "Field `WIN2_YUV2YUV_R2Y_MODE` writer - "]
pub type Win2Yuv2yuvR2yModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win2Yuv2yuvR2yMode>;
impl<'a, REG> Win2Yuv2yuvR2yModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "bt601_l"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win2Yuv2yuvR2yMode::B00)
    }
    #[doc = "bt709_l"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win2Yuv2yuvR2yMode::B01)
    }
    #[doc = "bt601_f"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win2Yuv2yuvR2yMode::B10)
    }
    #[doc = "bt2020"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win2Yuv2yuvR2yMode::B11)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3Yuv2yuvEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win3Yuv2yuvEn> for bool {
    #[inline(always)]
    fn from(variant: Win3Yuv2yuvEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_YUV2YUV_EN` reader - "]
pub type Win3Yuv2yuvEnR = crate::BitReader<Win3Yuv2yuvEn>;
impl Win3Yuv2yuvEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3Yuv2yuvEn {
        match self.bits {
            false => Win3Yuv2yuvEn::B0,
            true => Win3Yuv2yuvEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3Yuv2yuvEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3Yuv2yuvEn::B1
    }
}
#[doc = "Field `WIN3_YUV2YUV_EN` writer - "]
pub type Win3Yuv2yuvEnW<'a, REG> = crate::BitWriter<'a, REG, Win3Yuv2yuvEn>;
impl<'a, REG> Win3Yuv2yuvEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3Yuv2yuvEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3Yuv2yuvEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3Yuv2yuvR2yEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win3Yuv2yuvR2yEn> for bool {
    #[inline(always)]
    fn from(variant: Win3Yuv2yuvR2yEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_YUV2YUV_R2Y_EN` reader - "]
pub type Win3Yuv2yuvR2yEnR = crate::BitReader<Win3Yuv2yuvR2yEn>;
impl Win3Yuv2yuvR2yEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3Yuv2yuvR2yEn {
        match self.bits {
            false => Win3Yuv2yuvR2yEn::B0,
            true => Win3Yuv2yuvR2yEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3Yuv2yuvR2yEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3Yuv2yuvR2yEn::B1
    }
}
#[doc = "Field `WIN3_YUV2YUV_R2Y_EN` writer - "]
pub type Win3Yuv2yuvR2yEnW<'a, REG> = crate::BitWriter<'a, REG, Win3Yuv2yuvR2yEn>;
impl<'a, REG> Win3Yuv2yuvR2yEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3Yuv2yuvR2yEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3Yuv2yuvR2yEn::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3Yuv2yuvGammaMode {
    #[doc = "0: bt2020 to bt709 or bt709 to bt2020"]
    B0 = 0,
    #[doc = "1: bt2020 to srgb or srgb to bt2020"]
    B1 = 1,
}
impl From<Win3Yuv2yuvGammaMode> for bool {
    #[inline(always)]
    fn from(variant: Win3Yuv2yuvGammaMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_YUV2YUV_GAMMA_MODE` reader - "]
pub type Win3Yuv2yuvGammaModeR = crate::BitReader<Win3Yuv2yuvGammaMode>;
impl Win3Yuv2yuvGammaModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3Yuv2yuvGammaMode {
        match self.bits {
            false => Win3Yuv2yuvGammaMode::B0,
            true => Win3Yuv2yuvGammaMode::B1,
        }
    }
    #[doc = "bt2020 to bt709 or bt709 to bt2020"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3Yuv2yuvGammaMode::B0
    }
    #[doc = "bt2020 to srgb or srgb to bt2020"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3Yuv2yuvGammaMode::B1
    }
}
#[doc = "Field `WIN3_YUV2YUV_GAMMA_MODE` writer - "]
pub type Win3Yuv2yuvGammaModeW<'a, REG> = crate::BitWriter<'a, REG, Win3Yuv2yuvGammaMode>;
impl<'a, REG> Win3Yuv2yuvGammaModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bt2020 to bt709 or bt709 to bt2020"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3Yuv2yuvGammaMode::B0)
    }
    #[doc = "bt2020 to srgb or srgb to bt2020"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3Yuv2yuvGammaMode::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win3Yuv2yuvR2yMode {
    #[doc = "0: bt601_l"]
    B00 = 0,
    #[doc = "1: bt709_l"]
    B01 = 1,
    #[doc = "2: bt601_f"]
    B10 = 2,
    #[doc = "3: bt2020"]
    B11 = 3,
}
impl From<Win3Yuv2yuvR2yMode> for u8 {
    #[inline(always)]
    fn from(variant: Win3Yuv2yuvR2yMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win3Yuv2yuvR2yMode {
    type Ux = u8;
}
#[doc = "Field `WIN3_YUV2YUV_R2Y_MODE` reader - "]
pub type Win3Yuv2yuvR2yModeR = crate::FieldReader<Win3Yuv2yuvR2yMode>;
impl Win3Yuv2yuvR2yModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3Yuv2yuvR2yMode {
        match self.bits {
            0 => Win3Yuv2yuvR2yMode::B00,
            1 => Win3Yuv2yuvR2yMode::B01,
            2 => Win3Yuv2yuvR2yMode::B10,
            3 => Win3Yuv2yuvR2yMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "bt601_l"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win3Yuv2yuvR2yMode::B00
    }
    #[doc = "bt709_l"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win3Yuv2yuvR2yMode::B01
    }
    #[doc = "bt601_f"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win3Yuv2yuvR2yMode::B10
    }
    #[doc = "bt2020"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win3Yuv2yuvR2yMode::B11
    }
}
#[doc = "Field `WIN3_YUV2YUV_R2Y_MODE` writer - "]
pub type Win3Yuv2yuvR2yModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win3Yuv2yuvR2yMode>;
impl<'a, REG> Win3Yuv2yuvR2yModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "bt601_l"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win3Yuv2yuvR2yMode::B00)
    }
    #[doc = "bt709_l"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win3Yuv2yuvR2yMode::B01)
    }
    #[doc = "bt601_f"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win3Yuv2yuvR2yMode::B10)
    }
    #[doc = "bt2020"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win3Yuv2yuvR2yMode::B11)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn win0_yuv2yuv_en(&self) -> Win0Yuv2yuvEnR {
        Win0Yuv2yuvEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn win0_yuv2yuv_y2r_en(&self) -> Win0Yuv2yuvY2rEnR {
        Win0Yuv2yuvY2rEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn win0_yuv2yuv_r2y_en(&self) -> Win0Yuv2yuvR2yEnR {
        Win0Yuv2yuvR2yEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn win0_yuv2yuv_gamma_mode(&self) -> Win0Yuv2yuvGammaModeR {
        Win0Yuv2yuvGammaModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn win0_yuv2yuv_y2r_mode(&self) -> Win0Yuv2yuvY2rModeR {
        Win0Yuv2yuvY2rModeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn win0_yuv2yuv_r2y_mode(&self) -> Win0Yuv2yuvR2yModeR {
        Win0Yuv2yuvR2yModeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn win1_yuv2yuv_en(&self) -> Win1Yuv2yuvEnR {
        Win1Yuv2yuvEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - win1_yuv2yuv_y2r_en"]
    #[inline(always)]
    pub fn win1_yuv2yuv_y2r_en(&self) -> Win1Yuv2yuvY2rEnR {
        Win1Yuv2yuvY2rEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn win1_yuv2yuv_r2y_en(&self) -> Win1Yuv2yuvR2yEnR {
        Win1Yuv2yuvR2yEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn win1_yuv2yuv_gamma_mode(&self) -> Win1Yuv2yuvGammaModeR {
        Win1Yuv2yuvGammaModeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn win1_yuv2yuv_y2r_mode(&self) -> Win1Yuv2yuvY2rModeR {
        Win1Yuv2yuvY2rModeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn win1_yuv2yuv_r2y_mode(&self) -> Win1Yuv2yuvR2yModeR {
        Win1Yuv2yuvR2yModeR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn win2_yuv2yuv_en(&self) -> Win2Yuv2yuvEnR {
        Win2Yuv2yuvEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn win2_yuv2yuv_r2y_en(&self) -> Win2Yuv2yuvR2yEnR {
        Win2Yuv2yuvR2yEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn win2_yuv2yuv_gamma_mode(&self) -> Win2Yuv2yuvGammaModeR {
        Win2Yuv2yuvGammaModeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn win2_yuv2yuv_r2y_mode(&self) -> Win2Yuv2yuvR2yModeR {
        Win2Yuv2yuvR2yModeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn win3_yuv2yuv_en(&self) -> Win3Yuv2yuvEnR {
        Win3Yuv2yuvEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn win3_yuv2yuv_r2y_en(&self) -> Win3Yuv2yuvR2yEnR {
        Win3Yuv2yuvR2yEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn win3_yuv2yuv_gamma_mode(&self) -> Win3Yuv2yuvGammaModeR {
        Win3Yuv2yuvGammaModeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn win3_yuv2yuv_r2y_mode(&self) -> Win3Yuv2yuvR2yModeR {
        Win3Yuv2yuvR2yModeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn win0_yuv2yuv_en(&mut self) -> Win0Yuv2yuvEnW<Yuv2yuvWinSpec> {
        Win0Yuv2yuvEnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn win0_yuv2yuv_y2r_en(&mut self) -> Win0Yuv2yuvY2rEnW<Yuv2yuvWinSpec> {
        Win0Yuv2yuvY2rEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn win0_yuv2yuv_r2y_en(&mut self) -> Win0Yuv2yuvR2yEnW<Yuv2yuvWinSpec> {
        Win0Yuv2yuvR2yEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn win0_yuv2yuv_gamma_mode(&mut self) -> Win0Yuv2yuvGammaModeW<Yuv2yuvWinSpec> {
        Win0Yuv2yuvGammaModeW::new(self, 3)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn win0_yuv2yuv_y2r_mode(&mut self) -> Win0Yuv2yuvY2rModeW<Yuv2yuvWinSpec> {
        Win0Yuv2yuvY2rModeW::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn win0_yuv2yuv_r2y_mode(&mut self) -> Win0Yuv2yuvR2yModeW<Yuv2yuvWinSpec> {
        Win0Yuv2yuvR2yModeW::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn win1_yuv2yuv_en(&mut self) -> Win1Yuv2yuvEnW<Yuv2yuvWinSpec> {
        Win1Yuv2yuvEnW::new(self, 8)
    }
    #[doc = "Bit 9 - win1_yuv2yuv_y2r_en"]
    #[inline(always)]
    #[must_use]
    pub fn win1_yuv2yuv_y2r_en(&mut self) -> Win1Yuv2yuvY2rEnW<Yuv2yuvWinSpec> {
        Win1Yuv2yuvY2rEnW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn win1_yuv2yuv_r2y_en(&mut self) -> Win1Yuv2yuvR2yEnW<Yuv2yuvWinSpec> {
        Win1Yuv2yuvR2yEnW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn win1_yuv2yuv_gamma_mode(&mut self) -> Win1Yuv2yuvGammaModeW<Yuv2yuvWinSpec> {
        Win1Yuv2yuvGammaModeW::new(self, 11)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn win1_yuv2yuv_y2r_mode(&mut self) -> Win1Yuv2yuvY2rModeW<Yuv2yuvWinSpec> {
        Win1Yuv2yuvY2rModeW::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn win1_yuv2yuv_r2y_mode(&mut self) -> Win1Yuv2yuvR2yModeW<Yuv2yuvWinSpec> {
        Win1Yuv2yuvR2yModeW::new(self, 14)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn win2_yuv2yuv_en(&mut self) -> Win2Yuv2yuvEnW<Yuv2yuvWinSpec> {
        Win2Yuv2yuvEnW::new(self, 16)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn win2_yuv2yuv_r2y_en(&mut self) -> Win2Yuv2yuvR2yEnW<Yuv2yuvWinSpec> {
        Win2Yuv2yuvR2yEnW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn win2_yuv2yuv_gamma_mode(&mut self) -> Win2Yuv2yuvGammaModeW<Yuv2yuvWinSpec> {
        Win2Yuv2yuvGammaModeW::new(self, 19)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    #[must_use]
    pub fn win2_yuv2yuv_r2y_mode(&mut self) -> Win2Yuv2yuvR2yModeW<Yuv2yuvWinSpec> {
        Win2Yuv2yuvR2yModeW::new(self, 22)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn win3_yuv2yuv_en(&mut self) -> Win3Yuv2yuvEnW<Yuv2yuvWinSpec> {
        Win3Yuv2yuvEnW::new(self, 24)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn win3_yuv2yuv_r2y_en(&mut self) -> Win3Yuv2yuvR2yEnW<Yuv2yuvWinSpec> {
        Win3Yuv2yuvR2yEnW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn win3_yuv2yuv_gamma_mode(&mut self) -> Win3Yuv2yuvGammaModeW<Yuv2yuvWinSpec> {
        Win3Yuv2yuvGammaModeW::new(self, 27)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn win3_yuv2yuv_r2y_mode(&mut self) -> Win3Yuv2yuvR2yModeW<Yuv2yuvWinSpec> {
        Win3Yuv2yuvR2yModeW::new(self, 30)
    }
}
#[doc = "win yuv2yuv control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`yuv2yuv_win::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`yuv2yuv_win::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Yuv2yuvWinSpec;
impl crate::RegisterSpec for Yuv2yuvWinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`yuv2yuv_win::R`](R) reader structure"]
impl crate::Readable for Yuv2yuvWinSpec {}
#[doc = "`write(|w| ..)` method takes [`yuv2yuv_win::W`](W) writer structure"]
impl crate::Writable for Yuv2yuvWinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets YUV2YUV_WIN to value 0"]
impl crate::Resettable for Yuv2yuvWinSpec {
    const RESET_VALUE: u32 = 0;
}
