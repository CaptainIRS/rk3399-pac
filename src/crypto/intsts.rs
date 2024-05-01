#[doc = "Register `INTSTS` reader"]
pub type R = crate::R<IntstsSpec>;
#[doc = "Register `INTSTS` writer"]
pub type W = crate::W<IntstsSpec>;
#[doc = "Field `BCDMA_DONE_INT` reader - Specifies the interrupt of block cipher Done"]
pub type BcdmaDoneIntR = crate::BitReader;
#[doc = "Field `BCDMA_DONE_INT` writer - Specifies the interrupt of block cipher Done"]
pub type BcdmaDoneIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BCDMA_ERR_INT` reader - Specifies the interrupt of block cipher Error"]
pub type BcdmaErrIntR = crate::BitReader;
#[doc = "Field `BCDMA_ERR_INT` writer - Specifies the interrupt of block cipher Error"]
pub type BcdmaErrIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `HRDMA_DONE_INT` reader - Specifies the interrupt of hash receiving DMA Done"]
pub type HrdmaDoneIntR = crate::BitReader;
#[doc = "Field `HRDMA_DONE_INT` writer - Specifies the interrupt of hash receiving DMA Done"]
pub type HrdmaDoneIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `HRDMA_ERR_INT` reader - Specifies the interrupt of hash receiving DMA Error"]
pub type HrdmaErrIntR = crate::BitReader;
#[doc = "Field `HRDMA_ERR_INT` writer - Specifies the interrupt of hash receiving DMA Error"]
pub type HrdmaErrIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `HASH_DONE_INT` reader - Hash Done Interrupt"]
pub type HashDoneIntR = crate::BitReader;
#[doc = "Field `HASH_DONE_INT` writer - Hash Done Interrupt"]
pub type HashDoneIntW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PKA_DONE_INT` reader - PKA Done Interrupt"]
pub type PkaDoneIntR = crate::BitReader;
#[doc = "Field `PKA_DONE_INT` writer - PKA Done Interrupt"]
pub type PkaDoneIntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Specifies the interrupt of block cipher Done"]
    #[inline(always)]
    pub fn bcdma_done_int(&self) -> BcdmaDoneIntR {
        BcdmaDoneIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Specifies the interrupt of block cipher Error"]
    #[inline(always)]
    pub fn bcdma_err_int(&self) -> BcdmaErrIntR {
        BcdmaErrIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Specifies the interrupt of hash receiving DMA Done"]
    #[inline(always)]
    pub fn hrdma_done_int(&self) -> HrdmaDoneIntR {
        HrdmaDoneIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Specifies the interrupt of hash receiving DMA Error"]
    #[inline(always)]
    pub fn hrdma_err_int(&self) -> HrdmaErrIntR {
        HrdmaErrIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Hash Done Interrupt"]
    #[inline(always)]
    pub fn hash_done_int(&self) -> HashDoneIntR {
        HashDoneIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PKA Done Interrupt"]
    #[inline(always)]
    pub fn pka_done_int(&self) -> PkaDoneIntR {
        PkaDoneIntR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies the interrupt of block cipher Done"]
    #[inline(always)]
    #[must_use]
    pub fn bcdma_done_int(&mut self) -> BcdmaDoneIntW<IntstsSpec> {
        BcdmaDoneIntW::new(self, 0)
    }
    #[doc = "Bit 1 - Specifies the interrupt of block cipher Error"]
    #[inline(always)]
    #[must_use]
    pub fn bcdma_err_int(&mut self) -> BcdmaErrIntW<IntstsSpec> {
        BcdmaErrIntW::new(self, 1)
    }
    #[doc = "Bit 2 - Specifies the interrupt of hash receiving DMA Done"]
    #[inline(always)]
    #[must_use]
    pub fn hrdma_done_int(&mut self) -> HrdmaDoneIntW<IntstsSpec> {
        HrdmaDoneIntW::new(self, 2)
    }
    #[doc = "Bit 3 - Specifies the interrupt of hash receiving DMA Error"]
    #[inline(always)]
    #[must_use]
    pub fn hrdma_err_int(&mut self) -> HrdmaErrIntW<IntstsSpec> {
        HrdmaErrIntW::new(self, 3)
    }
    #[doc = "Bit 4 - Hash Done Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn hash_done_int(&mut self) -> HashDoneIntW<IntstsSpec> {
        HashDoneIntW::new(self, 4)
    }
    #[doc = "Bit 5 - PKA Done Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn pka_done_int(&mut self) -> PkaDoneIntW<IntstsSpec> {
        PkaDoneIntW::new(self, 5)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstsSpec;
impl crate::RegisterSpec for IntstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intsts::R`](R) reader structure"]
impl crate::Readable for IntstsSpec {}
#[doc = "`write(|w| ..)` method takes [`intsts::W`](W) writer structure"]
impl crate::Writable for IntstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x1f;
}
#[doc = "`reset()` method sets INTSTS to value 0"]
impl crate::Resettable for IntstsSpec {
    const RESET_VALUE: u32 = 0;
}
