#[doc = "Register `PMUGRF_GPIO1D_SMT` reader"]
pub type R = crate::R<PmugrfGpio1dSmtSpec>;
#[doc = "Register `PMUGRF_GPIO1D_SMT` writer"]
pub type W = crate::W<PmugrfGpio1dSmtSpec>;
#[doc = "GPIO1D drive strength control, every GPIO bit corresponding to 2bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Gpio1dSmt {
    #[doc = "0: reserved"]
    B00 = 0,
    #[doc = "1: reserved"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio1dSmt> for u16 {
    #[inline(always)]
    fn from(variant: Gpio1dSmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1dSmt {
    type Ux = u16;
}
#[doc = "Field `GPIO1D_SMT` reader - GPIO1D drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio1dSmtR = crate::FieldReader<Gpio1dSmt>;
impl Gpio1dSmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1dSmt> {
        match self.bits {
            0 => Some(Gpio1dSmt::B00),
            1 => Some(Gpio1dSmt::B01),
            2 => Some(Gpio1dSmt::B10),
            3 => Some(Gpio1dSmt::B11),
            _ => None,
        }
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1dSmt::B00
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1dSmt::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1dSmt::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio1dSmt::B11
    }
}
#[doc = "Field `GPIO1D_SMT` writer - GPIO1D drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio1dSmtW<'a, REG> = crate::FieldWriter<'a, REG, 16, Gpio1dSmt>;
impl<'a, REG> Gpio1dSmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1dSmt::B00)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1dSmt::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1dSmt::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1dSmt::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPIO1D drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio1d_smt(&self) -> Gpio1dSmtR {
        Gpio1dSmtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPIO1D drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1d_smt(&mut self) -> Gpio1dSmtW<PmugrfGpio1dSmtSpec> {
        Gpio1dSmtW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<PmugrfGpio1dSmtSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO1D smit control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1d_smt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1d_smt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmugrfGpio1dSmtSpec;
impl crate::RegisterSpec for PmugrfGpio1dSmtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmugrf_gpio1d_smt::R`](R) reader structure"]
impl crate::Readable for PmugrfGpio1dSmtSpec {}
#[doc = "`write(|w| ..)` method takes [`pmugrf_gpio1d_smt::W`](W) writer structure"]
impl crate::Writable for PmugrfGpio1dSmtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUGRF_GPIO1D_SMT to value 0"]
impl crate::Resettable for PmugrfGpio1dSmtSpec {
    const RESET_VALUE: u32 = 0;
}
