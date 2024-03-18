#[doc = "Register `TIMER_n_CONTROLREG` reader"]
pub type R = crate::R<TimerNControlregSpec>;
#[doc = "Register `TIMER_n_CONTROLREG` writer"]
pub type W = crate::W<TimerNControlregSpec>;
#[doc = "Timer enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerEn {
    #[doc = "0: enable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<TimerEn> for bool {
    #[inline(always)]
    fn from(variant: TimerEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER_EN` reader - Timer enable."]
pub type TimerEnR = crate::BitReader<TimerEn>;
impl TimerEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimerEn {
        match self.bits {
            false => TimerEn::B0,
            true => TimerEn::B1,
        }
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TimerEn::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TimerEn::B1
    }
}
#[doc = "Field `TIMER_EN` writer - Timer enable."]
pub type TimerEnW<'a, REG> = crate::BitWriter<'a, REG, TimerEn>;
impl<'a, REG> TimerEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEn::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TimerEn::B1)
    }
}
#[doc = "Timer mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerMode {
    #[doc = "0: user-defined count mode"]
    B0 = 0,
    #[doc = "1: user-defined count mode"]
    B1 = 1,
}
impl From<TimerMode> for bool {
    #[inline(always)]
    fn from(variant: TimerMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER_MODE` reader - Timer mode."]
pub type TimerModeR = crate::BitReader<TimerMode>;
impl TimerModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimerMode {
        match self.bits {
            false => TimerMode::B0,
            true => TimerMode::B1,
        }
    }
    #[doc = "user-defined count mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TimerMode::B0
    }
    #[doc = "user-defined count mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TimerMode::B1
    }
}
#[doc = "Field `TIMER_MODE` writer - Timer mode."]
pub type TimerModeW<'a, REG> = crate::BitWriter<'a, REG, TimerMode>;
impl<'a, REG> TimerModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "user-defined count mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TimerMode::B0)
    }
    #[doc = "user-defined count mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TimerMode::B1)
    }
}
#[doc = "Timer interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEn {
    #[doc = "0: not mask"]
    B0 = 0,
    #[doc = "1: not mask"]
    B1 = 1,
}
impl From<IntEn> for bool {
    #[inline(always)]
    fn from(variant: IntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EN` reader - Timer interrupt mask"]
pub type IntEnR = crate::BitReader<IntEn>;
impl IntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEn {
        match self.bits {
            false => IntEn::B0,
            true => IntEn::B1,
        }
    }
    #[doc = "not mask"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntEn::B0
    }
    #[doc = "not mask"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntEn::B1
    }
}
#[doc = "Field `INT_EN` writer - Timer interrupt mask"]
pub type IntEnW<'a, REG> = crate::BitWriter<'a, REG, IntEn>;
impl<'a, REG> IntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "not mask"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntEn::B0)
    }
    #[doc = "not mask"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntEn::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Timer enable."]
    #[inline(always)]
    pub fn timer_en(&self) -> TimerEnR {
        TimerEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer mode."]
    #[inline(always)]
    pub fn timer_mode(&self) -> TimerModeR {
        TimerModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer interrupt mask"]
    #[inline(always)]
    pub fn int_en(&self) -> IntEnR {
        IntEnR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer enable."]
    #[inline(always)]
    #[must_use]
    pub fn timer_en(&mut self) -> TimerEnW<TimerNControlregSpec> {
        TimerEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer mode."]
    #[inline(always)]
    #[must_use]
    pub fn timer_mode(&mut self) -> TimerModeW<TimerNControlregSpec> {
        TimerModeW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn int_en(&mut self) -> IntEnW<TimerNControlregSpec> {
        IntEnW::new(self, 2)
    }
}
#[doc = "Timer n Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_n_controlreg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_n_controlreg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimerNControlregSpec;
impl crate::RegisterSpec for TimerNControlregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_n_controlreg::R`](R) reader structure"]
impl crate::Readable for TimerNControlregSpec {}
#[doc = "`write(|w| ..)` method takes [`timer_n_controlreg::W`](W) writer structure"]
impl crate::Writable for TimerNControlregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER_n_CONTROLREG to value 0"]
impl crate::Resettable for TimerNControlregSpec {
    const RESET_VALUE: u32 = 0;
}
