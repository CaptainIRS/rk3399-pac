#[doc = "Register `TSADC_DATA0` reader"]
pub type R = crate::R<TsadcData0Spec>;
#[doc = "Field `ADC_DATA` reader - A/D value of the channel 0 last conversion (DOUT\\[9:0\\])."]
pub type AdcDataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - A/D value of the channel 0 last conversion (DOUT\\[9:0\\])."]
    #[inline(always)]
    pub fn adc_data(&self) -> AdcDataR {
        AdcDataR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "This register contains the data after A/D Conversion.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_data0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsadcData0Spec;
impl crate::RegisterSpec for TsadcData0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsadc_data0::R`](R) reader structure"]
impl crate::Readable for TsadcData0Spec {}
#[doc = "`reset()` method sets TSADC_DATA0 to value 0"]
impl crate::Resettable for TsadcData0Spec {
    const RESET_VALUE: u32 = 0;
}
