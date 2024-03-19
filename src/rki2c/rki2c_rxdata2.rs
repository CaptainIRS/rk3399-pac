#[doc = "Register `RKI2C_RXDATA2` reader"]
pub type R = crate::R<Rki2cRxdata2Spec>;
#[doc = "Field `RXDATA2` reader - data2 received\n\n32 bits data"]
pub type Rxdata2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - data2 received\n\n32 bits data"]
    #[inline(always)]
    pub fn rxdata2(&self) -> Rxdata2R {
        Rxdata2R::new(self.bits)
    }
}
#[doc = "I2C rx data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rki2c_rxdata2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rki2cRxdata2Spec;
impl crate::RegisterSpec for Rki2cRxdata2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rki2c_rxdata2::R`](R) reader structure"]
impl crate::Readable for Rki2cRxdata2Spec {}
#[doc = "`reset()` method sets RKI2C_RXDATA2 to value 0"]
impl crate::Resettable for Rki2cRxdata2Spec {
    const RESET_VALUE: u32 = 0;
}
