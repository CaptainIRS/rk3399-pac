#[doc = "Register `GPIO0B_P` reader"]
pub type R = crate::R<Gpio0bPSpec>;
#[doc = "Register `GPIO0B_P` writer"]
pub type W = crate::W<Gpio0bPSpec>;
#[doc = "GPIO0A PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits\\[PS:PE\\]\n\nValue on reset: 1367"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Gpio0bP {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "3: weak 1(pull-up);"]
    B11 = 3,
    #[doc = "1: weak 0(pull-down);"]
    B01 = 1,
    #[doc = "2: Z(Noraml operaton);"]
    B10 = 2,
}
impl From<Gpio0bP> for u16 {
    #[inline(always)]
    fn from(variant: Gpio0bP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0bP {
    type Ux = u16;
}
#[doc = "Field `GPIO0B_P` reader - GPIO0A PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits\\[PS:PE\\]"]
pub type Gpio0bPR = crate::FieldReader<Gpio0bP>;
impl Gpio0bPR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0bP> {
        match self.bits {
            0 => Some(Gpio0bP::B00),
            3 => Some(Gpio0bP::B11),
            1 => Some(Gpio0bP::B01),
            2 => Some(Gpio0bP::B10),
            _ => None,
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio0bP::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio0bP::B11
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio0bP::B01
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio0bP::B10
    }
}
#[doc = "Field `GPIO0B_P` writer - GPIO0A PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits\\[PS:PE\\]"]
pub type Gpio0bPW<'a, REG> = crate::FieldWriter<'a, REG, 16, Gpio0bP>;
impl<'a, REG> Gpio0bPW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bP::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bP::B11)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bP::B01)
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bP::B10)
    }
}
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPIO0A PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits\\[PS:PE\\]"]
    #[inline(always)]
    pub fn gpio0b_p(&self) -> Gpio0bPR {
        Gpio0bPR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPIO0A PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits\\[PS:PE\\]"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0b_p(&mut self) -> Gpio0bPW<Gpio0bPSpec> {
        Gpio0bPW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio0bPSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO0B PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0b_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0b_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0bPSpec;
impl crate::RegisterSpec for Gpio0bPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0b_p::R`](R) reader structure"]
impl crate::Readable for Gpio0bPSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio0b_p::W`](W) writer structure"]
impl crate::Writable for Gpio0bPSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO0B_P to value 0x0557"]
impl crate::Resettable for Gpio0bPSpec {
    const RESET_VALUE: u32 = 0x0557;
}
