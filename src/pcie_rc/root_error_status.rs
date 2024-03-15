#[doc = "Register `ROOT_ERROR_STATUS` reader"]
pub type R = crate::R<RootErrorStatusSpec>;
#[doc = "Register `ROOT_ERROR_STATUS` writer"]
pub type W = crate::W<RootErrorStatusSpec>;
#[doc = "Field `ECR` reader - Correctable Error Message Received \\[ECR\\]
This bit is set when the RC receives a Correctable error message from the link. STICKY"]
pub type EcrR = crate::BitReader;
#[doc = "Field `ECR` writer - Correctable Error Message Received \\[ECR\\]
This bit is set when the RC receives a Correctable error message from the link. STICKY"]
pub type EcrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MECR` reader - Multiple Correctable Error Messages Received \\[MECR\\]
This bit is set when the RC receives a Correctable error message from the link, if the ERR_COR received bit is already set. STICKY"]
pub type MecrR = crate::BitReader;
#[doc = "Field `MECR` writer - Multiple Correctable Error Messages Received \\[MECR\\]
This bit is set when the RC receives a Correctable error message from the link, if the ERR_COR received bit is already set. STICKY"]
pub type MecrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EFNR` reader - Fatal/Non- Fatal Error Message Received \\[EFNR\\]
This bit is set when the RC receives either a Fatal or Non-Fatal error message from the link. STICKY"]
pub type EfnrR = crate::BitReader;
#[doc = "Field `EFNR` writer - Fatal/Non- Fatal Error Message Received \\[EFNR\\]
This bit is set when the RC receives either a Fatal or Non-Fatal error message from the link. STICKY"]
pub type EfnrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MEFNR` reader - Multiple Fatal/ Non- Fatal Error Messages Received \\[MEFNR\\]
This bit is set when the RC receives either a Fatal or Non-Fatal error message from the link, and the ERR_FATAL/NONFATAL Received bit is already set. STICKY"]
pub type MefnrR = crate::BitReader;
#[doc = "Field `MEFNR` writer - Multiple Fatal/ Non- Fatal Error Messages Received \\[MEFNR\\]
This bit is set when the RC receives either a Fatal or Non-Fatal error message from the link, and the ERR_FATAL/NONFATAL Received bit is already set. STICKY"]
pub type MefnrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FUF` reader - First Uncorrectable Fatal \\[FUF\\]
This bit, when set, indicates that the first Uncorrectable error message received was for a Fatal error. STICKY"]
pub type FufR = crate::BitReader;
#[doc = "Field `FUF` writer - First Uncorrectable Fatal \\[FUF\\]
This bit, when set, indicates that the first Uncorrectable error message received was for a Fatal error. STICKY"]
pub type FufW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `NEMR` reader - Non-Fatal Error Messages Received \\[NEMR\\]
This bit, when set, indicates that the RC has received one or more Non- Fatal error messages from the link. STICKY"]
pub type NemrR = crate::BitReader;
#[doc = "Field `NEMR` writer - Non-Fatal Error Messages Received \\[NEMR\\]
This bit, when set, indicates that the RC has received one or more Non- Fatal error messages from the link. STICKY"]
pub type NemrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FEMR` reader - Fatal Error Messages Received \\[FEMR\\]
This bit, when set, indicates that the RC has received one or more Fatal error messages from the link. STICKY"]
pub type FemrR = crate::BitReader;
#[doc = "Field `FEMR` writer - Fatal Error Messages Received \\[FEMR\\]
This bit, when set, indicates that the RC has received one or more Fatal error messages from the link. STICKY"]
pub type FemrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R45` reader - Reserved \\[R45\\]
Reserved"]
pub type R45R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Correctable Error Message Received \\[ECR\\]
This bit is set when the RC receives a Correctable error message from the link. STICKY"]
    #[inline(always)]
    pub fn ecr(&self) -> EcrR {
        EcrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Multiple Correctable Error Messages Received \\[MECR\\]
This bit is set when the RC receives a Correctable error message from the link, if the ERR_COR received bit is already set. STICKY"]
    #[inline(always)]
    pub fn mecr(&self) -> MecrR {
        MecrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fatal/Non- Fatal Error Message Received \\[EFNR\\]
This bit is set when the RC receives either a Fatal or Non-Fatal error message from the link. STICKY"]
    #[inline(always)]
    pub fn efnr(&self) -> EfnrR {
        EfnrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Multiple Fatal/ Non- Fatal Error Messages Received \\[MEFNR\\]
This bit is set when the RC receives either a Fatal or Non-Fatal error message from the link, and the ERR_FATAL/NONFATAL Received bit is already set. STICKY"]
    #[inline(always)]
    pub fn mefnr(&self) -> MefnrR {
        MefnrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - First Uncorrectable Fatal \\[FUF\\]
This bit, when set, indicates that the first Uncorrectable error message received was for a Fatal error. STICKY"]
    #[inline(always)]
    pub fn fuf(&self) -> FufR {
        FufR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Non-Fatal Error Messages Received \\[NEMR\\]
This bit, when set, indicates that the RC has received one or more Non- Fatal error messages from the link. STICKY"]
    #[inline(always)]
    pub fn nemr(&self) -> NemrR {
        NemrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fatal Error Messages Received \\[FEMR\\]
This bit, when set, indicates that the RC has received one or more Fatal error messages from the link. STICKY"]
    #[inline(always)]
    pub fn femr(&self) -> FemrR {
        FemrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:31 - Reserved \\[R45\\]
Reserved"]
    #[inline(always)]
    pub fn r45(&self) -> R45R {
        R45R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Correctable Error Message Received \\[ECR\\]
This bit is set when the RC receives a Correctable error message from the link. STICKY"]
    #[inline(always)]
    #[must_use]
    pub fn ecr(&mut self) -> EcrW<RootErrorStatusSpec> {
        EcrW::new(self, 0)
    }
    #[doc = "Bit 1 - Multiple Correctable Error Messages Received \\[MECR\\]
This bit is set when the RC receives a Correctable error message from the link, if the ERR_COR received bit is already set. STICKY"]
    #[inline(always)]
    #[must_use]
    pub fn mecr(&mut self) -> MecrW<RootErrorStatusSpec> {
        MecrW::new(self, 1)
    }
    #[doc = "Bit 2 - Fatal/Non- Fatal Error Message Received \\[EFNR\\]
This bit is set when the RC receives either a Fatal or Non-Fatal error message from the link. STICKY"]
    #[inline(always)]
    #[must_use]
    pub fn efnr(&mut self) -> EfnrW<RootErrorStatusSpec> {
        EfnrW::new(self, 2)
    }
    #[doc = "Bit 3 - Multiple Fatal/ Non- Fatal Error Messages Received \\[MEFNR\\]
This bit is set when the RC receives either a Fatal or Non-Fatal error message from the link, and the ERR_FATAL/NONFATAL Received bit is already set. STICKY"]
    #[inline(always)]
    #[must_use]
    pub fn mefnr(&mut self) -> MefnrW<RootErrorStatusSpec> {
        MefnrW::new(self, 3)
    }
    #[doc = "Bit 4 - First Uncorrectable Fatal \\[FUF\\]
This bit, when set, indicates that the first Uncorrectable error message received was for a Fatal error. STICKY"]
    #[inline(always)]
    #[must_use]
    pub fn fuf(&mut self) -> FufW<RootErrorStatusSpec> {
        FufW::new(self, 4)
    }
    #[doc = "Bit 5 - Non-Fatal Error Messages Received \\[NEMR\\]
This bit, when set, indicates that the RC has received one or more Non- Fatal error messages from the link. STICKY"]
    #[inline(always)]
    #[must_use]
    pub fn nemr(&mut self) -> NemrW<RootErrorStatusSpec> {
        NemrW::new(self, 5)
    }
    #[doc = "Bit 6 - Fatal Error Messages Received \\[FEMR\\]
This bit, when set, indicates that the RC has received one or more Fatal error messages from the link. STICKY"]
    #[inline(always)]
    #[must_use]
    pub fn femr(&mut self) -> FemrW<RootErrorStatusSpec> {
        FemrW::new(self, 6)
    }
}
#[doc = "Root Error Status Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`root_error_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`root_error_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RootErrorStatusSpec;
impl crate::RegisterSpec for RootErrorStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`root_error_status::R`](R) reader structure"]
impl crate::Readable for RootErrorStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`root_error_status::W`](W) writer structure"]
impl crate::Writable for RootErrorStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x7f;
}
#[doc = "`reset()` method sets ROOT_ERROR_STATUS to value 0"]
impl crate::Resettable for RootErrorStatusSpec {
    const RESET_VALUE: u32 = 0;
}
