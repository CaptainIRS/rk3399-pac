#[doc = "Register `WIN3_CTRL0` reader"]
pub type R = crate::R<Win3Ctrl0Spec>;
#[doc = "Register `WIN3_CTRL0` writer"]
pub type W = crate::W<Win3Ctrl0Spec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3En {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win3En> for bool {
    #[inline(always)]
    fn from(variant: Win3En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_EN` reader - "]
pub type Win3EnR = crate::BitReader<Win3En>;
impl Win3EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3En {
        match self.bits {
            false => Win3En::B0,
            true => Win3En::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3En::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3En::B1
    }
}
#[doc = "Field `WIN3_EN` writer - "]
pub type Win3EnW<'a, REG> = crate::BitWriter<'a, REG, Win3En>;
impl<'a, REG> Win3EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3En::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3En::B1)
    }
}
#[doc = "Win3 interlace read mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3InterlaceRead {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win3InterlaceRead> for bool {
    #[inline(always)]
    fn from(variant: Win3InterlaceRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_INTERLACE_READ` reader - Win3 interlace read mode"]
pub type Win3InterlaceReadR = crate::BitReader<Win3InterlaceRead>;
impl Win3InterlaceReadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3InterlaceRead {
        match self.bits {
            false => Win3InterlaceRead::B0,
            true => Win3InterlaceRead::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3InterlaceRead::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3InterlaceRead::B1
    }
}
#[doc = "Field `WIN3_INTERLACE_READ` writer - Win3 interlace read mode"]
pub type Win3InterlaceReadW<'a, REG> = crate::BitWriter<'a, REG, Win3InterlaceRead>;
impl<'a, REG> Win3InterlaceReadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3InterlaceRead::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3InterlaceRead::B1)
    }
}
#[doc = "Win3 RGB2YUV conversion mode\n\nColor space conversion:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win3CscMode {
    #[doc = "0: BT601_L"]
    B00 = 0,
    #[doc = "1: BT709_L"]
    B01 = 1,
    #[doc = "2: BT601_F"]
    B10 = 2,
    #[doc = "3: BT2020"]
    B11 = 3,
}
impl From<Win3CscMode> for u8 {
    #[inline(always)]
    fn from(variant: Win3CscMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win3CscMode {
    type Ux = u8;
}
#[doc = "Field `WIN3_CSC_MODE` reader - Win3 RGB2YUV conversion mode\n\nColor space conversion:"]
pub type Win3CscModeR = crate::FieldReader<Win3CscMode>;
impl Win3CscModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3CscMode {
        match self.bits {
            0 => Win3CscMode::B00,
            1 => Win3CscMode::B01,
            2 => Win3CscMode::B10,
            3 => Win3CscMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "BT601_L"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win3CscMode::B00
    }
    #[doc = "BT709_L"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win3CscMode::B01
    }
    #[doc = "BT601_F"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win3CscMode::B10
    }
    #[doc = "BT2020"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win3CscMode::B11
    }
}
#[doc = "Field `WIN3_CSC_MODE` writer - Win3 RGB2YUV conversion mode\n\nColor space conversion:"]
pub type Win3CscModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win3CscMode>;
impl<'a, REG> Win3CscModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "BT601_L"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win3CscMode::B00)
    }
    #[doc = "BT709_L"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win3CscMode::B01)
    }
    #[doc = "BT601_F"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win3CscMode::B10)
    }
    #[doc = "BT2020"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win3CscMode::B11)
    }
}
#[doc = "Win3 master0 enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3Mst0En {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win3Mst0En> for bool {
    #[inline(always)]
    fn from(variant: Win3Mst0En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_MST0_EN` reader - Win3 master0 enable"]
pub type Win3Mst0EnR = crate::BitReader<Win3Mst0En>;
impl Win3Mst0EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3Mst0En {
        match self.bits {
            false => Win3Mst0En::B0,
            true => Win3Mst0En::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3Mst0En::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3Mst0En::B1
    }
}
#[doc = "Field `WIN3_MST0_EN` writer - Win3 master0 enable"]
pub type Win3Mst0EnW<'a, REG> = crate::BitWriter<'a, REG, Win3Mst0En>;
impl<'a, REG> Win3Mst0EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3Mst0En::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3Mst0En::B1)
    }
}
#[doc = "Win3 region 0 data format\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win3DataFmt0 {
    #[doc = "0: ARGB888"]
    B00 = 0,
    #[doc = "1: RGB888"]
    B01 = 1,
    #[doc = "2: RGB565"]
    B10 = 2,
    #[doc = "3: 8bpp"]
    B11 = 3,
}
impl From<Win3DataFmt0> for u8 {
    #[inline(always)]
    fn from(variant: Win3DataFmt0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win3DataFmt0 {
    type Ux = u8;
}
#[doc = "Field `WIN3_DATA_FMT0` reader - Win3 region 0 data format"]
pub type Win3DataFmt0R = crate::FieldReader<Win3DataFmt0>;
impl Win3DataFmt0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3DataFmt0 {
        match self.bits {
            0 => Win3DataFmt0::B00,
            1 => Win3DataFmt0::B01,
            2 => Win3DataFmt0::B10,
            3 => Win3DataFmt0::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "ARGB888"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win3DataFmt0::B00
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win3DataFmt0::B01
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win3DataFmt0::B10
    }
    #[doc = "8bpp"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win3DataFmt0::B11
    }
}
#[doc = "Field `WIN3_DATA_FMT0` writer - Win3 region 0 data format"]
pub type Win3DataFmt0W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win3DataFmt0>;
impl<'a, REG> Win3DataFmt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ARGB888"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win3DataFmt0::B00)
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win3DataFmt0::B01)
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win3DataFmt0::B10)
    }
    #[doc = "8bpp"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win3DataFmt0::B11)
    }
}
#[doc = "win3 master1 enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3Mst1En {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win3Mst1En> for bool {
    #[inline(always)]
    fn from(variant: Win3Mst1En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_MST1_EN` reader - win3 master1 enable"]
pub type Win3Mst1EnR = crate::BitReader<Win3Mst1En>;
impl Win3Mst1EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3Mst1En {
        match self.bits {
            false => Win3Mst1En::B0,
            true => Win3Mst1En::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3Mst1En::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3Mst1En::B1
    }
}
#[doc = "Field `WIN3_MST1_EN` writer - win3 master1 enable"]
pub type Win3Mst1EnW<'a, REG> = crate::BitWriter<'a, REG, Win3Mst1En>;
impl<'a, REG> Win3Mst1EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3Mst1En::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3Mst1En::B1)
    }
}
#[doc = "Win3 region 1 data format\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win3DataFmt1 {
    #[doc = "0: ARGB888"]
    B00 = 0,
    #[doc = "1: RGB888"]
    B01 = 1,
    #[doc = "2: RGB565"]
    B10 = 2,
    #[doc = "3: 8bpp"]
    B11 = 3,
}
impl From<Win3DataFmt1> for u8 {
    #[inline(always)]
    fn from(variant: Win3DataFmt1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win3DataFmt1 {
    type Ux = u8;
}
#[doc = "Field `WIN3_DATA_FMT1` reader - Win3 region 1 data format"]
pub type Win3DataFmt1R = crate::FieldReader<Win3DataFmt1>;
impl Win3DataFmt1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3DataFmt1 {
        match self.bits {
            0 => Win3DataFmt1::B00,
            1 => Win3DataFmt1::B01,
            2 => Win3DataFmt1::B10,
            3 => Win3DataFmt1::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "ARGB888"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win3DataFmt1::B00
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win3DataFmt1::B01
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win3DataFmt1::B10
    }
    #[doc = "8bpp"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win3DataFmt1::B11
    }
}
#[doc = "Field `WIN3_DATA_FMT1` writer - Win3 region 1 data format"]
pub type Win3DataFmt1W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win3DataFmt1>;
impl<'a, REG> Win3DataFmt1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ARGB888"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win3DataFmt1::B00)
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win3DataFmt1::B01)
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win3DataFmt1::B10)
    }
    #[doc = "8bpp"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win3DataFmt1::B11)
    }
}
#[doc = "win3 master2 enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3Mst2En {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win3Mst2En> for bool {
    #[inline(always)]
    fn from(variant: Win3Mst2En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_MST2_EN` reader - win3 master2 enable"]
pub type Win3Mst2EnR = crate::BitReader<Win3Mst2En>;
impl Win3Mst2EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3Mst2En {
        match self.bits {
            false => Win3Mst2En::B0,
            true => Win3Mst2En::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3Mst2En::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3Mst2En::B1
    }
}
#[doc = "Field `WIN3_MST2_EN` writer - win3 master2 enable"]
pub type Win3Mst2EnW<'a, REG> = crate::BitWriter<'a, REG, Win3Mst2En>;
impl<'a, REG> Win3Mst2EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3Mst2En::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3Mst2En::B1)
    }
}
#[doc = "Win3 region 2 data format\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win3DataFmt2 {
    #[doc = "0: ARGB888"]
    B00 = 0,
    #[doc = "1: RGB888"]
    B01 = 1,
    #[doc = "2: RGB565"]
    B10 = 2,
    #[doc = "3: 8bpp"]
    B11 = 3,
}
impl From<Win3DataFmt2> for u8 {
    #[inline(always)]
    fn from(variant: Win3DataFmt2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win3DataFmt2 {
    type Ux = u8;
}
#[doc = "Field `WIN3_DATA_FMT2` reader - Win3 region 2 data format"]
pub type Win3DataFmt2R = crate::FieldReader<Win3DataFmt2>;
impl Win3DataFmt2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3DataFmt2 {
        match self.bits {
            0 => Win3DataFmt2::B00,
            1 => Win3DataFmt2::B01,
            2 => Win3DataFmt2::B10,
            3 => Win3DataFmt2::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "ARGB888"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win3DataFmt2::B00
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win3DataFmt2::B01
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win3DataFmt2::B10
    }
    #[doc = "8bpp"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win3DataFmt2::B11
    }
}
#[doc = "Field `WIN3_DATA_FMT2` writer - Win3 region 2 data format"]
pub type Win3DataFmt2W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win3DataFmt2>;
impl<'a, REG> Win3DataFmt2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ARGB888"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win3DataFmt2::B00)
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win3DataFmt2::B01)
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win3DataFmt2::B10)
    }
    #[doc = "8bpp"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win3DataFmt2::B11)
    }
}
#[doc = "win3 master3 enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3Mst3En {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win3Mst3En> for bool {
    #[inline(always)]
    fn from(variant: Win3Mst3En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_MST3_EN` reader - win3 master3 enable"]
pub type Win3Mst3EnR = crate::BitReader<Win3Mst3En>;
impl Win3Mst3EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3Mst3En {
        match self.bits {
            false => Win3Mst3En::B0,
            true => Win3Mst3En::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3Mst3En::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3Mst3En::B1
    }
}
#[doc = "Field `WIN3_MST3_EN` writer - win3 master3 enable"]
pub type Win3Mst3EnW<'a, REG> = crate::BitWriter<'a, REG, Win3Mst3En>;
impl<'a, REG> Win3Mst3EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3Mst3En::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3Mst3En::B1)
    }
}
#[doc = "Win3 region 3 data format\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win3DataFmt3 {
    #[doc = "0: ARGB888"]
    B00 = 0,
    #[doc = "1: RGB888"]
    B01 = 1,
    #[doc = "2: RGB565"]
    B10 = 2,
    #[doc = "3: 8bpp"]
    B11 = 3,
}
impl From<Win3DataFmt3> for u8 {
    #[inline(always)]
    fn from(variant: Win3DataFmt3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win3DataFmt3 {
    type Ux = u8;
}
#[doc = "Field `WIN3_DATA_FMT3` reader - Win3 region 3 data format"]
pub type Win3DataFmt3R = crate::FieldReader<Win3DataFmt3>;
impl Win3DataFmt3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3DataFmt3 {
        match self.bits {
            0 => Win3DataFmt3::B00,
            1 => Win3DataFmt3::B01,
            2 => Win3DataFmt3::B10,
            3 => Win3DataFmt3::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "ARGB888"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win3DataFmt3::B00
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win3DataFmt3::B01
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win3DataFmt3::B10
    }
    #[doc = "8bpp"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win3DataFmt3::B11
    }
}
#[doc = "Field `WIN3_DATA_FMT3` writer - Win3 region 3 data format"]
pub type Win3DataFmt3W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win3DataFmt3>;
impl<'a, REG> Win3DataFmt3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ARGB888"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win3DataFmt3::B00)
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win3DataFmt3::B01)
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win3DataFmt3::B10)
    }
    #[doc = "8bpp"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win3DataFmt3::B11)
    }
}
#[doc = "Win3 region0 RGB Red and Blue swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3RbSwap0 {
    #[doc = "0: RGB"]
    B0 = 0,
    #[doc = "1: BGR"]
    B1 = 1,
}
impl From<Win3RbSwap0> for bool {
    #[inline(always)]
    fn from(variant: Win3RbSwap0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_RB_SWAP0` reader - Win3 region0 RGB Red and Blue swap"]
pub type Win3RbSwap0R = crate::BitReader<Win3RbSwap0>;
impl Win3RbSwap0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3RbSwap0 {
        match self.bits {
            false => Win3RbSwap0::B0,
            true => Win3RbSwap0::B1,
        }
    }
    #[doc = "RGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3RbSwap0::B0
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3RbSwap0::B1
    }
}
#[doc = "Field `WIN3_RB_SWAP0` writer - Win3 region0 RGB Red and Blue swap"]
pub type Win3RbSwap0W<'a, REG> = crate::BitWriter<'a, REG, Win3RbSwap0>;
impl<'a, REG> Win3RbSwap0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3RbSwap0::B0)
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3RbSwap0::B1)
    }
}
#[doc = "Win3 region0 RGB alpha swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3AlphaSwap0 {
    #[doc = "0: ARGB"]
    B0 = 0,
    #[doc = "1: RGBA"]
    B1 = 1,
}
impl From<Win3AlphaSwap0> for bool {
    #[inline(always)]
    fn from(variant: Win3AlphaSwap0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_ALPHA_SWAP0` reader - Win3 region0 RGB alpha swap"]
pub type Win3AlphaSwap0R = crate::BitReader<Win3AlphaSwap0>;
impl Win3AlphaSwap0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3AlphaSwap0 {
        match self.bits {
            false => Win3AlphaSwap0::B0,
            true => Win3AlphaSwap0::B1,
        }
    }
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3AlphaSwap0::B0
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3AlphaSwap0::B1
    }
}
#[doc = "Field `WIN3_ALPHA_SWAP0` writer - Win3 region0 RGB alpha swap"]
pub type Win3AlphaSwap0W<'a, REG> = crate::BitWriter<'a, REG, Win3AlphaSwap0>;
impl<'a, REG> Win3AlphaSwap0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3AlphaSwap0::B0)
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3AlphaSwap0::B1)
    }
}
#[doc = "Win3 region0 8pp palette data Big-endian/ Little-endian select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3EndianSwap0 {
    #[doc = "0: Big-endian"]
    B0 = 0,
    #[doc = "1: Little-endian"]
    B1 = 1,
}
impl From<Win3EndianSwap0> for bool {
    #[inline(always)]
    fn from(variant: Win3EndianSwap0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_ENDIAN_SWAP0` reader - Win3 region0 8pp palette data Big-endian/ Little-endian select"]
pub type Win3EndianSwap0R = crate::BitReader<Win3EndianSwap0>;
impl Win3EndianSwap0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3EndianSwap0 {
        match self.bits {
            false => Win3EndianSwap0::B0,
            true => Win3EndianSwap0::B1,
        }
    }
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3EndianSwap0::B0
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3EndianSwap0::B1
    }
}
#[doc = "Field `WIN3_ENDIAN_SWAP0` writer - Win3 region0 8pp palette data Big-endian/ Little-endian select"]
pub type Win3EndianSwap0W<'a, REG> = crate::BitWriter<'a, REG, Win3EndianSwap0>;
impl<'a, REG> Win3EndianSwap0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3EndianSwap0::B0)
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3EndianSwap0::B1)
    }
}
#[doc = "Win3 region1 RGB Red and Blue swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3RbSwap1 {
    #[doc = "0: RGB"]
    B0 = 0,
    #[doc = "1: BGR"]
    B1 = 1,
}
impl From<Win3RbSwap1> for bool {
    #[inline(always)]
    fn from(variant: Win3RbSwap1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_RB_SWAP1` reader - Win3 region1 RGB Red and Blue swap"]
pub type Win3RbSwap1R = crate::BitReader<Win3RbSwap1>;
impl Win3RbSwap1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3RbSwap1 {
        match self.bits {
            false => Win3RbSwap1::B0,
            true => Win3RbSwap1::B1,
        }
    }
    #[doc = "RGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3RbSwap1::B0
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3RbSwap1::B1
    }
}
#[doc = "Field `WIN3_RB_SWAP1` writer - Win3 region1 RGB Red and Blue swap"]
pub type Win3RbSwap1W<'a, REG> = crate::BitWriter<'a, REG, Win3RbSwap1>;
impl<'a, REG> Win3RbSwap1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3RbSwap1::B0)
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3RbSwap1::B1)
    }
}
#[doc = "Win3 region1 RGB alpha swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3AlphaSwap1 {
    #[doc = "0: ARGB"]
    B0 = 0,
    #[doc = "1: RGBA"]
    B1 = 1,
}
impl From<Win3AlphaSwap1> for bool {
    #[inline(always)]
    fn from(variant: Win3AlphaSwap1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_ALPHA_SWAP1` reader - Win3 region1 RGB alpha swap"]
pub type Win3AlphaSwap1R = crate::BitReader<Win3AlphaSwap1>;
impl Win3AlphaSwap1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3AlphaSwap1 {
        match self.bits {
            false => Win3AlphaSwap1::B0,
            true => Win3AlphaSwap1::B1,
        }
    }
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3AlphaSwap1::B0
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3AlphaSwap1::B1
    }
}
#[doc = "Field `WIN3_ALPHA_SWAP1` writer - Win3 region1 RGB alpha swap"]
pub type Win3AlphaSwap1W<'a, REG> = crate::BitWriter<'a, REG, Win3AlphaSwap1>;
impl<'a, REG> Win3AlphaSwap1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3AlphaSwap1::B0)
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3AlphaSwap1::B1)
    }
}
#[doc = "Win3 region1 8pp palette data Big-endian/ Little-endian select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3EndianSwap1 {
    #[doc = "0: Big-endian"]
    B0 = 0,
    #[doc = "1: Little-endian"]
    B1 = 1,
}
impl From<Win3EndianSwap1> for bool {
    #[inline(always)]
    fn from(variant: Win3EndianSwap1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_ENDIAN_SWAP1` reader - Win3 region1 8pp palette data Big-endian/ Little-endian select"]
pub type Win3EndianSwap1R = crate::BitReader<Win3EndianSwap1>;
impl Win3EndianSwap1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3EndianSwap1 {
        match self.bits {
            false => Win3EndianSwap1::B0,
            true => Win3EndianSwap1::B1,
        }
    }
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3EndianSwap1::B0
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3EndianSwap1::B1
    }
}
#[doc = "Field `WIN3_ENDIAN_SWAP1` writer - Win3 region1 8pp palette data Big-endian/ Little-endian select"]
pub type Win3EndianSwap1W<'a, REG> = crate::BitWriter<'a, REG, Win3EndianSwap1>;
impl<'a, REG> Win3EndianSwap1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3EndianSwap1::B0)
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3EndianSwap1::B1)
    }
}
#[doc = "Win3 region2 RGB Red and Blue swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3RbSwap2 {
    #[doc = "0: RGB"]
    B0 = 0,
    #[doc = "1: BGR"]
    B1 = 1,
}
impl From<Win3RbSwap2> for bool {
    #[inline(always)]
    fn from(variant: Win3RbSwap2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_RB_SWAP2` reader - Win3 region2 RGB Red and Blue swap"]
pub type Win3RbSwap2R = crate::BitReader<Win3RbSwap2>;
impl Win3RbSwap2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3RbSwap2 {
        match self.bits {
            false => Win3RbSwap2::B0,
            true => Win3RbSwap2::B1,
        }
    }
    #[doc = "RGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3RbSwap2::B0
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3RbSwap2::B1
    }
}
#[doc = "Field `WIN3_RB_SWAP2` writer - Win3 region2 RGB Red and Blue swap"]
pub type Win3RbSwap2W<'a, REG> = crate::BitWriter<'a, REG, Win3RbSwap2>;
impl<'a, REG> Win3RbSwap2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3RbSwap2::B0)
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3RbSwap2::B1)
    }
}
#[doc = "Win3 region2 RGB alpha swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3AlphaSwap2 {
    #[doc = "0: ARGB"]
    B0 = 0,
    #[doc = "1: RGBA"]
    B1 = 1,
}
impl From<Win3AlphaSwap2> for bool {
    #[inline(always)]
    fn from(variant: Win3AlphaSwap2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_ALPHA_SWAP2` reader - Win3 region2 RGB alpha swap"]
pub type Win3AlphaSwap2R = crate::BitReader<Win3AlphaSwap2>;
impl Win3AlphaSwap2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3AlphaSwap2 {
        match self.bits {
            false => Win3AlphaSwap2::B0,
            true => Win3AlphaSwap2::B1,
        }
    }
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3AlphaSwap2::B0
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3AlphaSwap2::B1
    }
}
#[doc = "Field `WIN3_ALPHA_SWAP2` writer - Win3 region2 RGB alpha swap"]
pub type Win3AlphaSwap2W<'a, REG> = crate::BitWriter<'a, REG, Win3AlphaSwap2>;
impl<'a, REG> Win3AlphaSwap2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3AlphaSwap2::B0)
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3AlphaSwap2::B1)
    }
}
#[doc = "Win3 region2 8pp palette data Big-endian/ Little-endian select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3EndianSwap2 {
    #[doc = "0: Big-endian"]
    B0 = 0,
    #[doc = "1: Little-endian"]
    B1 = 1,
}
impl From<Win3EndianSwap2> for bool {
    #[inline(always)]
    fn from(variant: Win3EndianSwap2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_ENDIAN_SWAP2` reader - Win3 region2 8pp palette data Big-endian/ Little-endian select"]
pub type Win3EndianSwap2R = crate::BitReader<Win3EndianSwap2>;
impl Win3EndianSwap2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3EndianSwap2 {
        match self.bits {
            false => Win3EndianSwap2::B0,
            true => Win3EndianSwap2::B1,
        }
    }
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3EndianSwap2::B0
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3EndianSwap2::B1
    }
}
#[doc = "Field `WIN3_ENDIAN_SWAP2` writer - Win3 region2 8pp palette data Big-endian/ Little-endian select"]
pub type Win3EndianSwap2W<'a, REG> = crate::BitWriter<'a, REG, Win3EndianSwap2>;
impl<'a, REG> Win3EndianSwap2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3EndianSwap2::B0)
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3EndianSwap2::B1)
    }
}
#[doc = "Win3 region3 RGB Red and Blue swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3RbSwap3 {
    #[doc = "0: RGB"]
    B0 = 0,
    #[doc = "1: BGR"]
    B1 = 1,
}
impl From<Win3RbSwap3> for bool {
    #[inline(always)]
    fn from(variant: Win3RbSwap3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_RB_SWAP3` reader - Win3 region3 RGB Red and Blue swap"]
pub type Win3RbSwap3R = crate::BitReader<Win3RbSwap3>;
impl Win3RbSwap3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3RbSwap3 {
        match self.bits {
            false => Win3RbSwap3::B0,
            true => Win3RbSwap3::B1,
        }
    }
    #[doc = "RGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3RbSwap3::B0
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3RbSwap3::B1
    }
}
#[doc = "Field `WIN3_RB_SWAP3` writer - Win3 region3 RGB Red and Blue swap"]
pub type Win3RbSwap3W<'a, REG> = crate::BitWriter<'a, REG, Win3RbSwap3>;
impl<'a, REG> Win3RbSwap3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3RbSwap3::B0)
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3RbSwap3::B1)
    }
}
#[doc = "Win3 region3 RGB alpha swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3AlphaSwap3 {
    #[doc = "0: ARGB"]
    B0 = 0,
    #[doc = "1: RGBA"]
    B1 = 1,
}
impl From<Win3AlphaSwap3> for bool {
    #[inline(always)]
    fn from(variant: Win3AlphaSwap3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_ALPHA_SWAP3` reader - Win3 region3 RGB alpha swap"]
pub type Win3AlphaSwap3R = crate::BitReader<Win3AlphaSwap3>;
impl Win3AlphaSwap3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3AlphaSwap3 {
        match self.bits {
            false => Win3AlphaSwap3::B0,
            true => Win3AlphaSwap3::B1,
        }
    }
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3AlphaSwap3::B0
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3AlphaSwap3::B1
    }
}
#[doc = "Field `WIN3_ALPHA_SWAP3` writer - Win3 region3 RGB alpha swap"]
pub type Win3AlphaSwap3W<'a, REG> = crate::BitWriter<'a, REG, Win3AlphaSwap3>;
impl<'a, REG> Win3AlphaSwap3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3AlphaSwap3::B0)
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3AlphaSwap3::B1)
    }
}
#[doc = "Win3 region3 8pp palette data Big-endian/ Little-endian select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win3EndianSwap3 {
    #[doc = "0: Big-endian"]
    B0 = 0,
    #[doc = "1: Little-endian"]
    B1 = 1,
}
impl From<Win3EndianSwap3> for bool {
    #[inline(always)]
    fn from(variant: Win3EndianSwap3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN3_ENDIAN_SWAP3` reader - Win3 region3 8pp palette data Big-endian/ Little-endian select"]
pub type Win3EndianSwap3R = crate::BitReader<Win3EndianSwap3>;
impl Win3EndianSwap3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win3EndianSwap3 {
        match self.bits {
            false => Win3EndianSwap3::B0,
            true => Win3EndianSwap3::B1,
        }
    }
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win3EndianSwap3::B0
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win3EndianSwap3::B1
    }
}
#[doc = "Field `WIN3_ENDIAN_SWAP3` writer - Win3 region3 8pp palette data Big-endian/ Little-endian select"]
pub type Win3EndianSwap3W<'a, REG> = crate::BitWriter<'a, REG, Win3EndianSwap3>;
impl<'a, REG> Win3EndianSwap3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win3EndianSwap3::B0)
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win3EndianSwap3::B1)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn win3_en(&self) -> Win3EnR {
        Win3EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Win3 interlace read mode"]
    #[inline(always)]
    pub fn win3_interlace_read(&self) -> Win3InterlaceReadR {
        Win3InterlaceReadR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Win3 RGB2YUV conversion mode\n\nColor space conversion:"]
    #[inline(always)]
    pub fn win3_csc_mode(&self) -> Win3CscModeR {
        Win3CscModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Win3 master0 enable"]
    #[inline(always)]
    pub fn win3_mst0_en(&self) -> Win3Mst0EnR {
        Win3Mst0EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Win3 region 0 data format"]
    #[inline(always)]
    pub fn win3_data_fmt0(&self) -> Win3DataFmt0R {
        Win3DataFmt0R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - win3 master1 enable"]
    #[inline(always)]
    pub fn win3_mst1_en(&self) -> Win3Mst1EnR {
        Win3Mst1EnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Win3 region 1 data format"]
    #[inline(always)]
    pub fn win3_data_fmt1(&self) -> Win3DataFmt1R {
        Win3DataFmt1R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 12 - win3 master2 enable"]
    #[inline(always)]
    pub fn win3_mst2_en(&self) -> Win3Mst2EnR {
        Win3Mst2EnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Win3 region 2 data format"]
    #[inline(always)]
    pub fn win3_data_fmt2(&self) -> Win3DataFmt2R {
        Win3DataFmt2R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 16 - win3 master3 enable"]
    #[inline(always)]
    pub fn win3_mst3_en(&self) -> Win3Mst3EnR {
        Win3Mst3EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Win3 region 3 data format"]
    #[inline(always)]
    pub fn win3_data_fmt3(&self) -> Win3DataFmt3R {
        Win3DataFmt3R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 20 - Win3 region0 RGB Red and Blue swap"]
    #[inline(always)]
    pub fn win3_rb_swap0(&self) -> Win3RbSwap0R {
        Win3RbSwap0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Win3 region0 RGB alpha swap"]
    #[inline(always)]
    pub fn win3_alpha_swap0(&self) -> Win3AlphaSwap0R {
        Win3AlphaSwap0R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Win3 region0 8pp palette data Big-endian/ Little-endian select"]
    #[inline(always)]
    pub fn win3_endian_swap0(&self) -> Win3EndianSwap0R {
        Win3EndianSwap0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Win3 region1 RGB Red and Blue swap"]
    #[inline(always)]
    pub fn win3_rb_swap1(&self) -> Win3RbSwap1R {
        Win3RbSwap1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Win3 region1 RGB alpha swap"]
    #[inline(always)]
    pub fn win3_alpha_swap1(&self) -> Win3AlphaSwap1R {
        Win3AlphaSwap1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Win3 region1 8pp palette data Big-endian/ Little-endian select"]
    #[inline(always)]
    pub fn win3_endian_swap1(&self) -> Win3EndianSwap1R {
        Win3EndianSwap1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Win3 region2 RGB Red and Blue swap"]
    #[inline(always)]
    pub fn win3_rb_swap2(&self) -> Win3RbSwap2R {
        Win3RbSwap2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Win3 region2 RGB alpha swap"]
    #[inline(always)]
    pub fn win3_alpha_swap2(&self) -> Win3AlphaSwap2R {
        Win3AlphaSwap2R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Win3 region2 8pp palette data Big-endian/ Little-endian select"]
    #[inline(always)]
    pub fn win3_endian_swap2(&self) -> Win3EndianSwap2R {
        Win3EndianSwap2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Win3 region3 RGB Red and Blue swap"]
    #[inline(always)]
    pub fn win3_rb_swap3(&self) -> Win3RbSwap3R {
        Win3RbSwap3R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Win3 region3 RGB alpha swap"]
    #[inline(always)]
    pub fn win3_alpha_swap3(&self) -> Win3AlphaSwap3R {
        Win3AlphaSwap3R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Win3 region3 8pp palette data Big-endian/ Little-endian select"]
    #[inline(always)]
    pub fn win3_endian_swap3(&self) -> Win3EndianSwap3R {
        Win3EndianSwap3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn win3_en(&mut self) -> Win3EnW<Win3Ctrl0Spec> {
        Win3EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Win3 interlace read mode"]
    #[inline(always)]
    #[must_use]
    pub fn win3_interlace_read(&mut self) -> Win3InterlaceReadW<Win3Ctrl0Spec> {
        Win3InterlaceReadW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Win3 RGB2YUV conversion mode\n\nColor space conversion:"]
    #[inline(always)]
    #[must_use]
    pub fn win3_csc_mode(&mut self) -> Win3CscModeW<Win3Ctrl0Spec> {
        Win3CscModeW::new(self, 2)
    }
    #[doc = "Bit 4 - Win3 master0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn win3_mst0_en(&mut self) -> Win3Mst0EnW<Win3Ctrl0Spec> {
        Win3Mst0EnW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Win3 region 0 data format"]
    #[inline(always)]
    #[must_use]
    pub fn win3_data_fmt0(&mut self) -> Win3DataFmt0W<Win3Ctrl0Spec> {
        Win3DataFmt0W::new(self, 5)
    }
    #[doc = "Bit 8 - win3 master1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn win3_mst1_en(&mut self) -> Win3Mst1EnW<Win3Ctrl0Spec> {
        Win3Mst1EnW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Win3 region 1 data format"]
    #[inline(always)]
    #[must_use]
    pub fn win3_data_fmt1(&mut self) -> Win3DataFmt1W<Win3Ctrl0Spec> {
        Win3DataFmt1W::new(self, 9)
    }
    #[doc = "Bit 12 - win3 master2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn win3_mst2_en(&mut self) -> Win3Mst2EnW<Win3Ctrl0Spec> {
        Win3Mst2EnW::new(self, 12)
    }
    #[doc = "Bits 13:14 - Win3 region 2 data format"]
    #[inline(always)]
    #[must_use]
    pub fn win3_data_fmt2(&mut self) -> Win3DataFmt2W<Win3Ctrl0Spec> {
        Win3DataFmt2W::new(self, 13)
    }
    #[doc = "Bit 16 - win3 master3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn win3_mst3_en(&mut self) -> Win3Mst3EnW<Win3Ctrl0Spec> {
        Win3Mst3EnW::new(self, 16)
    }
    #[doc = "Bits 17:18 - Win3 region 3 data format"]
    #[inline(always)]
    #[must_use]
    pub fn win3_data_fmt3(&mut self) -> Win3DataFmt3W<Win3Ctrl0Spec> {
        Win3DataFmt3W::new(self, 17)
    }
    #[doc = "Bit 20 - Win3 region0 RGB Red and Blue swap"]
    #[inline(always)]
    #[must_use]
    pub fn win3_rb_swap0(&mut self) -> Win3RbSwap0W<Win3Ctrl0Spec> {
        Win3RbSwap0W::new(self, 20)
    }
    #[doc = "Bit 21 - Win3 region0 RGB alpha swap"]
    #[inline(always)]
    #[must_use]
    pub fn win3_alpha_swap0(&mut self) -> Win3AlphaSwap0W<Win3Ctrl0Spec> {
        Win3AlphaSwap0W::new(self, 21)
    }
    #[doc = "Bit 22 - Win3 region0 8pp palette data Big-endian/ Little-endian select"]
    #[inline(always)]
    #[must_use]
    pub fn win3_endian_swap0(&mut self) -> Win3EndianSwap0W<Win3Ctrl0Spec> {
        Win3EndianSwap0W::new(self, 22)
    }
    #[doc = "Bit 23 - Win3 region1 RGB Red and Blue swap"]
    #[inline(always)]
    #[must_use]
    pub fn win3_rb_swap1(&mut self) -> Win3RbSwap1W<Win3Ctrl0Spec> {
        Win3RbSwap1W::new(self, 23)
    }
    #[doc = "Bit 24 - Win3 region1 RGB alpha swap"]
    #[inline(always)]
    #[must_use]
    pub fn win3_alpha_swap1(&mut self) -> Win3AlphaSwap1W<Win3Ctrl0Spec> {
        Win3AlphaSwap1W::new(self, 24)
    }
    #[doc = "Bit 25 - Win3 region1 8pp palette data Big-endian/ Little-endian select"]
    #[inline(always)]
    #[must_use]
    pub fn win3_endian_swap1(&mut self) -> Win3EndianSwap1W<Win3Ctrl0Spec> {
        Win3EndianSwap1W::new(self, 25)
    }
    #[doc = "Bit 26 - Win3 region2 RGB Red and Blue swap"]
    #[inline(always)]
    #[must_use]
    pub fn win3_rb_swap2(&mut self) -> Win3RbSwap2W<Win3Ctrl0Spec> {
        Win3RbSwap2W::new(self, 26)
    }
    #[doc = "Bit 27 - Win3 region2 RGB alpha swap"]
    #[inline(always)]
    #[must_use]
    pub fn win3_alpha_swap2(&mut self) -> Win3AlphaSwap2W<Win3Ctrl0Spec> {
        Win3AlphaSwap2W::new(self, 27)
    }
    #[doc = "Bit 28 - Win3 region2 8pp palette data Big-endian/ Little-endian select"]
    #[inline(always)]
    #[must_use]
    pub fn win3_endian_swap2(&mut self) -> Win3EndianSwap2W<Win3Ctrl0Spec> {
        Win3EndianSwap2W::new(self, 28)
    }
    #[doc = "Bit 29 - Win3 region3 RGB Red and Blue swap"]
    #[inline(always)]
    #[must_use]
    pub fn win3_rb_swap3(&mut self) -> Win3RbSwap3W<Win3Ctrl0Spec> {
        Win3RbSwap3W::new(self, 29)
    }
    #[doc = "Bit 30 - Win3 region3 RGB alpha swap"]
    #[inline(always)]
    #[must_use]
    pub fn win3_alpha_swap3(&mut self) -> Win3AlphaSwap3W<Win3Ctrl0Spec> {
        Win3AlphaSwap3W::new(self, 30)
    }
    #[doc = "Bit 31 - Win3 region3 8pp palette data Big-endian/ Little-endian select"]
    #[inline(always)]
    #[must_use]
    pub fn win3_endian_swap3(&mut self) -> Win3EndianSwap3W<Win3Ctrl0Spec> {
        Win3EndianSwap3W::new(self, 31)
    }
}
#[doc = "Win3 ctrl register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win3Ctrl0Spec;
impl crate::RegisterSpec for Win3Ctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win3_ctrl0::R`](R) reader structure"]
impl crate::Readable for Win3Ctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`win3_ctrl0::W`](W) writer structure"]
impl crate::Writable for Win3Ctrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN3_CTRL0 to value 0"]
impl crate::Resettable for Win3Ctrl0Spec {
    const RESET_VALUE: u32 = 0;
}
