#[doc = "Register `PMUGRF_GPIO1H_SR` reader"]
pub type R = crate::R<PmugrfGpio1hSrSpec>;
#[doc = "Register `PMUGRF_GPIO1H_SR` writer"]
pub type W = crate::W<PmugrfGpio1hSrSpec>;
#[doc = "GPIO1C slew rate control for each bit\n\nValue on reset: 15"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1cSr {
    #[doc = "0: slow (half frequency)"]
    B0 = 0,
    #[doc = "1: fast"]
    B1 = 1,
}
impl From<Gpio1cSr> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1cSr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1cSr {
    type Ux = u8;
}
#[doc = "Field `GPIO1C_SR` reader - GPIO1C slew rate control for each bit"]
pub type Gpio1cSrR = crate::FieldReader<Gpio1cSr>;
impl Gpio1cSrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1cSr> {
        match self.bits {
            0 => Some(Gpio1cSr::B0),
            1 => Some(Gpio1cSr::B1),
            _ => None,
        }
    }
    #[doc = "slow (half frequency)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1cSr::B0
    }
    #[doc = "fast"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1cSr::B1
    }
}
#[doc = "Field `GPIO1C_SR` writer - GPIO1C slew rate control for each bit"]
pub type Gpio1cSrW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1cSr>;
impl<'a, REG> Gpio1cSrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "slow (half frequency)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1cSr::B0)
    }
    #[doc = "fast"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1cSr::B1)
    }
}
#[doc = "GPIO0D slew rate control for each bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0dSr {
    #[doc = "0: slow (half frequency)"]
    B0 = 0,
    #[doc = "1: fast"]
    B1 = 1,
}
impl From<Gpio0dSr> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0dSr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0dSr {
    type Ux = u8;
}
#[doc = "Field `GPIO0D_SR` reader - GPIO0D slew rate control for each bit"]
pub type Gpio0dSrR = crate::FieldReader<Gpio0dSr>;
impl Gpio0dSrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0dSr> {
        match self.bits {
            0 => Some(Gpio0dSr::B0),
            1 => Some(Gpio0dSr::B1),
            _ => None,
        }
    }
    #[doc = "slow (half frequency)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0dSr::B0
    }
    #[doc = "fast"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0dSr::B1
    }
}
#[doc = "Field `GPIO0D_SR` writer - GPIO0D slew rate control for each bit"]
pub type Gpio0dSrW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0dSr>;
impl<'a, REG> Gpio0dSrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "slow (half frequency)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0dSr::B0)
    }
    #[doc = "fast"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0dSr::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - GPIO1C slew rate control for each bit"]
    #[inline(always)]
    pub fn gpio1c_sr(&self) -> Gpio1cSrR {
        Gpio1cSrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO0D slew rate control for each bit"]
    #[inline(always)]
    pub fn gpio0d_sr(&self) -> Gpio0dSrR {
        Gpio0dSrR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO1C slew rate control for each bit"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1c_sr(&mut self) -> Gpio1cSrW<PmugrfGpio1hSrSpec> {
        Gpio1cSrW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO0D slew rate control for each bit"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0d_sr(&mut self) -> Gpio0dSrW<PmugrfGpio1hSrSpec> {
        Gpio0dSrW::new(self, 8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<PmugrfGpio1hSrSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO1C/D SR control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1h_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1h_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmugrfGpio1hSrSpec;
impl crate::RegisterSpec for PmugrfGpio1hSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmugrf_gpio1h_sr::R`](R) reader structure"]
impl crate::Readable for PmugrfGpio1hSrSpec {}
#[doc = "`write(|w| ..)` method takes [`pmugrf_gpio1h_sr::W`](W) writer structure"]
impl crate::Writable for PmugrfGpio1hSrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUGRF_GPIO1H_SR to value 0x0f"]
impl crate::Resettable for PmugrfGpio1hSrSpec {
    const RESET_VALUE: u32 = 0x0f;
}
