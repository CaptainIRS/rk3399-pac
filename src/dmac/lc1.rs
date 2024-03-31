#[doc = "Register `LC1%s` reader"]
pub type R = crate::R<Lc1Spec>;
#[doc = "Field `LC1_BITS_1` reader - Loop counter 1 iterations"]
pub type Lc1Bits1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Loop counter 1 iterations"]
    #[inline(always)]
    pub fn lc1_bits_1(&self) -> Lc1Bits1R {
        Lc1Bits1R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Loop Counter 1 Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lc1Spec;
impl crate::RegisterSpec for Lc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lc1::R`](R) reader structure"]
impl crate::Readable for Lc1Spec {}
#[doc = "`reset()` method sets LC1%s to value 0"]
impl crate::Resettable for Lc1Spec {
    const RESET_VALUE: u32 = 0;
}
