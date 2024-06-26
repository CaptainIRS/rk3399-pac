#[doc = "Register `GPIO0A_P` reader"]
pub type R = crate::R<Gpio0aPSpec>;
#[doc = "Register `GPIO0A_P` writer"]
pub type W = crate::W<Gpio0aPSpec>;
#[doc = "GPIO0A PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits\\[PS:PE\\]\n\nValue on reset: 56671"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Gpio0aP {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "3: weak 1(pull-up);"]
    B11 = 3,
    #[doc = "1: weak 0(pull-down);"]
    B01 = 1,
    #[doc = "2: Z(Noraml operaton);"]
    B10 = 2,
}
impl From<Gpio0aP> for u16 {
    #[inline(always)]
    fn from(variant: Gpio0aP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0aP {
    type Ux = u16;
}
#[doc = "Field `GPIO0A_P` reader - GPIO0A PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits\\[PS:PE\\]"]
pub type Gpio0aPR = crate::FieldReader<Gpio0aP>;
impl Gpio0aPR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0aP> {
        match self.bits {
            0 => Some(Gpio0aP::B00),
            3 => Some(Gpio0aP::B11),
            1 => Some(Gpio0aP::B01),
            2 => Some(Gpio0aP::B10),
            _ => None,
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio0aP::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio0aP::B11
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio0aP::B01
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio0aP::B10
    }
}
#[doc = "Field `GPIO0A_P` writer - GPIO0A PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits\\[PS:PE\\]"]
pub type Gpio0aPW<'a, REG> = crate::FieldWriter<'a, REG, 16, Gpio0aP>;
impl<'a, REG> Gpio0aPW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0aP::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0aP::B11)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0aP::B01)
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0aP::B10)
    }
}
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPIO0A PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits\\[PS:PE\\]"]
    #[inline(always)]
    pub fn gpio0a_p(&self) -> Gpio0aPR {
        Gpio0aPR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPIO0A PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits\\[PS:PE\\]"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0a_p(&mut self) -> Gpio0aPW<Gpio0aPSpec> {
        Gpio0aPW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio0aPSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO0A PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0a_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0a_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0aPSpec;
impl crate::RegisterSpec for Gpio0aPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0a_p::R`](R) reader structure"]
impl crate::Readable for Gpio0aPSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio0a_p::W`](W) writer structure"]
impl crate::Writable for Gpio0aPSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO0A_P to value 0xdd5f"]
impl crate::Resettable for Gpio0aPSpec {
    const RESET_VALUE: u32 = 0xdd5f;
}
