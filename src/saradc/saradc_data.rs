#[doc = "Register `SARADC_DATA` reader"]
pub type R = crate::R<SaradcDataSpec>;
#[doc = "Field `ADC_DATA` reader - A/D value of the last conversion (DOUT\\[9:0\\])."]
pub type AdcDataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - A/D value of the last conversion (DOUT\\[9:0\\])."]
    #[inline(always)]
    pub fn adc_data(&self) -> AdcDataR {
        AdcDataR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "This register contains the data after A/D Conversion.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`saradc_data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaradcDataSpec;
impl crate::RegisterSpec for SaradcDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saradc_data::R`](R) reader structure"]
impl crate::Readable for SaradcDataSpec {}
#[doc = "`reset()` method sets SARADC_DATA to value 0"]
impl crate::Resettable for SaradcDataSpec {
    const RESET_VALUE: u32 = 0;
}
