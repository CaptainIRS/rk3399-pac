#[doc = "Register `GRF_GPIO2C_SR` reader"]
pub type R = crate::R<GrfGpio2cSrSpec>;
#[doc = "Register `GRF_GPIO2C_SR` writer"]
pub type W = crate::W<GrfGpio2cSrSpec>;
#[doc = "GPIO slew rate programmation section, every GPIO bit corresponding to 1bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2cSr {
    #[doc = "0: fast"]
    B0 = 0,
    #[doc = "1: fast"]
    B1 = 1,
}
impl From<Gpio2cSr> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2cSr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2cSr {
    type Ux = u8;
}
#[doc = "Field `GPIO2C_SR` reader - GPIO slew rate programmation section, every GPIO bit corresponding to 1bits"]
pub type Gpio2cSrR = crate::FieldReader<Gpio2cSr>;
impl Gpio2cSrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio2cSr> {
        match self.bits {
            0 => Some(Gpio2cSr::B0),
            1 => Some(Gpio2cSr::B1),
            _ => None,
        }
    }
    #[doc = "fast"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio2cSr::B0
    }
    #[doc = "fast"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio2cSr::B1
    }
}
#[doc = "Field `GPIO2C_SR` writer - GPIO slew rate programmation section, every GPIO bit corresponding to 1bits"]
pub type Gpio2cSrW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio2cSr>;
impl<'a, REG> Gpio2cSrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fast"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2cSr::B0)
    }
    #[doc = "fast"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2cSr::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - GPIO slew rate programmation section, every GPIO bit corresponding to 1bits"]
    #[inline(always)]
    pub fn gpio2c_sr(&self) -> Gpio2cSrR {
        Gpio2cSrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO slew rate programmation section, every GPIO bit corresponding to 1bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c_sr(&mut self) -> Gpio2cSrW<GrfGpio2cSrSpec> {
        Gpio2cSrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio2cSrSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO2C slew rate control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2c_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2c_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio2cSrSpec;
impl crate::RegisterSpec for GrfGpio2cSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio2c_sr::R`](R) reader structure"]
impl crate::Readable for GrfGpio2cSrSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio2c_sr::W`](W) writer structure"]
impl crate::Writable for GrfGpio2cSrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO2C_SR to value 0"]
impl crate::Resettable for GrfGpio2cSrSpec {
    const RESET_VALUE: u32 = 0;
}
