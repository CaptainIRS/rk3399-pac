#[doc = "Register `GRF_GPIO3D_IOMUX` reader"]
pub type R = crate::R<GrfGpio3dIomuxSpec>;
#[doc = "Register `GRF_GPIO3D_IOMUX` writer"]
pub type W = crate::W<GrfGpio3dIomuxSpec>;
#[doc = "GPIO3D\\[0\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d0Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: i2s0_sclk"]
    B01 = 1,
    #[doc = "2: trace_data0"]
    B10 = 2,
    #[doc = "3: a72core0_wfi"]
    B11 = 3,
}
impl From<Gpio3d0Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d0Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d0Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3D0_SEL` reader - GPIO3D\\[0\\]
iomux select"]
pub type Gpio3d0SelR = crate::FieldReader<Gpio3d0Sel>;
impl Gpio3d0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d0Sel {
        match self.bits {
            0 => Gpio3d0Sel::B00,
            1 => Gpio3d0Sel::B01,
            2 => Gpio3d0Sel::B10,
            3 => Gpio3d0Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d0Sel::B00
    }
    #[doc = "i2s0_sclk"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d0Sel::B01
    }
    #[doc = "trace_data0"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d0Sel::B10
    }
    #[doc = "a72core0_wfi"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d0Sel::B11
    }
}
#[doc = "Field `GPIO3D0_SEL` writer - GPIO3D\\[0\\]
iomux select"]
pub type Gpio3d0SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d0Sel>;
impl<'a, REG> Gpio3d0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d0Sel::B00)
    }
    #[doc = "i2s0_sclk"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d0Sel::B01)
    }
    #[doc = "trace_data0"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d0Sel::B10)
    }
    #[doc = "a72core0_wfi"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d0Sel::B11)
    }
}
#[doc = "GPIO3D\\[1\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d1Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: i2s0_lrckrx"]
    B01 = 1,
    #[doc = "2: trace_data1"]
    B10 = 2,
    #[doc = "3: a72core1_wfi"]
    B11 = 3,
}
impl From<Gpio3d1Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d1Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d1Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3D1_SEL` reader - GPIO3D\\[1\\]
iomux select"]
pub type Gpio3d1SelR = crate::FieldReader<Gpio3d1Sel>;
impl Gpio3d1SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d1Sel {
        match self.bits {
            0 => Gpio3d1Sel::B00,
            1 => Gpio3d1Sel::B01,
            2 => Gpio3d1Sel::B10,
            3 => Gpio3d1Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d1Sel::B00
    }
    #[doc = "i2s0_lrckrx"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d1Sel::B01
    }
    #[doc = "trace_data1"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d1Sel::B10
    }
    #[doc = "a72core1_wfi"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d1Sel::B11
    }
}
#[doc = "Field `GPIO3D1_SEL` writer - GPIO3D\\[1\\]
iomux select"]
pub type Gpio3d1SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d1Sel>;
impl<'a, REG> Gpio3d1SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d1Sel::B00)
    }
    #[doc = "i2s0_lrckrx"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d1Sel::B01)
    }
    #[doc = "trace_data1"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d1Sel::B10)
    }
    #[doc = "a72core1_wfi"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d1Sel::B11)
    }
}
#[doc = "GPIO3D\\[2\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d2Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: i2s0_lrcktx"]
    B01 = 1,
    #[doc = "2: trace_data2"]
    B10 = 2,
    #[doc = "3: a53core0_wfi"]
    B11 = 3,
}
impl From<Gpio3d2Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d2Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d2Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3D2_SEL` reader - GPIO3D\\[2\\]
iomux select"]
pub type Gpio3d2SelR = crate::FieldReader<Gpio3d2Sel>;
impl Gpio3d2SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d2Sel {
        match self.bits {
            0 => Gpio3d2Sel::B00,
            1 => Gpio3d2Sel::B01,
            2 => Gpio3d2Sel::B10,
            3 => Gpio3d2Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d2Sel::B00
    }
    #[doc = "i2s0_lrcktx"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d2Sel::B01
    }
    #[doc = "trace_data2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d2Sel::B10
    }
    #[doc = "a53core0_wfi"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d2Sel::B11
    }
}
#[doc = "Field `GPIO3D2_SEL` writer - GPIO3D\\[2\\]
iomux select"]
pub type Gpio3d2SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d2Sel>;
impl<'a, REG> Gpio3d2SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d2Sel::B00)
    }
    #[doc = "i2s0_lrcktx"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d2Sel::B01)
    }
    #[doc = "trace_data2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d2Sel::B10)
    }
    #[doc = "a53core0_wfi"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d2Sel::B11)
    }
}
#[doc = "GPIO3D\\[3\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d3Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: i2s0_sdi0"]
    B01 = 1,
    #[doc = "2: trace_data3"]
    B10 = 2,
    #[doc = "3: a53core1_wfi"]
    B11 = 3,
}
impl From<Gpio3d3Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d3Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d3Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3D3_SEL` reader - GPIO3D\\[3\\]
iomux select"]
pub type Gpio3d3SelR = crate::FieldReader<Gpio3d3Sel>;
impl Gpio3d3SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d3Sel {
        match self.bits {
            0 => Gpio3d3Sel::B00,
            1 => Gpio3d3Sel::B01,
            2 => Gpio3d3Sel::B10,
            3 => Gpio3d3Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d3Sel::B00
    }
    #[doc = "i2s0_sdi0"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d3Sel::B01
    }
    #[doc = "trace_data3"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d3Sel::B10
    }
    #[doc = "a53core1_wfi"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d3Sel::B11
    }
}
#[doc = "Field `GPIO3D3_SEL` writer - GPIO3D\\[3\\]
iomux select"]
pub type Gpio3d3SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d3Sel>;
impl<'a, REG> Gpio3d3SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d3Sel::B00)
    }
    #[doc = "i2s0_sdi0"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d3Sel::B01)
    }
    #[doc = "trace_data3"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d3Sel::B10)
    }
    #[doc = "a53core1_wfi"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d3Sel::B11)
    }
}
#[doc = "GPIO3D\\[4\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d4Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: i2s0_sdi1sdo3"]
    B01 = 1,
    #[doc = "2: trace_data4"]
    B10 = 2,
    #[doc = "3: a53core2_wfi"]
    B11 = 3,
}
impl From<Gpio3d4Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d4Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d4Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3D4_SEL` reader - GPIO3D\\[4\\]
iomux select"]
pub type Gpio3d4SelR = crate::FieldReader<Gpio3d4Sel>;
impl Gpio3d4SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d4Sel {
        match self.bits {
            0 => Gpio3d4Sel::B00,
            1 => Gpio3d4Sel::B01,
            2 => Gpio3d4Sel::B10,
            3 => Gpio3d4Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d4Sel::B00
    }
    #[doc = "i2s0_sdi1sdo3"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d4Sel::B01
    }
    #[doc = "trace_data4"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d4Sel::B10
    }
    #[doc = "a53core2_wfi"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d4Sel::B11
    }
}
#[doc = "Field `GPIO3D4_SEL` writer - GPIO3D\\[4\\]
iomux select"]
pub type Gpio3d4SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d4Sel>;
impl<'a, REG> Gpio3d4SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d4Sel::B00)
    }
    #[doc = "i2s0_sdi1sdo3"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d4Sel::B01)
    }
    #[doc = "trace_data4"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d4Sel::B10)
    }
    #[doc = "a53core2_wfi"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d4Sel::B11)
    }
}
#[doc = "GPIO3D\\[5\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d5Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: i2s0_sdi2sdo2"]
    B01 = 1,
    #[doc = "2: trace_data5"]
    B10 = 2,
    #[doc = "3: a53core3_wfi"]
    B11 = 3,
}
impl From<Gpio3d5Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d5Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d5Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3D5_SEL` reader - GPIO3D\\[5\\]
iomux select"]
pub type Gpio3d5SelR = crate::FieldReader<Gpio3d5Sel>;
impl Gpio3d5SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d5Sel {
        match self.bits {
            0 => Gpio3d5Sel::B00,
            1 => Gpio3d5Sel::B01,
            2 => Gpio3d5Sel::B10,
            3 => Gpio3d5Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d5Sel::B00
    }
    #[doc = "i2s0_sdi2sdo2"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d5Sel::B01
    }
    #[doc = "trace_data5"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d5Sel::B10
    }
    #[doc = "a53core3_wfi"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d5Sel::B11
    }
}
#[doc = "Field `GPIO3D5_SEL` writer - GPIO3D\\[5\\]
iomux select"]
pub type Gpio3d5SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d5Sel>;
impl<'a, REG> Gpio3d5SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d5Sel::B00)
    }
    #[doc = "i2s0_sdi2sdo2"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d5Sel::B01)
    }
    #[doc = "trace_data5"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d5Sel::B10)
    }
    #[doc = "a53core3_wfi"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d5Sel::B11)
    }
}
#[doc = "GPIO3D\\[6\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d6Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: i2s0_sdi3sdo1"]
    B01 = 1,
    #[doc = "2: trace_data6"]
    B10 = 2,
    #[doc = "3: a72l2_wfi"]
    B11 = 3,
}
impl From<Gpio3d6Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d6Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d6Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3D6_SEL` reader - GPIO3D\\[6\\]
iomux select"]
pub type Gpio3d6SelR = crate::FieldReader<Gpio3d6Sel>;
impl Gpio3d6SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d6Sel {
        match self.bits {
            0 => Gpio3d6Sel::B00,
            1 => Gpio3d6Sel::B01,
            2 => Gpio3d6Sel::B10,
            3 => Gpio3d6Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d6Sel::B00
    }
    #[doc = "i2s0_sdi3sdo1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d6Sel::B01
    }
    #[doc = "trace_data6"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d6Sel::B10
    }
    #[doc = "a72l2_wfi"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d6Sel::B11
    }
}
#[doc = "Field `GPIO3D6_SEL` writer - GPIO3D\\[6\\]
iomux select"]
pub type Gpio3d6SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d6Sel>;
impl<'a, REG> Gpio3d6SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d6Sel::B00)
    }
    #[doc = "i2s0_sdi3sdo1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d6Sel::B01)
    }
    #[doc = "trace_data6"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d6Sel::B10)
    }
    #[doc = "a72l2_wfi"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d6Sel::B11)
    }
}
#[doc = "GPIO3D\\[7\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d7Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: i2s0_sdo0"]
    B01 = 1,
    #[doc = "2: trace_data7"]
    B10 = 2,
    #[doc = "3: a53l2_wfi"]
    B11 = 3,
}
impl From<Gpio3d7Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d7Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d7Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3D7_SEL` reader - GPIO3D\\[7\\]
iomux select"]
pub type Gpio3d7SelR = crate::FieldReader<Gpio3d7Sel>;
impl Gpio3d7SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d7Sel {
        match self.bits {
            0 => Gpio3d7Sel::B00,
            1 => Gpio3d7Sel::B01,
            2 => Gpio3d7Sel::B10,
            3 => Gpio3d7Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d7Sel::B00
    }
    #[doc = "i2s0_sdo0"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d7Sel::B01
    }
    #[doc = "trace_data7"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d7Sel::B10
    }
    #[doc = "a53l2_wfi"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d7Sel::B11
    }
}
#[doc = "Field `GPIO3D7_SEL` writer - GPIO3D\\[7\\]
iomux select"]
pub type Gpio3d7SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d7Sel>;
impl<'a, REG> Gpio3d7SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d7Sel::B00)
    }
    #[doc = "i2s0_sdo0"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d7Sel::B01)
    }
    #[doc = "trace_data7"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d7Sel::B10)
    }
    #[doc = "a53l2_wfi"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d7Sel::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO3D\\[0\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3d0_sel(&self) -> Gpio3d0SelR {
        Gpio3d0SelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO3D\\[1\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3d1_sel(&self) -> Gpio3d1SelR {
        Gpio3d1SelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO3D\\[2\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3d2_sel(&self) -> Gpio3d2SelR {
        Gpio3d2SelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO3D\\[3\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3d3_sel(&self) -> Gpio3d3SelR {
        Gpio3d3SelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO3D\\[4\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3d4_sel(&self) -> Gpio3d4SelR {
        Gpio3d4SelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO3D\\[5\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3d5_sel(&self) -> Gpio3d5SelR {
        Gpio3d5SelR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO3D\\[6\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3d6_sel(&self) -> Gpio3d6SelR {
        Gpio3d6SelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO3D\\[7\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3d7_sel(&self) -> Gpio3d7SelR {
        Gpio3d7SelR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO3D\\[0\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d0_sel(&mut self) -> Gpio3d0SelW<GrfGpio3dIomuxSpec> {
        Gpio3d0SelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO3D\\[1\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d1_sel(&mut self) -> Gpio3d1SelW<GrfGpio3dIomuxSpec> {
        Gpio3d1SelW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO3D\\[2\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d2_sel(&mut self) -> Gpio3d2SelW<GrfGpio3dIomuxSpec> {
        Gpio3d2SelW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO3D\\[3\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d3_sel(&mut self) -> Gpio3d3SelW<GrfGpio3dIomuxSpec> {
        Gpio3d3SelW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO3D\\[4\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d4_sel(&mut self) -> Gpio3d4SelW<GrfGpio3dIomuxSpec> {
        Gpio3d4SelW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO3D\\[5\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d5_sel(&mut self) -> Gpio3d5SelW<GrfGpio3dIomuxSpec> {
        Gpio3d5SelW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO3D\\[6\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d6_sel(&mut self) -> Gpio3d6SelW<GrfGpio3dIomuxSpec> {
        Gpio3d6SelW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO3D\\[7\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d7_sel(&mut self) -> Gpio3d7SelW<GrfGpio3dIomuxSpec> {
        Gpio3d7SelW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio3dIomuxSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO3D iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3d_iomux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3d_iomux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio3dIomuxSpec;
impl crate::RegisterSpec for GrfGpio3dIomuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio3d_iomux::R`](R) reader structure"]
impl crate::Readable for GrfGpio3dIomuxSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio3d_iomux::W`](W) writer structure"]
impl crate::Writable for GrfGpio3dIomuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO3D_IOMUX to value 0"]
impl crate::Resettable for GrfGpio3dIomuxSpec {
    const RESET_VALUE: u32 = 0;
}
