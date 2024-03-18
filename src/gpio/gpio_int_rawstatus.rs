#[doc = "Register `GPIO_INT_RAWSTATUS` reader"]
pub type R = crate::R<GpioIntRawstatusSpec>;
#[doc = "Field `GPIO_INT_RAWSTATUS` reader - Raw interrupt of status of Port A (premasking bits)"]
pub type GpioIntRawstatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Raw interrupt of status of Port A (premasking bits)"]
    #[inline(always)]
    pub fn gpio_int_rawstatus(&self) -> GpioIntRawstatusR {
        GpioIntRawstatusR::new(self.bits)
    }
}
#[doc = "Raw Interrupt status of port A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_int_rawstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIntRawstatusSpec;
impl crate::RegisterSpec for GpioIntRawstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_int_rawstatus::R`](R) reader structure"]
impl crate::Readable for GpioIntRawstatusSpec {}
#[doc = "`reset()` method sets GPIO_INT_RAWSTATUS to value 0"]
impl crate::Resettable for GpioIntRawstatusSpec {
    const RESET_VALUE: u32 = 0;
}
