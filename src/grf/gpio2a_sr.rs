#[doc = "Register `GPIO2A_SR` reader"]
pub type R = crate::R<Gpio2aSrSpec>;
#[doc = "Register `GPIO2A_SR` writer"]
pub type W = crate::W<Gpio2aSrSpec>;
#[doc = "GPIO slew rate programmation section, every\n\nGPIO bit corresponding to 1bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2aSr {
    #[doc = "0: slow"]
    B0 = 0,
    #[doc = "1: fast"]
    B1 = 1,
}
impl From<Gpio2aSr> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2aSr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2aSr {
    type Ux = u8;
}
#[doc = "Field `GPIO2A_SR` reader - GPIO slew rate programmation section, every\n\nGPIO bit corresponding to 1bits"]
pub type Gpio2aSrR = crate::FieldReader<Gpio2aSr>;
impl Gpio2aSrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio2aSr> {
        match self.bits {
            0 => Some(Gpio2aSr::B0),
            1 => Some(Gpio2aSr::B1),
            _ => None,
        }
    }
    #[doc = "slow"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio2aSr::B0
    }
    #[doc = "fast"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio2aSr::B1
    }
}
#[doc = "Field `GPIO2A_SR` writer - GPIO slew rate programmation section, every\n\nGPIO bit corresponding to 1bits"]
pub type Gpio2aSrW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio2aSr>;
impl<'a, REG> Gpio2aSrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "slow"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2aSr::B0)
    }
    #[doc = "fast"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2aSr::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - GPIO slew rate programmation section, every\n\nGPIO bit corresponding to 1bits"]
    #[inline(always)]
    pub fn gpio2a_sr(&self) -> Gpio2aSrR {
        Gpio2aSrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO slew rate programmation section, every\n\nGPIO bit corresponding to 1bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a_sr(&mut self) -> Gpio2aSrW<Gpio2aSrSpec> {
        Gpio2aSrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio2aSrSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO2A slew rate control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2a_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2a_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio2aSrSpec;
impl crate::RegisterSpec for Gpio2aSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio2a_sr::R`](R) reader structure"]
impl crate::Readable for Gpio2aSrSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio2a_sr::W`](W) writer structure"]
impl crate::Writable for Gpio2aSrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO2A_SR to value 0"]
impl crate::Resettable for Gpio2aSrSpec {
    const RESET_VALUE: u32 = 0;
}
