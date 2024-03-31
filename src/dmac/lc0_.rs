#[doc = "Register `LC0_%s` reader"]
pub type R = crate::R<Lc0_Spec>;
#[doc = "Field `LC0__BITS_1` reader - Loop counter 0 iterations"]
pub type Lc0_Bits1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Loop counter 0 iterations"]
    #[inline(always)]
    pub fn lc0__bits_1(&self) -> Lc0_Bits1R {
        Lc0_Bits1R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Loop Counter 0 Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lc0_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lc0_Spec;
impl crate::RegisterSpec for Lc0_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lc0_::R`](R) reader structure"]
impl crate::Readable for Lc0_Spec {}
#[doc = "`reset()` method sets LC0_%s to value 0"]
impl crate::Resettable for Lc0_Spec {
    const RESET_VALUE: u32 = 0;
}
