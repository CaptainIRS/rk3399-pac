#[doc = "Register `WIN0_CTRL1` reader"]
pub type R = crate::R<Win0Ctrl1Spec>;
#[doc = "Register `WIN0_CTRL1` writer"]
pub type W = crate::W<Win0Ctrl1Spec>;
#[doc = "Field `WIN0_YRGB_AXI_GATHER_EN` reader - win0 axi bus yrgb data gather transfer enable"]
pub type Win0YrgbAxiGatherEnR = crate::BitReader;
#[doc = "Field `WIN0_YRGB_AXI_GATHER_EN` writer - win0 axi bus yrgb data gather transfer enable"]
pub type Win0YrgbAxiGatherEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN0_CBR_AXI_GATHER_EN` reader - win0 axi bus cbr data gather transfer enable"]
pub type Win0CbrAxiGatherEnR = crate::BitReader;
#[doc = "Field `WIN0_CBR_AXI_GATHER_EN` writer - win0 axi bus cbr data gather transfer enable"]
pub type Win0CbrAxiGatherEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win0BicCoeSel {
    #[doc = "0: PRECISE"]
    B00 = 0,
    #[doc = "1: SPLINE"]
    B01 = 1,
    #[doc = "2: CATROM"]
    B10 = 2,
    #[doc = "3: MITCHELL"]
    B11 = 3,
}
impl From<Win0BicCoeSel> for u8 {
    #[inline(always)]
    fn from(variant: Win0BicCoeSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win0BicCoeSel {
    type Ux = u8;
}
#[doc = "Field `WIN0_BIC_COE_SEL` reader - "]
pub type Win0BicCoeSelR = crate::FieldReader<Win0BicCoeSel>;
impl Win0BicCoeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0BicCoeSel {
        match self.bits {
            0 => Win0BicCoeSel::B00,
            1 => Win0BicCoeSel::B01,
            2 => Win0BicCoeSel::B10,
            3 => Win0BicCoeSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "PRECISE"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win0BicCoeSel::B00
    }
    #[doc = "SPLINE"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win0BicCoeSel::B01
    }
    #[doc = "CATROM"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win0BicCoeSel::B10
    }
    #[doc = "MITCHELL"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win0BicCoeSel::B11
    }
}
#[doc = "Field `WIN0_BIC_COE_SEL` writer - "]
pub type Win0BicCoeSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win0BicCoeSel>;
impl<'a, REG> Win0BicCoeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRECISE"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win0BicCoeSel::B00)
    }
    #[doc = "SPLINE"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win0BicCoeSel::B01)
    }
    #[doc = "CATROM"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win0BicCoeSel::B10)
    }
    #[doc = "MITCHELL"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win0BicCoeSel::B11)
    }
}
#[doc = "Field `WIN0_VSD_YRGB_GT4` reader - yrgb_src/yrgb_dst >= 4"]
pub type Win0VsdYrgbGt4R = crate::BitReader;
#[doc = "Field `WIN0_VSD_YRGB_GT4` writer - yrgb_src/yrgb_dst >= 4"]
pub type Win0VsdYrgbGt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN0_VSD_YRGB_GT2` reader - yrgb_src/yrgb_dst >= 2"]
pub type Win0VsdYrgbGt2R = crate::BitReader;
#[doc = "Field `WIN0_VSD_YRGB_GT2` writer - yrgb_src/yrgb_dst >= 2"]
pub type Win0VsdYrgbGt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN0_VSD_CBR_GT4` reader - cbr_src/cbr_dst >= 4"]
pub type Win0VsdCbrGt4R = crate::BitReader;
#[doc = "Field `WIN0_VSD_CBR_GT4` writer - cbr_src/cbr_dst >= 4"]
pub type Win0VsdCbrGt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN0_VSD_CBR_GT2` reader - cbr_src/cbr_dst >= 2"]
pub type Win0VsdCbrGt2R = crate::BitReader;
#[doc = "Field `WIN0_VSD_CBR_GT2` writer - cbr_src/cbr_dst >= 2"]
pub type Win0VsdCbrGt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN0_YRGB_AXI_GATHER_NUM` reader - win0 axi yrgb data transfer gather number"]
pub type Win0YrgbAxiGatherNumR = crate::FieldReader;
#[doc = "Field `WIN0_YRGB_AXI_GATHER_NUM` writer - win0 axi yrgb data transfer gather number"]
pub type Win0YrgbAxiGatherNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WIN0_CBR_AXI_GATHER_NUM` reader - win0 axi cbr data transfer gather number"]
pub type Win0CbrAxiGatherNumR = crate::FieldReader;
#[doc = "Field `WIN0_CBR_AXI_GATHER_NUM` writer - win0 axi cbr data transfer gather number"]
pub type Win0CbrAxiGatherNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "when yuv fmt,\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0LineLoadMode {
    #[doc = "0: load data by axi trans"]
    B0 = 0,
    #[doc = "1: load data by lines"]
    B1 = 1,
}
impl From<Win0LineLoadMode> for bool {
    #[inline(always)]
    fn from(variant: Win0LineLoadMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_LINE_LOAD_MODE` reader - when yuv fmt,"]
pub type Win0LineLoadModeR = crate::BitReader<Win0LineLoadMode>;
impl Win0LineLoadModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0LineLoadMode {
        match self.bits {
            false => Win0LineLoadMode::B0,
            true => Win0LineLoadMode::B1,
        }
    }
    #[doc = "load data by axi trans"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0LineLoadMode::B0
    }
    #[doc = "load data by lines"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0LineLoadMode::B1
    }
}
#[doc = "Field `WIN0_LINE_LOAD_MODE` writer - when yuv fmt,"]
pub type Win0LineLoadModeW<'a, REG> = crate::BitWriter<'a, REG, Win0LineLoadMode>;
impl<'a, REG> Win0LineLoadModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "load data by axi trans"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0LineLoadMode::B0)
    }
    #[doc = "load data by lines"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0LineLoadMode::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win0YrgbHorSclMode {
    #[doc = "0: no scale"]
    B00 = 0,
    #[doc = "1: scale up"]
    B01 = 1,
    #[doc = "2: scale down"]
    B10 = 2,
    #[doc = "3: no scale"]
    B11 = 3,
}
impl From<Win0YrgbHorSclMode> for u8 {
    #[inline(always)]
    fn from(variant: Win0YrgbHorSclMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win0YrgbHorSclMode {
    type Ux = u8;
}
#[doc = "Field `WIN0_YRGB_HOR_SCL_MODE` reader - "]
pub type Win0YrgbHorSclModeR = crate::FieldReader<Win0YrgbHorSclMode>;
impl Win0YrgbHorSclModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0YrgbHorSclMode {
        match self.bits {
            0 => Win0YrgbHorSclMode::B00,
            1 => Win0YrgbHorSclMode::B01,
            2 => Win0YrgbHorSclMode::B10,
            3 => Win0YrgbHorSclMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win0YrgbHorSclMode::B00
    }
    #[doc = "scale up"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win0YrgbHorSclMode::B01
    }
    #[doc = "scale down"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win0YrgbHorSclMode::B10
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win0YrgbHorSclMode::B11
    }
}
#[doc = "Field `WIN0_YRGB_HOR_SCL_MODE` writer - "]
pub type Win0YrgbHorSclModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win0YrgbHorSclMode>;
impl<'a, REG> Win0YrgbHorSclModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no scale"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win0YrgbHorSclMode::B00)
    }
    #[doc = "scale up"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win0YrgbHorSclMode::B01)
    }
    #[doc = "scale down"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win0YrgbHorSclMode::B10)
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win0YrgbHorSclMode::B11)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win0YrgbVerSclMode {
    #[doc = "0: no scale"]
    B00 = 0,
    #[doc = "1: scale up"]
    B01 = 1,
    #[doc = "2: scale down"]
    B10 = 2,
    #[doc = "3: no scale"]
    B11 = 3,
}
impl From<Win0YrgbVerSclMode> for u8 {
    #[inline(always)]
    fn from(variant: Win0YrgbVerSclMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win0YrgbVerSclMode {
    type Ux = u8;
}
#[doc = "Field `WIN0_YRGB_VER_SCL_MODE` reader - "]
pub type Win0YrgbVerSclModeR = crate::FieldReader<Win0YrgbVerSclMode>;
impl Win0YrgbVerSclModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0YrgbVerSclMode {
        match self.bits {
            0 => Win0YrgbVerSclMode::B00,
            1 => Win0YrgbVerSclMode::B01,
            2 => Win0YrgbVerSclMode::B10,
            3 => Win0YrgbVerSclMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win0YrgbVerSclMode::B00
    }
    #[doc = "scale up"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win0YrgbVerSclMode::B01
    }
    #[doc = "scale down"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win0YrgbVerSclMode::B10
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win0YrgbVerSclMode::B11
    }
}
#[doc = "Field `WIN0_YRGB_VER_SCL_MODE` writer - "]
pub type Win0YrgbVerSclModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win0YrgbVerSclMode>;
impl<'a, REG> Win0YrgbVerSclModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no scale"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win0YrgbVerSclMode::B00)
    }
    #[doc = "scale up"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win0YrgbVerSclMode::B01)
    }
    #[doc = "scale down"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win0YrgbVerSclMode::B10)
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win0YrgbVerSclMode::B11)
    }
}
#[doc = "win0 horizontal scaler down mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win0YrgbHsdMode {
    #[doc = "0: bilinear"]
    B00 = 0,
    #[doc = "1: average"]
    B01 = 1,
}
impl From<Win0YrgbHsdMode> for u8 {
    #[inline(always)]
    fn from(variant: Win0YrgbHsdMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win0YrgbHsdMode {
    type Ux = u8;
}
#[doc = "Field `WIN0_YRGB_HSD_MODE` reader - win0 horizontal scaler down mode select"]
pub type Win0YrgbHsdModeR = crate::FieldReader<Win0YrgbHsdMode>;
impl Win0YrgbHsdModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Win0YrgbHsdMode> {
        match self.bits {
            0 => Some(Win0YrgbHsdMode::B00),
            1 => Some(Win0YrgbHsdMode::B01),
            _ => None,
        }
    }
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win0YrgbHsdMode::B00
    }
    #[doc = "average"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win0YrgbHsdMode::B01
    }
}
#[doc = "Field `WIN0_YRGB_HSD_MODE` writer - win0 horizontal scaler down mode select"]
pub type Win0YrgbHsdModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Win0YrgbHsdMode>;
impl<'a, REG> Win0YrgbHsdModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win0YrgbHsdMode::B00)
    }
    #[doc = "average"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win0YrgbHsdMode::B01)
    }
}
#[doc = "win0 vertical scaler down mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0YrgbVsuMode {
    #[doc = "0: bilinear"]
    B0 = 0,
    #[doc = "1: bicubic"]
    B1 = 1,
}
impl From<Win0YrgbVsuMode> for bool {
    #[inline(always)]
    fn from(variant: Win0YrgbVsuMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_YRGB_VSU_MODE` reader - win0 vertical scaler down mode select"]
pub type Win0YrgbVsuModeR = crate::BitReader<Win0YrgbVsuMode>;
impl Win0YrgbVsuModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0YrgbVsuMode {
        match self.bits {
            false => Win0YrgbVsuMode::B0,
            true => Win0YrgbVsuMode::B1,
        }
    }
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0YrgbVsuMode::B0
    }
    #[doc = "bicubic"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0YrgbVsuMode::B1
    }
}
#[doc = "Field `WIN0_YRGB_VSU_MODE` writer - win0 vertical scaler down mode select"]
pub type Win0YrgbVsuModeW<'a, REG> = crate::BitWriter<'a, REG, Win0YrgbVsuMode>;
impl<'a, REG> Win0YrgbVsuModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0YrgbVsuMode::B0)
    }
    #[doc = "bicubic"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0YrgbVsuMode::B1)
    }
}
#[doc = "win0 vertical scaler down mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0YrgbVsdMode {
    #[doc = "0: bilinear"]
    B0 = 0,
    #[doc = "1: average"]
    B1 = 1,
}
impl From<Win0YrgbVsdMode> for bool {
    #[inline(always)]
    fn from(variant: Win0YrgbVsdMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_YRGB_VSD_MODE` reader - win0 vertical scaler down mode select"]
pub type Win0YrgbVsdModeR = crate::BitReader<Win0YrgbVsdMode>;
impl Win0YrgbVsdModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0YrgbVsdMode {
        match self.bits {
            false => Win0YrgbVsdMode::B0,
            true => Win0YrgbVsdMode::B1,
        }
    }
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0YrgbVsdMode::B0
    }
    #[doc = "average"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0YrgbVsdMode::B1
    }
}
#[doc = "Field `WIN0_YRGB_VSD_MODE` writer - win0 vertical scaler down mode select"]
pub type Win0YrgbVsdModeW<'a, REG> = crate::BitWriter<'a, REG, Win0YrgbVsdMode>;
impl<'a, REG> Win0YrgbVsdModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0YrgbVsdMode::B0)
    }
    #[doc = "average"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0YrgbVsdMode::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win0CbrHorSclMode {
    #[doc = "0: no scale"]
    B00 = 0,
    #[doc = "1: scale up"]
    B01 = 1,
    #[doc = "2: scale down"]
    B10 = 2,
    #[doc = "3: no scale"]
    B11 = 3,
}
impl From<Win0CbrHorSclMode> for u8 {
    #[inline(always)]
    fn from(variant: Win0CbrHorSclMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win0CbrHorSclMode {
    type Ux = u8;
}
#[doc = "Field `WIN0_CBR_HOR_SCL_MODE` reader - "]
pub type Win0CbrHorSclModeR = crate::FieldReader<Win0CbrHorSclMode>;
impl Win0CbrHorSclModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0CbrHorSclMode {
        match self.bits {
            0 => Win0CbrHorSclMode::B00,
            1 => Win0CbrHorSclMode::B01,
            2 => Win0CbrHorSclMode::B10,
            3 => Win0CbrHorSclMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win0CbrHorSclMode::B00
    }
    #[doc = "scale up"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win0CbrHorSclMode::B01
    }
    #[doc = "scale down"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win0CbrHorSclMode::B10
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win0CbrHorSclMode::B11
    }
}
#[doc = "Field `WIN0_CBR_HOR_SCL_MODE` writer - "]
pub type Win0CbrHorSclModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win0CbrHorSclMode>;
impl<'a, REG> Win0CbrHorSclModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no scale"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win0CbrHorSclMode::B00)
    }
    #[doc = "scale up"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win0CbrHorSclMode::B01)
    }
    #[doc = "scale down"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win0CbrHorSclMode::B10)
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win0CbrHorSclMode::B11)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win0CbrVerSclMode {
    #[doc = "0: no scale"]
    B00 = 0,
    #[doc = "1: scale up"]
    B01 = 1,
    #[doc = "2: scale down"]
    B10 = 2,
    #[doc = "3: no scale"]
    B11 = 3,
}
impl From<Win0CbrVerSclMode> for u8 {
    #[inline(always)]
    fn from(variant: Win0CbrVerSclMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win0CbrVerSclMode {
    type Ux = u8;
}
#[doc = "Field `WIN0_CBR_VER_SCL_MODE` reader - "]
pub type Win0CbrVerSclModeR = crate::FieldReader<Win0CbrVerSclMode>;
impl Win0CbrVerSclModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0CbrVerSclMode {
        match self.bits {
            0 => Win0CbrVerSclMode::B00,
            1 => Win0CbrVerSclMode::B01,
            2 => Win0CbrVerSclMode::B10,
            3 => Win0CbrVerSclMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win0CbrVerSclMode::B00
    }
    #[doc = "scale up"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win0CbrVerSclMode::B01
    }
    #[doc = "scale down"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win0CbrVerSclMode::B10
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win0CbrVerSclMode::B11
    }
}
#[doc = "Field `WIN0_CBR_VER_SCL_MODE` writer - "]
pub type Win0CbrVerSclModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win0CbrVerSclMode>;
impl<'a, REG> Win0CbrVerSclModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no scale"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win0CbrVerSclMode::B00)
    }
    #[doc = "scale up"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win0CbrVerSclMode::B01)
    }
    #[doc = "scale down"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win0CbrVerSclMode::B10)
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win0CbrVerSclMode::B11)
    }
}
#[doc = "win0 horizontal scaler down mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win0CbrHsdMode {
    #[doc = "0: bilinear"]
    B00 = 0,
    #[doc = "1: bicubic"]
    B01 = 1,
    #[doc = "2: average"]
    B10 = 2,
}
impl From<Win0CbrHsdMode> for u8 {
    #[inline(always)]
    fn from(variant: Win0CbrHsdMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win0CbrHsdMode {
    type Ux = u8;
}
#[doc = "Field `WIN0_CBR_HSD_MODE` reader - win0 horizontal scaler down mode select"]
pub type Win0CbrHsdModeR = crate::FieldReader<Win0CbrHsdMode>;
impl Win0CbrHsdModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Win0CbrHsdMode> {
        match self.bits {
            0 => Some(Win0CbrHsdMode::B00),
            1 => Some(Win0CbrHsdMode::B01),
            2 => Some(Win0CbrHsdMode::B10),
            _ => None,
        }
    }
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win0CbrHsdMode::B00
    }
    #[doc = "bicubic"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win0CbrHsdMode::B01
    }
    #[doc = "average"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win0CbrHsdMode::B10
    }
}
#[doc = "Field `WIN0_CBR_HSD_MODE` writer - win0 horizontal scaler down mode select"]
pub type Win0CbrHsdModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Win0CbrHsdMode>;
impl<'a, REG> Win0CbrHsdModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win0CbrHsdMode::B00)
    }
    #[doc = "bicubic"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win0CbrHsdMode::B01)
    }
    #[doc = "average"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win0CbrHsdMode::B10)
    }
}
#[doc = "win0 vertical scaler down mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0CbrVsuMode {
    #[doc = "0: bilinear"]
    B0 = 0,
    #[doc = "1: bicubic"]
    B1 = 1,
}
impl From<Win0CbrVsuMode> for bool {
    #[inline(always)]
    fn from(variant: Win0CbrVsuMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_CBR_VSU_MODE` reader - win0 vertical scaler down mode select"]
pub type Win0CbrVsuModeR = crate::BitReader<Win0CbrVsuMode>;
impl Win0CbrVsuModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0CbrVsuMode {
        match self.bits {
            false => Win0CbrVsuMode::B0,
            true => Win0CbrVsuMode::B1,
        }
    }
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0CbrVsuMode::B0
    }
    #[doc = "bicubic"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0CbrVsuMode::B1
    }
}
#[doc = "Field `WIN0_CBR_VSU_MODE` writer - win0 vertical scaler down mode select"]
pub type Win0CbrVsuModeW<'a, REG> = crate::BitWriter<'a, REG, Win0CbrVsuMode>;
impl<'a, REG> Win0CbrVsuModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0CbrVsuMode::B0)
    }
    #[doc = "bicubic"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0CbrVsuMode::B1)
    }
}
#[doc = "win0 vertical scaler down mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win0CbrVsdMode {
    #[doc = "0: bilinear"]
    B0 = 0,
    #[doc = "1: average"]
    B1 = 1,
}
impl From<Win0CbrVsdMode> for bool {
    #[inline(always)]
    fn from(variant: Win0CbrVsdMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN0_CBR_VSD_MODE` reader - win0 vertical scaler down mode select"]
pub type Win0CbrVsdModeR = crate::BitReader<Win0CbrVsdMode>;
impl Win0CbrVsdModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win0CbrVsdMode {
        match self.bits {
            false => Win0CbrVsdMode::B0,
            true => Win0CbrVsdMode::B1,
        }
    }
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win0CbrVsdMode::B0
    }
    #[doc = "average"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win0CbrVsdMode::B1
    }
}
#[doc = "Field `WIN0_CBR_VSD_MODE` writer - win0 vertical scaler down mode select"]
pub type Win0CbrVsdModeW<'a, REG> = crate::BitWriter<'a, REG, Win0CbrVsdMode>;
impl<'a, REG> Win0CbrVsdModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win0CbrVsdMode::B0)
    }
    #[doc = "average"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win0CbrVsdMode::B1)
    }
}
impl R {
    #[doc = "Bit 0 - win0 axi bus yrgb data gather transfer enable"]
    #[inline(always)]
    pub fn win0_yrgb_axi_gather_en(&self) -> Win0YrgbAxiGatherEnR {
        Win0YrgbAxiGatherEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - win0 axi bus cbr data gather transfer enable"]
    #[inline(always)]
    pub fn win0_cbr_axi_gather_en(&self) -> Win0CbrAxiGatherEnR {
        Win0CbrAxiGatherEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn win0_bic_coe_sel(&self) -> Win0BicCoeSelR {
        Win0BicCoeSelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - yrgb_src/yrgb_dst >= 4"]
    #[inline(always)]
    pub fn win0_vsd_yrgb_gt4(&self) -> Win0VsdYrgbGt4R {
        Win0VsdYrgbGt4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - yrgb_src/yrgb_dst >= 2"]
    #[inline(always)]
    pub fn win0_vsd_yrgb_gt2(&self) -> Win0VsdYrgbGt2R {
        Win0VsdYrgbGt2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - cbr_src/cbr_dst >= 4"]
    #[inline(always)]
    pub fn win0_vsd_cbr_gt4(&self) -> Win0VsdCbrGt4R {
        Win0VsdCbrGt4R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - cbr_src/cbr_dst >= 2"]
    #[inline(always)]
    pub fn win0_vsd_cbr_gt2(&self) -> Win0VsdCbrGt2R {
        Win0VsdCbrGt2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - win0 axi yrgb data transfer gather number"]
    #[inline(always)]
    pub fn win0_yrgb_axi_gather_num(&self) -> Win0YrgbAxiGatherNumR {
        Win0YrgbAxiGatherNumR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - win0 axi cbr data transfer gather number"]
    #[inline(always)]
    pub fn win0_cbr_axi_gather_num(&self) -> Win0CbrAxiGatherNumR {
        Win0CbrAxiGatherNumR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - when yuv fmt,"]
    #[inline(always)]
    pub fn win0_line_load_mode(&self) -> Win0LineLoadModeR {
        Win0LineLoadModeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn win0_yrgb_hor_scl_mode(&self) -> Win0YrgbHorSclModeR {
        Win0YrgbHorSclModeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn win0_yrgb_ver_scl_mode(&self) -> Win0YrgbVerSclModeR {
        Win0YrgbVerSclModeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - win0 horizontal scaler down mode select"]
    #[inline(always)]
    pub fn win0_yrgb_hsd_mode(&self) -> Win0YrgbHsdModeR {
        Win0YrgbHsdModeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - win0 vertical scaler down mode select"]
    #[inline(always)]
    pub fn win0_yrgb_vsu_mode(&self) -> Win0YrgbVsuModeR {
        Win0YrgbVsuModeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - win0 vertical scaler down mode select"]
    #[inline(always)]
    pub fn win0_yrgb_vsd_mode(&self) -> Win0YrgbVsdModeR {
        Win0YrgbVsdModeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn win0_cbr_hor_scl_mode(&self) -> Win0CbrHorSclModeR {
        Win0CbrHorSclModeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn win0_cbr_ver_scl_mode(&self) -> Win0CbrVerSclModeR {
        Win0CbrVerSclModeR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - win0 horizontal scaler down mode select"]
    #[inline(always)]
    pub fn win0_cbr_hsd_mode(&self) -> Win0CbrHsdModeR {
        Win0CbrHsdModeR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - win0 vertical scaler down mode select"]
    #[inline(always)]
    pub fn win0_cbr_vsu_mode(&self) -> Win0CbrVsuModeR {
        Win0CbrVsuModeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - win0 vertical scaler down mode select"]
    #[inline(always)]
    pub fn win0_cbr_vsd_mode(&self) -> Win0CbrVsdModeR {
        Win0CbrVsdModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - win0 axi bus yrgb data gather transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn win0_yrgb_axi_gather_en(&mut self) -> Win0YrgbAxiGatherEnW<Win0Ctrl1Spec> {
        Win0YrgbAxiGatherEnW::new(self, 0)
    }
    #[doc = "Bit 1 - win0 axi bus cbr data gather transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn win0_cbr_axi_gather_en(&mut self) -> Win0CbrAxiGatherEnW<Win0Ctrl1Spec> {
        Win0CbrAxiGatherEnW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn win0_bic_coe_sel(&mut self) -> Win0BicCoeSelW<Win0Ctrl1Spec> {
        Win0BicCoeSelW::new(self, 2)
    }
    #[doc = "Bit 4 - yrgb_src/yrgb_dst >= 4"]
    #[inline(always)]
    #[must_use]
    pub fn win0_vsd_yrgb_gt4(&mut self) -> Win0VsdYrgbGt4W<Win0Ctrl1Spec> {
        Win0VsdYrgbGt4W::new(self, 4)
    }
    #[doc = "Bit 5 - yrgb_src/yrgb_dst >= 2"]
    #[inline(always)]
    #[must_use]
    pub fn win0_vsd_yrgb_gt2(&mut self) -> Win0VsdYrgbGt2W<Win0Ctrl1Spec> {
        Win0VsdYrgbGt2W::new(self, 5)
    }
    #[doc = "Bit 6 - cbr_src/cbr_dst >= 4"]
    #[inline(always)]
    #[must_use]
    pub fn win0_vsd_cbr_gt4(&mut self) -> Win0VsdCbrGt4W<Win0Ctrl1Spec> {
        Win0VsdCbrGt4W::new(self, 6)
    }
    #[doc = "Bit 7 - cbr_src/cbr_dst >= 2"]
    #[inline(always)]
    #[must_use]
    pub fn win0_vsd_cbr_gt2(&mut self) -> Win0VsdCbrGt2W<Win0Ctrl1Spec> {
        Win0VsdCbrGt2W::new(self, 7)
    }
    #[doc = "Bits 8:11 - win0 axi yrgb data transfer gather number"]
    #[inline(always)]
    #[must_use]
    pub fn win0_yrgb_axi_gather_num(&mut self) -> Win0YrgbAxiGatherNumW<Win0Ctrl1Spec> {
        Win0YrgbAxiGatherNumW::new(self, 8)
    }
    #[doc = "Bits 12:14 - win0 axi cbr data transfer gather number"]
    #[inline(always)]
    #[must_use]
    pub fn win0_cbr_axi_gather_num(&mut self) -> Win0CbrAxiGatherNumW<Win0Ctrl1Spec> {
        Win0CbrAxiGatherNumW::new(self, 12)
    }
    #[doc = "Bit 15 - when yuv fmt,"]
    #[inline(always)]
    #[must_use]
    pub fn win0_line_load_mode(&mut self) -> Win0LineLoadModeW<Win0Ctrl1Spec> {
        Win0LineLoadModeW::new(self, 15)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn win0_yrgb_hor_scl_mode(&mut self) -> Win0YrgbHorSclModeW<Win0Ctrl1Spec> {
        Win0YrgbHorSclModeW::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn win0_yrgb_ver_scl_mode(&mut self) -> Win0YrgbVerSclModeW<Win0Ctrl1Spec> {
        Win0YrgbVerSclModeW::new(self, 18)
    }
    #[doc = "Bits 20:21 - win0 horizontal scaler down mode select"]
    #[inline(always)]
    #[must_use]
    pub fn win0_yrgb_hsd_mode(&mut self) -> Win0YrgbHsdModeW<Win0Ctrl1Spec> {
        Win0YrgbHsdModeW::new(self, 20)
    }
    #[doc = "Bit 22 - win0 vertical scaler down mode select"]
    #[inline(always)]
    #[must_use]
    pub fn win0_yrgb_vsu_mode(&mut self) -> Win0YrgbVsuModeW<Win0Ctrl1Spec> {
        Win0YrgbVsuModeW::new(self, 22)
    }
    #[doc = "Bit 23 - win0 vertical scaler down mode select"]
    #[inline(always)]
    #[must_use]
    pub fn win0_yrgb_vsd_mode(&mut self) -> Win0YrgbVsdModeW<Win0Ctrl1Spec> {
        Win0YrgbVsdModeW::new(self, 23)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn win0_cbr_hor_scl_mode(&mut self) -> Win0CbrHorSclModeW<Win0Ctrl1Spec> {
        Win0CbrHorSclModeW::new(self, 24)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    #[must_use]
    pub fn win0_cbr_ver_scl_mode(&mut self) -> Win0CbrVerSclModeW<Win0Ctrl1Spec> {
        Win0CbrVerSclModeW::new(self, 26)
    }
    #[doc = "Bits 28:29 - win0 horizontal scaler down mode select"]
    #[inline(always)]
    #[must_use]
    pub fn win0_cbr_hsd_mode(&mut self) -> Win0CbrHsdModeW<Win0Ctrl1Spec> {
        Win0CbrHsdModeW::new(self, 28)
    }
    #[doc = "Bit 30 - win0 vertical scaler down mode select"]
    #[inline(always)]
    #[must_use]
    pub fn win0_cbr_vsu_mode(&mut self) -> Win0CbrVsuModeW<Win0Ctrl1Spec> {
        Win0CbrVsuModeW::new(self, 30)
    }
    #[doc = "Bit 31 - win0 vertical scaler down mode select"]
    #[inline(always)]
    #[must_use]
    pub fn win0_cbr_vsd_mode(&mut self) -> Win0CbrVsdModeW<Win0Ctrl1Spec> {
        Win0CbrVsdModeW::new(self, 31)
    }
}
#[doc = "Win0 ctrl register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win0Ctrl1Spec;
impl crate::RegisterSpec for Win0Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win0_ctrl1::R`](R) reader structure"]
impl crate::Readable for Win0Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`win0_ctrl1::W`](W) writer structure"]
impl crate::Writable for Win0Ctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN0_CTRL1 to value 0"]
impl crate::Resettable for Win0Ctrl1Spec {
    const RESET_VALUE: u32 = 0;
}
