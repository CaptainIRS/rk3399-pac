#[doc = "Register `GPIO0B_E` reader"]
pub type R = crate::R<Gpio0bESpec>;
#[doc = "Register `GPIO0B_E` writer"]
pub type W = crate::W<Gpio0bESpec>;
#[doc = "GPIO0B drive strength control, every GPIO bit\n\ncorresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Gpio0bE {
    #[doc = "0: 2mA"]
    B00 = 0,
    #[doc = "1: 4mA"]
    B01 = 1,
    #[doc = "2: 8mA"]
    B10 = 2,
    #[doc = "3: 12mA"]
    B11 = 3,
}
impl From<Gpio0bE> for u16 {
    #[inline(always)]
    fn from(variant: Gpio0bE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0bE {
    type Ux = u16;
}
#[doc = "Field `GPIO0B_E` reader - GPIO0B drive strength control, every GPIO bit\n\ncorresponding to 2bits"]
pub type Gpio0bER = crate::FieldReader<Gpio0bE>;
impl Gpio0bER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0bE> {
        match self.bits {
            0 => Some(Gpio0bE::B00),
            1 => Some(Gpio0bE::B01),
            2 => Some(Gpio0bE::B10),
            3 => Some(Gpio0bE::B11),
            _ => None,
        }
    }
    #[doc = "2mA"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio0bE::B00
    }
    #[doc = "4mA"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio0bE::B01
    }
    #[doc = "8mA"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio0bE::B10
    }
    #[doc = "12mA"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio0bE::B11
    }
}
#[doc = "Field `GPIO0B_E` writer - GPIO0B drive strength control, every GPIO bit\n\ncorresponding to 2bits"]
pub type Gpio0bEW<'a, REG> = crate::FieldWriter<'a, REG, 16, Gpio0bE>;
impl<'a, REG> Gpio0bEW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "2mA"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bE::B00)
    }
    #[doc = "4mA"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bE::B01)
    }
    #[doc = "8mA"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bE::B10)
    }
    #[doc = "12mA"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bE::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPIO0B drive strength control, every GPIO bit\n\ncorresponding to 2bits"]
    #[inline(always)]
    pub fn gpio0b_e(&self) -> Gpio0bER {
        Gpio0bER::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPIO0B drive strength control, every GPIO bit\n\ncorresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0b_e(&mut self) -> Gpio0bEW<Gpio0bESpec> {
        Gpio0bEW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio0bESpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO0D drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0b_e::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0b_e::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0bESpec;
impl crate::RegisterSpec for Gpio0bESpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0b_e::R`](R) reader structure"]
impl crate::Readable for Gpio0bESpec {}
#[doc = "`write(|w| ..)` method takes [`gpio0b_e::W`](W) writer structure"]
impl crate::Writable for Gpio0bESpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO0B_E to value 0"]
impl crate::Resettable for Gpio0bESpec {
    const RESET_VALUE: u32 = 0;
}
