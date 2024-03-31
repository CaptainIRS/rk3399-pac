#[doc = "Register `GPIO4A_SMT` reader"]
pub type R = crate::R<Gpio4aSmtSpec>;
#[doc = "Register `GPIO4A_SMT` writer"]
pub type W = crate::W<Gpio4aSmtSpec>;
#[doc = "GPIO schmitt trigger control, every GPIO bit\n\ncorresponding to 1bits .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4aSmt {
    #[doc = "0: No hysteresis"]
    B0 = 0,
    #[doc = "1: Schmitt trigger enabled."]
    B1 = 1,
}
impl From<Gpio4aSmt> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4aSmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4aSmt {
    type Ux = u8;
}
#[doc = "Field `GPIO4A_SMT` reader - GPIO schmitt trigger control, every GPIO bit\n\ncorresponding to 1bits ."]
pub type Gpio4aSmtR = crate::FieldReader<Gpio4aSmt>;
impl Gpio4aSmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio4aSmt> {
        match self.bits {
            0 => Some(Gpio4aSmt::B0),
            1 => Some(Gpio4aSmt::B1),
            _ => None,
        }
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio4aSmt::B0
    }
    #[doc = "Schmitt trigger enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio4aSmt::B1
    }
}
#[doc = "Field `GPIO4A_SMT` writer - GPIO schmitt trigger control, every GPIO bit\n\ncorresponding to 1bits ."]
pub type Gpio4aSmtW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio4aSmt>;
impl<'a, REG> Gpio4aSmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4aSmt::B0)
    }
    #[doc = "Schmitt trigger enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4aSmt::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - GPIO schmitt trigger control, every GPIO bit\n\ncorresponding to 1bits ."]
    #[inline(always)]
    pub fn gpio4a_smt(&self) -> Gpio4aSmtR {
        Gpio4aSmtR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO schmitt trigger control, every GPIO bit\n\ncorresponding to 1bits ."]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a_smt(&mut self) -> Gpio4aSmtW<Gpio4aSmtSpec> {
        Gpio4aSmtW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio4aSmtSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO4A smitter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4a_smt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4a_smt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio4aSmtSpec;
impl crate::RegisterSpec for Gpio4aSmtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio4a_smt::R`](R) reader structure"]
impl crate::Readable for Gpio4aSmtSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio4a_smt::W`](W) writer structure"]
impl crate::Writable for Gpio4aSmtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO4A_SMT to value 0"]
impl crate::Resettable for Gpio4aSmtSpec {
    const RESET_VALUE: u32 = 0;
}
