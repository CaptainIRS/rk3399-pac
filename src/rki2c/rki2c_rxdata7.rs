#[doc = "Register `RKI2C_RXDATA7` reader"]
pub type R = crate::R<Rki2cRxdata7Spec>;
#[doc = "Field `RXDATA7` reader - data7 received 32 bits data"]
pub type Rxdata7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - data7 received 32 bits data"]
    #[inline(always)]
    pub fn rxdata7(&self) -> Rxdata7R {
        Rxdata7R::new(self.bits)
    }
}
#[doc = "I2C rx data register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_rxdata7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rki2cRxdata7Spec;
impl crate::RegisterSpec for Rki2cRxdata7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rki2c_rxdata7::R`](R) reader structure"]
impl crate::Readable for Rki2cRxdata7Spec {}
#[doc = "`reset()` method sets RKI2C_RXDATA7 to value 0"]
impl crate::Resettable for Rki2cRxdata7Spec {
    const RESET_VALUE: u32 = 0;
}
