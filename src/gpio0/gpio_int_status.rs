#[doc = "Register `GPIO_INT_STATUS` reader"]
pub type R = crate::R<GpioIntStatusSpec>;
#[doc = "Field `GPIO_INT_STATUS` reader - Interrupt status of Port A"]
pub type GpioIntStatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt status of Port A"]
    #[inline(always)]
    pub fn gpio_int_status(&self) -> GpioIntStatusR {
        GpioIntStatusR::new(self.bits)
    }
}
#[doc = "Interrupt status of port A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_int_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIntStatusSpec;
impl crate::RegisterSpec for GpioIntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_int_status::R`](R) reader structure"]
impl crate::Readable for GpioIntStatusSpec {}
#[doc = "`reset()` method sets GPIO_INT_STATUS to value 0"]
impl crate::Resettable for GpioIntStatusSpec {
    const RESET_VALUE: u32 = 0;
}
