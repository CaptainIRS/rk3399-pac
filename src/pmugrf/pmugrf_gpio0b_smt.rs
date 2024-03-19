#[doc = "Register `PMUGRF_GPIO0B_SMT` reader"]
pub type R = crate::R<PmugrfGpio0bSmtSpec>;
#[doc = "Register `PMUGRF_GPIO0B_SMT` writer"]
pub type W = crate::W<PmugrfGpio0bSmtSpec>;
#[doc = "GPIO0B drive strength control, every GPIO bit\n\ncorresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Gpio0bSmt {
    #[doc = "0: smit disable"]
    B00 = 0,
    #[doc = "1: smit enable"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio0bSmt> for u16 {
    #[inline(always)]
    fn from(variant: Gpio0bSmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0bSmt {
    type Ux = u16;
}
#[doc = "Field `GPIO0B_SMT` reader - GPIO0B drive strength control, every GPIO bit\n\ncorresponding to 2bits"]
pub type Gpio0bSmtR = crate::FieldReader<Gpio0bSmt>;
impl Gpio0bSmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0bSmt> {
        match self.bits {
            0 => Some(Gpio0bSmt::B00),
            1 => Some(Gpio0bSmt::B01),
            2 => Some(Gpio0bSmt::B10),
            3 => Some(Gpio0bSmt::B11),
            _ => None,
        }
    }
    #[doc = "smit disable"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio0bSmt::B00
    }
    #[doc = "smit enable"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio0bSmt::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio0bSmt::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio0bSmt::B11
    }
}
#[doc = "Field `GPIO0B_SMT` writer - GPIO0B drive strength control, every GPIO bit\n\ncorresponding to 2bits"]
pub type Gpio0bSmtW<'a, REG> = crate::FieldWriter<'a, REG, 16, Gpio0bSmt>;
impl<'a, REG> Gpio0bSmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "smit disable"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bSmt::B00)
    }
    #[doc = "smit enable"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bSmt::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bSmt::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bSmt::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPIO0B drive strength control, every GPIO bit\n\ncorresponding to 2bits"]
    #[inline(always)]
    pub fn gpio0b_smt(&self) -> Gpio0bSmtR {
        Gpio0bSmtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPIO0B drive strength control, every GPIO bit\n\ncorresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0b_smt(&mut self) -> Gpio0bSmtW<PmugrfGpio0bSmtSpec> {
        Gpio0bSmtW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<PmugrfGpio0bSmtSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO0B smit control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio0b_smt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio0b_smt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmugrfGpio0bSmtSpec;
impl crate::RegisterSpec for PmugrfGpio0bSmtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmugrf_gpio0b_smt::R`](R) reader structure"]
impl crate::Readable for PmugrfGpio0bSmtSpec {}
#[doc = "`write(|w| ..)` method takes [`pmugrf_gpio0b_smt::W`](W) writer structure"]
impl crate::Writable for PmugrfGpio0bSmtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUGRF_GPIO0B_SMT to value 0"]
impl crate::Resettable for PmugrfGpio0bSmtSpec {
    const RESET_VALUE: u32 = 0;
}
