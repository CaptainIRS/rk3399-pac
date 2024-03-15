#[doc = "Register `GRF_GPIO3B_SMT` reader"]
pub type R = crate::R<GrfGpio3bSmtSpec>;
#[doc = "Register `GRF_GPIO3B_SMT` writer"]
pub type W = crate::W<GrfGpio3bSmtSpec>;
#[doc = "GPIO schmitt trigger control, every GPIO bit corresponding to 1bits .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3bSmt {
    #[doc = "0: Schmitt trigger enabled."]
    B0 = 0,
    #[doc = "1: Schmitt trigger enabled."]
    B1 = 1,
}
impl From<Gpio3bSmt> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3bSmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3bSmt {
    type Ux = u8;
}
#[doc = "Field `GPIO3B_SMT` reader - GPIO schmitt trigger control, every GPIO bit corresponding to 1bits ."]
pub type Gpio3bSmtR = crate::FieldReader<Gpio3bSmt>;
impl Gpio3bSmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio3bSmt> {
        match self.bits {
            0 => Some(Gpio3bSmt::B0),
            1 => Some(Gpio3bSmt::B1),
            _ => None,
        }
    }
    #[doc = "Schmitt trigger enabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio3bSmt::B0
    }
    #[doc = "Schmitt trigger enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio3bSmt::B1
    }
}
#[doc = "Field `GPIO3B_SMT` writer - GPIO schmitt trigger control, every GPIO bit corresponding to 1bits ."]
pub type Gpio3bSmtW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio3bSmt>;
impl<'a, REG> Gpio3bSmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Schmitt trigger enabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3bSmt::B0)
    }
    #[doc = "Schmitt trigger enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3bSmt::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - GPIO schmitt trigger control, every GPIO bit corresponding to 1bits ."]
    #[inline(always)]
    pub fn gpio3b_smt(&self) -> Gpio3bSmtR {
        Gpio3bSmtR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO schmitt trigger control, every GPIO bit corresponding to 1bits ."]
    #[inline(always)]
    #[must_use]
    pub fn gpio3b_smt(&mut self) -> Gpio3bSmtW<GrfGpio3bSmtSpec> {
        Gpio3bSmtW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio3bSmtSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO3B smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3b_smt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3b_smt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio3bSmtSpec;
impl crate::RegisterSpec for GrfGpio3bSmtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio3b_smt::R`](R) reader structure"]
impl crate::Readable for GrfGpio3bSmtSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio3b_smt::W`](W) writer structure"]
impl crate::Writable for GrfGpio3bSmtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO3B_SMT to value 0"]
impl crate::Resettable for GrfGpio3bSmtSpec {
    const RESET_VALUE: u32 = 0;
}
