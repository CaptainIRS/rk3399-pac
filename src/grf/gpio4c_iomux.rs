#[doc = "Register `GPIO4C_IOMUX` reader"]
pub type R = crate::R<Gpio4cIomuxSpec>;
#[doc = "Register `GPIO4C_IOMUX` writer"]
pub type W = crate::W<Gpio4cIomuxSpec>;
#[doc = "GPIO4C\\[0\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c0Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: i2c3hdmi_sda"]
    B01 = 1,
    #[doc = "2: uart2dbgb_sin"]
    B10 = 2,
    #[doc = "3: hdmii2c_sda"]
    B11 = 3,
}
impl From<Gpio4c0Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c0Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c0Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4C0_SEL` reader - GPIO4C\\[0\\]
iomux select"]
pub type Gpio4c0SelR = crate::FieldReader<Gpio4c0Sel>;
impl Gpio4c0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4c0Sel {
        match self.bits {
            0 => Gpio4c0Sel::B00,
            1 => Gpio4c0Sel::B01,
            2 => Gpio4c0Sel::B10,
            3 => Gpio4c0Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c0Sel::B00
    }
    #[doc = "i2c3hdmi_sda"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c0Sel::B01
    }
    #[doc = "uart2dbgb_sin"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4c0Sel::B10
    }
    #[doc = "hdmii2c_sda"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4c0Sel::B11
    }
}
#[doc = "Field `GPIO4C0_SEL` writer - GPIO4C\\[0\\]
iomux select"]
pub type Gpio4c0SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4c0Sel>;
impl<'a, REG> Gpio4c0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c0Sel::B00)
    }
    #[doc = "i2c3hdmi_sda"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c0Sel::B01)
    }
    #[doc = "uart2dbgb_sin"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c0Sel::B10)
    }
    #[doc = "hdmii2c_sda"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c0Sel::B11)
    }
}
#[doc = "GPIO4C\\[1\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c1Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: i2c3hdmi_scl"]
    B01 = 1,
    #[doc = "2: uart2dbgb_sout"]
    B10 = 2,
    #[doc = "3: hdmii2c_scl"]
    B11 = 3,
}
impl From<Gpio4c1Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c1Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c1Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4C1_SEL` reader - GPIO4C\\[1\\]
iomux select"]
pub type Gpio4c1SelR = crate::FieldReader<Gpio4c1Sel>;
impl Gpio4c1SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4c1Sel {
        match self.bits {
            0 => Gpio4c1Sel::B00,
            1 => Gpio4c1Sel::B01,
            2 => Gpio4c1Sel::B10,
            3 => Gpio4c1Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c1Sel::B00
    }
    #[doc = "i2c3hdmi_scl"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c1Sel::B01
    }
    #[doc = "uart2dbgb_sout"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4c1Sel::B10
    }
    #[doc = "hdmii2c_scl"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4c1Sel::B11
    }
}
#[doc = "Field `GPIO4C1_SEL` writer - GPIO4C\\[1\\]
iomux select"]
pub type Gpio4c1SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4c1Sel>;
impl<'a, REG> Gpio4c1SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c1Sel::B00)
    }
    #[doc = "i2c3hdmi_scl"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c1Sel::B01)
    }
    #[doc = "uart2dbgb_sout"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c1Sel::B10)
    }
    #[doc = "hdmii2c_scl"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c1Sel::B11)
    }
}
#[doc = "GPIO4C\\[2\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c2Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: pwm_0"]
    B01 = 1,
    #[doc = "2: vop0_pwm"]
    B10 = 2,
    #[doc = "3: vop1_pwm"]
    B11 = 3,
}
impl From<Gpio4c2Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c2Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c2Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4C2_SEL` reader - GPIO4C\\[2\\]
iomux select"]
pub type Gpio4c2SelR = crate::FieldReader<Gpio4c2Sel>;
impl Gpio4c2SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4c2Sel {
        match self.bits {
            0 => Gpio4c2Sel::B00,
            1 => Gpio4c2Sel::B01,
            2 => Gpio4c2Sel::B10,
            3 => Gpio4c2Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c2Sel::B00
    }
    #[doc = "pwm_0"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c2Sel::B01
    }
    #[doc = "vop0_pwm"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4c2Sel::B10
    }
    #[doc = "vop1_pwm"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4c2Sel::B11
    }
}
#[doc = "Field `GPIO4C2_SEL` writer - GPIO4C\\[2\\]
iomux select"]
pub type Gpio4c2SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4c2Sel>;
impl<'a, REG> Gpio4c2SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c2Sel::B00)
    }
    #[doc = "pwm_0"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c2Sel::B01)
    }
    #[doc = "vop0_pwm"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c2Sel::B10)
    }
    #[doc = "vop1_pwm"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c2Sel::B11)
    }
}
#[doc = "GPIO4C\\[3\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c3Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: uart2dbgc_sin"]
    B01 = 1,
    #[doc = "2: uarthdcp_sin"]
    B10 = 2,
}
impl From<Gpio4c3Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c3Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c3Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4C3_SEL` reader - GPIO4C\\[3\\]
iomux select"]
pub type Gpio4c3SelR = crate::FieldReader<Gpio4c3Sel>;
impl Gpio4c3SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio4c3Sel> {
        match self.bits {
            0 => Some(Gpio4c3Sel::B00),
            1 => Some(Gpio4c3Sel::B01),
            2 => Some(Gpio4c3Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c3Sel::B00
    }
    #[doc = "uart2dbgc_sin"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c3Sel::B01
    }
    #[doc = "uarthdcp_sin"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4c3Sel::B10
    }
}
#[doc = "Field `GPIO4C3_SEL` writer - GPIO4C\\[3\\]
iomux select"]
pub type Gpio4c3SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio4c3Sel>;
impl<'a, REG> Gpio4c3SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c3Sel::B00)
    }
    #[doc = "uart2dbgc_sin"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c3Sel::B01)
    }
    #[doc = "uarthdcp_sin"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c3Sel::B10)
    }
}
#[doc = "GPIO4C\\[4\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c4Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: uart2dbgc_sout"]
    B01 = 1,
    #[doc = "2: uarthdcp_sout"]
    B10 = 2,
}
impl From<Gpio4c4Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c4Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c4Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4C4_SEL` reader - GPIO4C\\[4\\]
iomux select"]
pub type Gpio4c4SelR = crate::FieldReader<Gpio4c4Sel>;
impl Gpio4c4SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio4c4Sel> {
        match self.bits {
            0 => Some(Gpio4c4Sel::B00),
            1 => Some(Gpio4c4Sel::B01),
            2 => Some(Gpio4c4Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c4Sel::B00
    }
    #[doc = "uart2dbgc_sout"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c4Sel::B01
    }
    #[doc = "uarthdcp_sout"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4c4Sel::B10
    }
}
#[doc = "Field `GPIO4C4_SEL` writer - GPIO4C\\[4\\]
iomux select"]
pub type Gpio4c4SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio4c4Sel>;
impl<'a, REG> Gpio4c4SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c4Sel::B00)
    }
    #[doc = "uart2dbgc_sout"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c4Sel::B01)
    }
    #[doc = "uarthdcp_sout"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c4Sel::B10)
    }
}
#[doc = "GPIO4C\\[5\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c5Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: spdif_tx"]
    B01 = 1,
}
impl From<Gpio4c5Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c5Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c5Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4C5_SEL` reader - GPIO4C\\[5\\]
iomux select"]
pub type Gpio4c5SelR = crate::FieldReader<Gpio4c5Sel>;
impl Gpio4c5SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio4c5Sel> {
        match self.bits {
            0 => Some(Gpio4c5Sel::B00),
            1 => Some(Gpio4c5Sel::B01),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c5Sel::B00
    }
    #[doc = "spdif_tx"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c5Sel::B01
    }
}
#[doc = "Field `GPIO4C5_SEL` writer - GPIO4C\\[5\\]
iomux select"]
pub type Gpio4c5SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio4c5Sel>;
impl<'a, REG> Gpio4c5SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c5Sel::B00)
    }
    #[doc = "spdif_tx"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c5Sel::B01)
    }
}
#[doc = "GPIO4C\\[6\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c6Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: pwm_1"]
    B01 = 1,
}
impl From<Gpio4c6Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c6Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c6Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4C6_SEL` reader - GPIO4C\\[6\\]
iomux select"]
pub type Gpio4c6SelR = crate::FieldReader<Gpio4c6Sel>;
impl Gpio4c6SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio4c6Sel> {
        match self.bits {
            0 => Some(Gpio4c6Sel::B00),
            1 => Some(Gpio4c6Sel::B01),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c6Sel::B00
    }
    #[doc = "pwm_1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c6Sel::B01
    }
}
#[doc = "Field `GPIO4C6_SEL` writer - GPIO4C\\[6\\]
iomux select"]
pub type Gpio4c6SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio4c6Sel>;
impl<'a, REG> Gpio4c6SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c6Sel::B00)
    }
    #[doc = "pwm_1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c6Sel::B01)
    }
}
#[doc = "GPIO4C\\[7\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c7Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: hdmi_cecinout"]
    B01 = 1,
    #[doc = "2: edp_hotplug"]
    B10 = 2,
}
impl From<Gpio4c7Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c7Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c7Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4C7_SEL` reader - GPIO4C\\[7\\]
iomux select"]
pub type Gpio4c7SelR = crate::FieldReader<Gpio4c7Sel>;
impl Gpio4c7SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio4c7Sel> {
        match self.bits {
            0 => Some(Gpio4c7Sel::B00),
            1 => Some(Gpio4c7Sel::B01),
            2 => Some(Gpio4c7Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c7Sel::B00
    }
    #[doc = "hdmi_cecinout"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c7Sel::B01
    }
    #[doc = "edp_hotplug"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4c7Sel::B10
    }
}
#[doc = "Field `GPIO4C7_SEL` writer - GPIO4C\\[7\\]
iomux select"]
pub type Gpio4c7SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio4c7Sel>;
impl<'a, REG> Gpio4c7SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c7Sel::B00)
    }
    #[doc = "hdmi_cecinout"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c7Sel::B01)
    }
    #[doc = "edp_hotplug"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c7Sel::B10)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO4C\\[0\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4c0_sel(&self) -> Gpio4c0SelR {
        Gpio4c0SelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO4C\\[1\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4c1_sel(&self) -> Gpio4c1SelR {
        Gpio4c1SelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO4C\\[2\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4c2_sel(&self) -> Gpio4c2SelR {
        Gpio4c2SelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO4C\\[3\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4c3_sel(&self) -> Gpio4c3SelR {
        Gpio4c3SelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO4C\\[4\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4c4_sel(&self) -> Gpio4c4SelR {
        Gpio4c4SelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO4C\\[5\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4c5_sel(&self) -> Gpio4c5SelR {
        Gpio4c5SelR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO4C\\[6\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4c6_sel(&self) -> Gpio4c6SelR {
        Gpio4c6SelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO4C\\[7\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4c7_sel(&self) -> Gpio4c7SelR {
        Gpio4c7SelR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO4C\\[0\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c0_sel(&mut self) -> Gpio4c0SelW<Gpio4cIomuxSpec> {
        Gpio4c0SelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO4C\\[1\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c1_sel(&mut self) -> Gpio4c1SelW<Gpio4cIomuxSpec> {
        Gpio4c1SelW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO4C\\[2\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c2_sel(&mut self) -> Gpio4c2SelW<Gpio4cIomuxSpec> {
        Gpio4c2SelW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO4C\\[3\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c3_sel(&mut self) -> Gpio4c3SelW<Gpio4cIomuxSpec> {
        Gpio4c3SelW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO4C\\[4\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c4_sel(&mut self) -> Gpio4c4SelW<Gpio4cIomuxSpec> {
        Gpio4c4SelW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO4C\\[5\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c5_sel(&mut self) -> Gpio4c5SelW<Gpio4cIomuxSpec> {
        Gpio4c5SelW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO4C\\[6\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c6_sel(&mut self) -> Gpio4c6SelW<Gpio4cIomuxSpec> {
        Gpio4c6SelW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO4C\\[7\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c7_sel(&mut self) -> Gpio4c7SelW<Gpio4cIomuxSpec> {
        Gpio4c7SelW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio4cIomuxSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO4C iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4c_iomux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4c_iomux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio4cIomuxSpec;
impl crate::RegisterSpec for Gpio4cIomuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio4c_iomux::R`](R) reader structure"]
impl crate::Readable for Gpio4cIomuxSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio4c_iomux::W`](W) writer structure"]
impl crate::Writable for Gpio4cIomuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO4C_IOMUX to value 0"]
impl crate::Resettable for Gpio4cIomuxSpec {
    const RESET_VALUE: u32 = 0;
}
