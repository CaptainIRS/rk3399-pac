#[doc = "Register `DATA` reader"]
pub type R = crate::R<DataSpec>;
#[doc = "Field `ADC_DATA` reader - A/D value of the last conversion (DOUT\\[9:0\\])."]
pub type AdcDataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - A/D value of the last conversion (DOUT\\[9:0\\])."]
    #[inline(always)]
    pub fn adc_data(&self) -> AdcDataR {
        AdcDataR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "This register contains the data after A/D Conversion.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSpec;
impl crate::RegisterSpec for DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DataSpec {}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DataSpec {
    const RESET_VALUE: u32 = 0;
}
