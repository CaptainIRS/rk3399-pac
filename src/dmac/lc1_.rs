#[doc = "Register `LC1_%s` reader"]
pub type R = crate::R<Lc1_Spec>;
#[doc = "Field `LC1__BITS_1` reader - Loop counter 1 iterations"]
pub type Lc1_Bits1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Loop counter 1 iterations"]
    #[inline(always)]
    pub fn lc1__bits_1(&self) -> Lc1_Bits1R {
        Lc1_Bits1R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Loop Counter 1 Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc1_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lc1_Spec;
impl crate::RegisterSpec for Lc1_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lc1_::R`](R) reader structure"]
impl crate::Readable for Lc1_Spec {}
#[doc = "`reset()` method sets LC1_%s to value 0"]
impl crate::Resettable for Lc1_Spec {
    const RESET_VALUE: u32 = 0;
}
