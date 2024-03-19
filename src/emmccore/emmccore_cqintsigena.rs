#[doc = "Register `EMMCCORE_CQINTSIGENA` reader"]
pub type R = crate::R<EmmccoreCqintsigenaSpec>;
#[doc = "Register `EMMCCORE_CQINTSIGENA` writer"]
pub type W = crate::W<EmmccoreCqintsigenaSpec>;
#[doc = "Halt Complete Interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hac {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<Hac> for bool {
    #[inline(always)]
    fn from(variant: Hac) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HAC` reader - Halt Complete Interrupt"]
pub type HacR = crate::BitReader<Hac>;
impl HacR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hac {
        match self.bits {
            true => Hac::B1,
            false => Hac::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Hac::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Hac::B0
    }
}
#[doc = "Field `HAC` writer - Halt Complete Interrupt"]
pub type HacW<'a, REG> = crate::BitWriter<'a, REG, Hac>;
impl<'a, REG> HacW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Hac::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Hac::B0)
    }
}
#[doc = "Task Complete Interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcc {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<Tcc> for bool {
    #[inline(always)]
    fn from(variant: Tcc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCC` reader - Task Complete Interrupt"]
pub type TccR = crate::BitReader<Tcc>;
impl TccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcc {
        match self.bits {
            true => Tcc::B1,
            false => Tcc::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tcc::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tcc::B0
    }
}
#[doc = "Field `TCC` writer - Task Complete Interrupt"]
pub type TccW<'a, REG> = crate::BitWriter<'a, REG, Tcc>;
impl<'a, REG> TccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcc::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcc::B0)
    }
}
#[doc = "Response Error Detected Interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Red {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<Red> for bool {
    #[inline(always)]
    fn from(variant: Red) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RED` reader - Response Error Detected Interrupt"]
pub type RedR = crate::BitReader<Red>;
impl RedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Red {
        match self.bits {
            true => Red::B1,
            false => Red::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Red::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Red::B0
    }
}
#[doc = "Field `RED` writer - Response Error Detected Interrupt"]
pub type RedW<'a, REG> = crate::BitWriter<'a, REG, Red>;
impl<'a, REG> RedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Red::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Red::B0)
    }
}
#[doc = "Task Cleared\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcl {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<Tcl> for bool {
    #[inline(always)]
    fn from(variant: Tcl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCL` reader - Task Cleared"]
pub type TclR = crate::BitReader<Tcl>;
impl TclR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcl {
        match self.bits {
            true => Tcl::B1,
            false => Tcl::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tcl::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tcl::B0
    }
}
#[doc = "Field `TCL` writer - Task Cleared"]
pub type TclW<'a, REG> = crate::BitWriter<'a, REG, Tcl>;
impl<'a, REG> TclW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcl::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcl::B0)
    }
}
#[doc = "Task Error Interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Terr {
    #[doc = "1: enable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<Terr> for bool {
    #[inline(always)]
    fn from(variant: Terr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TERR` reader - Task Error Interrupt"]
pub type TerrR = crate::BitReader<Terr>;
impl TerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Terr {
        match self.bits {
            true => Terr::B1,
            false => Terr::B0,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Terr::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Terr::B0
    }
}
#[doc = "Field `TERR` writer - Task Error Interrupt"]
pub type TerrW<'a, REG> = crate::BitWriter<'a, REG, Terr>;
impl<'a, REG> TerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Terr::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Terr::B0)
    }
}
impl R {
    #[doc = "Bit 0 - Halt Complete Interrupt"]
    #[inline(always)]
    pub fn hac(&self) -> HacR {
        HacR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Task Complete Interrupt"]
    #[inline(always)]
    pub fn tcc(&self) -> TccR {
        TccR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Response Error Detected Interrupt"]
    #[inline(always)]
    pub fn red(&self) -> RedR {
        RedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Task Cleared"]
    #[inline(always)]
    pub fn tcl(&self) -> TclR {
        TclR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Task Error Interrupt"]
    #[inline(always)]
    pub fn terr(&self) -> TerrR {
        TerrR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Halt Complete Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn hac(&mut self) -> HacW<EmmccoreCqintsigenaSpec> {
        HacW::new(self, 0)
    }
    #[doc = "Bit 1 - Task Complete Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tcc(&mut self) -> TccW<EmmccoreCqintsigenaSpec> {
        TccW::new(self, 1)
    }
    #[doc = "Bit 2 - Response Error Detected Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn red(&mut self) -> RedW<EmmccoreCqintsigenaSpec> {
        RedW::new(self, 2)
    }
    #[doc = "Bit 3 - Task Cleared"]
    #[inline(always)]
    #[must_use]
    pub fn tcl(&mut self) -> TclW<EmmccoreCqintsigenaSpec> {
        TclW::new(self, 3)
    }
    #[doc = "Bit 4 - Task Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn terr(&mut self) -> TerrW<EmmccoreCqintsigenaSpec> {
        TerrW::new(self, 4)
    }
}
#[doc = "Command queueing interrupt signal enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cqintsigena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cqintsigena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreCqintsigenaSpec;
impl crate::RegisterSpec for EmmccoreCqintsigenaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_cqintsigena::R`](R) reader structure"]
impl crate::Readable for EmmccoreCqintsigenaSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_cqintsigena::W`](W) writer structure"]
impl crate::Writable for EmmccoreCqintsigenaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_CQINTSIGENA to value 0"]
impl crate::Resettable for EmmccoreCqintsigenaSpec {
    const RESET_VALUE: u32 = 0;
}
