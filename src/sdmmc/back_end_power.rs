#[doc = "Register `BACK_END_POWER` reader"]
pub type R = crate::R<BackEndPowerSpec>;
#[doc = "Register `BACK_END_POWER` writer"]
pub type W = crate::W<BackEndPowerSpec>;
#[doc = "Back end power\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackEndPower {
    #[doc = "0: Off; Reset"]
    B0 = 0,
    #[doc = "1: Back-end Power supplied to card application"]
    B1 = 1,
}
impl From<BackEndPower> for bool {
    #[inline(always)]
    fn from(variant: BackEndPower) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BACK_END_POWER` reader - Back end power"]
pub type BackEndPowerR = crate::BitReader<BackEndPower>;
impl BackEndPowerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BackEndPower {
        match self.bits {
            false => BackEndPower::B0,
            true => BackEndPower::B1,
        }
    }
    #[doc = "Off; Reset"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BackEndPower::B0
    }
    #[doc = "Back-end Power supplied to card application"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BackEndPower::B1
    }
}
#[doc = "Field `BACK_END_POWER` writer - Back end power"]
pub type BackEndPowerW<'a, REG> = crate::BitWriter<'a, REG, BackEndPower>;
impl<'a, REG> BackEndPowerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Off; Reset"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BackEndPower::B0)
    }
    #[doc = "Back-end Power supplied to card application"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BackEndPower::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Back end power"]
    #[inline(always)]
    pub fn back_end_power(&self) -> BackEndPowerR {
        BackEndPowerR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Back end power"]
    #[inline(always)]
    #[must_use]
    pub fn back_end_power(&mut self) -> BackEndPowerW<BackEndPowerSpec> {
        BackEndPowerW::new(self, 0)
    }
}
#[doc = "Back-end power register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`back_end_power::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`back_end_power::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BackEndPowerSpec;
impl crate::RegisterSpec for BackEndPowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`back_end_power::R`](R) reader structure"]
impl crate::Readable for BackEndPowerSpec {}
#[doc = "`write(|w| ..)` method takes [`back_end_power::W`](W) writer structure"]
impl crate::Writable for BackEndPowerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BACK_END_POWER to value 0"]
impl crate::Resettable for BackEndPowerSpec {
    const RESET_VALUE: u32 = 0;
}
