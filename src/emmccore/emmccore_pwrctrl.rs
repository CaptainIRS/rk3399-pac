#[doc = "Register `EMMCCORE_PWRCTRL` reader"]
pub type R = crate::R<EmmccorePwrctrlSpec>;
#[doc = "Register `EMMCCORE_PWRCTRL` writer"]
pub type W = crate::W<EmmccorePwrctrlSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdbuspower {
    #[doc = "1: Power on"]
    B1 = 1,
    #[doc = "0: Power off"]
    B0 = 0,
}
impl From<Sdbuspower> for bool {
    #[inline(always)]
    fn from(variant: Sdbuspower) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDBUSPOWER` reader - "]
pub type SdbuspowerR = crate::BitReader<Sdbuspower>;
impl SdbuspowerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdbuspower {
        match self.bits {
            true => Sdbuspower::B1,
            false => Sdbuspower::B0,
        }
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sdbuspower::B1
    }
    #[doc = "Power off"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sdbuspower::B0
    }
}
#[doc = "Field `SDBUSPOWER` writer - "]
pub type SdbuspowerW<'a, REG> = crate::BitWriter<'a, REG, Sdbuspower>;
impl<'a, REG> SdbuspowerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power on"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdbuspower::B1)
    }
    #[doc = "Power off"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sdbuspower::B0)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sdbuspower(&self) -> SdbuspowerR {
        SdbuspowerR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sdbuspower(&mut self) -> SdbuspowerW<EmmccorePwrctrlSpec> {
        SdbuspowerW::new(self, 0)
    }
}
#[doc = "Power control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_pwrctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_pwrctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccorePwrctrlSpec;
impl crate::RegisterSpec for EmmccorePwrctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`emmccore_pwrctrl::R`](R) reader structure"]
impl crate::Readable for EmmccorePwrctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_pwrctrl::W`](W) writer structure"]
impl crate::Writable for EmmccorePwrctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_PWRCTRL to value 0"]
impl crate::Resettable for EmmccorePwrctrlSpec {
    const RESET_VALUE: u8 = 0;
}
