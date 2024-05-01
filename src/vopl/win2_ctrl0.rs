#[doc = "Register `WIN2_CTRL0` reader"]
pub type R = crate::R<Win2Ctrl0Spec>;
#[doc = "Register `WIN2_CTRL0` writer"]
pub type W = crate::W<Win2Ctrl0Spec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2En {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win2En> for bool {
    #[inline(always)]
    fn from(variant: Win2En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_EN` reader - "]
pub type Win2EnR = crate::BitReader<Win2En>;
impl Win2EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2En {
        match self.bits {
            false => Win2En::B0,
            true => Win2En::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2En::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2En::B1
    }
}
#[doc = "Field `WIN2_EN` writer - "]
pub type Win2EnW<'a, REG> = crate::BitWriter<'a, REG, Win2En>;
impl<'a, REG> Win2EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2En::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2En::B1)
    }
}
#[doc = "Win2 interlace read mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2InterlaceRead {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win2InterlaceRead> for bool {
    #[inline(always)]
    fn from(variant: Win2InterlaceRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_INTERLACE_READ` reader - Win2 interlace read mode"]
pub type Win2InterlaceReadR = crate::BitReader<Win2InterlaceRead>;
impl Win2InterlaceReadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2InterlaceRead {
        match self.bits {
            false => Win2InterlaceRead::B0,
            true => Win2InterlaceRead::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2InterlaceRead::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2InterlaceRead::B1
    }
}
#[doc = "Field `WIN2_INTERLACE_READ` writer - Win2 interlace read mode"]
pub type Win2InterlaceReadW<'a, REG> = crate::BitWriter<'a, REG, Win2InterlaceRead>;
impl<'a, REG> Win2InterlaceReadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2InterlaceRead::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2InterlaceRead::B1)
    }
}
#[doc = "Win2 RGB2YUV conversion mode\n\nColor space conversion:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win2CscMode {
    #[doc = "0: BT601_L"]
    B00 = 0,
    #[doc = "1: BT709_L"]
    B01 = 1,
    #[doc = "2: BT601_F"]
    B10 = 2,
    #[doc = "3: BT2020"]
    B11 = 3,
}
impl From<Win2CscMode> for u8 {
    #[inline(always)]
    fn from(variant: Win2CscMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win2CscMode {
    type Ux = u8;
}
#[doc = "Field `WIN2_CSC_MODE` reader - Win2 RGB2YUV conversion mode\n\nColor space conversion:"]
pub type Win2CscModeR = crate::FieldReader<Win2CscMode>;
impl Win2CscModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2CscMode {
        match self.bits {
            0 => Win2CscMode::B00,
            1 => Win2CscMode::B01,
            2 => Win2CscMode::B10,
            3 => Win2CscMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "BT601_L"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win2CscMode::B00
    }
    #[doc = "BT709_L"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win2CscMode::B01
    }
    #[doc = "BT601_F"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win2CscMode::B10
    }
    #[doc = "BT2020"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win2CscMode::B11
    }
}
#[doc = "Field `WIN2_CSC_MODE` writer - Win2 RGB2YUV conversion mode\n\nColor space conversion:"]
pub type Win2CscModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win2CscMode>;
impl<'a, REG> Win2CscModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "BT601_L"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win2CscMode::B00)
    }
    #[doc = "BT709_L"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win2CscMode::B01)
    }
    #[doc = "BT601_F"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win2CscMode::B10)
    }
    #[doc = "BT2020"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win2CscMode::B11)
    }
}
#[doc = "win2 master0 enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2Mst0En {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win2Mst0En> for bool {
    #[inline(always)]
    fn from(variant: Win2Mst0En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_MST0_EN` reader - win2 master0 enable"]
pub type Win2Mst0EnR = crate::BitReader<Win2Mst0En>;
impl Win2Mst0EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2Mst0En {
        match self.bits {
            false => Win2Mst0En::B0,
            true => Win2Mst0En::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2Mst0En::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2Mst0En::B1
    }
}
#[doc = "Field `WIN2_MST0_EN` writer - win2 master0 enable"]
pub type Win2Mst0EnW<'a, REG> = crate::BitWriter<'a, REG, Win2Mst0En>;
impl<'a, REG> Win2Mst0EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2Mst0En::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2Mst0En::B1)
    }
}
#[doc = "Win2 region 0 data format\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win2DataFmt0 {
    #[doc = "0: ARGB888"]
    B00 = 0,
    #[doc = "1: RGB888"]
    B01 = 1,
    #[doc = "2: RGB565"]
    B10 = 2,
    #[doc = "3: 8bpp"]
    B11 = 3,
}
impl From<Win2DataFmt0> for u8 {
    #[inline(always)]
    fn from(variant: Win2DataFmt0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win2DataFmt0 {
    type Ux = u8;
}
#[doc = "Field `WIN2_DATA_FMT0` reader - Win2 region 0 data format"]
pub type Win2DataFmt0R = crate::FieldReader<Win2DataFmt0>;
impl Win2DataFmt0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2DataFmt0 {
        match self.bits {
            0 => Win2DataFmt0::B00,
            1 => Win2DataFmt0::B01,
            2 => Win2DataFmt0::B10,
            3 => Win2DataFmt0::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "ARGB888"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win2DataFmt0::B00
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win2DataFmt0::B01
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win2DataFmt0::B10
    }
    #[doc = "8bpp"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win2DataFmt0::B11
    }
}
#[doc = "Field `WIN2_DATA_FMT0` writer - Win2 region 0 data format"]
pub type Win2DataFmt0W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win2DataFmt0>;
impl<'a, REG> Win2DataFmt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ARGB888"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win2DataFmt0::B00)
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win2DataFmt0::B01)
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win2DataFmt0::B10)
    }
    #[doc = "8bpp"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win2DataFmt0::B11)
    }
}
#[doc = "win2 master1 enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2Mst1En {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win2Mst1En> for bool {
    #[inline(always)]
    fn from(variant: Win2Mst1En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_MST1_EN` reader - win2 master1 enable"]
pub type Win2Mst1EnR = crate::BitReader<Win2Mst1En>;
impl Win2Mst1EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2Mst1En {
        match self.bits {
            false => Win2Mst1En::B0,
            true => Win2Mst1En::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2Mst1En::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2Mst1En::B1
    }
}
#[doc = "Field `WIN2_MST1_EN` writer - win2 master1 enable"]
pub type Win2Mst1EnW<'a, REG> = crate::BitWriter<'a, REG, Win2Mst1En>;
impl<'a, REG> Win2Mst1EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2Mst1En::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2Mst1En::B1)
    }
}
#[doc = "Win2 region 1 data format\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win2DataFmt1 {
    #[doc = "0: ARGB888"]
    B00 = 0,
    #[doc = "1: RGB888"]
    B01 = 1,
    #[doc = "2: RGB565"]
    B10 = 2,
    #[doc = "3: 8bpp"]
    B11 = 3,
}
impl From<Win2DataFmt1> for u8 {
    #[inline(always)]
    fn from(variant: Win2DataFmt1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win2DataFmt1 {
    type Ux = u8;
}
#[doc = "Field `WIN2_DATA_FMT1` reader - Win2 region 1 data format"]
pub type Win2DataFmt1R = crate::FieldReader<Win2DataFmt1>;
impl Win2DataFmt1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2DataFmt1 {
        match self.bits {
            0 => Win2DataFmt1::B00,
            1 => Win2DataFmt1::B01,
            2 => Win2DataFmt1::B10,
            3 => Win2DataFmt1::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "ARGB888"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win2DataFmt1::B00
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win2DataFmt1::B01
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win2DataFmt1::B10
    }
    #[doc = "8bpp"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win2DataFmt1::B11
    }
}
#[doc = "Field `WIN2_DATA_FMT1` writer - Win2 region 1 data format"]
pub type Win2DataFmt1W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win2DataFmt1>;
impl<'a, REG> Win2DataFmt1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ARGB888"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win2DataFmt1::B00)
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win2DataFmt1::B01)
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win2DataFmt1::B10)
    }
    #[doc = "8bpp"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win2DataFmt1::B11)
    }
}
#[doc = "win2 master2 enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2Mst2En {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win2Mst2En> for bool {
    #[inline(always)]
    fn from(variant: Win2Mst2En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_MST2_EN` reader - win2 master2 enable"]
pub type Win2Mst2EnR = crate::BitReader<Win2Mst2En>;
impl Win2Mst2EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2Mst2En {
        match self.bits {
            false => Win2Mst2En::B0,
            true => Win2Mst2En::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2Mst2En::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2Mst2En::B1
    }
}
#[doc = "Field `WIN2_MST2_EN` writer - win2 master2 enable"]
pub type Win2Mst2EnW<'a, REG> = crate::BitWriter<'a, REG, Win2Mst2En>;
impl<'a, REG> Win2Mst2EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2Mst2En::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2Mst2En::B1)
    }
}
#[doc = "Win2 region 2 data format\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win2DataFmt2 {
    #[doc = "0: ARGB888"]
    B00 = 0,
    #[doc = "1: RGB888"]
    B01 = 1,
    #[doc = "2: RGB565"]
    B10 = 2,
    #[doc = "3: 8bpp"]
    B11 = 3,
}
impl From<Win2DataFmt2> for u8 {
    #[inline(always)]
    fn from(variant: Win2DataFmt2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win2DataFmt2 {
    type Ux = u8;
}
#[doc = "Field `WIN2_DATA_FMT2` reader - Win2 region 2 data format"]
pub type Win2DataFmt2R = crate::FieldReader<Win2DataFmt2>;
impl Win2DataFmt2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2DataFmt2 {
        match self.bits {
            0 => Win2DataFmt2::B00,
            1 => Win2DataFmt2::B01,
            2 => Win2DataFmt2::B10,
            3 => Win2DataFmt2::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "ARGB888"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win2DataFmt2::B00
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win2DataFmt2::B01
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win2DataFmt2::B10
    }
    #[doc = "8bpp"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win2DataFmt2::B11
    }
}
#[doc = "Field `WIN2_DATA_FMT2` writer - Win2 region 2 data format"]
pub type Win2DataFmt2W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win2DataFmt2>;
impl<'a, REG> Win2DataFmt2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ARGB888"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win2DataFmt2::B00)
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win2DataFmt2::B01)
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win2DataFmt2::B10)
    }
    #[doc = "8bpp"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win2DataFmt2::B11)
    }
}
#[doc = "win2 master3 enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2Mst3En {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Win2Mst3En> for bool {
    #[inline(always)]
    fn from(variant: Win2Mst3En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_MST3_EN` reader - win2 master3 enable"]
pub type Win2Mst3EnR = crate::BitReader<Win2Mst3En>;
impl Win2Mst3EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2Mst3En {
        match self.bits {
            false => Win2Mst3En::B0,
            true => Win2Mst3En::B1,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2Mst3En::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2Mst3En::B1
    }
}
#[doc = "Field `WIN2_MST3_EN` writer - win2 master3 enable"]
pub type Win2Mst3EnW<'a, REG> = crate::BitWriter<'a, REG, Win2Mst3En>;
impl<'a, REG> Win2Mst3EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2Mst3En::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2Mst3En::B1)
    }
}
#[doc = "Win2 region 3 data format\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Win2DataFmt3 {
    #[doc = "0: ARGB888"]
    B00 = 0,
    #[doc = "1: RGB888"]
    B01 = 1,
    #[doc = "2: RGB565"]
    B10 = 2,
    #[doc = "3: 8bpp"]
    B11 = 3,
}
impl From<Win2DataFmt3> for u8 {
    #[inline(always)]
    fn from(variant: Win2DataFmt3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Win2DataFmt3 {
    type Ux = u8;
}
#[doc = "Field `WIN2_DATA_FMT3` reader - Win2 region 3 data format"]
pub type Win2DataFmt3R = crate::FieldReader<Win2DataFmt3>;
impl Win2DataFmt3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2DataFmt3 {
        match self.bits {
            0 => Win2DataFmt3::B00,
            1 => Win2DataFmt3::B01,
            2 => Win2DataFmt3::B10,
            3 => Win2DataFmt3::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "ARGB888"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Win2DataFmt3::B00
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Win2DataFmt3::B01
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Win2DataFmt3::B10
    }
    #[doc = "8bpp"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Win2DataFmt3::B11
    }
}
#[doc = "Field `WIN2_DATA_FMT3` writer - Win2 region 3 data format"]
pub type Win2DataFmt3W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Win2DataFmt3>;
impl<'a, REG> Win2DataFmt3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ARGB888"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Win2DataFmt3::B00)
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Win2DataFmt3::B01)
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Win2DataFmt3::B10)
    }
    #[doc = "8bpp"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Win2DataFmt3::B11)
    }
}
#[doc = "Win2 region0 RGB Red and Blue swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2RbSwap0 {
    #[doc = "0: RGB"]
    B0 = 0,
    #[doc = "1: BGR"]
    B1 = 1,
}
impl From<Win2RbSwap0> for bool {
    #[inline(always)]
    fn from(variant: Win2RbSwap0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_RB_SWAP0` reader - Win2 region0 RGB Red and Blue swap"]
pub type Win2RbSwap0R = crate::BitReader<Win2RbSwap0>;
impl Win2RbSwap0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2RbSwap0 {
        match self.bits {
            false => Win2RbSwap0::B0,
            true => Win2RbSwap0::B1,
        }
    }
    #[doc = "RGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2RbSwap0::B0
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2RbSwap0::B1
    }
}
#[doc = "Field `WIN2_RB_SWAP0` writer - Win2 region0 RGB Red and Blue swap"]
pub type Win2RbSwap0W<'a, REG> = crate::BitWriter<'a, REG, Win2RbSwap0>;
impl<'a, REG> Win2RbSwap0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2RbSwap0::B0)
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2RbSwap0::B1)
    }
}
#[doc = "Win2 region0 RGB alpha swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2AlphaSwap0 {
    #[doc = "0: ARGB"]
    B0 = 0,
    #[doc = "1: RGBA"]
    B1 = 1,
}
impl From<Win2AlphaSwap0> for bool {
    #[inline(always)]
    fn from(variant: Win2AlphaSwap0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_ALPHA_SWAP0` reader - Win2 region0 RGB alpha swap"]
pub type Win2AlphaSwap0R = crate::BitReader<Win2AlphaSwap0>;
impl Win2AlphaSwap0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2AlphaSwap0 {
        match self.bits {
            false => Win2AlphaSwap0::B0,
            true => Win2AlphaSwap0::B1,
        }
    }
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2AlphaSwap0::B0
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2AlphaSwap0::B1
    }
}
#[doc = "Field `WIN2_ALPHA_SWAP0` writer - Win2 region0 RGB alpha swap"]
pub type Win2AlphaSwap0W<'a, REG> = crate::BitWriter<'a, REG, Win2AlphaSwap0>;
impl<'a, REG> Win2AlphaSwap0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2AlphaSwap0::B0)
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2AlphaSwap0::B1)
    }
}
#[doc = "Win2 region0 8pp palette data Big-endian/ Little-endian select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2EndianSwap0 {
    #[doc = "0: Big-endian"]
    B0 = 0,
    #[doc = "1: Little-endian"]
    B1 = 1,
}
impl From<Win2EndianSwap0> for bool {
    #[inline(always)]
    fn from(variant: Win2EndianSwap0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_ENDIAN_SWAP0` reader - Win2 region0 8pp palette data Big-endian/ Little-endian select"]
pub type Win2EndianSwap0R = crate::BitReader<Win2EndianSwap0>;
impl Win2EndianSwap0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2EndianSwap0 {
        match self.bits {
            false => Win2EndianSwap0::B0,
            true => Win2EndianSwap0::B1,
        }
    }
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2EndianSwap0::B0
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2EndianSwap0::B1
    }
}
#[doc = "Field `WIN2_ENDIAN_SWAP0` writer - Win2 region0 8pp palette data Big-endian/ Little-endian select"]
pub type Win2EndianSwap0W<'a, REG> = crate::BitWriter<'a, REG, Win2EndianSwap0>;
impl<'a, REG> Win2EndianSwap0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2EndianSwap0::B0)
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2EndianSwap0::B1)
    }
}
#[doc = "Win2 region1 RGB Red and Blue swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2RbSwap1 {
    #[doc = "0: RGB"]
    B0 = 0,
    #[doc = "1: BGR"]
    B1 = 1,
}
impl From<Win2RbSwap1> for bool {
    #[inline(always)]
    fn from(variant: Win2RbSwap1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_RB_SWAP1` reader - Win2 region1 RGB Red and Blue swap"]
pub type Win2RbSwap1R = crate::BitReader<Win2RbSwap1>;
impl Win2RbSwap1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2RbSwap1 {
        match self.bits {
            false => Win2RbSwap1::B0,
            true => Win2RbSwap1::B1,
        }
    }
    #[doc = "RGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2RbSwap1::B0
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2RbSwap1::B1
    }
}
#[doc = "Field `WIN2_RB_SWAP1` writer - Win2 region1 RGB Red and Blue swap"]
pub type Win2RbSwap1W<'a, REG> = crate::BitWriter<'a, REG, Win2RbSwap1>;
impl<'a, REG> Win2RbSwap1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2RbSwap1::B0)
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2RbSwap1::B1)
    }
}
#[doc = "Win2 region1 RGB alpha swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2AlphaSwap1 {
    #[doc = "0: ARGB"]
    B0 = 0,
    #[doc = "1: RGBA"]
    B1 = 1,
}
impl From<Win2AlphaSwap1> for bool {
    #[inline(always)]
    fn from(variant: Win2AlphaSwap1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_ALPHA_SWAP1` reader - Win2 region1 RGB alpha swap"]
pub type Win2AlphaSwap1R = crate::BitReader<Win2AlphaSwap1>;
impl Win2AlphaSwap1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2AlphaSwap1 {
        match self.bits {
            false => Win2AlphaSwap1::B0,
            true => Win2AlphaSwap1::B1,
        }
    }
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2AlphaSwap1::B0
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2AlphaSwap1::B1
    }
}
#[doc = "Field `WIN2_ALPHA_SWAP1` writer - Win2 region1 RGB alpha swap"]
pub type Win2AlphaSwap1W<'a, REG> = crate::BitWriter<'a, REG, Win2AlphaSwap1>;
impl<'a, REG> Win2AlphaSwap1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2AlphaSwap1::B0)
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2AlphaSwap1::B1)
    }
}
#[doc = "Win2 region1 8pp palette data Big-endian/ Little-endian select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2EndianSwap1 {
    #[doc = "0: Big-endian"]
    B0 = 0,
    #[doc = "1: Little-endian"]
    B1 = 1,
}
impl From<Win2EndianSwap1> for bool {
    #[inline(always)]
    fn from(variant: Win2EndianSwap1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_ENDIAN_SWAP1` reader - Win2 region1 8pp palette data Big-endian/ Little-endian select"]
pub type Win2EndianSwap1R = crate::BitReader<Win2EndianSwap1>;
impl Win2EndianSwap1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2EndianSwap1 {
        match self.bits {
            false => Win2EndianSwap1::B0,
            true => Win2EndianSwap1::B1,
        }
    }
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2EndianSwap1::B0
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2EndianSwap1::B1
    }
}
#[doc = "Field `WIN2_ENDIAN_SWAP1` writer - Win2 region1 8pp palette data Big-endian/ Little-endian select"]
pub type Win2EndianSwap1W<'a, REG> = crate::BitWriter<'a, REG, Win2EndianSwap1>;
impl<'a, REG> Win2EndianSwap1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2EndianSwap1::B0)
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2EndianSwap1::B1)
    }
}
#[doc = "Win2 region2 RGB Red and Blue swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2RbSwap2 {
    #[doc = "0: RGB"]
    B0 = 0,
    #[doc = "1: BGR"]
    B1 = 1,
}
impl From<Win2RbSwap2> for bool {
    #[inline(always)]
    fn from(variant: Win2RbSwap2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_RB_SWAP2` reader - Win2 region2 RGB Red and Blue swap"]
pub type Win2RbSwap2R = crate::BitReader<Win2RbSwap2>;
impl Win2RbSwap2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2RbSwap2 {
        match self.bits {
            false => Win2RbSwap2::B0,
            true => Win2RbSwap2::B1,
        }
    }
    #[doc = "RGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2RbSwap2::B0
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2RbSwap2::B1
    }
}
#[doc = "Field `WIN2_RB_SWAP2` writer - Win2 region2 RGB Red and Blue swap"]
pub type Win2RbSwap2W<'a, REG> = crate::BitWriter<'a, REG, Win2RbSwap2>;
impl<'a, REG> Win2RbSwap2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2RbSwap2::B0)
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2RbSwap2::B1)
    }
}
#[doc = "Win2 region2 RGB alpha swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2AlphaSwap2 {
    #[doc = "0: ARGB"]
    B0 = 0,
    #[doc = "1: RGBA"]
    B1 = 1,
}
impl From<Win2AlphaSwap2> for bool {
    #[inline(always)]
    fn from(variant: Win2AlphaSwap2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_ALPHA_SWAP2` reader - Win2 region2 RGB alpha swap"]
pub type Win2AlphaSwap2R = crate::BitReader<Win2AlphaSwap2>;
impl Win2AlphaSwap2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2AlphaSwap2 {
        match self.bits {
            false => Win2AlphaSwap2::B0,
            true => Win2AlphaSwap2::B1,
        }
    }
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2AlphaSwap2::B0
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2AlphaSwap2::B1
    }
}
#[doc = "Field `WIN2_ALPHA_SWAP2` writer - Win2 region2 RGB alpha swap"]
pub type Win2AlphaSwap2W<'a, REG> = crate::BitWriter<'a, REG, Win2AlphaSwap2>;
impl<'a, REG> Win2AlphaSwap2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2AlphaSwap2::B0)
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2AlphaSwap2::B1)
    }
}
#[doc = "Win2 region2 8pp palette data Big-endian/ Little-endian select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2EndianSwap2 {
    #[doc = "0: Big-endian"]
    B0 = 0,
    #[doc = "1: Little-endian"]
    B1 = 1,
}
impl From<Win2EndianSwap2> for bool {
    #[inline(always)]
    fn from(variant: Win2EndianSwap2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_ENDIAN_SWAP2` reader - Win2 region2 8pp palette data Big-endian/ Little-endian select"]
pub type Win2EndianSwap2R = crate::BitReader<Win2EndianSwap2>;
impl Win2EndianSwap2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2EndianSwap2 {
        match self.bits {
            false => Win2EndianSwap2::B0,
            true => Win2EndianSwap2::B1,
        }
    }
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2EndianSwap2::B0
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2EndianSwap2::B1
    }
}
#[doc = "Field `WIN2_ENDIAN_SWAP2` writer - Win2 region2 8pp palette data Big-endian/ Little-endian select"]
pub type Win2EndianSwap2W<'a, REG> = crate::BitWriter<'a, REG, Win2EndianSwap2>;
impl<'a, REG> Win2EndianSwap2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2EndianSwap2::B0)
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2EndianSwap2::B1)
    }
}
#[doc = "Win2 region3 RGB Red and Blue swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2RbSwap3 {
    #[doc = "0: RGB"]
    B0 = 0,
    #[doc = "1: BGR"]
    B1 = 1,
}
impl From<Win2RbSwap3> for bool {
    #[inline(always)]
    fn from(variant: Win2RbSwap3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_RB_SWAP3` reader - Win2 region3 RGB Red and Blue swap"]
pub type Win2RbSwap3R = crate::BitReader<Win2RbSwap3>;
impl Win2RbSwap3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2RbSwap3 {
        match self.bits {
            false => Win2RbSwap3::B0,
            true => Win2RbSwap3::B1,
        }
    }
    #[doc = "RGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2RbSwap3::B0
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2RbSwap3::B1
    }
}
#[doc = "Field `WIN2_RB_SWAP3` writer - Win2 region3 RGB Red and Blue swap"]
pub type Win2RbSwap3W<'a, REG> = crate::BitWriter<'a, REG, Win2RbSwap3>;
impl<'a, REG> Win2RbSwap3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2RbSwap3::B0)
    }
    #[doc = "BGR"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2RbSwap3::B1)
    }
}
#[doc = "Win2 region3 RGB alpha swap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2AlphaSwap3 {
    #[doc = "0: ARGB"]
    B0 = 0,
    #[doc = "1: RGBA"]
    B1 = 1,
}
impl From<Win2AlphaSwap3> for bool {
    #[inline(always)]
    fn from(variant: Win2AlphaSwap3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_ALPHA_SWAP3` reader - Win2 region3 RGB alpha swap"]
pub type Win2AlphaSwap3R = crate::BitReader<Win2AlphaSwap3>;
impl Win2AlphaSwap3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2AlphaSwap3 {
        match self.bits {
            false => Win2AlphaSwap3::B0,
            true => Win2AlphaSwap3::B1,
        }
    }
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2AlphaSwap3::B0
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2AlphaSwap3::B1
    }
}
#[doc = "Field `WIN2_ALPHA_SWAP3` writer - Win2 region3 RGB alpha swap"]
pub type Win2AlphaSwap3W<'a, REG> = crate::BitWriter<'a, REG, Win2AlphaSwap3>;
impl<'a, REG> Win2AlphaSwap3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ARGB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2AlphaSwap3::B0)
    }
    #[doc = "RGBA"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2AlphaSwap3::B1)
    }
}
#[doc = "Win2 region3 8pp palette data Big-endian/ Little-endian select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Win2EndianSwap3 {
    #[doc = "0: Big-endian"]
    B0 = 0,
    #[doc = "1: Little-endian"]
    B1 = 1,
}
impl From<Win2EndianSwap3> for bool {
    #[inline(always)]
    fn from(variant: Win2EndianSwap3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN2_ENDIAN_SWAP3` reader - Win2 region3 8pp palette data Big-endian/ Little-endian select"]
pub type Win2EndianSwap3R = crate::BitReader<Win2EndianSwap3>;
impl Win2EndianSwap3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Win2EndianSwap3 {
        match self.bits {
            false => Win2EndianSwap3::B0,
            true => Win2EndianSwap3::B1,
        }
    }
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Win2EndianSwap3::B0
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Win2EndianSwap3::B1
    }
}
#[doc = "Field `WIN2_ENDIAN_SWAP3` writer - Win2 region3 8pp palette data Big-endian/ Little-endian select"]
pub type Win2EndianSwap3W<'a, REG> = crate::BitWriter<'a, REG, Win2EndianSwap3>;
impl<'a, REG> Win2EndianSwap3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Big-endian"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Win2EndianSwap3::B0)
    }
    #[doc = "Little-endian"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Win2EndianSwap3::B1)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn win2_en(&self) -> Win2EnR {
        Win2EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Win2 interlace read mode"]
    #[inline(always)]
    pub fn win2_interlace_read(&self) -> Win2InterlaceReadR {
        Win2InterlaceReadR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Win2 RGB2YUV conversion mode\n\nColor space conversion:"]
    #[inline(always)]
    pub fn win2_csc_mode(&self) -> Win2CscModeR {
        Win2CscModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - win2 master0 enable"]
    #[inline(always)]
    pub fn win2_mst0_en(&self) -> Win2Mst0EnR {
        Win2Mst0EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Win2 region 0 data format"]
    #[inline(always)]
    pub fn win2_data_fmt0(&self) -> Win2DataFmt0R {
        Win2DataFmt0R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - win2 master1 enable"]
    #[inline(always)]
    pub fn win2_mst1_en(&self) -> Win2Mst1EnR {
        Win2Mst1EnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Win2 region 1 data format"]
    #[inline(always)]
    pub fn win2_data_fmt1(&self) -> Win2DataFmt1R {
        Win2DataFmt1R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 12 - win2 master2 enable"]
    #[inline(always)]
    pub fn win2_mst2_en(&self) -> Win2Mst2EnR {
        Win2Mst2EnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Win2 region 2 data format"]
    #[inline(always)]
    pub fn win2_data_fmt2(&self) -> Win2DataFmt2R {
        Win2DataFmt2R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 16 - win2 master3 enable"]
    #[inline(always)]
    pub fn win2_mst3_en(&self) -> Win2Mst3EnR {
        Win2Mst3EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Win2 region 3 data format"]
    #[inline(always)]
    pub fn win2_data_fmt3(&self) -> Win2DataFmt3R {
        Win2DataFmt3R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 20 - Win2 region0 RGB Red and Blue swap"]
    #[inline(always)]
    pub fn win2_rb_swap0(&self) -> Win2RbSwap0R {
        Win2RbSwap0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Win2 region0 RGB alpha swap"]
    #[inline(always)]
    pub fn win2_alpha_swap0(&self) -> Win2AlphaSwap0R {
        Win2AlphaSwap0R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Win2 region0 8pp palette data Big-endian/ Little-endian select"]
    #[inline(always)]
    pub fn win2_endian_swap0(&self) -> Win2EndianSwap0R {
        Win2EndianSwap0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Win2 region1 RGB Red and Blue swap"]
    #[inline(always)]
    pub fn win2_rb_swap1(&self) -> Win2RbSwap1R {
        Win2RbSwap1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Win2 region1 RGB alpha swap"]
    #[inline(always)]
    pub fn win2_alpha_swap1(&self) -> Win2AlphaSwap1R {
        Win2AlphaSwap1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Win2 region1 8pp palette data Big-endian/ Little-endian select"]
    #[inline(always)]
    pub fn win2_endian_swap1(&self) -> Win2EndianSwap1R {
        Win2EndianSwap1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Win2 region2 RGB Red and Blue swap"]
    #[inline(always)]
    pub fn win2_rb_swap2(&self) -> Win2RbSwap2R {
        Win2RbSwap2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Win2 region2 RGB alpha swap"]
    #[inline(always)]
    pub fn win2_alpha_swap2(&self) -> Win2AlphaSwap2R {
        Win2AlphaSwap2R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Win2 region2 8pp palette data Big-endian/ Little-endian select"]
    #[inline(always)]
    pub fn win2_endian_swap2(&self) -> Win2EndianSwap2R {
        Win2EndianSwap2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Win2 region3 RGB Red and Blue swap"]
    #[inline(always)]
    pub fn win2_rb_swap3(&self) -> Win2RbSwap3R {
        Win2RbSwap3R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Win2 region3 RGB alpha swap"]
    #[inline(always)]
    pub fn win2_alpha_swap3(&self) -> Win2AlphaSwap3R {
        Win2AlphaSwap3R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Win2 region3 8pp palette data Big-endian/ Little-endian select"]
    #[inline(always)]
    pub fn win2_endian_swap3(&self) -> Win2EndianSwap3R {
        Win2EndianSwap3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn win2_en(&mut self) -> Win2EnW<Win2Ctrl0Spec> {
        Win2EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Win2 interlace read mode"]
    #[inline(always)]
    #[must_use]
    pub fn win2_interlace_read(&mut self) -> Win2InterlaceReadW<Win2Ctrl0Spec> {
        Win2InterlaceReadW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Win2 RGB2YUV conversion mode\n\nColor space conversion:"]
    #[inline(always)]
    #[must_use]
    pub fn win2_csc_mode(&mut self) -> Win2CscModeW<Win2Ctrl0Spec> {
        Win2CscModeW::new(self, 2)
    }
    #[doc = "Bit 4 - win2 master0 enable"]
    #[inline(always)]
    #[must_use]
    pub fn win2_mst0_en(&mut self) -> Win2Mst0EnW<Win2Ctrl0Spec> {
        Win2Mst0EnW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Win2 region 0 data format"]
    #[inline(always)]
    #[must_use]
    pub fn win2_data_fmt0(&mut self) -> Win2DataFmt0W<Win2Ctrl0Spec> {
        Win2DataFmt0W::new(self, 5)
    }
    #[doc = "Bit 8 - win2 master1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn win2_mst1_en(&mut self) -> Win2Mst1EnW<Win2Ctrl0Spec> {
        Win2Mst1EnW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Win2 region 1 data format"]
    #[inline(always)]
    #[must_use]
    pub fn win2_data_fmt1(&mut self) -> Win2DataFmt1W<Win2Ctrl0Spec> {
        Win2DataFmt1W::new(self, 9)
    }
    #[doc = "Bit 12 - win2 master2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn win2_mst2_en(&mut self) -> Win2Mst2EnW<Win2Ctrl0Spec> {
        Win2Mst2EnW::new(self, 12)
    }
    #[doc = "Bits 13:14 - Win2 region 2 data format"]
    #[inline(always)]
    #[must_use]
    pub fn win2_data_fmt2(&mut self) -> Win2DataFmt2W<Win2Ctrl0Spec> {
        Win2DataFmt2W::new(self, 13)
    }
    #[doc = "Bit 16 - win2 master3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn win2_mst3_en(&mut self) -> Win2Mst3EnW<Win2Ctrl0Spec> {
        Win2Mst3EnW::new(self, 16)
    }
    #[doc = "Bits 17:18 - Win2 region 3 data format"]
    #[inline(always)]
    #[must_use]
    pub fn win2_data_fmt3(&mut self) -> Win2DataFmt3W<Win2Ctrl0Spec> {
        Win2DataFmt3W::new(self, 17)
    }
    #[doc = "Bit 20 - Win2 region0 RGB Red and Blue swap"]
    #[inline(always)]
    #[must_use]
    pub fn win2_rb_swap0(&mut self) -> Win2RbSwap0W<Win2Ctrl0Spec> {
        Win2RbSwap0W::new(self, 20)
    }
    #[doc = "Bit 21 - Win2 region0 RGB alpha swap"]
    #[inline(always)]
    #[must_use]
    pub fn win2_alpha_swap0(&mut self) -> Win2AlphaSwap0W<Win2Ctrl0Spec> {
        Win2AlphaSwap0W::new(self, 21)
    }
    #[doc = "Bit 22 - Win2 region0 8pp palette data Big-endian/ Little-endian select"]
    #[inline(always)]
    #[must_use]
    pub fn win2_endian_swap0(&mut self) -> Win2EndianSwap0W<Win2Ctrl0Spec> {
        Win2EndianSwap0W::new(self, 22)
    }
    #[doc = "Bit 23 - Win2 region1 RGB Red and Blue swap"]
    #[inline(always)]
    #[must_use]
    pub fn win2_rb_swap1(&mut self) -> Win2RbSwap1W<Win2Ctrl0Spec> {
        Win2RbSwap1W::new(self, 23)
    }
    #[doc = "Bit 24 - Win2 region1 RGB alpha swap"]
    #[inline(always)]
    #[must_use]
    pub fn win2_alpha_swap1(&mut self) -> Win2AlphaSwap1W<Win2Ctrl0Spec> {
        Win2AlphaSwap1W::new(self, 24)
    }
    #[doc = "Bit 25 - Win2 region1 8pp palette data Big-endian/ Little-endian select"]
    #[inline(always)]
    #[must_use]
    pub fn win2_endian_swap1(&mut self) -> Win2EndianSwap1W<Win2Ctrl0Spec> {
        Win2EndianSwap1W::new(self, 25)
    }
    #[doc = "Bit 26 - Win2 region2 RGB Red and Blue swap"]
    #[inline(always)]
    #[must_use]
    pub fn win2_rb_swap2(&mut self) -> Win2RbSwap2W<Win2Ctrl0Spec> {
        Win2RbSwap2W::new(self, 26)
    }
    #[doc = "Bit 27 - Win2 region2 RGB alpha swap"]
    #[inline(always)]
    #[must_use]
    pub fn win2_alpha_swap2(&mut self) -> Win2AlphaSwap2W<Win2Ctrl0Spec> {
        Win2AlphaSwap2W::new(self, 27)
    }
    #[doc = "Bit 28 - Win2 region2 8pp palette data Big-endian/ Little-endian select"]
    #[inline(always)]
    #[must_use]
    pub fn win2_endian_swap2(&mut self) -> Win2EndianSwap2W<Win2Ctrl0Spec> {
        Win2EndianSwap2W::new(self, 28)
    }
    #[doc = "Bit 29 - Win2 region3 RGB Red and Blue swap"]
    #[inline(always)]
    #[must_use]
    pub fn win2_rb_swap3(&mut self) -> Win2RbSwap3W<Win2Ctrl0Spec> {
        Win2RbSwap3W::new(self, 29)
    }
    #[doc = "Bit 30 - Win2 region3 RGB alpha swap"]
    #[inline(always)]
    #[must_use]
    pub fn win2_alpha_swap3(&mut self) -> Win2AlphaSwap3W<Win2Ctrl0Spec> {
        Win2AlphaSwap3W::new(self, 30)
    }
    #[doc = "Bit 31 - Win2 region3 8pp palette data Big-endian/ Little-endian select"]
    #[inline(always)]
    #[must_use]
    pub fn win2_endian_swap3(&mut self) -> Win2EndianSwap3W<Win2Ctrl0Spec> {
        Win2EndianSwap3W::new(self, 31)
    }
}
#[doc = "win2 ctrl register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win2Ctrl0Spec;
impl crate::RegisterSpec for Win2Ctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win2_ctrl0::R`](R) reader structure"]
impl crate::Readable for Win2Ctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`win2_ctrl0::W`](W) writer structure"]
impl crate::Writable for Win2Ctrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN2_CTRL0 to value 0"]
impl crate::Resettable for Win2Ctrl0Spec {
    const RESET_VALUE: u32 = 0;
}
