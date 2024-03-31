#[doc = "Register `RXDATA2` reader"]
pub type R = crate::R<Rxdata2Spec>;
#[doc = "Field `RXDATA2` reader - data2 received\n\n32 bits data"]
pub type Rxdata2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - data2 received\n\n32 bits data"]
    #[inline(always)]
    pub fn rxdata2(&self) -> Rxdata2R {
        Rxdata2R::new(self.bits)
    }
}
#[doc = "I2C rx data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxdata2Spec;
impl crate::RegisterSpec for Rxdata2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdata2::R`](R) reader structure"]
impl crate::Readable for Rxdata2Spec {}
#[doc = "`reset()` method sets RXDATA2 to value 0"]
impl crate::Resettable for Rxdata2Spec {
    const RESET_VALUE: u32 = 0;
}
