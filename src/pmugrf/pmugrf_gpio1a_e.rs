#[doc = "Register `PMUGRF_GPIO1A_E` reader"]
pub type R = crate::R<PmugrfGpio1aESpec>;
#[doc = "Register `PMUGRF_GPIO1A_E` writer"]
pub type W = crate::W<PmugrfGpio1aESpec>;
#[doc = "GPIO1A drive strength control, every GPIO bit corresponding to 2bits\n\nValue on reset: 16384"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Gpio1aE {
    #[doc = "0: 12mA"]
    B00 = 0,
    #[doc = "1: 12mA"]
    B01 = 1,
    #[doc = "2: 12mA"]
    B10 = 2,
    #[doc = "3: 12mA"]
    B11 = 3,
}
impl From<Gpio1aE> for u16 {
    #[inline(always)]
    fn from(variant: Gpio1aE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1aE {
    type Ux = u16;
}
#[doc = "Field `GPIO1A_E` reader - GPIO1A drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio1aER = crate::FieldReader<Gpio1aE>;
impl Gpio1aER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1aE> {
        match self.bits {
            0 => Some(Gpio1aE::B00),
            1 => Some(Gpio1aE::B01),
            2 => Some(Gpio1aE::B10),
            3 => Some(Gpio1aE::B11),
            _ => None,
        }
    }
    #[doc = "12mA"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1aE::B00
    }
    #[doc = "12mA"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1aE::B01
    }
    #[doc = "12mA"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1aE::B10
    }
    #[doc = "12mA"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio1aE::B11
    }
}
#[doc = "Field `GPIO1A_E` writer - GPIO1A drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio1aEW<'a, REG> = crate::FieldWriter<'a, REG, 16, Gpio1aE>;
impl<'a, REG> Gpio1aEW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "12mA"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1aE::B00)
    }
    #[doc = "12mA"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1aE::B01)
    }
    #[doc = "12mA"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1aE::B10)
    }
    #[doc = "12mA"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1aE::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPIO1A drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio1a_e(&self) -> Gpio1aER {
        Gpio1aER::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPIO1A drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1a_e(&mut self) -> Gpio1aEW<PmugrfGpio1aESpec> {
        Gpio1aEW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<PmugrfGpio1aESpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO1A drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1a_e::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1a_e::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmugrfGpio1aESpec;
impl crate::RegisterSpec for PmugrfGpio1aESpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmugrf_gpio1a_e::R`](R) reader structure"]
impl crate::Readable for PmugrfGpio1aESpec {}
#[doc = "`write(|w| ..)` method takes [`pmugrf_gpio1a_e::W`](W) writer structure"]
impl crate::Writable for PmugrfGpio1aESpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUGRF_GPIO1A_E to value 0x4000"]
impl crate::Resettable for PmugrfGpio1aESpec {
    const RESET_VALUE: u32 = 0x4000;
}
