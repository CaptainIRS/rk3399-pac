#[doc = "Register `PMUGRF_GPIO1D_P` reader"]
pub type R = crate::R<PmugrfGpio1dPSpec>;
#[doc = "Register `PMUGRF_GPIO1D_P` writer"]
pub type W = crate::W<PmugrfGpio1dPSpec>;
#[doc = "GPIO1D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Gpio1dP {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio1dP> for u16 {
    #[inline(always)]
    fn from(variant: Gpio1dP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1dP {
    type Ux = u16;
}
#[doc = "Field `GPIO1D_P` reader - GPIO1D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio1dPR = crate::FieldReader<Gpio1dP>;
impl Gpio1dPR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1dP> {
        match self.bits {
            0 => Some(Gpio1dP::B00),
            1 => Some(Gpio1dP::B01),
            2 => Some(Gpio1dP::B10),
            3 => Some(Gpio1dP::B11),
            _ => None,
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1dP::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1dP::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1dP::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio1dP::B11
    }
}
#[doc = "Field `GPIO1D_P` writer - GPIO1D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio1dPW<'a, REG> = crate::FieldWriter<'a, REG, 16, Gpio1dP>;
impl<'a, REG> Gpio1dPW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1dP::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1dP::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1dP::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1dP::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPIO1D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio1d_p(&self) -> Gpio1dPR {
        Gpio1dPR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPIO1D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1d_p(&mut self) -> Gpio1dPW<PmugrfGpio1dPSpec> {
        Gpio1dPW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<PmugrfGpio1dPSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO0D PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1d_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1d_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmugrfGpio1dPSpec;
impl crate::RegisterSpec for PmugrfGpio1dPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmugrf_gpio1d_p::R`](R) reader structure"]
impl crate::Readable for PmugrfGpio1dPSpec {}
#[doc = "`write(|w| ..)` method takes [`pmugrf_gpio1d_p::W`](W) writer structure"]
impl crate::Writable for PmugrfGpio1dPSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUGRF_GPIO1D_P to value 0x02"]
impl crate::Resettable for PmugrfGpio1dPSpec {
    const RESET_VALUE: u32 = 0x02;
}
