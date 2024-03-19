#[doc = "Register `RKI2C_RXDATA0` reader"]
pub type R = crate::R<Rki2cRxdata0Spec>;
#[doc = "Field `RXDATA0` reader - data0 received\n\n32 bits data"]
pub type Rxdata0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - data0 received\n\n32 bits data"]
    #[inline(always)]
    pub fn rxdata0(&self) -> Rxdata0R {
        Rxdata0R::new(self.bits)
    }
}
#[doc = "I2C rx data register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_rxdata0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rki2cRxdata0Spec;
impl crate::RegisterSpec for Rki2cRxdata0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rki2c_rxdata0::R`](R) reader structure"]
impl crate::Readable for Rki2cRxdata0Spec {}
#[doc = "`reset()` method sets RKI2C_RXDATA0 to value 0"]
impl crate::Resettable for Rki2cRxdata0Spec {
    const RESET_VALUE: u32 = 0;
}
