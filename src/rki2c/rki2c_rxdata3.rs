#[doc = "Register `RKI2C_RXDATA3` reader"]
pub type R = crate::R<Rki2cRxdata3Spec>;
#[doc = "Field `RXDATA3` reader - data3 received\n\n32 bits data"]
pub type Rxdata3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - data3 received\n\n32 bits data"]
    #[inline(always)]
    pub fn rxdata3(&self) -> Rxdata3R {
        Rxdata3R::new(self.bits)
    }
}
#[doc = "I2C rx data register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_rxdata3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rki2cRxdata3Spec;
impl crate::RegisterSpec for Rki2cRxdata3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rki2c_rxdata3::R`](R) reader structure"]
impl crate::Readable for Rki2cRxdata3Spec {}
#[doc = "`reset()` method sets RKI2C_RXDATA3 to value 0"]
impl crate::Resettable for Rki2cRxdata3Spec {
    const RESET_VALUE: u32 = 0;
}
