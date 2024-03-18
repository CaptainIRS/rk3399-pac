#[doc = "Register `RKI2C_RXDATA6` reader"]
pub type R = crate::R<Rki2cRxdata6Spec>;
#[doc = "Field `RXDATA6` reader - data6 received 32 bits data"]
pub type Rxdata6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - data6 received 32 bits data"]
    #[inline(always)]
    pub fn rxdata6(&self) -> Rxdata6R {
        Rxdata6R::new(self.bits)
    }
}
#[doc = "I2C rx data register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_rxdata6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rki2cRxdata6Spec;
impl crate::RegisterSpec for Rki2cRxdata6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rki2c_rxdata6::R`](R) reader structure"]
impl crate::Readable for Rki2cRxdata6Spec {}
#[doc = "`reset()` method sets RKI2C_RXDATA6 to value 0"]
impl crate::Resettable for Rki2cRxdata6Spec {
    const RESET_VALUE: u32 = 0;
}
