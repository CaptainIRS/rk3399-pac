#[doc = "Register `GRF_GPIO2C_SMT` reader"]
pub type R = crate::R<GrfGpio2cSmtSpec>;
#[doc = "Register `GRF_GPIO2C_SMT` writer"]
pub type W = crate::W<GrfGpio2cSmtSpec>;
#[doc = "GPIO schmitt trigger control, every GPIO bit corresponding to 2 bits .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Gpio2cSmt {
    #[doc = "0: level 3"]
    B00 = 0,
    #[doc = "1: level 3"]
    B01 = 1,
    #[doc = "2: level 3"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2cSmt> for u16 {
    #[inline(always)]
    fn from(variant: Gpio2cSmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2cSmt {
    type Ux = u16;
}
#[doc = "Field `GPIO2C_SMT` reader - GPIO schmitt trigger control, every GPIO bit corresponding to 2 bits ."]
pub type Gpio2cSmtR = crate::FieldReader<Gpio2cSmt>;
impl Gpio2cSmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio2cSmt> {
        match self.bits {
            0 => Some(Gpio2cSmt::B00),
            1 => Some(Gpio2cSmt::B01),
            2 => Some(Gpio2cSmt::B10),
            3 => Some(Gpio2cSmt::B11),
            _ => None,
        }
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2cSmt::B00
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2cSmt::B01
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2cSmt::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2cSmt::B11
    }
}
#[doc = "Field `GPIO2C_SMT` writer - GPIO schmitt trigger control, every GPIO bit corresponding to 2 bits ."]
pub type Gpio2cSmtW<'a, REG> = crate::FieldWriter<'a, REG, 16, Gpio2cSmt>;
impl<'a, REG> Gpio2cSmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2cSmt::B00)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2cSmt::B01)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2cSmt::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2cSmt::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPIO schmitt trigger control, every GPIO bit corresponding to 2 bits ."]
    #[inline(always)]
    pub fn gpio2c_smt(&self) -> Gpio2cSmtR {
        Gpio2cSmtR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPIO schmitt trigger control, every GPIO bit corresponding to 2 bits ."]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c_smt(&mut self) -> Gpio2cSmtW<GrfGpio2cSmtSpec> {
        Gpio2cSmtW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio2cSmtSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO2C smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2c_smt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2c_smt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio2cSmtSpec;
impl crate::RegisterSpec for GrfGpio2cSmtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio2c_smt::R`](R) reader structure"]
impl crate::Readable for GrfGpio2cSmtSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio2c_smt::W`](W) writer structure"]
impl crate::Writable for GrfGpio2cSmtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO2C_SMT to value 0"]
impl crate::Resettable for GrfGpio2cSmtSpec {
    const RESET_VALUE: u32 = 0;
}
