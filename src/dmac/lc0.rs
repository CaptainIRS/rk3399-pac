#[doc = "Register `LC0%s` reader"]
pub type R = crate::R<Lc0Spec>;
#[doc = "Field `LC0_BITS_1` reader - Loop counter 0 iterations"]
pub type Lc0Bits1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Loop counter 0 iterations"]
    #[inline(always)]
    pub fn lc0_bits_1(&self) -> Lc0Bits1R {
        Lc0Bits1R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Loop Counter 0 Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lc0Spec;
impl crate::RegisterSpec for Lc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lc0::R`](R) reader structure"]
impl crate::Readable for Lc0Spec {}
#[doc = "`reset()` method sets LC0%s to value 0"]
impl crate::Resettable for Lc0Spec {
    const RESET_VALUE: u32 = 0;
}
