#[doc = "Register `RAW_CONFIG0` reader"]
pub type R = crate::R<RawConfig0Spec>;
#[doc = "Register `RAW_CONFIG0` writer"]
pub type W = crate::W<RawConfig0Spec>;
#[doc = "VOP direct path enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VopPathEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<VopPathEn> for bool {
    #[inline(always)]
    fn from(variant: VopPathEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOP_PATH_EN` reader - VOP direct path enable"]
pub type VopPathEnR = crate::BitReader<VopPathEn>;
impl VopPathEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VopPathEn {
        match self.bits {
            false => VopPathEn::B0,
            true => VopPathEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VopPathEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VopPathEn::B1
    }
}
#[doc = "Field `VOP_PATH_EN` writer - VOP direct path enable"]
pub type VopPathEnW<'a, REG> = crate::BitWriter<'a, REG, VopPathEn>;
impl<'a, REG> VopPathEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VopPathEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VopPathEn::B1)
    }
}
#[doc = "Field `DIL_HF_FCT` reader - deinterlace high frequency factor"]
pub type DilHfFctR = crate::FieldReader;
#[doc = "Field `DIL_HF_FCT` writer - deinterlace high frequency factor"]
pub type DilHfFctW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Deinterlace mode select:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DilMode {
    #[doc = "0: YUV deinterlace and bypass path disable;"]
    B000 = 0,
    #[doc = "1: I4O2 mode"]
    B001 = 1,
    #[doc = "2: I4O1B mode"]
    B010 = 2,
    #[doc = "3: I4O1T mode"]
    B011 = 3,
    #[doc = "4: I2O1B mode"]
    B100 = 4,
    #[doc = "5: I2O1T mode"]
    B101 = 5,
    #[doc = "6: bypass mode"]
    B110 = 6,
}
impl From<DilMode> for u8 {
    #[inline(always)]
    fn from(variant: DilMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DilMode {
    type Ux = u8;
}
#[doc = "Field `DIL_MODE` reader - Deinterlace mode select:"]
pub type DilModeR = crate::FieldReader<DilMode>;
impl DilModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DilMode> {
        match self.bits {
            0 => Some(DilMode::B000),
            1 => Some(DilMode::B001),
            2 => Some(DilMode::B010),
            3 => Some(DilMode::B011),
            4 => Some(DilMode::B100),
            5 => Some(DilMode::B101),
            6 => Some(DilMode::B110),
            _ => None,
        }
    }
    #[doc = "YUV deinterlace and bypass path disable;"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == DilMode::B000
    }
    #[doc = "I4O2 mode"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == DilMode::B001
    }
    #[doc = "I4O1B mode"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == DilMode::B010
    }
    #[doc = "I4O1T mode"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == DilMode::B011
    }
    #[doc = "I2O1B mode"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == DilMode::B100
    }
    #[doc = "I2O1T mode"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == DilMode::B101
    }
    #[doc = "bypass mode"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == DilMode::B110
    }
}
#[doc = "Field `DIL_MODE` writer - Deinterlace mode select:"]
pub type DilModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, DilMode>;
impl<'a, REG> DilModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "YUV deinterlace and bypass path disable;"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(DilMode::B000)
    }
    #[doc = "I4O2 mode"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(DilMode::B001)
    }
    #[doc = "I4O1B mode"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(DilMode::B010)
    }
    #[doc = "I4O1T mode"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(DilMode::B011)
    }
    #[doc = "I2O1B mode"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(DilMode::B100)
    }
    #[doc = "I2O1T mode"]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(DilMode::B101)
    }
    #[doc = "bypass mode"]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(DilMode::B110)
    }
}
#[doc = "deinterlace high frequency calculation enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DilHfEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<DilHfEn> for bool {
    #[inline(always)]
    fn from(variant: DilHfEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIL_HF_EN` reader - deinterlace high frequency calculation enable"]
pub type DilHfEnR = crate::BitReader<DilHfEn>;
impl DilHfEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DilHfEn {
        match self.bits {
            false => DilHfEn::B0,
            true => DilHfEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DilHfEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DilHfEn::B1
    }
}
#[doc = "Field `DIL_HF_EN` writer - deinterlace high frequency calculation enable"]
pub type DilHfEnW<'a, REG> = crate::BitWriter<'a, REG, DilHfEn>;
impl<'a, REG> DilHfEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DilHfEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DilHfEn::B1)
    }
}
#[doc = "deinterlace edge interpolation\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DilEiMode {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<DilEiMode> for bool {
    #[inline(always)]
    fn from(variant: DilEiMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIL_EI_MODE` reader - deinterlace edge interpolation"]
pub type DilEiModeR = crate::BitReader<DilEiMode>;
impl DilEiModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DilEiMode {
        match self.bits {
            false => DilEiMode::B0,
            true => DilEiMode::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DilEiMode::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DilEiMode::B1
    }
}
#[doc = "Field `DIL_EI_MODE` writer - deinterlace edge interpolation"]
pub type DilEiModeW<'a, REG> = crate::BitWriter<'a, REG, DilEiMode>;
impl<'a, REG> DilEiModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DilEiMode::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DilEiMode::B1)
    }
}
#[doc = "YUV 3D denoise enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum YuvDnsEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<YuvDnsEn> for bool {
    #[inline(always)]
    fn from(variant: YuvDnsEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `YUV_DNS_EN` reader - YUV 3D denoise enable"]
pub type YuvDnsEnR = crate::BitReader<YuvDnsEn>;
impl YuvDnsEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> YuvDnsEn {
        match self.bits {
            false => YuvDnsEn::B0,
            true => YuvDnsEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == YuvDnsEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == YuvDnsEn::B1
    }
}
#[doc = "Field `YUV_DNS_EN` writer - YUV 3D denoise enable"]
pub type YuvDnsEnW<'a, REG> = crate::BitWriter<'a, REG, YuvDnsEn>;
impl<'a, REG> YuvDnsEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(YuvDnsEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(YuvDnsEn::B1)
    }
}
#[doc = "yuv enhancement enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum YuvEnhEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<YuvEnhEn> for bool {
    #[inline(always)]
    fn from(variant: YuvEnhEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `YUV_ENH_EN` reader - yuv enhancement enable"]
pub type YuvEnhEnR = crate::BitReader<YuvEnhEn>;
impl YuvEnhEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> YuvEnhEn {
        match self.bits {
            false => YuvEnhEn::B0,
            true => YuvEnhEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == YuvEnhEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == YuvEnhEn::B1
    }
}
#[doc = "Field `YUV_ENH_EN` writer - yuv enhancement enable"]
pub type YuvEnhEnW<'a, REG> = crate::BitWriter<'a, REG, YuvEnhEn>;
impl<'a, REG> YuvEnhEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(YuvEnhEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(YuvEnhEn::B1)
    }
}
#[doc = "deinterlace edge interpolation for smooth effect\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DilEiSmooth {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<DilEiSmooth> for bool {
    #[inline(always)]
    fn from(variant: DilEiSmooth) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIL_EI_SMOOTH` reader - deinterlace edge interpolation for smooth effect"]
pub type DilEiSmoothR = crate::BitReader<DilEiSmooth>;
impl DilEiSmoothR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DilEiSmooth {
        match self.bits {
            false => DilEiSmooth::B0,
            true => DilEiSmooth::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DilEiSmooth::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DilEiSmooth::B1
    }
}
#[doc = "Field `DIL_EI_SMOOTH` writer - deinterlace edge interpolation for smooth effect"]
pub type DilEiSmoothW<'a, REG> = crate::BitWriter<'a, REG, DilEiSmooth>;
impl<'a, REG> DilEiSmoothW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DilEiSmooth::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DilEiSmooth::B1)
    }
}
#[doc = "RGB color enhancement enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RgbColorEnhEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<RgbColorEnhEn> for bool {
    #[inline(always)]
    fn from(variant: RgbColorEnhEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGB_COLOR_ENH_EN` reader - RGB color enhancement enable"]
pub type RgbColorEnhEnR = crate::BitReader<RgbColorEnhEn>;
impl RgbColorEnhEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RgbColorEnhEn {
        match self.bits {
            false => RgbColorEnhEn::B0,
            true => RgbColorEnhEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RgbColorEnhEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RgbColorEnhEn::B1
    }
}
#[doc = "Field `RGB_COLOR_ENH_EN` writer - RGB color enhancement enable"]
pub type RgbColorEnhEnW<'a, REG> = crate::BitWriter<'a, REG, RgbColorEnhEn>;
impl<'a, REG> RgbColorEnhEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RgbColorEnhEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RgbColorEnhEn::B1)
    }
}
#[doc = "RGB contrast enhancement and gamma adjustment enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RgbConGamEn {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<RgbConGamEn> for bool {
    #[inline(always)]
    fn from(variant: RgbConGamEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGB_CON_GAM_EN` reader - RGB contrast enhancement and gamma adjustment enable"]
pub type RgbConGamEnR = crate::BitReader<RgbConGamEn>;
impl RgbConGamEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RgbConGamEn {
        match self.bits {
            false => RgbConGamEn::B0,
            true => RgbConGamEn::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RgbConGamEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RgbConGamEn::B1
    }
}
#[doc = "Field `RGB_CON_GAM_EN` writer - RGB contrast enhancement and gamma adjustment enable"]
pub type RgbConGamEnW<'a, REG> = crate::BitWriter<'a, REG, RgbConGamEn>;
impl<'a, REG> RgbConGamEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RgbConGamEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RgbConGamEn::B1)
    }
}
#[doc = "RGB enhancement select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RgbEnhSel {
    #[doc = "0: no operation"]
    B00 = 0,
    #[doc = "1: denoise"]
    B01 = 1,
    #[doc = "2: detail enhancement"]
    B10 = 2,
    #[doc = "3: edge enhancement"]
    B11 = 3,
}
impl From<RgbEnhSel> for u8 {
    #[inline(always)]
    fn from(variant: RgbEnhSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RgbEnhSel {
    type Ux = u8;
}
#[doc = "Field `RGB_ENH_SEL` reader - RGB enhancement select"]
pub type RgbEnhSelR = crate::FieldReader<RgbEnhSel>;
impl RgbEnhSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RgbEnhSel {
        match self.bits {
            0 => RgbEnhSel::B00,
            1 => RgbEnhSel::B01,
            2 => RgbEnhSel::B10,
            3 => RgbEnhSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "no operation"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == RgbEnhSel::B00
    }
    #[doc = "denoise"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == RgbEnhSel::B01
    }
    #[doc = "detail enhancement"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == RgbEnhSel::B10
    }
    #[doc = "edge enhancement"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == RgbEnhSel::B11
    }
}
#[doc = "Field `RGB_ENH_SEL` writer - RGB enhancement select"]
pub type RgbEnhSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RgbEnhSel>;
impl<'a, REG> RgbEnhSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no operation"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(RgbEnhSel::B00)
    }
    #[doc = "denoise"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(RgbEnhSel::B01)
    }
    #[doc = "detail enhancement"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(RgbEnhSel::B10)
    }
    #[doc = "edge enhancement"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(RgbEnhSel::B11)
    }
}
#[doc = "RGB contrast enhancement and gamma adjustment operation\n\norder select.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RgbConGamOrder {
    #[doc = "0: CG prior to DDE"]
    B0 = 0,
    #[doc = "1: DDE prior to CG (CG represent for contrast &amp; gamma operation, and DDE represent for de-noise, detail or edge enhancement operation)"]
    B1 = 1,
}
impl From<RgbConGamOrder> for bool {
    #[inline(always)]
    fn from(variant: RgbConGamOrder) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGB_CON_GAM_ORDER` reader - RGB contrast enhancement and gamma adjustment operation\n\norder select."]
pub type RgbConGamOrderR = crate::BitReader<RgbConGamOrder>;
impl RgbConGamOrderR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RgbConGamOrder {
        match self.bits {
            false => RgbConGamOrder::B0,
            true => RgbConGamOrder::B1,
        }
    }
    #[doc = "CG prior to DDE"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RgbConGamOrder::B0
    }
    #[doc = "DDE prior to CG (CG represent for contrast &amp; gamma operation, and DDE represent for de-noise, detail or edge enhancement operation)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RgbConGamOrder::B1
    }
}
#[doc = "Field `RGB_CON_GAM_ORDER` writer - RGB contrast enhancement and gamma adjustment operation\n\norder select."]
pub type RgbConGamOrderW<'a, REG> = crate::BitWriter<'a, REG, RgbConGamOrder>;
impl<'a, REG> RgbConGamOrderW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CG prior to DDE"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RgbConGamOrder::B0)
    }
    #[doc = "DDE prior to CG (CG represent for contrast &amp; gamma operation, and DDE represent for de-noise, detail or edge enhancement operation)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RgbConGamOrder::B1)
    }
}
#[doc = "Field `DIL_EI_RADIUS` reader - deinterlace edge interpolation radius"]
pub type DilEiRadiusR = crate::FieldReader;
#[doc = "Field `DIL_EI_RADIUS` writer - deinterlace edge interpolation radius"]
pub type DilEiRadiusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DIL_EI_SEL` reader - deinterlace edge interpolation select"]
pub type DilEiSelR = crate::BitReader;
#[doc = "Field `DIL_EI_SEL` writer - deinterlace edge interpolation select"]
pub type DilEiSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VOP direct path enable"]
    #[inline(always)]
    pub fn vop_path_en(&self) -> VopPathEnR {
        VopPathEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - deinterlace high frequency factor"]
    #[inline(always)]
    pub fn dil_hf_fct(&self) -> DilHfFctR {
        DilHfFctR::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - Deinterlace mode select:"]
    #[inline(always)]
    pub fn dil_mode(&self) -> DilModeR {
        DilModeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - deinterlace high frequency calculation enable"]
    #[inline(always)]
    pub fn dil_hf_en(&self) -> DilHfEnR {
        DilHfEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - deinterlace edge interpolation"]
    #[inline(always)]
    pub fn dil_ei_mode(&self) -> DilEiModeR {
        DilEiModeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - YUV 3D denoise enable"]
    #[inline(always)]
    pub fn yuv_dns_en(&self) -> YuvDnsEnR {
        YuvDnsEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - yuv enhancement enable"]
    #[inline(always)]
    pub fn yuv_enh_en(&self) -> YuvEnhEnR {
        YuvEnhEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - deinterlace edge interpolation for smooth effect"]
    #[inline(always)]
    pub fn dil_ei_smooth(&self) -> DilEiSmoothR {
        DilEiSmoothR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RGB color enhancement enable"]
    #[inline(always)]
    pub fn rgb_color_enh_en(&self) -> RgbColorEnhEnR {
        RgbColorEnhEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RGB contrast enhancement and gamma adjustment enable"]
    #[inline(always)]
    pub fn rgb_con_gam_en(&self) -> RgbConGamEnR {
        RgbConGamEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - RGB enhancement select"]
    #[inline(always)]
    pub fn rgb_enh_sel(&self) -> RgbEnhSelR {
        RgbEnhSelR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - RGB contrast enhancement and gamma adjustment operation\n\norder select."]
    #[inline(always)]
    pub fn rgb_con_gam_order(&self) -> RgbConGamOrderR {
        RgbConGamOrderR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - deinterlace edge interpolation radius"]
    #[inline(always)]
    pub fn dil_ei_radius(&self) -> DilEiRadiusR {
        DilEiRadiusR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - deinterlace edge interpolation select"]
    #[inline(always)]
    pub fn dil_ei_sel(&self) -> DilEiSelR {
        DilEiSelR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VOP direct path enable"]
    #[inline(always)]
    #[must_use]
    pub fn vop_path_en(&mut self) -> VopPathEnW<RawConfig0Spec> {
        VopPathEnW::new(self, 0)
    }
    #[doc = "Bits 1:7 - deinterlace high frequency factor"]
    #[inline(always)]
    #[must_use]
    pub fn dil_hf_fct(&mut self) -> DilHfFctW<RawConfig0Spec> {
        DilHfFctW::new(self, 1)
    }
    #[doc = "Bits 8:10 - Deinterlace mode select:"]
    #[inline(always)]
    #[must_use]
    pub fn dil_mode(&mut self) -> DilModeW<RawConfig0Spec> {
        DilModeW::new(self, 8)
    }
    #[doc = "Bit 11 - deinterlace high frequency calculation enable"]
    #[inline(always)]
    #[must_use]
    pub fn dil_hf_en(&mut self) -> DilHfEnW<RawConfig0Spec> {
        DilHfEnW::new(self, 11)
    }
    #[doc = "Bit 12 - deinterlace edge interpolation"]
    #[inline(always)]
    #[must_use]
    pub fn dil_ei_mode(&mut self) -> DilEiModeW<RawConfig0Spec> {
        DilEiModeW::new(self, 12)
    }
    #[doc = "Bit 13 - YUV 3D denoise enable"]
    #[inline(always)]
    #[must_use]
    pub fn yuv_dns_en(&mut self) -> YuvDnsEnW<RawConfig0Spec> {
        YuvDnsEnW::new(self, 13)
    }
    #[doc = "Bit 14 - yuv enhancement enable"]
    #[inline(always)]
    #[must_use]
    pub fn yuv_enh_en(&mut self) -> YuvEnhEnW<RawConfig0Spec> {
        YuvEnhEnW::new(self, 14)
    }
    #[doc = "Bit 15 - deinterlace edge interpolation for smooth effect"]
    #[inline(always)]
    #[must_use]
    pub fn dil_ei_smooth(&mut self) -> DilEiSmoothW<RawConfig0Spec> {
        DilEiSmoothW::new(self, 15)
    }
    #[doc = "Bit 16 - RGB color enhancement enable"]
    #[inline(always)]
    #[must_use]
    pub fn rgb_color_enh_en(&mut self) -> RgbColorEnhEnW<RawConfig0Spec> {
        RgbColorEnhEnW::new(self, 16)
    }
    #[doc = "Bit 17 - RGB contrast enhancement and gamma adjustment enable"]
    #[inline(always)]
    #[must_use]
    pub fn rgb_con_gam_en(&mut self) -> RgbConGamEnW<RawConfig0Spec> {
        RgbConGamEnW::new(self, 17)
    }
    #[doc = "Bits 18:19 - RGB enhancement select"]
    #[inline(always)]
    #[must_use]
    pub fn rgb_enh_sel(&mut self) -> RgbEnhSelW<RawConfig0Spec> {
        RgbEnhSelW::new(self, 18)
    }
    #[doc = "Bit 20 - RGB contrast enhancement and gamma adjustment operation\n\norder select."]
    #[inline(always)]
    #[must_use]
    pub fn rgb_con_gam_order(&mut self) -> RgbConGamOrderW<RawConfig0Spec> {
        RgbConGamOrderW::new(self, 20)
    }
    #[doc = "Bits 21:22 - deinterlace edge interpolation radius"]
    #[inline(always)]
    #[must_use]
    pub fn dil_ei_radius(&mut self) -> DilEiRadiusW<RawConfig0Spec> {
        DilEiRadiusW::new(self, 21)
    }
    #[doc = "Bit 23 - deinterlace edge interpolation select"]
    #[inline(always)]
    #[must_use]
    pub fn dil_ei_sel(&mut self) -> DilEiSelW<RawConfig0Spec> {
        DilEiSelW::new(self, 23)
    }
}
#[doc = "configuration register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_config0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`raw_config0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawConfig0Spec;
impl crate::RegisterSpec for RawConfig0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_config0::R`](R) reader structure"]
impl crate::Readable for RawConfig0Spec {}
#[doc = "`write(|w| ..)` method takes [`raw_config0::W`](W) writer structure"]
impl crate::Writable for RawConfig0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAW_CONFIG0 to value 0"]
impl crate::Resettable for RawConfig0Spec {
    const RESET_VALUE: u32 = 0;
}
