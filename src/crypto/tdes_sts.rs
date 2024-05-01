#[doc = "Register `TDES_STS` reader"]
pub type R = crate::R<TdesStsSpec>;
#[doc = "Register `TDES_STS` writer"]
pub type W = crate::W<TdesStsSpec>;
#[doc = "When DES/TDES finishes, it will be HIGH, And it will not be LOW\n\nuntil it restart .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdesDone {
    #[doc = "1: done"]
    B1 = 1,
    #[doc = "0: not done"]
    B0 = 0,
}
impl From<TdesDone> for bool {
    #[inline(always)]
    fn from(variant: TdesDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDES_DONE` reader - When DES/TDES finishes, it will be HIGH, And it will not be LOW\n\nuntil it restart ."]
pub type TdesDoneR = crate::BitReader<TdesDone>;
impl TdesDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TdesDone {
        match self.bits {
            true => TdesDone::B1,
            false => TdesDone::B0,
        }
    }
    #[doc = "done"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TdesDone::B1
    }
    #[doc = "not done"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TdesDone::B0
    }
}
#[doc = "Field `TDES_DONE` writer - When DES/TDES finishes, it will be HIGH, And it will not be LOW\n\nuntil it restart ."]
pub type TdesDoneW<'a, REG> = crate::BitWriter<'a, REG, TdesDone>;
impl<'a, REG> TdesDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "done"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TdesDone::B1)
    }
    #[doc = "not done"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TdesDone::B0)
    }
}
impl R {
    #[doc = "Bit 0 - When DES/TDES finishes, it will be HIGH, And it will not be LOW\n\nuntil it restart ."]
    #[inline(always)]
    pub fn tdes_done(&self) -> TdesDoneR {
        TdesDoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When DES/TDES finishes, it will be HIGH, And it will not be LOW\n\nuntil it restart ."]
    #[inline(always)]
    #[must_use]
    pub fn tdes_done(&mut self) -> TdesDoneW<TdesStsSpec> {
        TdesDoneW::new(self, 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdes_sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdes_sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdesStsSpec;
impl crate::RegisterSpec for TdesStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdes_sts::R`](R) reader structure"]
impl crate::Readable for TdesStsSpec {}
#[doc = "`write(|w| ..)` method takes [`tdes_sts::W`](W) writer structure"]
impl crate::Writable for TdesStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TDES_STS to value 0"]
impl crate::Resettable for TdesStsSpec {
    const RESET_VALUE: u32 = 0;
}
