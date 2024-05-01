#[doc = "Register `WIN1_CTRL1` reader"]
pub type R = crate::R<Win1Ctrl1Spec>;
#[doc = "Register `WIN1_CTRL1` writer"]
pub type W = crate::W<Win1Ctrl1Spec>;
#[doc = "Field `WIN1_YRGB_AXI_GATHER_EN` reader - win1 yrgb axi bus gather enable"]
pub type Win1YrgbAxiGatherEnR = crate::BitReader;
#[doc = "Field `WIN1_YRGB_AXI_GATHER_EN` writer - win1 yrgb axi bus gather enable"]
pub type Win1YrgbAxiGatherEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN1_CBR_AXI_GATHER_EN` reader - win1 cbr axi bus gather enable"]
pub type Win1CbrAxiGatherEnR = crate::BitReader;
#[doc = "Field `WIN1_CBR_AXI_GATHER_EN` writer - win1 cbr axi bus gather enable"]
pub type Win1CbrAxiGatherEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win1BicCoeSel {
    #[doc = "0: PRECISE"]
    B00 = 0,
    #[doc = "1: SPLINE"]
    B01 = 1,
    #[doc = "2: CATROM"]
    B10 = 2,
    #[doc = "3: MITCHELL"]
    B11 = 3,
}
impl From<Win1BicCoeSel> for u8 {
    #[inline(always)]
    fn from(variant: Win1BicCoeSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win1BicCoeSel {
    type Ux = u8;
}
#[doc = "Field `WIN1_BIC_COE_SEL` reader - "]
pub type Win1BicCoeSelR = crate::FieldReader<Win1BicCoeSel>;
impl Win1BicCoeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1BicCoeSel {
        match self.bits {
            0 => Win1BicCoeSel::B00,
            1 => Win1BicCoeSel::B01,
            2 => Win1BicCoeSel::B10,
            3 => Win1BicCoeSel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "PRECISE"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win1BicCoeSel::B00
    }
    #[doc = "SPLINE"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win1BicCoeSel::B01
    }
    #[doc = "CATROM"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win1BicCoeSel::B10
    }
    #[doc = "MITCHELL"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win1BicCoeSel::B11
    }
}
#[doc = "Field `WIN1_BIC_COE_SEL` writer - "]
pub type Win1BicCoeSelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win1BicCoeSel>;
impl<'a, REG> Win1BicCoeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRECISE"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win1BicCoeSel::B00)
    }
    #[doc = "SPLINE"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win1BicCoeSel::B01)
    }
    #[doc = "CATROM"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win1BicCoeSel::B10)
    }
    #[doc = "MITCHELL"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win1BicCoeSel::B11)
    }
}
#[doc = "Field `WIN1_VSD_YRGB_GT4` reader - yrgb_src/yrgb_dst >= 4"]
pub type Win1VsdYrgbGt4R = crate::BitReader;
#[doc = "Field `WIN1_VSD_YRGB_GT4` writer - yrgb_src/yrgb_dst >= 4"]
pub type Win1VsdYrgbGt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN1_VSD_YRGB_GT2` reader - yrgb_src/yrgb_dst >= 2"]
pub type Win1VsdYrgbGt2R = crate::BitReader;
#[doc = "Field `WIN1_VSD_YRGB_GT2` writer - yrgb_src/yrgb_dst >= 2"]
pub type Win1VsdYrgbGt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN1_VSD_CBR_GT4` reader - cbr_src/cbr_dst >= 4"]
pub type Win1VsdCbrGt4R = crate::BitReader;
#[doc = "Field `WIN1_VSD_CBR_GT4` writer - cbr_src/cbr_dst >= 4"]
pub type Win1VsdCbrGt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN1_VSD_CBR_GT2` reader - cbr_src/cbr_dst >= 2"]
pub type Win1VsdCbrGt2R = crate::BitReader;
#[doc = "Field `WIN1_VSD_CBR_GT2` writer - cbr_src/cbr_dst >= 2"]
pub type Win1VsdCbrGt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN1_YRGB_AXI_GATHER_NUM` reader - win1 axi yrgb data transfer gather number"]
pub type Win1YrgbAxiGatherNumR = crate::FieldReader;
#[doc = "Field `WIN1_YRGB_AXI_GATHER_NUM` writer - win1 axi yrgb data transfer gather number"]
pub type Win1YrgbAxiGatherNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WIN1_CBR_AXI_GATHER_NUM` reader - win1 axi cbr data transfer gather number"]
pub type Win1CbrAxiGatherNumR = crate::FieldReader;
#[doc = "Field `WIN1_CBR_AXI_GATHER_NUM` writer - win1 axi cbr data transfer gather number"]
pub type Win1CbrAxiGatherNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "when yuv fmt,\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1LineLoadMode {
    #[doc = "0: load data by pixels"]
    B0 = 0,
    #[doc = "1: load data by lines"]
    B1 = 1,
}
impl From<Win1LineLoadMode> for bool {
    #[inline(always)]
    fn from(variant: Win1LineLoadMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_LINE_LOAD_MODE` reader - when yuv fmt,"]
pub type Win1LineLoadModeR = crate::BitReader<Win1LineLoadMode>;
impl Win1LineLoadModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1LineLoadMode {
        match self.bits {
            false => Win1LineLoadMode::B0,
            true => Win1LineLoadMode::B1,
        }
    }
    #[doc = "load data by pixels"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1LineLoadMode::B0
    }
    #[doc = "load data by lines"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1LineLoadMode::B1
    }
}
#[doc = "Field `WIN1_LINE_LOAD_MODE` writer - when yuv fmt,"]
pub type Win1LineLoadModeW<'a, REG> = crate::BitWriter<'a, REG, Win1LineLoadMode>;
impl<'a, REG> Win1LineLoadModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "load data by pixels"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1LineLoadMode::B0)
    }
    #[doc = "load data by lines"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1LineLoadMode::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win1YrgbHorSclMode {
    #[doc = "0: no scale"]
    B00 = 0,
    #[doc = "1: scale up"]
    B01 = 1,
    #[doc = "2: scale down"]
    B10 = 2,
    #[doc = "3: no scale"]
    B11 = 3,
}
impl From<Win1YrgbHorSclMode> for u8 {
    #[inline(always)]
    fn from(variant: Win1YrgbHorSclMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win1YrgbHorSclMode {
    type Ux = u8;
}
#[doc = "Field `WIN1_YRGB_HOR_SCL_MODE` reader - "]
pub type Win1YrgbHorSclModeR = crate::FieldReader<Win1YrgbHorSclMode>;
impl Win1YrgbHorSclModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1YrgbHorSclMode {
        match self.bits {
            0 => Win1YrgbHorSclMode::B00,
            1 => Win1YrgbHorSclMode::B01,
            2 => Win1YrgbHorSclMode::B10,
            3 => Win1YrgbHorSclMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win1YrgbHorSclMode::B00
    }
    #[doc = "scale up"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win1YrgbHorSclMode::B01
    }
    #[doc = "scale down"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win1YrgbHorSclMode::B10
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win1YrgbHorSclMode::B11
    }
}
#[doc = "Field `WIN1_YRGB_HOR_SCL_MODE` writer - "]
pub type Win1YrgbHorSclModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win1YrgbHorSclMode>;
impl<'a, REG> Win1YrgbHorSclModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no scale"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win1YrgbHorSclMode::B00)
    }
    #[doc = "scale up"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win1YrgbHorSclMode::B01)
    }
    #[doc = "scale down"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win1YrgbHorSclMode::B10)
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win1YrgbHorSclMode::B11)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win1YrgbVerSclMode {
    #[doc = "0: no scale"]
    B00 = 0,
    #[doc = "1: scale up"]
    B01 = 1,
    #[doc = "2: scale down"]
    B10 = 2,
    #[doc = "3: no scale"]
    B11 = 3,
}
impl From<Win1YrgbVerSclMode> for u8 {
    #[inline(always)]
    fn from(variant: Win1YrgbVerSclMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win1YrgbVerSclMode {
    type Ux = u8;
}
#[doc = "Field `WIN1_YRGB_VER_SCL_MODE` reader - "]
pub type Win1YrgbVerSclModeR = crate::FieldReader<Win1YrgbVerSclMode>;
impl Win1YrgbVerSclModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1YrgbVerSclMode {
        match self.bits {
            0 => Win1YrgbVerSclMode::B00,
            1 => Win1YrgbVerSclMode::B01,
            2 => Win1YrgbVerSclMode::B10,
            3 => Win1YrgbVerSclMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win1YrgbVerSclMode::B00
    }
    #[doc = "scale up"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win1YrgbVerSclMode::B01
    }
    #[doc = "scale down"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win1YrgbVerSclMode::B10
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win1YrgbVerSclMode::B11
    }
}
#[doc = "Field `WIN1_YRGB_VER_SCL_MODE` writer - "]
pub type Win1YrgbVerSclModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win1YrgbVerSclMode>;
impl<'a, REG> Win1YrgbVerSclModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no scale"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win1YrgbVerSclMode::B00)
    }
    #[doc = "scale up"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win1YrgbVerSclMode::B01)
    }
    #[doc = "scale down"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win1YrgbVerSclMode::B10)
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win1YrgbVerSclMode::B11)
    }
}
#[doc = "win1 horizontal scaler down mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win1YrgbHsdMode {
    #[doc = "0: bilinear"]
    B00 = 0,
    #[doc = "1: average"]
    B01 = 1,
}
impl From<Win1YrgbHsdMode> for u8 {
    #[inline(always)]
    fn from(variant: Win1YrgbHsdMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win1YrgbHsdMode {
    type Ux = u8;
}
#[doc = "Field `WIN1_YRGB_HSD_MODE` reader - win1 horizontal scaler down mode select"]
pub type Win1YrgbHsdModeR = crate::FieldReader<Win1YrgbHsdMode>;
impl Win1YrgbHsdModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Win1YrgbHsdMode> {
        match self.bits {
            0 => Some(Win1YrgbHsdMode::B00),
            1 => Some(Win1YrgbHsdMode::B01),
            _ => None,
        }
    }
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win1YrgbHsdMode::B00
    }
    #[doc = "average"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win1YrgbHsdMode::B01
    }
}
#[doc = "Field `WIN1_YRGB_HSD_MODE` writer - win1 horizontal scaler down mode select"]
pub type Win1YrgbHsdModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Win1YrgbHsdMode>;
impl<'a, REG> Win1YrgbHsdModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win1YrgbHsdMode::B00)
    }
    #[doc = "average"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win1YrgbHsdMode::B01)
    }
}
#[doc = "win1 vertical scaler up mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1YrgbVsuMode {
    #[doc = "0: bilinear"]
    B0 = 0,
    #[doc = "1: bicubic"]
    B1 = 1,
}
impl From<Win1YrgbVsuMode> for bool {
    #[inline(always)]
    fn from(variant: Win1YrgbVsuMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_YRGB_VSU_MODE` reader - win1 vertical scaler up mode select"]
pub type Win1YrgbVsuModeR = crate::BitReader<Win1YrgbVsuMode>;
impl Win1YrgbVsuModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1YrgbVsuMode {
        match self.bits {
            false => Win1YrgbVsuMode::B0,
            true => Win1YrgbVsuMode::B1,
        }
    }
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1YrgbVsuMode::B0
    }
    #[doc = "bicubic"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1YrgbVsuMode::B1
    }
}
#[doc = "Field `WIN1_YRGB_VSU_MODE` writer - win1 vertical scaler up mode select"]
pub type Win1YrgbVsuModeW<'a, REG> = crate::BitWriter<'a, REG, Win1YrgbVsuMode>;
impl<'a, REG> Win1YrgbVsuModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1YrgbVsuMode::B0)
    }
    #[doc = "bicubic"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1YrgbVsuMode::B1)
    }
}
#[doc = "win1 vertical scaler down mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1YrgbVsdMode {
    #[doc = "0: bilinear"]
    B0 = 0,
    #[doc = "1: average"]
    B1 = 1,
}
impl From<Win1YrgbVsdMode> for bool {
    #[inline(always)]
    fn from(variant: Win1YrgbVsdMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_YRGB_VSD_MODE` reader - win1 vertical scaler down mode select"]
pub type Win1YrgbVsdModeR = crate::BitReader<Win1YrgbVsdMode>;
impl Win1YrgbVsdModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1YrgbVsdMode {
        match self.bits {
            false => Win1YrgbVsdMode::B0,
            true => Win1YrgbVsdMode::B1,
        }
    }
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1YrgbVsdMode::B0
    }
    #[doc = "average"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1YrgbVsdMode::B1
    }
}
#[doc = "Field `WIN1_YRGB_VSD_MODE` writer - win1 vertical scaler down mode select"]
pub type Win1YrgbVsdModeW<'a, REG> = crate::BitWriter<'a, REG, Win1YrgbVsdMode>;
impl<'a, REG> Win1YrgbVsdModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1YrgbVsdMode::B0)
    }
    #[doc = "average"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1YrgbVsdMode::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win1CbrHorSclMode {
    #[doc = "0: no scale"]
    B00 = 0,
    #[doc = "1: scale up"]
    B01 = 1,
    #[doc = "2: scale down"]
    B10 = 2,
    #[doc = "3: no scale"]
    B11 = 3,
}
impl From<Win1CbrHorSclMode> for u8 {
    #[inline(always)]
    fn from(variant: Win1CbrHorSclMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win1CbrHorSclMode {
    type Ux = u8;
}
#[doc = "Field `WIN1_CBR_HOR_SCL_MODE` reader - "]
pub type Win1CbrHorSclModeR = crate::FieldReader<Win1CbrHorSclMode>;
impl Win1CbrHorSclModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1CbrHorSclMode {
        match self.bits {
            0 => Win1CbrHorSclMode::B00,
            1 => Win1CbrHorSclMode::B01,
            2 => Win1CbrHorSclMode::B10,
            3 => Win1CbrHorSclMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win1CbrHorSclMode::B00
    }
    #[doc = "scale up"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win1CbrHorSclMode::B01
    }
    #[doc = "scale down"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win1CbrHorSclMode::B10
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win1CbrHorSclMode::B11
    }
}
#[doc = "Field `WIN1_CBR_HOR_SCL_MODE` writer - "]
pub type Win1CbrHorSclModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win1CbrHorSclMode>;
impl<'a, REG> Win1CbrHorSclModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no scale"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win1CbrHorSclMode::B00)
    }
    #[doc = "scale up"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win1CbrHorSclMode::B01)
    }
    #[doc = "scale down"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win1CbrHorSclMode::B10)
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win1CbrHorSclMode::B11)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win1CbrVerSclMode {
    #[doc = "0: no scale"]
    B00 = 0,
    #[doc = "1: scale up"]
    B01 = 1,
    #[doc = "2: scale down"]
    B10 = 2,
    #[doc = "3: no scale"]
    B11 = 3,
}
impl From<Win1CbrVerSclMode> for u8 {
    #[inline(always)]
    fn from(variant: Win1CbrVerSclMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win1CbrVerSclMode {
    type Ux = u8;
}
#[doc = "Field `WIN1_CBR_VER_SCL_MODE` reader - "]
pub type Win1CbrVerSclModeR = crate::FieldReader<Win1CbrVerSclMode>;
impl Win1CbrVerSclModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1CbrVerSclMode {
        match self.bits {
            0 => Win1CbrVerSclMode::B00,
            1 => Win1CbrVerSclMode::B01,
            2 => Win1CbrVerSclMode::B10,
            3 => Win1CbrVerSclMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win1CbrVerSclMode::B00
    }
    #[doc = "scale up"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win1CbrVerSclMode::B01
    }
    #[doc = "scale down"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win1CbrVerSclMode::B10
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win1CbrVerSclMode::B11
    }
}
#[doc = "Field `WIN1_CBR_VER_SCL_MODE` writer - "]
pub type Win1CbrVerSclModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win1CbrVerSclMode>;
impl<'a, REG> Win1CbrVerSclModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no scale"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win1CbrVerSclMode::B00)
    }
    #[doc = "scale up"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win1CbrVerSclMode::B01)
    }
    #[doc = "scale down"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win1CbrVerSclMode::B10)
    }
    #[doc = "no scale"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win1CbrVerSclMode::B11)
    }
}
#[doc = "win1 horizontal scaler down mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win1CbrHsdMode {
    #[doc = "0: bilinear"]
    B00 = 0,
    #[doc = "1: bicubic"]
    B01 = 1,
    #[doc = "2: average"]
    B10 = 2,
}
impl From<Win1CbrHsdMode> for u8 {
    #[inline(always)]
    fn from(variant: Win1CbrHsdMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win1CbrHsdMode {
    type Ux = u8;
}
#[doc = "Field `WIN1_CBR_HSD_MODE` reader - win1 horizontal scaler down mode select"]
pub type Win1CbrHsdModeR = crate::FieldReader<Win1CbrHsdMode>;
impl Win1CbrHsdModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Win1CbrHsdMode> {
        match self.bits {
            0 => Some(Win1CbrHsdMode::B00),
            1 => Some(Win1CbrHsdMode::B01),
            2 => Some(Win1CbrHsdMode::B10),
            _ => None,
        }
    }
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win1CbrHsdMode::B00
    }
    #[doc = "bicubic"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win1CbrHsdMode::B01
    }
    #[doc = "average"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win1CbrHsdMode::B10
    }
}
#[doc = "Field `WIN1_CBR_HSD_MODE` writer - win1 horizontal scaler down mode select"]
pub type Win1CbrHsdModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Win1CbrHsdMode>;
impl<'a, REG> Win1CbrHsdModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win1CbrHsdMode::B00)
    }
    #[doc = "bicubic"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win1CbrHsdMode::B01)
    }
    #[doc = "average"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win1CbrHsdMode::B10)
    }
}
#[doc = "win1 vertical scaler up mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1CbrVsuMode {
    #[doc = "0: bilinear"]
    B0 = 0,
    #[doc = "1: bicubic"]
    B1 = 1,
}
impl From<Win1CbrVsuMode> for bool {
    #[inline(always)]
    fn from(variant: Win1CbrVsuMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_CBR_VSU_MODE` reader - win1 vertical scaler up mode select"]
pub type Win1CbrVsuModeR = crate::BitReader<Win1CbrVsuMode>;
impl Win1CbrVsuModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1CbrVsuMode {
        match self.bits {
            false => Win1CbrVsuMode::B0,
            true => Win1CbrVsuMode::B1,
        }
    }
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1CbrVsuMode::B0
    }
    #[doc = "bicubic"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1CbrVsuMode::B1
    }
}
#[doc = "Field `WIN1_CBR_VSU_MODE` writer - win1 vertical scaler up mode select"]
pub type Win1CbrVsuModeW<'a, REG> = crate::BitWriter<'a, REG, Win1CbrVsuMode>;
impl<'a, REG> Win1CbrVsuModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1CbrVsuMode::B0)
    }
    #[doc = "bicubic"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1CbrVsuMode::B1)
    }
}
#[doc = "win1 vertical scaler down mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win1CbrVsdMode {
    #[doc = "0: bilinear"]
    B0 = 0,
    #[doc = "1: average"]
    B1 = 1,
}
impl From<Win1CbrVsdMode> for bool {
    #[inline(always)]
    fn from(variant: Win1CbrVsdMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN1_CBR_VSD_MODE` reader - win1 vertical scaler down mode select"]
pub type Win1CbrVsdModeR = crate::BitReader<Win1CbrVsdMode>;
impl Win1CbrVsdModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win1CbrVsdMode {
        match self.bits {
            false => Win1CbrVsdMode::B0,
            true => Win1CbrVsdMode::B1,
        }
    }
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win1CbrVsdMode::B0
    }
    #[doc = "average"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win1CbrVsdMode::B1
    }
}
#[doc = "Field `WIN1_CBR_VSD_MODE` writer - win1 vertical scaler down mode select"]
pub type Win1CbrVsdModeW<'a, REG> = crate::BitWriter<'a, REG, Win1CbrVsdMode>;
impl<'a, REG> Win1CbrVsdModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "bilinear"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win1CbrVsdMode::B0)
    }
    #[doc = "average"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win1CbrVsdMode::B1)
    }
}
impl R {
    #[doc = "Bit 0 - win1 yrgb axi bus gather enable"]
    #[inline(always)]
    pub fn win1_yrgb_axi_gather_en(&self) -> Win1YrgbAxiGatherEnR {
        Win1YrgbAxiGatherEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - win1 cbr axi bus gather enable"]
    #[inline(always)]
    pub fn win1_cbr_axi_gather_en(&self) -> Win1CbrAxiGatherEnR {
        Win1CbrAxiGatherEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn win1_bic_coe_sel(&self) -> Win1BicCoeSelR {
        Win1BicCoeSelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - yrgb_src/yrgb_dst >= 4"]
    #[inline(always)]
    pub fn win1_vsd_yrgb_gt4(&self) -> Win1VsdYrgbGt4R {
        Win1VsdYrgbGt4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - yrgb_src/yrgb_dst >= 2"]
    #[inline(always)]
    pub fn win1_vsd_yrgb_gt2(&self) -> Win1VsdYrgbGt2R {
        Win1VsdYrgbGt2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - cbr_src/cbr_dst >= 4"]
    #[inline(always)]
    pub fn win1_vsd_cbr_gt4(&self) -> Win1VsdCbrGt4R {
        Win1VsdCbrGt4R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - cbr_src/cbr_dst >= 2"]
    #[inline(always)]
    pub fn win1_vsd_cbr_gt2(&self) -> Win1VsdCbrGt2R {
        Win1VsdCbrGt2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - win1 axi yrgb data transfer gather number"]
    #[inline(always)]
    pub fn win1_yrgb_axi_gather_num(&self) -> Win1YrgbAxiGatherNumR {
        Win1YrgbAxiGatherNumR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - win1 axi cbr data transfer gather number"]
    #[inline(always)]
    pub fn win1_cbr_axi_gather_num(&self) -> Win1CbrAxiGatherNumR {
        Win1CbrAxiGatherNumR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - when yuv fmt,"]
    #[inline(always)]
    pub fn win1_line_load_mode(&self) -> Win1LineLoadModeR {
        Win1LineLoadModeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn win1_yrgb_hor_scl_mode(&self) -> Win1YrgbHorSclModeR {
        Win1YrgbHorSclModeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn win1_yrgb_ver_scl_mode(&self) -> Win1YrgbVerSclModeR {
        Win1YrgbVerSclModeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - win1 horizontal scaler down mode select"]
    #[inline(always)]
    pub fn win1_yrgb_hsd_mode(&self) -> Win1YrgbHsdModeR {
        Win1YrgbHsdModeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - win1 vertical scaler up mode select"]
    #[inline(always)]
    pub fn win1_yrgb_vsu_mode(&self) -> Win1YrgbVsuModeR {
        Win1YrgbVsuModeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - win1 vertical scaler down mode select"]
    #[inline(always)]
    pub fn win1_yrgb_vsd_mode(&self) -> Win1YrgbVsdModeR {
        Win1YrgbVsdModeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn win1_cbr_hor_scl_mode(&self) -> Win1CbrHorSclModeR {
        Win1CbrHorSclModeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn win1_cbr_ver_scl_mode(&self) -> Win1CbrVerSclModeR {
        Win1CbrVerSclModeR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - win1 horizontal scaler down mode select"]
    #[inline(always)]
    pub fn win1_cbr_hsd_mode(&self) -> Win1CbrHsdModeR {
        Win1CbrHsdModeR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - win1 vertical scaler up mode select"]
    #[inline(always)]
    pub fn win1_cbr_vsu_mode(&self) -> Win1CbrVsuModeR {
        Win1CbrVsuModeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - win1 vertical scaler down mode select"]
    #[inline(always)]
    pub fn win1_cbr_vsd_mode(&self) -> Win1CbrVsdModeR {
        Win1CbrVsdModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - win1 yrgb axi bus gather enable"]
    #[inline(always)]
    #[must_use]
    pub fn win1_yrgb_axi_gather_en(&mut self) -> Win1YrgbAxiGatherEnW<Win1Ctrl1Spec> {
        Win1YrgbAxiGatherEnW::new(self, 0)
    }
    #[doc = "Bit 1 - win1 cbr axi bus gather enable"]
    #[inline(always)]
    #[must_use]
    pub fn win1_cbr_axi_gather_en(&mut self) -> Win1CbrAxiGatherEnW<Win1Ctrl1Spec> {
        Win1CbrAxiGatherEnW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn win1_bic_coe_sel(&mut self) -> Win1BicCoeSelW<Win1Ctrl1Spec> {
        Win1BicCoeSelW::new(self, 2)
    }
    #[doc = "Bit 4 - yrgb_src/yrgb_dst >= 4"]
    #[inline(always)]
    #[must_use]
    pub fn win1_vsd_yrgb_gt4(&mut self) -> Win1VsdYrgbGt4W<Win1Ctrl1Spec> {
        Win1VsdYrgbGt4W::new(self, 4)
    }
    #[doc = "Bit 5 - yrgb_src/yrgb_dst >= 2"]
    #[inline(always)]
    #[must_use]
    pub fn win1_vsd_yrgb_gt2(&mut self) -> Win1VsdYrgbGt2W<Win1Ctrl1Spec> {
        Win1VsdYrgbGt2W::new(self, 5)
    }
    #[doc = "Bit 6 - cbr_src/cbr_dst >= 4"]
    #[inline(always)]
    #[must_use]
    pub fn win1_vsd_cbr_gt4(&mut self) -> Win1VsdCbrGt4W<Win1Ctrl1Spec> {
        Win1VsdCbrGt4W::new(self, 6)
    }
    #[doc = "Bit 7 - cbr_src/cbr_dst >= 2"]
    #[inline(always)]
    #[must_use]
    pub fn win1_vsd_cbr_gt2(&mut self) -> Win1VsdCbrGt2W<Win1Ctrl1Spec> {
        Win1VsdCbrGt2W::new(self, 7)
    }
    #[doc = "Bits 8:11 - win1 axi yrgb data transfer gather number"]
    #[inline(always)]
    #[must_use]
    pub fn win1_yrgb_axi_gather_num(&mut self) -> Win1YrgbAxiGatherNumW<Win1Ctrl1Spec> {
        Win1YrgbAxiGatherNumW::new(self, 8)
    }
    #[doc = "Bits 12:14 - win1 axi cbr data transfer gather number"]
    #[inline(always)]
    #[must_use]
    pub fn win1_cbr_axi_gather_num(&mut self) -> Win1CbrAxiGatherNumW<Win1Ctrl1Spec> {
        Win1CbrAxiGatherNumW::new(self, 12)
    }
    #[doc = "Bit 15 - when yuv fmt,"]
    #[inline(always)]
    #[must_use]
    pub fn win1_line_load_mode(&mut self) -> Win1LineLoadModeW<Win1Ctrl1Spec> {
        Win1LineLoadModeW::new(self, 15)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn win1_yrgb_hor_scl_mode(&mut self) -> Win1YrgbHorSclModeW<Win1Ctrl1Spec> {
        Win1YrgbHorSclModeW::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn win1_yrgb_ver_scl_mode(&mut self) -> Win1YrgbVerSclModeW<Win1Ctrl1Spec> {
        Win1YrgbVerSclModeW::new(self, 18)
    }
    #[doc = "Bits 20:21 - win1 horizontal scaler down mode select"]
    #[inline(always)]
    #[must_use]
    pub fn win1_yrgb_hsd_mode(&mut self) -> Win1YrgbHsdModeW<Win1Ctrl1Spec> {
        Win1YrgbHsdModeW::new(self, 20)
    }
    #[doc = "Bit 22 - win1 vertical scaler up mode select"]
    #[inline(always)]
    #[must_use]
    pub fn win1_yrgb_vsu_mode(&mut self) -> Win1YrgbVsuModeW<Win1Ctrl1Spec> {
        Win1YrgbVsuModeW::new(self, 22)
    }
    #[doc = "Bit 23 - win1 vertical scaler down mode select"]
    #[inline(always)]
    #[must_use]
    pub fn win1_yrgb_vsd_mode(&mut self) -> Win1YrgbVsdModeW<Win1Ctrl1Spec> {
        Win1YrgbVsdModeW::new(self, 23)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn win1_cbr_hor_scl_mode(&mut self) -> Win1CbrHorSclModeW<Win1Ctrl1Spec> {
        Win1CbrHorSclModeW::new(self, 24)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    #[must_use]
    pub fn win1_cbr_ver_scl_mode(&mut self) -> Win1CbrVerSclModeW<Win1Ctrl1Spec> {
        Win1CbrVerSclModeW::new(self, 26)
    }
    #[doc = "Bits 28:29 - win1 horizontal scaler down mode select"]
    #[inline(always)]
    #[must_use]
    pub fn win1_cbr_hsd_mode(&mut self) -> Win1CbrHsdModeW<Win1Ctrl1Spec> {
        Win1CbrHsdModeW::new(self, 28)
    }
    #[doc = "Bit 30 - win1 vertical scaler up mode select"]
    #[inline(always)]
    #[must_use]
    pub fn win1_cbr_vsu_mode(&mut self) -> Win1CbrVsuModeW<Win1Ctrl1Spec> {
        Win1CbrVsuModeW::new(self, 30)
    }
    #[doc = "Bit 31 - win1 vertical scaler down mode select"]
    #[inline(always)]
    #[must_use]
    pub fn win1_cbr_vsd_mode(&mut self) -> Win1CbrVsdModeW<Win1Ctrl1Spec> {
        Win1CbrVsdModeW::new(self, 31)
    }
}
#[doc = "Win1 ctrl register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win1Ctrl1Spec;
impl crate::RegisterSpec for Win1Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win1_ctrl1::R`](R) reader structure"]
impl crate::Readable for Win1Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`win1_ctrl1::W`](W) writer structure"]
impl crate::Writable for Win1Ctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN1_CTRL1 to value 0"]
impl crate::Resettable for Win1Ctrl1Spec {
    const RESET_VALUE: u32 = 0;
}
