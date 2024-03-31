#[doc = "Register `GPIO2B_P` reader"]
pub type R = crate::R<Gpio2bPSpec>;
#[doc = "Register `GPIO2B_P` writer"]
pub type W = crate::W<Gpio2bPSpec>;
#[doc = "GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2b0P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio2b0P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2b0P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2b0P {
    type Ux = u8;
}
#[doc = "Field `GPIO2B0_P` reader - GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2b0PR = crate::FieldReader<Gpio2b0P>;
impl Gpio2b0PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2b0P {
        match self.bits {
            0 => Gpio2b0P::B00,
            1 => Gpio2b0P::B01,
            2 => Gpio2b0P::B10,
            3 => Gpio2b0P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2b0P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2b0P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2b0P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2b0P::B11
    }
}
#[doc = "Field `GPIO2B0_P` writer - GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2b0PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2b0P>;
impl<'a, REG> Gpio2b0PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b0P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b0P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b0P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b0P::B11)
    }
}
#[doc = "GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2b1P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio2b1P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2b1P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2b1P {
    type Ux = u8;
}
#[doc = "Field `GPIO2B1_P` reader - GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2b1PR = crate::FieldReader<Gpio2b1P>;
impl Gpio2b1PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2b1P {
        match self.bits {
            0 => Gpio2b1P::B00,
            1 => Gpio2b1P::B01,
            2 => Gpio2b1P::B10,
            3 => Gpio2b1P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2b1P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2b1P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2b1P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2b1P::B11
    }
}
#[doc = "Field `GPIO2B1_P` writer - GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2b1PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2b1P>;
impl<'a, REG> Gpio2b1PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b1P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b1P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b1P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b1P::B11)
    }
}
#[doc = "GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2b2P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio2b2P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2b2P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2b2P {
    type Ux = u8;
}
#[doc = "Field `GPIO2B2_P` reader - GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2b2PR = crate::FieldReader<Gpio2b2P>;
impl Gpio2b2PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2b2P {
        match self.bits {
            0 => Gpio2b2P::B00,
            1 => Gpio2b2P::B01,
            2 => Gpio2b2P::B10,
            3 => Gpio2b2P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2b2P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2b2P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2b2P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2b2P::B11
    }
}
#[doc = "Field `GPIO2B2_P` writer - GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2b2PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2b2P>;
impl<'a, REG> Gpio2b2PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b2P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b2P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b2P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b2P::B11)
    }
}
#[doc = "GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2b3P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio2b3P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2b3P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2b3P {
    type Ux = u8;
}
#[doc = "Field `GPIO2B3_P` reader - GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2b3PR = crate::FieldReader<Gpio2b3P>;
impl Gpio2b3PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2b3P {
        match self.bits {
            0 => Gpio2b3P::B00,
            1 => Gpio2b3P::B01,
            2 => Gpio2b3P::B10,
            3 => Gpio2b3P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2b3P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2b3P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2b3P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2b3P::B11
    }
}
#[doc = "Field `GPIO2B3_P` writer - GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2b3PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2b3P>;
impl<'a, REG> Gpio2b3PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b3P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b3P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b3P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b3P::B11)
    }
}
#[doc = "GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2b4P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio2b4P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2b4P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2b4P {
    type Ux = u8;
}
#[doc = "Field `GPIO2B4_P` reader - GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2b4PR = crate::FieldReader<Gpio2b4P>;
impl Gpio2b4PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2b4P {
        match self.bits {
            0 => Gpio2b4P::B00,
            1 => Gpio2b4P::B01,
            2 => Gpio2b4P::B10,
            3 => Gpio2b4P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2b4P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2b4P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2b4P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2b4P::B11
    }
}
#[doc = "Field `GPIO2B4_P` writer - GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2b4PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2b4P>;
impl<'a, REG> Gpio2b4PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b4P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b4P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b4P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b4P::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2b0_p(&self) -> Gpio2b0PR {
        Gpio2b0PR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2b1_p(&self) -> Gpio2b1PR {
        Gpio2b1PR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2b2_p(&self) -> Gpio2b2PR {
        Gpio2b2PR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2b3_p(&self) -> Gpio2b3PR {
        Gpio2b3PR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2b4_p(&self) -> Gpio2b4PR {
        Gpio2b4PR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2b0_p(&mut self) -> Gpio2b0PW<Gpio2bPSpec> {
        Gpio2b0PW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2b1_p(&mut self) -> Gpio2b1PW<Gpio2bPSpec> {
        Gpio2b1PW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2b2_p(&mut self) -> Gpio2b2PW<Gpio2bPSpec> {
        Gpio2b2PW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2b3_p(&mut self) -> Gpio2b3PW<Gpio2bPSpec> {
        Gpio2b3PW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO2B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2b4_p(&mut self) -> Gpio2b4PW<Gpio2bPSpec> {
        Gpio2b4PW::new(self, 8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio2bPSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO2B PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2b_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2b_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio2bPSpec;
impl crate::RegisterSpec for Gpio2bPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio2b_p::R`](R) reader structure"]
impl crate::Readable for Gpio2bPSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio2b_p::W`](W) writer structure"]
impl crate::Writable for Gpio2bPSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO2B_P to value 0x0155"]
impl crate::Resettable for Gpio2bPSpec {
    const RESET_VALUE: u32 = 0x0155;
}
