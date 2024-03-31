#[doc = "Register `DBGINST1` writer"]
pub type W = crate::W<Dbginst1Spec>;
#[doc = "Field `DBGINST1_BITS_3` writer - Instruction byte 2"]
pub type Dbginst1Bits3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DBGINST1_BITS_2` writer - Instruction byte 3"]
pub type Dbginst1Bits2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DBGINST1_BITS_1` writer - Instruction byte 4"]
pub type Dbginst1Bits1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DBGINST1_BITS_0` writer - Instruction byte 5"]
pub type Dbginst1Bits0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Instruction byte 2"]
    #[inline(always)]
    #[must_use]
    pub fn dbginst1_bits_3(&mut self) -> Dbginst1Bits3W<Dbginst1Spec> {
        Dbginst1Bits3W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Instruction byte 3"]
    #[inline(always)]
    #[must_use]
    pub fn dbginst1_bits_2(&mut self) -> Dbginst1Bits2W<Dbginst1Spec> {
        Dbginst1Bits2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Instruction byte 4"]
    #[inline(always)]
    #[must_use]
    pub fn dbginst1_bits_1(&mut self) -> Dbginst1Bits1W<Dbginst1Spec> {
        Dbginst1Bits1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Instruction byte 5"]
    #[inline(always)]
    #[must_use]
    pub fn dbginst1_bits_0(&mut self) -> Dbginst1Bits0W<Dbginst1Spec> {
        Dbginst1Bits0W::new(self, 24)
    }
}
#[doc = "Debug Instruction-1 Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbginst1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dbginst1Spec;
impl crate::RegisterSpec for Dbginst1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dbginst1::W`](W) writer structure"]
impl crate::Writable for Dbginst1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBGINST1 to value 0"]
impl crate::Resettable for Dbginst1Spec {
    const RESET_VALUE: u32 = 0;
}
