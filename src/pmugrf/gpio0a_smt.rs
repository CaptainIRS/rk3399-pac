#[doc = "Register `GPIO0A_SMT` reader"]
pub type R = crate::R<Gpio0aSmtSpec>;
#[doc = "Register `GPIO0A_SMT` writer"]
pub type W = crate::W<Gpio0aSmtSpec>;
#[doc = "GPIO0A drive strength control, every GPIO bit\n\ncorresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Gpio0aSmt {
    #[doc = "0: smit disable"]
    B00 = 0,
    #[doc = "1: smit enable"]
    B01 = 1,
}
impl From<Gpio0aSmt> for u16 {
    #[inline(always)]
    fn from(variant: Gpio0aSmt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0aSmt {
    type Ux = u16;
}
#[doc = "Field `GPIO0A_SMT` reader - GPIO0A drive strength control, every GPIO bit\n\ncorresponding to 2bits"]
pub type Gpio0aSmtR = crate::FieldReader<Gpio0aSmt>;
impl Gpio0aSmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0aSmt> {
        match self.bits {
            0 => Some(Gpio0aSmt::B00),
            1 => Some(Gpio0aSmt::B01),
            _ => None,
        }
    }
    #[doc = "smit disable"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio0aSmt::B00
    }
    #[doc = "smit enable"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio0aSmt::B01
    }
}
#[doc = "Field `GPIO0A_SMT` writer - GPIO0A drive strength control, every GPIO bit\n\ncorresponding to 2bits"]
pub type Gpio0aSmtW<'a, REG> = crate::FieldWriter<'a, REG, 16, Gpio0aSmt>;
impl<'a, REG> Gpio0aSmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "smit disable"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0aSmt::B00)
    }
    #[doc = "smit enable"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0aSmt::B01)
    }
}
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPIO0A drive strength control, every GPIO bit\n\ncorresponding to 2bits"]
    #[inline(always)]
    pub fn gpio0a_smt(&self) -> Gpio0aSmtR {
        Gpio0aSmtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPIO0A drive strength control, every GPIO bit\n\ncorresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0a_smt(&mut self) -> Gpio0aSmtW<Gpio0aSmtSpec> {
        Gpio0aSmtW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio0aSmtSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO0A smit control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio0a_smt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio0a_smt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0aSmtSpec;
impl crate::RegisterSpec for Gpio0aSmtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0a_smt::R`](R) reader structure"]
impl crate::Readable for Gpio0aSmtSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio0a_smt::W`](W) writer structure"]
impl crate::Writable for Gpio0aSmtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO0A_SMT to value 0"]
impl crate::Resettable for Gpio0aSmtSpec {
    const RESET_VALUE: u32 = 0;
}
