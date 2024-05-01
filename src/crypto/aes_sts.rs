#[doc = "Register `AES_STS` reader"]
pub type R = crate::R<AesStsSpec>;
#[doc = "Register `AES_STS` writer"]
pub type W = crate::W<AesStsSpec>;
#[doc = "When AES finish, it will be HIGH, And it will not be LOW until it\n\nrestart .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AesDone {
    #[doc = "1: done"]
    B1 = 1,
    #[doc = "0: not done"]
    B0 = 0,
}
impl From<AesDone> for bool {
    #[inline(always)]
    fn from(variant: AesDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AES_DONE` reader - When AES finish, it will be HIGH, And it will not be LOW until it\n\nrestart ."]
pub type AesDoneR = crate::BitReader<AesDone>;
impl AesDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AesDone {
        match self.bits {
            true => AesDone::B1,
            false => AesDone::B0,
        }
    }
    #[doc = "done"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AesDone::B1
    }
    #[doc = "not done"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AesDone::B0
    }
}
#[doc = "Field `AES_DONE` writer - When AES finish, it will be HIGH, And it will not be LOW until it\n\nrestart ."]
pub type AesDoneW<'a, REG> = crate::BitWriter<'a, REG, AesDone>;
impl<'a, REG> AesDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "done"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AesDone::B1)
    }
    #[doc = "not done"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AesDone::B0)
    }
}
impl R {
    #[doc = "Bit 0 - When AES finish, it will be HIGH, And it will not be LOW until it\n\nrestart ."]
    #[inline(always)]
    pub fn aes_done(&self) -> AesDoneR {
        AesDoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When AES finish, it will be HIGH, And it will not be LOW until it\n\nrestart ."]
    #[inline(always)]
    #[must_use]
    pub fn aes_done(&mut self) -> AesDoneW<AesStsSpec> {
        AesDoneW::new(self, 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesStsSpec;
impl crate::RegisterSpec for AesStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_sts::R`](R) reader structure"]
impl crate::Readable for AesStsSpec {}
#[doc = "`write(|w| ..)` method takes [`aes_sts::W`](W) writer structure"]
impl crate::Writable for AesStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_STS to value 0"]
impl crate::Resettable for AesStsSpec {
    const RESET_VALUE: u32 = 0;
}
