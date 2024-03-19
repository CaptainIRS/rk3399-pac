#[doc = "Register `GRF_GPIO3C_P` reader"]
pub type R = crate::R<GrfGpio3cPSpec>;
#[doc = "Register `GRF_GPIO3C_P` writer"]
pub type W = crate::W<GrfGpio3cPSpec>;
#[doc = "GPIO3C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3c0P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio3c0P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3c0P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3c0P {
    type Ux = u8;
}
#[doc = "Field `GPIO3C0_P` reader - GPIO3C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3c0PR = crate::FieldReader<Gpio3c0P>;
impl Gpio3c0PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3c0P {
        match self.bits {
            0 => Gpio3c0P::B00,
            1 => Gpio3c0P::B01,
            2 => Gpio3c0P::B10,
            3 => Gpio3c0P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3c0P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3c0P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3c0P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3c0P::B11
    }
}
#[doc = "Field `GPIO3C0_P` writer - GPIO3C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3c0PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3c0P>;
impl<'a, REG> Gpio3c0PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3c0P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3c0P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3c0P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3c0P::B11)
    }
}
#[doc = "GPIO3A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3c1P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio3c1P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3c1P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3c1P {
    type Ux = u8;
}
#[doc = "Field `GPIO3C1_P` reader - GPIO3A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3c1PR = crate::FieldReader<Gpio3c1P>;
impl Gpio3c1PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3c1P {
        match self.bits {
            0 => Gpio3c1P::B00,
            1 => Gpio3c1P::B01,
            2 => Gpio3c1P::B10,
            3 => Gpio3c1P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3c1P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3c1P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3c1P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3c1P::B11
    }
}
#[doc = "Field `GPIO3C1_P` writer - GPIO3A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3c1PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3c1P>;
impl<'a, REG> Gpio3c1PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3c1P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3c1P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3c1P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3c1P::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO3C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3c0_p(&self) -> Gpio3c0PR {
        Gpio3c0PR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO3A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3c1_p(&self) -> Gpio3c1PR {
        Gpio3c1PR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO3C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3c0_p(&mut self) -> Gpio3c0PW<GrfGpio3cPSpec> {
        Gpio3c0PW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO3A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3c1_p(&mut self) -> Gpio3c1PW<GrfGpio3cPSpec> {
        Gpio3c1PW::new(self, 2)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio3cPSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO3C PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3c_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3c_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio3cPSpec;
impl crate::RegisterSpec for GrfGpio3cPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio3c_p::R`](R) reader structure"]
impl crate::Readable for GrfGpio3cPSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio3c_p::W`](W) writer structure"]
impl crate::Writable for GrfGpio3cPSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO3C_P to value 0x05"]
impl crate::Resettable for GrfGpio3cPSpec {
    const RESET_VALUE: u32 = 0x05;
}
