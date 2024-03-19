#[doc = "Register `INT_STATE_1` reader"]
pub type R = crate::R<IntState1Spec>;
#[doc = "Register `INT_STATE_1` writer"]
pub type W = crate::W<IntState1Spec>;
#[doc = "Interrupt request status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntState {
    #[doc = "1: Interrupt service is requested,"]
    B1 = 1,
    #[doc = "0: No interrupt service is requested."]
    B0 = 0,
}
impl From<IntState> for bool {
    #[inline(always)]
    fn from(variant: IntState) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_STATE` reader - Interrupt request status"]
pub type IntStateR = crate::BitReader<IntState>;
impl IntStateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntState {
        match self.bits {
            true => IntState::B1,
            false => IntState::B0,
        }
    }
    #[doc = "Interrupt service is requested,"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntState::B1
    }
    #[doc = "No interrupt service is requested."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntState::B0
    }
}
#[doc = "Field `INT_STATE` writer - Interrupt request status"]
pub type IntStateW<'a, REG> = crate::BitWriter<'a, REG, IntState>;
impl<'a, REG> IntStateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt service is requested,"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntState::B1)
    }
    #[doc = "No interrupt service is requested."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntState::B0)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt request status"]
    #[inline(always)]
    pub fn int_state(&self) -> IntStateR {
        IntStateR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt request status"]
    #[inline(always)]
    #[must_use]
    pub fn int_state(&mut self) -> IntStateW<IntState1Spec> {
        IntStateW::new(self, 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_state_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_state_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntState1Spec;
impl crate::RegisterSpec for IntState1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_state_1::R`](R) reader structure"]
impl crate::Readable for IntState1Spec {}
#[doc = "`write(|w| ..)` method takes [`int_state_1::W`](W) writer structure"]
impl crate::Writable for IntState1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_STATE_1 to value 0"]
impl crate::Resettable for IntState1Spec {
    const RESET_VALUE: u32 = 0;
}
