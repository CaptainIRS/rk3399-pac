#[doc = "Register `PMUGRF_GPIO0L_SR` reader"]
pub type R = crate::R<PmugrfGpio0lSrSpec>;
#[doc = "Register `PMUGRF_GPIO0L_SR` writer"]
pub type W = crate::W<PmugrfGpio0lSrSpec>;
#[doc = "GPIO0A slew rate control for each bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0aSr {
    #[doc = "0: fast"]
    B0 = 0,
    #[doc = "1: fast"]
    B1 = 1,
}
impl From<Gpio0aSr> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0aSr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0aSr {
    type Ux = u8;
}
#[doc = "Field `GPIO0A_SR` reader - GPIO0A slew rate control for each bit"]
pub type Gpio0aSrR = crate::FieldReader<Gpio0aSr>;
impl Gpio0aSrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0aSr> {
        match self.bits {
            0 => Some(Gpio0aSr::B0),
            1 => Some(Gpio0aSr::B1),
            _ => None,
        }
    }
    #[doc = "fast"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0aSr::B0
    }
    #[doc = "fast"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0aSr::B1
    }
}
#[doc = "Field `GPIO0A_SR` writer - GPIO0A slew rate control for each bit"]
pub type Gpio0aSrW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0aSr>;
impl<'a, REG> Gpio0aSrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fast"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0aSr::B0)
    }
    #[doc = "fast"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0aSr::B1)
    }
}
#[doc = "GPIO0B slew rate control for each bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0bSr {
    #[doc = "0: fast"]
    B0 = 0,
    #[doc = "1: fast"]
    B1 = 1,
}
impl From<Gpio0bSr> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0bSr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0bSr {
    type Ux = u8;
}
#[doc = "Field `GPIO0B_SR` reader - GPIO0B slew rate control for each bit"]
pub type Gpio0bSrR = crate::FieldReader<Gpio0bSr>;
impl Gpio0bSrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio0bSr> {
        match self.bits {
            0 => Some(Gpio0bSr::B0),
            1 => Some(Gpio0bSr::B1),
            _ => None,
        }
    }
    #[doc = "fast"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio0bSr::B0
    }
    #[doc = "fast"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio0bSr::B1
    }
}
#[doc = "Field `GPIO0B_SR` writer - GPIO0B slew rate control for each bit"]
pub type Gpio0bSrW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio0bSr>;
impl<'a, REG> Gpio0bSrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fast"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bSr::B0)
    }
    #[doc = "fast"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0bSr::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - GPIO0A slew rate control for each bit"]
    #[inline(always)]
    pub fn gpio0a_sr(&self) -> Gpio0aSrR {
        Gpio0aSrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO0B slew rate control for each bit"]
    #[inline(always)]
    pub fn gpio0b_sr(&self) -> Gpio0bSrR {
        Gpio0bSrR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO0A slew rate control for each bit"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0a_sr(&mut self) -> Gpio0aSrW<PmugrfGpio0lSrSpec> {
        Gpio0aSrW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO0B slew rate control for each bit"]
    #[inline(always)]
    #[must_use]
    pub fn gpio0b_sr(&mut self) -> Gpio0bSrW<PmugrfGpio0lSrSpec> {
        Gpio0bSrW::new(self, 8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<PmugrfGpio0lSrSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO0 A/B SR control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio0l_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio0l_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmugrfGpio0lSrSpec;
impl crate::RegisterSpec for PmugrfGpio0lSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmugrf_gpio0l_sr::R`](R) reader structure"]
impl crate::Readable for PmugrfGpio0lSrSpec {}
#[doc = "`write(|w| ..)` method takes [`pmugrf_gpio0l_sr::W`](W) writer structure"]
impl crate::Writable for PmugrfGpio0lSrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUGRF_GPIO0L_SR to value 0"]
impl crate::Resettable for PmugrfGpio0lSrSpec {
    const RESET_VALUE: u32 = 0;
}
