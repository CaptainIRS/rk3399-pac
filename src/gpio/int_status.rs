#[doc = "Register `INT_STATUS` reader"]
pub type R = crate::R<IntStatusSpec>;
#[doc = "Field `GPIO_INT_STATUS` reader - Interrupt status of Port A"]
pub type GpioIntStatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt status of Port A"]
    #[inline(always)]
    pub fn gpio_int_status(&self) -> GpioIntStatusR {
        GpioIntStatusR::new(self.bits)
    }
}
#[doc = "Interrupt status of port A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStatusSpec;
impl crate::RegisterSpec for IntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_status::R`](R) reader structure"]
impl crate::Readable for IntStatusSpec {}
#[doc = "`reset()` method sets INT_STATUS to value 0"]
impl crate::Resettable for IntStatusSpec {
    const RESET_VALUE: u32 = 0;
}
