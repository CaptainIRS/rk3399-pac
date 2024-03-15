#[doc = "Register `PMUGRF_GPIO1A_P` reader"]
pub type R = crate::R<PmugrfGpio1aPSpec>;
#[doc = "Register `PMUGRF_GPIO1A_P` writer"]
pub type W = crate::W<PmugrfGpio1aPSpec>;
#[doc = "GPIO1A PU/PD programmation section, every GPIO bit corresponding to 2bits\n\nValue on reset: 27306"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Gpio1aP {
    #[doc = "0: Repeater(Bus keeper)"]
    B00 = 0,
    #[doc = "1: Repeater(Bus keeper)"]
    B01 = 1,
    #[doc = "2: Repeater(Bus keeper)"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio1aP> for u16 {
    #[inline(always)]
    fn from(variant: Gpio1aP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1aP {
    type Ux = u16;
}
#[doc = "Field `GPIO1A_P` reader - GPIO1A PU/PD programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio1aPR = crate::FieldReader<Gpio1aP>;
impl Gpio1aPR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1aP> {
        match self.bits {
            0 => Some(Gpio1aP::B00),
            1 => Some(Gpio1aP::B01),
            2 => Some(Gpio1aP::B10),
            3 => Some(Gpio1aP::B11),
            _ => None,
        }
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1aP::B00
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1aP::B01
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1aP::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio1aP::B11
    }
}
#[doc = "Field `GPIO1A_P` writer - GPIO1A PU/PD programmation section, every GPIO bit corresponding to 2bits"]
pub type Gpio1aPW<'a, REG> = crate::FieldWriter<'a, REG, 16, Gpio1aP>;
impl<'a, REG> Gpio1aPW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1aP::B00)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1aP::B01)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1aP::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1aP::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPIO1A PU/PD programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio1a_p(&self) -> Gpio1aPR {
        Gpio1aPR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPIO1A PU/PD programmation section, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1a_p(&mut self) -> Gpio1aPW<PmugrfGpio1aPSpec> {
        Gpio1aPW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<PmugrfGpio1aPSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO1A PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1a_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1a_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmugrfGpio1aPSpec;
impl crate::RegisterSpec for PmugrfGpio1aPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmugrf_gpio1a_p::R`](R) reader structure"]
impl crate::Readable for PmugrfGpio1aPSpec {}
#[doc = "`write(|w| ..)` method takes [`pmugrf_gpio1a_p::W`](W) writer structure"]
impl crate::Writable for PmugrfGpio1aPSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUGRF_GPIO1A_P to value 0x6aaa"]
impl crate::Resettable for PmugrfGpio1aPSpec {
    const RESET_VALUE: u32 = 0x6aaa;
}
