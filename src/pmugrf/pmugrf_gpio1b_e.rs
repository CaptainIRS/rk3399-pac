#[doc = "Register `PMUGRF_GPIO1B_E` reader"]
pub type R = crate::R<PmugrfGpio1bESpec>;
#[doc = "Register `PMUGRF_GPIO1B_E` writer"]
pub type W = crate::W<PmugrfGpio1bESpec>;
#[doc = "GPIO1B drive strength control, every GPIO bit\n\ncorresponding to 2bits\n\nValue on reset: 21"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Gpio1bE {
    #[doc = "0: 2mA"]
    B00 = 0,
    #[doc = "1: 4mA"]
    B01 = 1,
    #[doc = "2: 8mA"]
    B10 = 2,
    #[doc = "3: 12mA"]
    B11 = 3,
}
impl From<Gpio1bE> for u16 {
    #[inline(always)]
    fn from(variant: Gpio1bE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1bE {
    type Ux = u16;
}
#[doc = "Field `GPIO1B_E` reader - GPIO1B drive strength control, every GPIO bit\n\ncorresponding to 2bits"]
pub type Gpio1bER = crate::FieldReader<Gpio1bE>;
impl Gpio1bER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1bE> {
        match self.bits {
            0 => Some(Gpio1bE::B00),
            1 => Some(Gpio1bE::B01),
            2 => Some(Gpio1bE::B10),
            3 => Some(Gpio1bE::B11),
            _ => None,
        }
    }
    #[doc = "2mA"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1bE::B00
    }
    #[doc = "4mA"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1bE::B01
    }
    #[doc = "8mA"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1bE::B10
    }
    #[doc = "12mA"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio1bE::B11
    }
}
#[doc = "Field `GPIO1B_E` writer - GPIO1B drive strength control, every GPIO bit\n\ncorresponding to 2bits"]
pub type Gpio1bEW<'a, REG> = crate::FieldWriter<'a, REG, 16, Gpio1bE>;
impl<'a, REG> Gpio1bEW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "2mA"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1bE::B00)
    }
    #[doc = "4mA"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1bE::B01)
    }
    #[doc = "8mA"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1bE::B10)
    }
    #[doc = "12mA"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1bE::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPIO1B drive strength control, every GPIO bit\n\ncorresponding to 2bits"]
    #[inline(always)]
    pub fn gpio1b_e(&self) -> Gpio1bER {
        Gpio1bER::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPIO1B drive strength control, every GPIO bit\n\ncorresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1b_e(&mut self) -> Gpio1bEW<PmugrfGpio1bESpec> {
        Gpio1bEW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<PmugrfGpio1bESpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO1D drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1b_e::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1b_e::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmugrfGpio1bESpec;
impl crate::RegisterSpec for PmugrfGpio1bESpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmugrf_gpio1b_e::R`](R) reader structure"]
impl crate::Readable for PmugrfGpio1bESpec {}
#[doc = "`write(|w| ..)` method takes [`pmugrf_gpio1b_e::W`](W) writer structure"]
impl crate::Writable for PmugrfGpio1bESpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUGRF_GPIO1B_E to value 0x15"]
impl crate::Resettable for PmugrfGpio1bESpec {
    const RESET_VALUE: u32 = 0x15;
}
