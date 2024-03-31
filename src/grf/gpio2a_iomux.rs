#[doc = "Register `GPIO2A_IOMUX` reader"]
pub type R = crate::R<Gpio2aIomuxSpec>;
#[doc = "Register `GPIO2A_IOMUX` writer"]
pub type W = crate::W<Gpio2aIomuxSpec>;
#[doc = "GPIO2A\\[0\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a0Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: vop_data0"]
    B01 = 1,
    #[doc = "2: i2c2tp_sda"]
    B10 = 2,
    #[doc = "3: cif_data0"]
    B11 = 3,
}
impl From<Gpio2a0Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a0Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a0Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2A0_SEL` reader - GPIO2A\\[0\\]
iomux select"]
pub type Gpio2a0SelR = crate::FieldReader<Gpio2a0Sel>;
impl Gpio2a0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a0Sel {
        match self.bits {
            0 => Gpio2a0Sel::B00,
            1 => Gpio2a0Sel::B01,
            2 => Gpio2a0Sel::B10,
            3 => Gpio2a0Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a0Sel::B00
    }
    #[doc = "vop_data0"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a0Sel::B01
    }
    #[doc = "i2c2tp_sda"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a0Sel::B10
    }
    #[doc = "cif_data0"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a0Sel::B11
    }
}
#[doc = "Field `GPIO2A0_SEL` writer - GPIO2A\\[0\\]
iomux select"]
pub type Gpio2a0SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a0Sel>;
impl<'a, REG> Gpio2a0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a0Sel::B00)
    }
    #[doc = "vop_data0"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a0Sel::B01)
    }
    #[doc = "i2c2tp_sda"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a0Sel::B10)
    }
    #[doc = "cif_data0"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a0Sel::B11)
    }
}
#[doc = "GPIO2A\\[1\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a1Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: vop_data1"]
    B01 = 1,
    #[doc = "2: i2c2tp_scl"]
    B10 = 2,
    #[doc = "3: cif_data1"]
    B11 = 3,
}
impl From<Gpio2a1Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a1Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a1Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2A1_SEL` reader - GPIO2A\\[1\\]
iomux select"]
pub type Gpio2a1SelR = crate::FieldReader<Gpio2a1Sel>;
impl Gpio2a1SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a1Sel {
        match self.bits {
            0 => Gpio2a1Sel::B00,
            1 => Gpio2a1Sel::B01,
            2 => Gpio2a1Sel::B10,
            3 => Gpio2a1Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a1Sel::B00
    }
    #[doc = "vop_data1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a1Sel::B01
    }
    #[doc = "i2c2tp_scl"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a1Sel::B10
    }
    #[doc = "cif_data1"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a1Sel::B11
    }
}
#[doc = "Field `GPIO2A1_SEL` writer - GPIO2A\\[1\\]
iomux select"]
pub type Gpio2a1SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a1Sel>;
impl<'a, REG> Gpio2a1SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a1Sel::B00)
    }
    #[doc = "vop_data1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a1Sel::B01)
    }
    #[doc = "i2c2tp_scl"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a1Sel::B10)
    }
    #[doc = "cif_data1"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a1Sel::B11)
    }
}
#[doc = "GPIO2A\\[2\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a2Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: vop_data2"]
    B01 = 1,
    #[doc = "2: uphyjtag_trstn"]
    B10 = 2,
    #[doc = "3: cif_data2"]
    B11 = 3,
}
impl From<Gpio2a2Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a2Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a2Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2A2_SEL` reader - GPIO2A\\[2\\]
iomux select"]
pub type Gpio2a2SelR = crate::FieldReader<Gpio2a2Sel>;
impl Gpio2a2SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a2Sel {
        match self.bits {
            0 => Gpio2a2Sel::B00,
            1 => Gpio2a2Sel::B01,
            2 => Gpio2a2Sel::B10,
            3 => Gpio2a2Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a2Sel::B00
    }
    #[doc = "vop_data2"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a2Sel::B01
    }
    #[doc = "uphyjtag_trstn"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a2Sel::B10
    }
    #[doc = "cif_data2"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a2Sel::B11
    }
}
#[doc = "Field `GPIO2A2_SEL` writer - GPIO2A\\[2\\]
iomux select"]
pub type Gpio2a2SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a2Sel>;
impl<'a, REG> Gpio2a2SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a2Sel::B00)
    }
    #[doc = "vop_data2"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a2Sel::B01)
    }
    #[doc = "uphyjtag_trstn"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a2Sel::B10)
    }
    #[doc = "cif_data2"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a2Sel::B11)
    }
}
#[doc = "GPIO2A\\[3\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a3Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: vop_data3"]
    B01 = 1,
    #[doc = "2: uphyjtag_tdi"]
    B10 = 2,
    #[doc = "3: cif_data3"]
    B11 = 3,
}
impl From<Gpio2a3Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a3Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a3Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2A3_SEL` reader - GPIO2A\\[3\\]
iomux select"]
pub type Gpio2a3SelR = crate::FieldReader<Gpio2a3Sel>;
impl Gpio2a3SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a3Sel {
        match self.bits {
            0 => Gpio2a3Sel::B00,
            1 => Gpio2a3Sel::B01,
            2 => Gpio2a3Sel::B10,
            3 => Gpio2a3Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a3Sel::B00
    }
    #[doc = "vop_data3"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a3Sel::B01
    }
    #[doc = "uphyjtag_tdi"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a3Sel::B10
    }
    #[doc = "cif_data3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a3Sel::B11
    }
}
#[doc = "Field `GPIO2A3_SEL` writer - GPIO2A\\[3\\]
iomux select"]
pub type Gpio2a3SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a3Sel>;
impl<'a, REG> Gpio2a3SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a3Sel::B00)
    }
    #[doc = "vop_data3"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a3Sel::B01)
    }
    #[doc = "uphyjtag_tdi"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a3Sel::B10)
    }
    #[doc = "cif_data3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a3Sel::B11)
    }
}
#[doc = "GPIO2A\\[4\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a4Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: vop_data4"]
    B01 = 1,
    #[doc = "2: uphyjtag_tdo"]
    B10 = 2,
    #[doc = "3: cif_data4"]
    B11 = 3,
}
impl From<Gpio2a4Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a4Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a4Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2A4_SEL` reader - GPIO2A\\[4\\]
iomux select"]
pub type Gpio2a4SelR = crate::FieldReader<Gpio2a4Sel>;
impl Gpio2a4SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a4Sel {
        match self.bits {
            0 => Gpio2a4Sel::B00,
            1 => Gpio2a4Sel::B01,
            2 => Gpio2a4Sel::B10,
            3 => Gpio2a4Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a4Sel::B00
    }
    #[doc = "vop_data4"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a4Sel::B01
    }
    #[doc = "uphyjtag_tdo"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a4Sel::B10
    }
    #[doc = "cif_data4"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a4Sel::B11
    }
}
#[doc = "Field `GPIO2A4_SEL` writer - GPIO2A\\[4\\]
iomux select"]
pub type Gpio2a4SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a4Sel>;
impl<'a, REG> Gpio2a4SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a4Sel::B00)
    }
    #[doc = "vop_data4"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a4Sel::B01)
    }
    #[doc = "uphyjtag_tdo"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a4Sel::B10)
    }
    #[doc = "cif_data4"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a4Sel::B11)
    }
}
#[doc = "GPIO2A\\[5\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a5Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: vop_data5"]
    B01 = 1,
    #[doc = "2: uphyjtag_tck"]
    B10 = 2,
    #[doc = "3: cif_data5"]
    B11 = 3,
}
impl From<Gpio2a5Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a5Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a5Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2A5_SEL` reader - GPIO2A\\[5\\]
iomux select"]
pub type Gpio2a5SelR = crate::FieldReader<Gpio2a5Sel>;
impl Gpio2a5SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a5Sel {
        match self.bits {
            0 => Gpio2a5Sel::B00,
            1 => Gpio2a5Sel::B01,
            2 => Gpio2a5Sel::B10,
            3 => Gpio2a5Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a5Sel::B00
    }
    #[doc = "vop_data5"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a5Sel::B01
    }
    #[doc = "uphyjtag_tck"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a5Sel::B10
    }
    #[doc = "cif_data5"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a5Sel::B11
    }
}
#[doc = "Field `GPIO2A5_SEL` writer - GPIO2A\\[5\\]
iomux select"]
pub type Gpio2a5SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a5Sel>;
impl<'a, REG> Gpio2a5SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a5Sel::B00)
    }
    #[doc = "vop_data5"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a5Sel::B01)
    }
    #[doc = "uphyjtag_tck"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a5Sel::B10)
    }
    #[doc = "cif_data5"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a5Sel::B11)
    }
}
#[doc = "GPIO2A\\[6\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a6Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: vop_data6"]
    B01 = 1,
    #[doc = "2: uphyjtag_tms"]
    B10 = 2,
    #[doc = "3: cif_data6"]
    B11 = 3,
}
impl From<Gpio2a6Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a6Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a6Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2A6_SEL` reader - GPIO2A\\[6\\]
iomux select"]
pub type Gpio2a6SelR = crate::FieldReader<Gpio2a6Sel>;
impl Gpio2a6SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a6Sel {
        match self.bits {
            0 => Gpio2a6Sel::B00,
            1 => Gpio2a6Sel::B01,
            2 => Gpio2a6Sel::B10,
            3 => Gpio2a6Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a6Sel::B00
    }
    #[doc = "vop_data6"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a6Sel::B01
    }
    #[doc = "uphyjtag_tms"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a6Sel::B10
    }
    #[doc = "cif_data6"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a6Sel::B11
    }
}
#[doc = "Field `GPIO2A6_SEL` writer - GPIO2A\\[6\\]
iomux select"]
pub type Gpio2a6SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a6Sel>;
impl<'a, REG> Gpio2a6SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a6Sel::B00)
    }
    #[doc = "vop_data6"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a6Sel::B01)
    }
    #[doc = "uphyjtag_tms"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a6Sel::B10)
    }
    #[doc = "cif_data6"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a6Sel::B11)
    }
}
#[doc = "GPIO2A\\[7\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a7Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: vop_data7"]
    B01 = 1,
    #[doc = "2: i2c7nfc_sda"]
    B10 = 2,
    #[doc = "3: cif_data7"]
    B11 = 3,
}
impl From<Gpio2a7Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a7Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a7Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO2A7_SEL` reader - GPIO2A\\[7\\]
iomux select"]
pub type Gpio2a7SelR = crate::FieldReader<Gpio2a7Sel>;
impl Gpio2a7SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a7Sel {
        match self.bits {
            0 => Gpio2a7Sel::B00,
            1 => Gpio2a7Sel::B01,
            2 => Gpio2a7Sel::B10,
            3 => Gpio2a7Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a7Sel::B00
    }
    #[doc = "vop_data7"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a7Sel::B01
    }
    #[doc = "i2c7nfc_sda"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a7Sel::B10
    }
    #[doc = "cif_data7"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a7Sel::B11
    }
}
#[doc = "Field `GPIO2A7_SEL` writer - GPIO2A\\[7\\]
iomux select"]
pub type Gpio2a7SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a7Sel>;
impl<'a, REG> Gpio2a7SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a7Sel::B00)
    }
    #[doc = "vop_data7"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a7Sel::B01)
    }
    #[doc = "i2c7nfc_sda"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a7Sel::B10)
    }
    #[doc = "cif_data7"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a7Sel::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO2A\\[0\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2a0_sel(&self) -> Gpio2a0SelR {
        Gpio2a0SelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO2A\\[1\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2a1_sel(&self) -> Gpio2a1SelR {
        Gpio2a1SelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO2A\\[2\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2a2_sel(&self) -> Gpio2a2SelR {
        Gpio2a2SelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO2A\\[3\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2a3_sel(&self) -> Gpio2a3SelR {
        Gpio2a3SelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO2A\\[4\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2a4_sel(&self) -> Gpio2a4SelR {
        Gpio2a4SelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO2A\\[5\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2a5_sel(&self) -> Gpio2a5SelR {
        Gpio2a5SelR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO2A\\[6\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2a6_sel(&self) -> Gpio2a6SelR {
        Gpio2a6SelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO2A\\[7\\]
iomux select"]
    #[inline(always)]
    pub fn gpio2a7_sel(&self) -> Gpio2a7SelR {
        Gpio2a7SelR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO2A\\[0\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a0_sel(&mut self) -> Gpio2a0SelW<Gpio2aIomuxSpec> {
        Gpio2a0SelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO2A\\[1\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a1_sel(&mut self) -> Gpio2a1SelW<Gpio2aIomuxSpec> {
        Gpio2a1SelW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO2A\\[2\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a2_sel(&mut self) -> Gpio2a2SelW<Gpio2aIomuxSpec> {
        Gpio2a2SelW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO2A\\[3\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a3_sel(&mut self) -> Gpio2a3SelW<Gpio2aIomuxSpec> {
        Gpio2a3SelW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO2A\\[4\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a4_sel(&mut self) -> Gpio2a4SelW<Gpio2aIomuxSpec> {
        Gpio2a4SelW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO2A\\[5\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a5_sel(&mut self) -> Gpio2a5SelW<Gpio2aIomuxSpec> {
        Gpio2a5SelW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO2A\\[6\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a6_sel(&mut self) -> Gpio2a6SelW<Gpio2aIomuxSpec> {
        Gpio2a6SelW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO2A\\[7\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a7_sel(&mut self) -> Gpio2a7SelW<Gpio2aIomuxSpec> {
        Gpio2a7SelW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio2aIomuxSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO2A iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2a_iomux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2a_iomux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio2aIomuxSpec;
impl crate::RegisterSpec for Gpio2aIomuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio2a_iomux::R`](R) reader structure"]
impl crate::Readable for Gpio2aIomuxSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio2a_iomux::W`](W) writer structure"]
impl crate::Writable for Gpio2aIomuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO2A_IOMUX to value 0"]
impl crate::Resettable for Gpio2aIomuxSpec {
    const RESET_VALUE: u32 = 0;
}
