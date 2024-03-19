#[doc = "Register `GPIO_EXT_PORTA` reader"]
pub type R = crate::R<GpioExtPortaSpec>;
#[doc = "Field `GPIO_EXT_PORTA` reader - When Port A is configured as Input, then reading this location\n\nreads the values on the signal. When the data direction of Port A\n\nis set as Output, reading this location reads the data register for\n\nPort A."]
pub type GpioExtPortaR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - When Port A is configured as Input, then reading this location\n\nreads the values on the signal. When the data direction of Port A\n\nis set as Output, reading this location reads the data register for\n\nPort A."]
    #[inline(always)]
    pub fn gpio_ext_porta(&self) -> GpioExtPortaR {
        GpioExtPortaR::new(self.bits)
    }
}
#[doc = "Port A external port register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_ext_porta::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioExtPortaSpec;
impl crate::RegisterSpec for GpioExtPortaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_ext_porta::R`](R) reader structure"]
impl crate::Readable for GpioExtPortaSpec {}
#[doc = "`reset()` method sets GPIO_EXT_PORTA to value 0"]
impl crate::Resettable for GpioExtPortaSpec {
    const RESET_VALUE: u32 = 0;
}
