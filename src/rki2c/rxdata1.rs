#[doc = "Register `RXDATA1` reader"]
pub type R = crate::R<Rxdata1Spec>;
#[doc = "Field `RXDATA1` reader - data1 received\n\n32 bits data"]
pub type Rxdata1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - data1 received\n\n32 bits data"]
    #[inline(always)]
    pub fn rxdata1(&self) -> Rxdata1R {
        Rxdata1R::new(self.bits)
    }
}
#[doc = "I2C rx data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxdata1Spec;
impl crate::RegisterSpec for Rxdata1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdata1::R`](R) reader structure"]
impl crate::Readable for Rxdata1Spec {}
#[doc = "`reset()` method sets RXDATA1 to value 0"]
impl crate::Resettable for Rxdata1Spec {
    const RESET_VALUE: u32 = 0;
}
