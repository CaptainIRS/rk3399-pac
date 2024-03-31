#[doc = "Register `RXDATA4` reader"]
pub type R = crate::R<Rxdata4Spec>;
#[doc = "Field `RXDATA4` reader - data4 received\n\n32 bits data"]
pub type Rxdata4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - data4 received\n\n32 bits data"]
    #[inline(always)]
    pub fn rxdata4(&self) -> Rxdata4R {
        Rxdata4R::new(self.bits)
    }
}
#[doc = "I2C rx data register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxdata4Spec;
impl crate::RegisterSpec for Rxdata4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdata4::R`](R) reader structure"]
impl crate::Readable for Rxdata4Spec {}
#[doc = "`reset()` method sets RXDATA4 to value 0"]
impl crate::Resettable for Rxdata4Spec {
    const RESET_VALUE: u32 = 0;
}
