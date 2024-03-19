#[doc = "Register `PMUGRF_GPIO1L_SR` reader"]
pub type R = crate::R<PmugrfGpio1lSrSpec>;
#[doc = "Register `PMUGRF_GPIO1L_SR` writer"]
pub type W = crate::W<PmugrfGpio1lSrSpec>;
#[doc = "GPIO1A slew rate control for each bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1aSr {
    #[doc = "0: slow (half frequency)"]
    B0 = 0,
    #[doc = "1: fast"]
    B1 = 1,
}
impl From<Gpio1aSr> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1aSr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1aSr {
    type Ux = u8;
}
#[doc = "Field `GPIO1A_SR` reader - GPIO1A slew rate control for each bit"]
pub type Gpio1aSrR = crate::FieldReader<Gpio1aSr>;
impl Gpio1aSrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1aSr> {
        match self.bits {
            0 => Some(Gpio1aSr::B0),
            1 => Some(Gpio1aSr::B1),
            _ => None,
        }
    }
    #[doc = "slow (half frequency)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1aSr::B0
    }
    #[doc = "fast"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1aSr::B1
    }
}
#[doc = "Field `GPIO1A_SR` writer - GPIO1A slew rate control for each bit"]
pub type Gpio1aSrW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1aSr>;
impl<'a, REG> Gpio1aSrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "slow (half frequency)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1aSr::B0)
    }
    #[doc = "fast"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1aSr::B1)
    }
}
#[doc = "GPIO1B slew rate control for each bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1bSr {
    #[doc = "0: slow (half frequency)"]
    B0 = 0,
    #[doc = "1: fast"]
    B1 = 1,
}
impl From<Gpio1bSr> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1bSr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1bSr {
    type Ux = u8;
}
#[doc = "Field `GPIO1B_SR` reader - GPIO1B slew rate control for each bit"]
pub type Gpio1bSrR = crate::FieldReader<Gpio1bSr>;
impl Gpio1bSrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1bSr> {
        match self.bits {
            0 => Some(Gpio1bSr::B0),
            1 => Some(Gpio1bSr::B1),
            _ => None,
        }
    }
    #[doc = "slow (half frequency)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio1bSr::B0
    }
    #[doc = "fast"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio1bSr::B1
    }
}
#[doc = "Field `GPIO1B_SR` writer - GPIO1B slew rate control for each bit"]
pub type Gpio1bSrW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio1bSr>;
impl<'a, REG> Gpio1bSrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "slow (half frequency)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1bSr::B0)
    }
    #[doc = "fast"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1bSr::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - GPIO1A slew rate control for each bit"]
    #[inline(always)]
    pub fn gpio1a_sr(&self) -> Gpio1aSrR {
        Gpio1aSrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO1B slew rate control for each bit"]
    #[inline(always)]
    pub fn gpio1b_sr(&self) -> Gpio1bSrR {
        Gpio1bSrR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO1A slew rate control for each bit"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1a_sr(&mut self) -> Gpio1aSrW<PmugrfGpio1lSrSpec> {
        Gpio1aSrW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO1B slew rate control for each bit"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1b_sr(&mut self) -> Gpio1bSrW<PmugrfGpio1lSrSpec> {
        Gpio1bSrW::new(self, 8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<PmugrfGpio1lSrSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO1 A/B SR control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1l_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1l_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmugrfGpio1lSrSpec;
impl crate::RegisterSpec for PmugrfGpio1lSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmugrf_gpio1l_sr::R`](R) reader structure"]
impl crate::Readable for PmugrfGpio1lSrSpec {}
#[doc = "`write(|w| ..)` method takes [`pmugrf_gpio1l_sr::W`](W) writer structure"]
impl crate::Writable for PmugrfGpio1lSrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUGRF_GPIO1L_SR to value 0"]
impl crate::Resettable for PmugrfGpio1lSrSpec {
    const RESET_VALUE: u32 = 0;
}
