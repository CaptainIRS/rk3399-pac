#[doc = "Register `HASH_STS` reader"]
pub type R = crate::R<HashStsSpec>;
#[doc = "Register `HASH_STS` writer"]
pub type W = crate::W<HashStsSpec>;
#[doc = "Hash Done Signal\n\nWhen HASH finishes, it will be HIGH, And it will not be LOW until it\n\nrestart\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashDone {
    #[doc = "1: done"]
    B1 = 1,
    #[doc = "0: not done"]
    B0 = 0,
}
impl From<HashDone> for bool {
    #[inline(always)]
    fn from(variant: HashDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HASH_DONE` reader - Hash Done Signal\n\nWhen HASH finishes, it will be HIGH, And it will not be LOW until it\n\nrestart"]
pub type HashDoneR = crate::BitReader<HashDone>;
impl HashDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HashDone {
        match self.bits {
            true => HashDone::B1,
            false => HashDone::B0,
        }
    }
    #[doc = "done"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HashDone::B1
    }
    #[doc = "not done"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HashDone::B0
    }
}
#[doc = "Field `HASH_DONE` writer - Hash Done Signal\n\nWhen HASH finishes, it will be HIGH, And it will not be LOW until it\n\nrestart"]
pub type HashDoneW<'a, REG> = crate::BitWriter<'a, REG, HashDone>;
impl<'a, REG> HashDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "done"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HashDone::B1)
    }
    #[doc = "not done"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HashDone::B0)
    }
}
impl R {
    #[doc = "Bit 0 - Hash Done Signal\n\nWhen HASH finishes, it will be HIGH, And it will not be LOW until it\n\nrestart"]
    #[inline(always)]
    pub fn hash_done(&self) -> HashDoneR {
        HashDoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hash Done Signal\n\nWhen HASH finishes, it will be HIGH, And it will not be LOW until it\n\nrestart"]
    #[inline(always)]
    #[must_use]
    pub fn hash_done(&mut self) -> HashDoneW<HashStsSpec> {
        HashDoneW::new(self, 0)
    }
}
#[doc = "Hash Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashStsSpec;
impl crate::RegisterSpec for HashStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_sts::R`](R) reader structure"]
impl crate::Readable for HashStsSpec {}
#[doc = "`write(|w| ..)` method takes [`hash_sts::W`](W) writer structure"]
impl crate::Writable for HashStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_STS to value 0"]
impl crate::Resettable for HashStsSpec {
    const RESET_VALUE: u32 = 0;
}
