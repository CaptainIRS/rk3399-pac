#[doc = "Register `FC_AUDSU` reader"]
pub type R = crate::R<FcAudsuSpec>;
#[doc = "Register `FC_AUDSU` writer"]
pub type W = crate::W<FcAudsuSpec>;
#[doc = "Field `U0L` reader - Set user bit \"U\" for Channel 0, Left"]
pub type U0lR = crate::BitReader;
#[doc = "Field `U0L` writer - Set user bit \"U\" for Channel 0, Left"]
pub type U0lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `U1L` reader - Set user bit \"U\" for Channel 1, Left"]
pub type U1lR = crate::BitReader;
#[doc = "Field `U1L` writer - Set user bit \"U\" for Channel 1, Left"]
pub type U1lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `U2L` reader - Set user bit \"U\" for Channel 2, Left"]
pub type U2lR = crate::BitReader;
#[doc = "Field `U2L` writer - Set user bit \"U\" for Channel 2, Left"]
pub type U2lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `U3L` reader - Set user bit \"U\" for Channel 3, Left"]
pub type U3lR = crate::BitReader;
#[doc = "Field `U3L` writer - Set user bit \"U\" for Channel 3, Left"]
pub type U3lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `U0R` reader - Set user bit \"U\" for Channel 0, Right"]
pub type U0rR = crate::BitReader;
#[doc = "Field `U0R` writer - Set user bit \"U\" for Channel 0, Right"]
pub type U0rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `U1R` reader - Set user bit \"U\" for Channel 1, Right"]
pub type U1rR = crate::BitReader;
#[doc = "Field `U1R` writer - Set user bit \"U\" for Channel 1, Right"]
pub type U1rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `U2R` reader - Set user bit \"U\" for Channel 2, Right"]
pub type U2rR = crate::BitReader;
#[doc = "Field `U2R` writer - Set user bit \"U\" for Channel 2, Right"]
pub type U2rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `U3R` reader - Set user bit \"U\" for Channel 3, Right"]
pub type U3rR = crate::BitReader;
#[doc = "Field `U3R` writer - Set user bit \"U\" for Channel 3, Right"]
pub type U3rW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set user bit \"U\" for Channel 0, Left"]
    #[inline(always)]
    pub fn u0l(&self) -> U0lR {
        U0lR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set user bit \"U\" for Channel 1, Left"]
    #[inline(always)]
    pub fn u1l(&self) -> U1lR {
        U1lR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set user bit \"U\" for Channel 2, Left"]
    #[inline(always)]
    pub fn u2l(&self) -> U2lR {
        U2lR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set user bit \"U\" for Channel 3, Left"]
    #[inline(always)]
    pub fn u3l(&self) -> U3lR {
        U3lR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set user bit \"U\" for Channel 0, Right"]
    #[inline(always)]
    pub fn u0r(&self) -> U0rR {
        U0rR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set user bit \"U\" for Channel 1, Right"]
    #[inline(always)]
    pub fn u1r(&self) -> U1rR {
        U1rR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set user bit \"U\" for Channel 2, Right"]
    #[inline(always)]
    pub fn u2r(&self) -> U2rR {
        U2rR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set user bit \"U\" for Channel 3, Right"]
    #[inline(always)]
    pub fn u3r(&self) -> U3rR {
        U3rR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set user bit \"U\" for Channel 0, Left"]
    #[inline(always)]
    #[must_use]
    pub fn u0l(&mut self) -> U0lW<FcAudsuSpec> {
        U0lW::new(self, 0)
    }
    #[doc = "Bit 1 - Set user bit \"U\" for Channel 1, Left"]
    #[inline(always)]
    #[must_use]
    pub fn u1l(&mut self) -> U1lW<FcAudsuSpec> {
        U1lW::new(self, 1)
    }
    #[doc = "Bit 2 - Set user bit \"U\" for Channel 2, Left"]
    #[inline(always)]
    #[must_use]
    pub fn u2l(&mut self) -> U2lW<FcAudsuSpec> {
        U2lW::new(self, 2)
    }
    #[doc = "Bit 3 - Set user bit \"U\" for Channel 3, Left"]
    #[inline(always)]
    #[must_use]
    pub fn u3l(&mut self) -> U3lW<FcAudsuSpec> {
        U3lW::new(self, 3)
    }
    #[doc = "Bit 4 - Set user bit \"U\" for Channel 0, Right"]
    #[inline(always)]
    #[must_use]
    pub fn u0r(&mut self) -> U0rW<FcAudsuSpec> {
        U0rW::new(self, 4)
    }
    #[doc = "Bit 5 - Set user bit \"U\" for Channel 1, Right"]
    #[inline(always)]
    #[must_use]
    pub fn u1r(&mut self) -> U1rW<FcAudsuSpec> {
        U1rW::new(self, 5)
    }
    #[doc = "Bit 6 - Set user bit \"U\" for Channel 2, Right"]
    #[inline(always)]
    #[must_use]
    pub fn u2r(&mut self) -> U2rW<FcAudsuSpec> {
        U2rW::new(self, 6)
    }
    #[doc = "Bit 7 - Set user bit \"U\" for Channel 3, Right"]
    #[inline(always)]
    #[must_use]
    pub fn u3r(&mut self) -> U3rW<FcAudsuSpec> {
        U3rW::new(self, 7)
    }
}
#[doc = "Set user bit \"U\" for Channel 0, Left\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_audsu::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_audsu::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcAudsuSpec;
impl crate::RegisterSpec for FcAudsuSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_audsu::R`](R) reader structure"]
impl crate::Readable for FcAudsuSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_audsu::W`](W) writer structure"]
impl crate::Writable for FcAudsuSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_AUDSU to value 0"]
impl crate::Resettable for FcAudsuSpec {
    const RESET_VALUE: u8 = 0;
}
