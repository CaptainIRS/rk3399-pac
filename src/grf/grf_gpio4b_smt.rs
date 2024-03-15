#[doc = "Register `GRF_GPIO4B_SMT` reader"]
pub type R = crate::R<GrfGpio4bSmtSpec>;
#[doc = "Register `GRF_GPIO4B_SMT` writer"]
pub type W = crate::W<GrfGpio4bSmtSpec>;
#[doc = "GPIO schmitt trigger control, every GPIO bit corresponding to 1bits .\n\nValue on reset: 63"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4bSmt {
    #[doc = "0: Schmitt trigger enabled."]
    B0 = 0,
    #[doc = "1: Schmitt trigger enabled."]
    B1 = 1,
}
impl From<Gpio4bSmt> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4bSmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4bSmt {
    type Ux = u8;
}
#[doc = "Field `GPIO4B_SMT` reader - GPIO schmitt trigger control, every GPIO bit corresponding to 1bits ."]
pub type Gpio4bSmtR = crate::FieldReader<Gpio4bSmt>;
impl Gpio4bSmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio4bSmt> {
        match self.bits {
            0 => Some(Gpio4bSmt::B0),
            1 => Some(Gpio4bSmt::B1),
            _ => None,
        }
    }
    #[doc = "Schmitt trigger enabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio4bSmt::B0
    }
    #[doc = "Schmitt trigger enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio4bSmt::B1
    }
}
#[doc = "Field `GPIO4B_SMT` writer - GPIO schmitt trigger control, every GPIO bit corresponding to 1bits ."]
pub type Gpio4bSmtW<'a, REG> = crate::FieldWriter<'a, REG, 6, Gpio4bSmt>;
impl<'a, REG> Gpio4bSmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Schmitt trigger enabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4bSmt::B0)
    }
    #[doc = "Schmitt trigger enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4bSmt::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:5 - GPIO schmitt trigger control, every GPIO bit corresponding to 1bits ."]
    #[inline(always)]
    pub fn gpio4b_smt(&self) -> Gpio4bSmtR {
        Gpio4bSmtR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - GPIO schmitt trigger control, every GPIO bit corresponding to 1bits ."]
    #[inline(always)]
    #[must_use]
    pub fn gpio4b_smt(&mut self) -> Gpio4bSmtW<GrfGpio4bSmtSpec> {
        Gpio4bSmtW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio4bSmtSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO4B smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4b_smt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4b_smt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio4bSmtSpec;
impl crate::RegisterSpec for GrfGpio4bSmtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio4b_smt::R`](R) reader structure"]
impl crate::Readable for GrfGpio4bSmtSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio4b_smt::W`](W) writer structure"]
impl crate::Writable for GrfGpio4bSmtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO4B_SMT to value 0x3f"]
impl crate::Resettable for GrfGpio4bSmtSpec {
    const RESET_VALUE: u32 = 0x3f;
}
