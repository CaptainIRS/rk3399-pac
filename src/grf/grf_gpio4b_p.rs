#[doc = "Register `GRF_GPIO4B_P` reader"]
pub type R = crate::R<GrfGpio4bPSpec>;
#[doc = "Register `GRF_GPIO4B_P` writer"]
pub type W = crate::W<GrfGpio4bPSpec>;
#[doc = "GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4b0P {
    #[doc = "0: Repeater(Bus keeper)"]
    B00 = 0,
    #[doc = "1: Repeater(Bus keeper)"]
    B01 = 1,
    #[doc = "2: Repeater(Bus keeper)"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio4b0P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4b0P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4b0P {
    type Ux = u8;
}
#[doc = "Field `GPIO4B0_P` reader - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio4b0PR = crate::FieldReader<Gpio4b0P>;
impl Gpio4b0PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4b0P {
        match self.bits {
            0 => Gpio4b0P::B00,
            1 => Gpio4b0P::B01,
            2 => Gpio4b0P::B10,
            3 => Gpio4b0P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4b0P::B00
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4b0P::B01
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4b0P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4b0P::B11
    }
}
#[doc = "Field `GPIO4B0_P` writer - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio4b0PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4b0P>;
impl<'a, REG> Gpio4b0PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b0P::B00)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b0P::B01)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b0P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b0P::B11)
    }
}
#[doc = "GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4b1P {
    #[doc = "0: Repeater(Bus keeper)"]
    B00 = 0,
    #[doc = "1: Repeater(Bus keeper)"]
    B01 = 1,
    #[doc = "2: Repeater(Bus keeper)"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio4b1P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4b1P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4b1P {
    type Ux = u8;
}
#[doc = "Field `GPIO4B1_P` reader - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio4b1PR = crate::FieldReader<Gpio4b1P>;
impl Gpio4b1PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4b1P {
        match self.bits {
            0 => Gpio4b1P::B00,
            1 => Gpio4b1P::B01,
            2 => Gpio4b1P::B10,
            3 => Gpio4b1P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4b1P::B00
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4b1P::B01
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4b1P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4b1P::B11
    }
}
#[doc = "Field `GPIO4B1_P` writer - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio4b1PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4b1P>;
impl<'a, REG> Gpio4b1PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b1P::B00)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b1P::B01)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b1P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b1P::B11)
    }
}
#[doc = "GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4b2P {
    #[doc = "0: Repeater(Bus keeper)"]
    B00 = 0,
    #[doc = "1: Repeater(Bus keeper)"]
    B01 = 1,
    #[doc = "2: Repeater(Bus keeper)"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio4b2P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4b2P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4b2P {
    type Ux = u8;
}
#[doc = "Field `GPIO4B2_P` reader - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio4b2PR = crate::FieldReader<Gpio4b2P>;
impl Gpio4b2PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4b2P {
        match self.bits {
            0 => Gpio4b2P::B00,
            1 => Gpio4b2P::B01,
            2 => Gpio4b2P::B10,
            3 => Gpio4b2P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4b2P::B00
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4b2P::B01
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4b2P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4b2P::B11
    }
}
#[doc = "Field `GPIO4B2_P` writer - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio4b2PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4b2P>;
impl<'a, REG> Gpio4b2PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b2P::B00)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b2P::B01)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b2P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b2P::B11)
    }
}
#[doc = "GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4b3P {
    #[doc = "0: Repeater(Bus keeper)"]
    B00 = 0,
    #[doc = "1: Repeater(Bus keeper)"]
    B01 = 1,
    #[doc = "2: Repeater(Bus keeper)"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio4b3P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4b3P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4b3P {
    type Ux = u8;
}
#[doc = "Field `GPIO4B3_P` reader - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio4b3PR = crate::FieldReader<Gpio4b3P>;
impl Gpio4b3PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4b3P {
        match self.bits {
            0 => Gpio4b3P::B00,
            1 => Gpio4b3P::B01,
            2 => Gpio4b3P::B10,
            3 => Gpio4b3P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4b3P::B00
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4b3P::B01
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4b3P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4b3P::B11
    }
}
#[doc = "Field `GPIO4B3_P` writer - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio4b3PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4b3P>;
impl<'a, REG> Gpio4b3PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b3P::B00)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b3P::B01)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b3P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b3P::B11)
    }
}
#[doc = "GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4b4P {
    #[doc = "0: Repeater(Bus keeper)"]
    B00 = 0,
    #[doc = "1: Repeater(Bus keeper)"]
    B01 = 1,
    #[doc = "2: Repeater(Bus keeper)"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio4b4P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4b4P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4b4P {
    type Ux = u8;
}
#[doc = "Field `GPIO4B4_P` reader - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio4b4PR = crate::FieldReader<Gpio4b4P>;
impl Gpio4b4PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4b4P {
        match self.bits {
            0 => Gpio4b4P::B00,
            1 => Gpio4b4P::B01,
            2 => Gpio4b4P::B10,
            3 => Gpio4b4P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4b4P::B00
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4b4P::B01
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4b4P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4b4P::B11
    }
}
#[doc = "Field `GPIO4B4_P` writer - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio4b4PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4b4P>;
impl<'a, REG> Gpio4b4PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b4P::B00)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b4P::B01)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b4P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b4P::B11)
    }
}
#[doc = "GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4b5P {
    #[doc = "0: Repeater(Bus keeper)"]
    B00 = 0,
    #[doc = "1: Repeater(Bus keeper)"]
    B01 = 1,
    #[doc = "2: Repeater(Bus keeper)"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio4b5P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4b5P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4b5P {
    type Ux = u8;
}
#[doc = "Field `GPIO4B5_P` reader - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio4b5PR = crate::FieldReader<Gpio4b5P>;
impl Gpio4b5PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4b5P {
        match self.bits {
            0 => Gpio4b5P::B00,
            1 => Gpio4b5P::B01,
            2 => Gpio4b5P::B10,
            3 => Gpio4b5P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4b5P::B00
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4b5P::B01
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4b5P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4b5P::B11
    }
}
#[doc = "Field `GPIO4B5_P` writer - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio4b5PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4b5P>;
impl<'a, REG> Gpio4b5PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b5P::B00)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b5P::B01)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b5P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4b5P::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4b0_p(&self) -> Gpio4b0PR {
        Gpio4b0PR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4b1_p(&self) -> Gpio4b1PR {
        Gpio4b1PR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4b2_p(&self) -> Gpio4b2PR {
        Gpio4b2PR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4b3_p(&self) -> Gpio4b3PR {
        Gpio4b3PR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4b4_p(&self) -> Gpio4b4PR {
        Gpio4b4PR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4b5_p(&self) -> Gpio4b5PR {
        Gpio4b5PR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4b0_p(&mut self) -> Gpio4b0PW<GrfGpio4bPSpec> {
        Gpio4b0PW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4b1_p(&mut self) -> Gpio4b1PW<GrfGpio4bPSpec> {
        Gpio4b1PW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4b2_p(&mut self) -> Gpio4b2PW<GrfGpio4bPSpec> {
        Gpio4b2PW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4b3_p(&mut self) -> Gpio4b3PW<GrfGpio4bPSpec> {
        Gpio4b3PW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4b4_p(&mut self) -> Gpio4b4PW<GrfGpio4bPSpec> {
        Gpio4b4PW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO4B PU/PD programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4b5_p(&mut self) -> Gpio4b5PW<GrfGpio4bPSpec> {
        Gpio4b5PW::new(self, 10)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio4bPSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO4B PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4b_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4b_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio4bPSpec;
impl crate::RegisterSpec for GrfGpio4bPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio4b_p::R`](R) reader structure"]
impl crate::Readable for GrfGpio4bPSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio4b_p::W`](W) writer structure"]
impl crate::Writable for GrfGpio4bPSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO4B_P to value 0x0655"]
impl crate::Resettable for GrfGpio4bPSpec {
    const RESET_VALUE: u32 = 0x0655;
}
