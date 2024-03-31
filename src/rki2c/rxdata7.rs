#[doc = "Register `RXDATA7` reader"]
pub type R = crate::R<Rxdata7Spec>;
#[doc = "Field `RXDATA7` reader - data7 received\n\n32 bits data"]
pub type Rxdata7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - data7 received\n\n32 bits data"]
    #[inline(always)]
    pub fn rxdata7(&self) -> Rxdata7R {
        Rxdata7R::new(self.bits)
    }
}
#[doc = "I2C rx data register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxdata7Spec;
impl crate::RegisterSpec for Rxdata7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdata7::R`](R) reader structure"]
impl crate::Readable for Rxdata7Spec {}
#[doc = "`reset()` method sets RXDATA7 to value 0"]
impl crate::Resettable for Rxdata7Spec {
    const RESET_VALUE: u32 = 0;
}
