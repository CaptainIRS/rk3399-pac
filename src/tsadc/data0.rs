#[doc = "Register `DATA0` reader"]
pub type R = crate::R<Data0Spec>;
#[doc = "Field `ADC_DATA` reader - A/D value of the channel 0 last conversion (DOUT\\[9:0\\])."]
pub type AdcDataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - A/D value of the channel 0 last conversion (DOUT\\[9:0\\])."]
    #[inline(always)]
    pub fn adc_data(&self) -> AdcDataR {
        AdcDataR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "This register contains the data after A/D Conversion.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data0Spec;
impl crate::RegisterSpec for Data0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0::R`](R) reader structure"]
impl crate::Readable for Data0Spec {}
#[doc = "`reset()` method sets DATA0 to value 0"]
impl crate::Resettable for Data0Spec {
    const RESET_VALUE: u32 = 0;
}
