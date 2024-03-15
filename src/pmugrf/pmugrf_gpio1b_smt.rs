#[doc = "Register `PMUGRF_GPIO1B_SMT` reader"]
pub type R = crate::R<PmugrfGpio1bSmtSpec>;
#[doc = "Register `PMUGRF_GPIO1B_SMT` writer"]
pub type W = crate::W<PmugrfGpio1bSmtSpec>;
#[doc = "GPIO1B drive strength control, every GPIO bit corresponding to 2bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Gpio1bSmt {
    #[doc = "0: reserved"]
    B00 = 0,
    #[doc = "1: reserved"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio1bSmt> for u16 {
    #[inline(always)]
    fn from(variant: Gpio1bSmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1bSmt {
    type Ux = u16;
}
#[doc = "Field `GPIO1B_SMT` reader - GPIO1B drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio1bSmtR = crate::FieldReader<Gpio1bSmt>;
impl Gpio1bSmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1bSmt> {
        match self.bits {
            0 => Some(Gpio1bSmt::B00),
            1 => Some(Gpio1bSmt::B01),
            2 => Some(Gpio1bSmt::B10),
            3 => Some(Gpio1bSmt::B11),
            _ => None,
        }
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1bSmt::B00
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1bSmt::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1bSmt::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio1bSmt::B11
    }
}
#[doc = "Field `GPIO1B_SMT` writer - GPIO1B drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio1bSmtW<'a, REG> = crate::FieldWriter<'a, REG, 16, Gpio1bSmt>;
impl<'a, REG> Gpio1bSmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1bSmt::B00)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1bSmt::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1bSmt::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1bSmt::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPIO1B drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio1b_smt(&self) -> Gpio1bSmtR {
        Gpio1bSmtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPIO1B drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1b_smt(&mut self) -> Gpio1bSmtW<PmugrfGpio1bSmtSpec> {
        Gpio1bSmtW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<PmugrfGpio1bSmtSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO1B smit control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1b_smt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1b_smt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmugrfGpio1bSmtSpec;
impl crate::RegisterSpec for PmugrfGpio1bSmtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmugrf_gpio1b_smt::R`](R) reader structure"]
impl crate::Readable for PmugrfGpio1bSmtSpec {}
#[doc = "`write(|w| ..)` method takes [`pmugrf_gpio1b_smt::W`](W) writer structure"]
impl crate::Writable for PmugrfGpio1bSmtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUGRF_GPIO1B_SMT to value 0"]
impl crate::Resettable for PmugrfGpio1bSmtSpec {
    const RESET_VALUE: u32 = 0;
}
