#[doc = "Register `ALPHA_CTRL1` reader"]
pub type R = crate::R<AlphaCtrl1Spec>;
#[doc = "Register `ALPHA_CTRL1` writer"]
pub type W = crate::W<AlphaCtrl1Spec>;
#[doc = "SRC color select(Cd’)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDstColorM0 {
    #[doc = "0: Cd"]
    B0 = 0,
    #[doc = "1: Cd * Ad0’’"]
    B1 = 1,
}
impl From<SwDstColorM0> for bool {
    #[inline(always)]
    fn from(variant: SwDstColorM0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DST_COLOR_M0` reader - SRC color select(Cd’)"]
pub type SwDstColorM0R = crate::BitReader<SwDstColorM0>;
impl SwDstColorM0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDstColorM0 {
        match self.bits {
            false => SwDstColorM0::B0,
            true => SwDstColorM0::B1,
        }
    }
    #[doc = "Cd"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDstColorM0::B0
    }
    #[doc = "Cd * Ad0’’"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDstColorM0::B1
    }
}
#[doc = "Field `SW_DST_COLOR_M0` writer - SRC color select(Cd’)"]
pub type SwDstColorM0W<'a, REG> = crate::BitWriter<'a, REG, SwDstColorM0>;
impl<'a, REG> SwDstColorM0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cd"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstColorM0::B0)
    }
    #[doc = "Cd * Ad0’’"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstColorM0::B1)
    }
}
#[doc = "SRC color select(Cs’)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwSrcColorM0 {
    #[doc = "0: Cs"]
    B0 = 0,
    #[doc = "1: Cs * As0’’"]
    B1 = 1,
}
impl From<SwSrcColorM0> for bool {
    #[inline(always)]
    fn from(variant: SwSrcColorM0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SRC_COLOR_M0` reader - SRC color select(Cs’)"]
pub type SwSrcColorM0R = crate::BitReader<SwSrcColorM0>;
impl SwSrcColorM0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrcColorM0 {
        match self.bits {
            false => SwSrcColorM0::B0,
            true => SwSrcColorM0::B1,
        }
    }
    #[doc = "Cs"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwSrcColorM0::B0
    }
    #[doc = "Cs * As0’’"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwSrcColorM0::B1
    }
}
#[doc = "Field `SW_SRC_COLOR_M0` writer - SRC color select(Cs’)"]
pub type SwSrcColorM0W<'a, REG> = crate::BitWriter<'a, REG, SwSrcColorM0>;
impl<'a, REG> SwSrcColorM0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cs"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcColorM0::B0)
    }
    #[doc = "Cs * As0’’"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcColorM0::B1)
    }
}
#[doc = "Dst factore mode of color channel(Fd0)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwDstFactorM0 {
    #[doc = "0: 0"]
    B000 = 0,
    #[doc = "1: 256"]
    B001 = 1,
    #[doc = "2: As0’’"]
    B010 = 2,
    #[doc = "3: 256-As0’’"]
    B011 = 3,
    #[doc = "4: Ad0’’"]
    B100 = 4,
}
impl From<SwDstFactorM0> for u8 {
    #[inline(always)]
    fn from(variant: SwDstFactorM0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwDstFactorM0 {
    type Ux = u8;
}
#[doc = "Field `SW_DST_FACTOR_M0` reader - Dst factore mode of color channel(Fd0)"]
pub type SwDstFactorM0R = crate::FieldReader<SwDstFactorM0>;
impl SwDstFactorM0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SwDstFactorM0> {
        match self.bits {
            0 => Some(SwDstFactorM0::B000),
            1 => Some(SwDstFactorM0::B001),
            2 => Some(SwDstFactorM0::B010),
            3 => Some(SwDstFactorM0::B011),
            4 => Some(SwDstFactorM0::B100),
            _ => None,
        }
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == SwDstFactorM0::B000
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == SwDstFactorM0::B001
    }
    #[doc = "As0’’"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == SwDstFactorM0::B010
    }
    #[doc = "256-As0’’"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == SwDstFactorM0::B011
    }
    #[doc = "Ad0’’"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == SwDstFactorM0::B100
    }
}
#[doc = "Field `SW_DST_FACTOR_M0` writer - Dst factore mode of color channel(Fd0)"]
pub type SwDstFactorM0W<'a, REG> = crate::FieldWriter<'a, REG, 3, SwDstFactorM0>;
impl<'a, REG> SwDstFactorM0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFactorM0::B000)
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFactorM0::B001)
    }
    #[doc = "As0’’"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFactorM0::B010)
    }
    #[doc = "256-As0’’"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFactorM0::B011)
    }
    #[doc = "Ad0’’"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFactorM0::B100)
    }
}
#[doc = "Src factore mode of color channel(Fs0)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwSrcFactorM0 {
    #[doc = "0: 0"]
    B000 = 0,
    #[doc = "1: 256"]
    B001 = 1,
    #[doc = "2: Ad0’’"]
    B010 = 2,
    #[doc = "3: 256-Ad0’’"]
    B011 = 3,
    #[doc = "4: As0’’"]
    B100 = 4,
}
impl From<SwSrcFactorM0> for u8 {
    #[inline(always)]
    fn from(variant: SwSrcFactorM0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwSrcFactorM0 {
    type Ux = u8;
}
#[doc = "Field `SW_SRC_FACTOR_M0` reader - Src factore mode of color channel(Fs0)"]
pub type SwSrcFactorM0R = crate::FieldReader<SwSrcFactorM0>;
impl SwSrcFactorM0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SwSrcFactorM0> {
        match self.bits {
            0 => Some(SwSrcFactorM0::B000),
            1 => Some(SwSrcFactorM0::B001),
            2 => Some(SwSrcFactorM0::B010),
            3 => Some(SwSrcFactorM0::B011),
            4 => Some(SwSrcFactorM0::B100),
            _ => None,
        }
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == SwSrcFactorM0::B000
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == SwSrcFactorM0::B001
    }
    #[doc = "Ad0’’"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == SwSrcFactorM0::B010
    }
    #[doc = "256-Ad0’’"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == SwSrcFactorM0::B011
    }
    #[doc = "As0’’"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == SwSrcFactorM0::B100
    }
}
#[doc = "Field `SW_SRC_FACTOR_M0` writer - Src factore mode of color channel(Fs0)"]
pub type SwSrcFactorM0W<'a, REG> = crate::FieldWriter<'a, REG, 3, SwSrcFactorM0>;
impl<'a, REG> SwSrcFactorM0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcFactorM0::B000)
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcFactorM0::B001)
    }
    #[doc = "Ad0’’"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcFactorM0::B010)
    }
    #[doc = "256-Ad0’’"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcFactorM0::B011)
    }
    #[doc = "As0’’"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcFactorM0::B100)
    }
}
#[doc = "Alpha dst calculate mode of color channel(Ad0’’)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDstAlphaCalM0 {
    #[doc = "0: Ad0’’= Ad0_’’ + (Ad0_’’>>7)"]
    B0 = 0,
    #[doc = "1: Ad0’’= Ad0_’’"]
    B1 = 1,
}
impl From<SwDstAlphaCalM0> for bool {
    #[inline(always)]
    fn from(variant: SwDstAlphaCalM0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DST_ALPHA_CAL_M0` reader - Alpha dst calculate mode of color channel(Ad0’’)"]
pub type SwDstAlphaCalM0R = crate::BitReader<SwDstAlphaCalM0>;
impl SwDstAlphaCalM0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDstAlphaCalM0 {
        match self.bits {
            false => SwDstAlphaCalM0::B0,
            true => SwDstAlphaCalM0::B1,
        }
    }
    #[doc = "Ad0’’= Ad0_’’ + (Ad0_’’>>7)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDstAlphaCalM0::B0
    }
    #[doc = "Ad0’’= Ad0_’’"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDstAlphaCalM0::B1
    }
}
#[doc = "Field `SW_DST_ALPHA_CAL_M0` writer - Alpha dst calculate mode of color channel(Ad0’’)"]
pub type SwDstAlphaCalM0W<'a, REG> = crate::BitWriter<'a, REG, SwDstAlphaCalM0>;
impl<'a, REG> SwDstAlphaCalM0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ad0’’= Ad0_’’ + (Ad0_’’>>7)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstAlphaCalM0::B0)
    }
    #[doc = "Ad0’’= Ad0_’’"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstAlphaCalM0::B1)
    }
}
#[doc = "Alpha src calculate mode of color channel(As0’’)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwSrcAlphaCalM0 {
    #[doc = "0: As0’’= As0_’’+ (As0_’’>>7)"]
    B0 = 0,
    #[doc = "1: As0’’= As0 _’’"]
    B1 = 1,
}
impl From<SwSrcAlphaCalM0> for bool {
    #[inline(always)]
    fn from(variant: SwSrcAlphaCalM0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SRC_ALPHA_CAL_M0` reader - Alpha src calculate mode of color channel(As0’’)"]
pub type SwSrcAlphaCalM0R = crate::BitReader<SwSrcAlphaCalM0>;
impl SwSrcAlphaCalM0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrcAlphaCalM0 {
        match self.bits {
            false => SwSrcAlphaCalM0::B0,
            true => SwSrcAlphaCalM0::B1,
        }
    }
    #[doc = "As0’’= As0_’’+ (As0_’’>>7)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwSrcAlphaCalM0::B0
    }
    #[doc = "As0’’= As0 _’’"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwSrcAlphaCalM0::B1
    }
}
#[doc = "Field `SW_SRC_ALPHA_CAL_M0` writer - Alpha src calculate mode of color channel(As0’’)"]
pub type SwSrcAlphaCalM0W<'a, REG> = crate::BitWriter<'a, REG, SwSrcAlphaCalM0>;
impl<'a, REG> SwSrcAlphaCalM0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "As0’’= As0_’’+ (As0_’’>>7)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcAlphaCalM0::B0)
    }
    #[doc = "As0’’= As0 _’’"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcAlphaCalM0::B1)
    }
}
#[doc = "Alpha dst blend mode select of color channel(Ad0_’’)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwDstBlendM0 {
    #[doc = "0: Agd"]
    B00 = 0,
    #[doc = "1: Ad0’"]
    B01 = 1,
    #[doc = "2: (Ad0’*Agd)>>8"]
    B10 = 2,
}
impl From<SwDstBlendM0> for u8 {
    #[inline(always)]
    fn from(variant: SwDstBlendM0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwDstBlendM0 {
    type Ux = u8;
}
#[doc = "Field `SW_DST_BLEND_M0` reader - Alpha dst blend mode select of color channel(Ad0_’’)"]
pub type SwDstBlendM0R = crate::FieldReader<SwDstBlendM0>;
impl SwDstBlendM0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SwDstBlendM0> {
        match self.bits {
            0 => Some(SwDstBlendM0::B00),
            1 => Some(SwDstBlendM0::B01),
            2 => Some(SwDstBlendM0::B10),
            _ => None,
        }
    }
    #[doc = "Agd"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == SwDstBlendM0::B00
    }
    #[doc = "Ad0’"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == SwDstBlendM0::B01
    }
    #[doc = "(Ad0’*Agd)>>8"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == SwDstBlendM0::B10
    }
}
#[doc = "Field `SW_DST_BLEND_M0` writer - Alpha dst blend mode select of color channel(Ad0_’’)"]
pub type SwDstBlendM0W<'a, REG> = crate::FieldWriter<'a, REG, 2, SwDstBlendM0>;
impl<'a, REG> SwDstBlendM0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Agd"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstBlendM0::B00)
    }
    #[doc = "Ad0’"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstBlendM0::B01)
    }
    #[doc = "(Ad0’*Agd)>>8"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstBlendM0::B10)
    }
}
#[doc = "Alpha src blend mode select of color channel (As0_’’)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwSrcBlendM0 {
    #[doc = "0: Ags"]
    B00 = 0,
    #[doc = "1: As0’"]
    B01 = 1,
    #[doc = "2: (As0’*Ags)>>8"]
    B10 = 2,
}
impl From<SwSrcBlendM0> for u8 {
    #[inline(always)]
    fn from(variant: SwSrcBlendM0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwSrcBlendM0 {
    type Ux = u8;
}
#[doc = "Field `SW_SRC_BLEND_M0` reader - Alpha src blend mode select of color channel (As0_’’)"]
pub type SwSrcBlendM0R = crate::FieldReader<SwSrcBlendM0>;
impl SwSrcBlendM0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SwSrcBlendM0> {
        match self.bits {
            0 => Some(SwSrcBlendM0::B00),
            1 => Some(SwSrcBlendM0::B01),
            2 => Some(SwSrcBlendM0::B10),
            _ => None,
        }
    }
    #[doc = "Ags"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == SwSrcBlendM0::B00
    }
    #[doc = "As0’"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == SwSrcBlendM0::B01
    }
    #[doc = "(As0’*Ags)>>8"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == SwSrcBlendM0::B10
    }
}
#[doc = "Field `SW_SRC_BLEND_M0` writer - Alpha src blend mode select of color channel (As0_’’)"]
pub type SwSrcBlendM0W<'a, REG> = crate::FieldWriter<'a, REG, 2, SwSrcBlendM0>;
impl<'a, REG> SwSrcBlendM0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Ags"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcBlendM0::B00)
    }
    #[doc = "As0’"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcBlendM0::B01)
    }
    #[doc = "(As0’*Ags)>>8"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcBlendM0::B10)
    }
}
#[doc = "Dst Transparent/opaque of color channel (Ad0’)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDstAlphaM0 {
    #[doc = "0: Ad"]
    B0 = 0,
    #[doc = "1: 255-Ad"]
    B1 = 1,
}
impl From<SwDstAlphaM0> for bool {
    #[inline(always)]
    fn from(variant: SwDstAlphaM0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DST_ALPHA_M0` reader - Dst Transparent/opaque of color channel (Ad0’)"]
pub type SwDstAlphaM0R = crate::BitReader<SwDstAlphaM0>;
impl SwDstAlphaM0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDstAlphaM0 {
        match self.bits {
            false => SwDstAlphaM0::B0,
            true => SwDstAlphaM0::B1,
        }
    }
    #[doc = "Ad"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDstAlphaM0::B0
    }
    #[doc = "255-Ad"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDstAlphaM0::B1
    }
}
#[doc = "Field `SW_DST_ALPHA_M0` writer - Dst Transparent/opaque of color channel (Ad0’)"]
pub type SwDstAlphaM0W<'a, REG> = crate::BitWriter<'a, REG, SwDstAlphaM0>;
impl<'a, REG> SwDstAlphaM0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ad"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstAlphaM0::B0)
    }
    #[doc = "255-Ad"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstAlphaM0::B1)
    }
}
#[doc = "Src Transparent/opaque of color channel (As0’)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwSrcAlphaM0 {
    #[doc = "0: As"]
    B0 = 0,
    #[doc = "1: 255-As"]
    B1 = 1,
}
impl From<SwSrcAlphaM0> for bool {
    #[inline(always)]
    fn from(variant: SwSrcAlphaM0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SRC_ALPHA_M0` reader - Src Transparent/opaque of color channel (As0’)"]
pub type SwSrcAlphaM0R = crate::BitReader<SwSrcAlphaM0>;
impl SwSrcAlphaM0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrcAlphaM0 {
        match self.bits {
            false => SwSrcAlphaM0::B0,
            true => SwSrcAlphaM0::B1,
        }
    }
    #[doc = "As"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwSrcAlphaM0::B0
    }
    #[doc = "255-As"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwSrcAlphaM0::B1
    }
}
#[doc = "Field `SW_SRC_ALPHA_M0` writer - Src Transparent/opaque of color channel (As0’)"]
pub type SwSrcAlphaM0W<'a, REG> = crate::BitWriter<'a, REG, SwSrcAlphaM0>;
impl<'a, REG> SwSrcAlphaM0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "As"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcAlphaM0::B0)
    }
    #[doc = "255-As"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcAlphaM0::B1)
    }
}
#[doc = "Dst factore mode of alpha channel(Fd1)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwDstFactorM1 {
    #[doc = "0: 0"]
    B000 = 0,
    #[doc = "1: 256"]
    B001 = 1,
    #[doc = "2: As1’’"]
    B010 = 2,
    #[doc = "3: 256-As1’’"]
    B011 = 3,
    #[doc = "4: Ad1’’"]
    B100 = 4,
}
impl From<SwDstFactorM1> for u8 {
    #[inline(always)]
    fn from(variant: SwDstFactorM1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwDstFactorM1 {
    type Ux = u8;
}
#[doc = "Field `SW_DST_FACTOR_M1` reader - Dst factore mode of alpha channel(Fd1)"]
pub type SwDstFactorM1R = crate::FieldReader<SwDstFactorM1>;
impl SwDstFactorM1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SwDstFactorM1> {
        match self.bits {
            0 => Some(SwDstFactorM1::B000),
            1 => Some(SwDstFactorM1::B001),
            2 => Some(SwDstFactorM1::B010),
            3 => Some(SwDstFactorM1::B011),
            4 => Some(SwDstFactorM1::B100),
            _ => None,
        }
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == SwDstFactorM1::B000
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == SwDstFactorM1::B001
    }
    #[doc = "As1’’"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == SwDstFactorM1::B010
    }
    #[doc = "256-As1’’"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == SwDstFactorM1::B011
    }
    #[doc = "Ad1’’"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == SwDstFactorM1::B100
    }
}
#[doc = "Field `SW_DST_FACTOR_M1` writer - Dst factore mode of alpha channel(Fd1)"]
pub type SwDstFactorM1W<'a, REG> = crate::FieldWriter<'a, REG, 3, SwDstFactorM1>;
impl<'a, REG> SwDstFactorM1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFactorM1::B000)
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFactorM1::B001)
    }
    #[doc = "As1’’"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFactorM1::B010)
    }
    #[doc = "256-As1’’"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFactorM1::B011)
    }
    #[doc = "Ad1’’"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstFactorM1::B100)
    }
}
#[doc = "Src factore mode of alpha channel(Fs1)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WSrcFactorM1 {
    #[doc = "0: 0"]
    B000 = 0,
    #[doc = "1: 256"]
    B001 = 1,
    #[doc = "2: Ad1’’"]
    B010 = 2,
    #[doc = "3: 256-Ad1’’"]
    B011 = 3,
    #[doc = "4: As1’’"]
    B100 = 4,
}
impl From<WSrcFactorM1> for u8 {
    #[inline(always)]
    fn from(variant: WSrcFactorM1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WSrcFactorM1 {
    type Ux = u8;
}
#[doc = "Field `W_SRC_FACTOR_M1` reader - Src factore mode of alpha channel(Fs1)"]
pub type WSrcFactorM1R = crate::FieldReader<WSrcFactorM1>;
impl WSrcFactorM1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WSrcFactorM1> {
        match self.bits {
            0 => Some(WSrcFactorM1::B000),
            1 => Some(WSrcFactorM1::B001),
            2 => Some(WSrcFactorM1::B010),
            3 => Some(WSrcFactorM1::B011),
            4 => Some(WSrcFactorM1::B100),
            _ => None,
        }
    }
    #[doc = "0"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == WSrcFactorM1::B000
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == WSrcFactorM1::B001
    }
    #[doc = "Ad1’’"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == WSrcFactorM1::B010
    }
    #[doc = "256-Ad1’’"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == WSrcFactorM1::B011
    }
    #[doc = "As1’’"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == WSrcFactorM1::B100
    }
}
#[doc = "Field `W_SRC_FACTOR_M1` writer - Src factore mode of alpha channel(Fs1)"]
pub type WSrcFactorM1W<'a, REG> = crate::FieldWriter<'a, REG, 3, WSrcFactorM1>;
impl<'a, REG> WSrcFactorM1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(WSrcFactorM1::B000)
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(WSrcFactorM1::B001)
    }
    #[doc = "Ad1’’"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(WSrcFactorM1::B010)
    }
    #[doc = "256-Ad1’’"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(WSrcFactorM1::B011)
    }
    #[doc = "As1’’"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(WSrcFactorM1::B100)
    }
}
#[doc = "Alpha dst calculate mode of alpha channel(Ad1’’)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDstAlphaCalM1 {
    #[doc = "0: Ad1’’= Ad1_’’ + (Ad1_’’>>7)"]
    B0 = 0,
    #[doc = "1: Ad1’’= Ad1_’’"]
    B1 = 1,
}
impl From<SwDstAlphaCalM1> for bool {
    #[inline(always)]
    fn from(variant: SwDstAlphaCalM1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DST_ALPHA_CAL_M1` reader - Alpha dst calculate mode of alpha channel(Ad1’’)"]
pub type SwDstAlphaCalM1R = crate::BitReader<SwDstAlphaCalM1>;
impl SwDstAlphaCalM1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDstAlphaCalM1 {
        match self.bits {
            false => SwDstAlphaCalM1::B0,
            true => SwDstAlphaCalM1::B1,
        }
    }
    #[doc = "Ad1’’= Ad1_’’ + (Ad1_’’>>7)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDstAlphaCalM1::B0
    }
    #[doc = "Ad1’’= Ad1_’’"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDstAlphaCalM1::B1
    }
}
#[doc = "Field `SW_DST_ALPHA_CAL_M1` writer - Alpha dst calculate mode of alpha channel(Ad1’’)"]
pub type SwDstAlphaCalM1W<'a, REG> = crate::BitWriter<'a, REG, SwDstAlphaCalM1>;
impl<'a, REG> SwDstAlphaCalM1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ad1’’= Ad1_’’ + (Ad1_’’>>7)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstAlphaCalM1::B0)
    }
    #[doc = "Ad1’’= Ad1_’’"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstAlphaCalM1::B1)
    }
}
#[doc = "Alpha src calculate mode of alpha channel(As1’’)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwSrcAlphaCalM1 {
    #[doc = "0: As1’’= As1_’’+ (As1_’’>>7)"]
    B0 = 0,
    #[doc = "1: As1’’= As1 _’’"]
    B1 = 1,
}
impl From<SwSrcAlphaCalM1> for bool {
    #[inline(always)]
    fn from(variant: SwSrcAlphaCalM1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SRC_ALPHA_CAL_M1` reader - Alpha src calculate mode of alpha channel(As1’’)"]
pub type SwSrcAlphaCalM1R = crate::BitReader<SwSrcAlphaCalM1>;
impl SwSrcAlphaCalM1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrcAlphaCalM1 {
        match self.bits {
            false => SwSrcAlphaCalM1::B0,
            true => SwSrcAlphaCalM1::B1,
        }
    }
    #[doc = "As1’’= As1_’’+ (As1_’’>>7)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwSrcAlphaCalM1::B0
    }
    #[doc = "As1’’= As1 _’’"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwSrcAlphaCalM1::B1
    }
}
#[doc = "Field `SW_SRC_ALPHA_CAL_M1` writer - Alpha src calculate mode of alpha channel(As1’’)"]
pub type SwSrcAlphaCalM1W<'a, REG> = crate::BitWriter<'a, REG, SwSrcAlphaCalM1>;
impl<'a, REG> SwSrcAlphaCalM1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "As1’’= As1_’’+ (As1_’’>>7)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcAlphaCalM1::B0)
    }
    #[doc = "As1’’= As1 _’’"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcAlphaCalM1::B1)
    }
}
#[doc = "Alpha dst blend mode select of alpha channel(Ad1_’’)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwDstBlendM1 {
    #[doc = "0: Agd"]
    B00 = 0,
    #[doc = "1: Ad1’"]
    B01 = 1,
    #[doc = "2: (Ad1’*Agd)>>8"]
    B10 = 2,
}
impl From<SwDstBlendM1> for u8 {
    #[inline(always)]
    fn from(variant: SwDstBlendM1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwDstBlendM1 {
    type Ux = u8;
}
#[doc = "Field `SW_DST_BLEND_M1` reader - Alpha dst blend mode select of alpha channel(Ad1_’’)"]
pub type SwDstBlendM1R = crate::FieldReader<SwDstBlendM1>;
impl SwDstBlendM1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SwDstBlendM1> {
        match self.bits {
            0 => Some(SwDstBlendM1::B00),
            1 => Some(SwDstBlendM1::B01),
            2 => Some(SwDstBlendM1::B10),
            _ => None,
        }
    }
    #[doc = "Agd"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == SwDstBlendM1::B00
    }
    #[doc = "Ad1’"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == SwDstBlendM1::B01
    }
    #[doc = "(Ad1’*Agd)>>8"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == SwDstBlendM1::B10
    }
}
#[doc = "Field `SW_DST_BLEND_M1` writer - Alpha dst blend mode select of alpha channel(Ad1_’’)"]
pub type SwDstBlendM1W<'a, REG> = crate::FieldWriter<'a, REG, 2, SwDstBlendM1>;
impl<'a, REG> SwDstBlendM1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Agd"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstBlendM1::B00)
    }
    #[doc = "Ad1’"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstBlendM1::B01)
    }
    #[doc = "(Ad1’*Agd)>>8"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstBlendM1::B10)
    }
}
#[doc = "Alpha src blend mode select of alpha channel (As1_’’)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwSrcBlendM1 {
    #[doc = "0: Ags"]
    B00 = 0,
    #[doc = "1: As1’"]
    B01 = 1,
    #[doc = "2: (As1’*Ags)>>8"]
    B10 = 2,
}
impl From<SwSrcBlendM1> for u8 {
    #[inline(always)]
    fn from(variant: SwSrcBlendM1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwSrcBlendM1 {
    type Ux = u8;
}
#[doc = "Field `SW_SRC_BLEND_M1` reader - Alpha src blend mode select of alpha channel (As1_’’)"]
pub type SwSrcBlendM1R = crate::FieldReader<SwSrcBlendM1>;
impl SwSrcBlendM1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SwSrcBlendM1> {
        match self.bits {
            0 => Some(SwSrcBlendM1::B00),
            1 => Some(SwSrcBlendM1::B01),
            2 => Some(SwSrcBlendM1::B10),
            _ => None,
        }
    }
    #[doc = "Ags"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == SwSrcBlendM1::B00
    }
    #[doc = "As1’"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == SwSrcBlendM1::B01
    }
    #[doc = "(As1’*Ags)>>8"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == SwSrcBlendM1::B10
    }
}
#[doc = "Field `SW_SRC_BLEND_M1` writer - Alpha src blend mode select of alpha channel (As1_’’)"]
pub type SwSrcBlendM1W<'a, REG> = crate::FieldWriter<'a, REG, 2, SwSrcBlendM1>;
impl<'a, REG> SwSrcBlendM1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Ags"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcBlendM1::B00)
    }
    #[doc = "As1’"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcBlendM1::B01)
    }
    #[doc = "(As1’*Ags)>>8"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcBlendM1::B10)
    }
}
#[doc = "Dst Transparent/opaque of alpha channel (Ad1’)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDstAlphaM1 {
    #[doc = "0: Ad"]
    B0 = 0,
    #[doc = "1: 255-Ad"]
    B1 = 1,
}
impl From<SwDstAlphaM1> for bool {
    #[inline(always)]
    fn from(variant: SwDstAlphaM1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_DST_ALPHA_M1` reader - Dst Transparent/opaque of alpha channel (Ad1’)"]
pub type SwDstAlphaM1R = crate::BitReader<SwDstAlphaM1>;
impl SwDstAlphaM1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDstAlphaM1 {
        match self.bits {
            false => SwDstAlphaM1::B0,
            true => SwDstAlphaM1::B1,
        }
    }
    #[doc = "Ad"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwDstAlphaM1::B0
    }
    #[doc = "255-Ad"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwDstAlphaM1::B1
    }
}
#[doc = "Field `SW_DST_ALPHA_M1` writer - Dst Transparent/opaque of alpha channel (Ad1’)"]
pub type SwDstAlphaM1W<'a, REG> = crate::BitWriter<'a, REG, SwDstAlphaM1>;
impl<'a, REG> SwDstAlphaM1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ad"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstAlphaM1::B0)
    }
    #[doc = "255-Ad"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwDstAlphaM1::B1)
    }
}
#[doc = "Src Transparent/opaque of alpha channel (As1’)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwSrcAlphaM1 {
    #[doc = "0: As"]
    B0 = 0,
    #[doc = "1: 255-As"]
    B1 = 1,
}
impl From<SwSrcAlphaM1> for bool {
    #[inline(always)]
    fn from(variant: SwSrcAlphaM1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW_SRC_ALPHA_M1` reader - Src Transparent/opaque of alpha channel (As1’)"]
pub type SwSrcAlphaM1R = crate::BitReader<SwSrcAlphaM1>;
impl SwSrcAlphaM1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwSrcAlphaM1 {
        match self.bits {
            false => SwSrcAlphaM1::B0,
            true => SwSrcAlphaM1::B1,
        }
    }
    #[doc = "As"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SwSrcAlphaM1::B0
    }
    #[doc = "255-As"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SwSrcAlphaM1::B1
    }
}
#[doc = "Field `SW_SRC_ALPHA_M1` writer - Src Transparent/opaque of alpha channel (As1’)"]
pub type SwSrcAlphaM1W<'a, REG> = crate::BitWriter<'a, REG, SwSrcAlphaM1>;
impl<'a, REG> SwSrcAlphaM1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "As"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcAlphaM1::B0)
    }
    #[doc = "255-As"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SwSrcAlphaM1::B1)
    }
}
impl R {
    #[doc = "Bit 0 - SRC color select(Cd’)"]
    #[inline(always)]
    pub fn sw_dst_color_m0(&self) -> SwDstColorM0R {
        SwDstColorM0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRC color select(Cs’)"]
    #[inline(always)]
    pub fn sw_src_color_m0(&self) -> SwSrcColorM0R {
        SwSrcColorM0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Dst factore mode of color channel(Fd0)"]
    #[inline(always)]
    pub fn sw_dst_factor_m0(&self) -> SwDstFactorM0R {
        SwDstFactorM0R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - Src factore mode of color channel(Fs0)"]
    #[inline(always)]
    pub fn sw_src_factor_m0(&self) -> SwSrcFactorM0R {
        SwSrcFactorM0R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Alpha dst calculate mode of color channel(Ad0’’)"]
    #[inline(always)]
    pub fn sw_dst_alpha_cal_m0(&self) -> SwDstAlphaCalM0R {
        SwDstAlphaCalM0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alpha src calculate mode of color channel(As0’’)"]
    #[inline(always)]
    pub fn sw_src_alpha_cal_m0(&self) -> SwSrcAlphaCalM0R {
        SwSrcAlphaCalM0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Alpha dst blend mode select of color channel(Ad0_’’)"]
    #[inline(always)]
    pub fn sw_dst_blend_m0(&self) -> SwDstBlendM0R {
        SwDstBlendM0R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Alpha src blend mode select of color channel (As0_’’)"]
    #[inline(always)]
    pub fn sw_src_blend_m0(&self) -> SwSrcBlendM0R {
        SwSrcBlendM0R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Dst Transparent/opaque of color channel (Ad0’)"]
    #[inline(always)]
    pub fn sw_dst_alpha_m0(&self) -> SwDstAlphaM0R {
        SwDstAlphaM0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Src Transparent/opaque of color channel (As0’)"]
    #[inline(always)]
    pub fn sw_src_alpha_m0(&self) -> SwSrcAlphaM0R {
        SwSrcAlphaM0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Dst factore mode of alpha channel(Fd1)"]
    #[inline(always)]
    pub fn sw_dst_factor_m1(&self) -> SwDstFactorM1R {
        SwDstFactorM1R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Src factore mode of alpha channel(Fs1)"]
    #[inline(always)]
    pub fn w_src_factor_m1(&self) -> WSrcFactorM1R {
        WSrcFactorM1R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bit 22 - Alpha dst calculate mode of alpha channel(Ad1’’)"]
    #[inline(always)]
    pub fn sw_dst_alpha_cal_m1(&self) -> SwDstAlphaCalM1R {
        SwDstAlphaCalM1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Alpha src calculate mode of alpha channel(As1’’)"]
    #[inline(always)]
    pub fn sw_src_alpha_cal_m1(&self) -> SwSrcAlphaCalM1R {
        SwSrcAlphaCalM1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Alpha dst blend mode select of alpha channel(Ad1_’’)"]
    #[inline(always)]
    pub fn sw_dst_blend_m1(&self) -> SwDstBlendM1R {
        SwDstBlendM1R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Alpha src blend mode select of alpha channel (As1_’’)"]
    #[inline(always)]
    pub fn sw_src_blend_m1(&self) -> SwSrcBlendM1R {
        SwSrcBlendM1R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - Dst Transparent/opaque of alpha channel (Ad1’)"]
    #[inline(always)]
    pub fn sw_dst_alpha_m1(&self) -> SwDstAlphaM1R {
        SwDstAlphaM1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Src Transparent/opaque of alpha channel (As1’)"]
    #[inline(always)]
    pub fn sw_src_alpha_m1(&self) -> SwSrcAlphaM1R {
        SwSrcAlphaM1R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRC color select(Cd’)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_color_m0(&mut self) -> SwDstColorM0W<AlphaCtrl1Spec> {
        SwDstColorM0W::new(self, 0)
    }
    #[doc = "Bit 1 - SRC color select(Cs’)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_color_m0(&mut self) -> SwSrcColorM0W<AlphaCtrl1Spec> {
        SwSrcColorM0W::new(self, 1)
    }
    #[doc = "Bits 2:4 - Dst factore mode of color channel(Fd0)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_factor_m0(&mut self) -> SwDstFactorM0W<AlphaCtrl1Spec> {
        SwDstFactorM0W::new(self, 2)
    }
    #[doc = "Bits 5:7 - Src factore mode of color channel(Fs0)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_factor_m0(&mut self) -> SwSrcFactorM0W<AlphaCtrl1Spec> {
        SwSrcFactorM0W::new(self, 5)
    }
    #[doc = "Bit 8 - Alpha dst calculate mode of color channel(Ad0’’)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_alpha_cal_m0(&mut self) -> SwDstAlphaCalM0W<AlphaCtrl1Spec> {
        SwDstAlphaCalM0W::new(self, 8)
    }
    #[doc = "Bit 9 - Alpha src calculate mode of color channel(As0’’)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_alpha_cal_m0(&mut self) -> SwSrcAlphaCalM0W<AlphaCtrl1Spec> {
        SwSrcAlphaCalM0W::new(self, 9)
    }
    #[doc = "Bits 10:11 - Alpha dst blend mode select of color channel(Ad0_’’)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_blend_m0(&mut self) -> SwDstBlendM0W<AlphaCtrl1Spec> {
        SwDstBlendM0W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Alpha src blend mode select of color channel (As0_’’)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_blend_m0(&mut self) -> SwSrcBlendM0W<AlphaCtrl1Spec> {
        SwSrcBlendM0W::new(self, 12)
    }
    #[doc = "Bit 14 - Dst Transparent/opaque of color channel (Ad0’)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_alpha_m0(&mut self) -> SwDstAlphaM0W<AlphaCtrl1Spec> {
        SwDstAlphaM0W::new(self, 14)
    }
    #[doc = "Bit 15 - Src Transparent/opaque of color channel (As0’)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_alpha_m0(&mut self) -> SwSrcAlphaM0W<AlphaCtrl1Spec> {
        SwSrcAlphaM0W::new(self, 15)
    }
    #[doc = "Bits 16:18 - Dst factore mode of alpha channel(Fd1)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_factor_m1(&mut self) -> SwDstFactorM1W<AlphaCtrl1Spec> {
        SwDstFactorM1W::new(self, 16)
    }
    #[doc = "Bits 19:21 - Src factore mode of alpha channel(Fs1)"]
    #[inline(always)]
    #[must_use]
    pub fn w_src_factor_m1(&mut self) -> WSrcFactorM1W<AlphaCtrl1Spec> {
        WSrcFactorM1W::new(self, 19)
    }
    #[doc = "Bit 22 - Alpha dst calculate mode of alpha channel(Ad1’’)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_alpha_cal_m1(&mut self) -> SwDstAlphaCalM1W<AlphaCtrl1Spec> {
        SwDstAlphaCalM1W::new(self, 22)
    }
    #[doc = "Bit 23 - Alpha src calculate mode of alpha channel(As1’’)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_alpha_cal_m1(&mut self) -> SwSrcAlphaCalM1W<AlphaCtrl1Spec> {
        SwSrcAlphaCalM1W::new(self, 23)
    }
    #[doc = "Bits 24:25 - Alpha dst blend mode select of alpha channel(Ad1_’’)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_blend_m1(&mut self) -> SwDstBlendM1W<AlphaCtrl1Spec> {
        SwDstBlendM1W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Alpha src blend mode select of alpha channel (As1_’’)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_blend_m1(&mut self) -> SwSrcBlendM1W<AlphaCtrl1Spec> {
        SwSrcBlendM1W::new(self, 26)
    }
    #[doc = "Bit 28 - Dst Transparent/opaque of alpha channel (Ad1’)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_alpha_m1(&mut self) -> SwDstAlphaM1W<AlphaCtrl1Spec> {
        SwDstAlphaM1W::new(self, 28)
    }
    #[doc = "Bit 29 - Src Transparent/opaque of alpha channel (As1’)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_alpha_m1(&mut self) -> SwSrcAlphaM1W<AlphaCtrl1Spec> {
        SwSrcAlphaM1W::new(self, 29)
    }
}
#[doc = "Register0000 Abstract\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alpha_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alpha_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlphaCtrl1Spec;
impl crate::RegisterSpec for AlphaCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alpha_ctrl1::R`](R) reader structure"]
impl crate::Readable for AlphaCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`alpha_ctrl1::W`](W) writer structure"]
impl crate::Writable for AlphaCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALPHA_CTRL1 to value 0"]
impl crate::Resettable for AlphaCtrl1Spec {
    const RESET_VALUE: u32 = 0;
}
