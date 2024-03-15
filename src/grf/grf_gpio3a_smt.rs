#[doc = "Register `GRF_GPIO3A_SMT` reader"]
pub type R = crate::R<GrfGpio3aSmtSpec>;
#[doc = "Register `GRF_GPIO3A_SMT` writer"]
pub type W = crate::W<GrfGpio3aSmtSpec>;
#[doc = "GPIO schmitt trigger control, every GPIO bit corresponding to 1bits .\n\nValue on reset: 240"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3aSmt {
    #[doc = "0: Schmitt trigger enabled."]
    B0 = 0,
    #[doc = "1: Schmitt trigger enabled."]
    B1 = 1,
}
impl From<Gpio3aSmt> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3aSmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3aSmt {
    type Ux = u8;
}
#[doc = "Field `GPIO3A_SMT` reader - GPIO schmitt trigger control, every GPIO bit corresponding to 1bits ."]
pub type Gpio3aSmtR = crate::FieldReader<Gpio3aSmt>;
impl Gpio3aSmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio3aSmt> {
        match self.bits {
            0 => Some(Gpio3aSmt::B0),
            1 => Some(Gpio3aSmt::B1),
            _ => None,
        }
    }
    #[doc = "Schmitt trigger enabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio3aSmt::B0
    }
    #[doc = "Schmitt trigger enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio3aSmt::B1
    }
}
#[doc = "Field `GPIO3A_SMT` writer - GPIO schmitt trigger control, every GPIO bit corresponding to 1bits ."]
pub type Gpio3aSmtW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio3aSmt>;
impl<'a, REG> Gpio3aSmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Schmitt trigger enabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3aSmt::B0)
    }
    #[doc = "Schmitt trigger enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3aSmt::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - GPIO schmitt trigger control, every GPIO bit corresponding to 1bits ."]
    #[inline(always)]
    pub fn gpio3a_smt(&self) -> Gpio3aSmtR {
        Gpio3aSmtR::new((self.bits & 0xff) as u8)
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
    pub fn gpio3a_smt(&mut self) -> Gpio3aSmtW<GrfGpio3aSmtSpec> {
        Gpio3aSmtW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio3aSmtSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO3A smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3a_smt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3a_smt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio3aSmtSpec;
impl crate::RegisterSpec for GrfGpio3aSmtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio3a_smt::R`](R) reader structure"]
impl crate::Readable for GrfGpio3aSmtSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio3a_smt::W`](W) writer structure"]
impl crate::Writable for GrfGpio3aSmtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO3A_SMT to value 0xf0"]
impl crate::Resettable for GrfGpio3aSmtSpec {
    const RESET_VALUE: u32 = 0xf0;
}
