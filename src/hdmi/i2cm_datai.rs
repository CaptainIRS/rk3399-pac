#[doc = "Register `I2CM_DATAI` reader"]
pub type R = crate::R<I2cmDataiSpec>;
#[doc = "Field `DATAI` reader - Data read from register pointed by address\\[7:0\\]."]
pub type DataiR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Data read from register pointed by address\\[7:0\\]."]
    #[inline(always)]
    pub fn datai(&self) -> DataiR {
        DataiR::new(self.bits)
    }
}
#[doc = "Data read from register pointed by address\\[7:0\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_datai::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmDataiSpec;
impl crate::RegisterSpec for I2cmDataiSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2cm_datai::R`](R) reader structure"]
impl crate::Readable for I2cmDataiSpec {}
#[doc = "`reset()` method sets I2CM_DATAI to value 0"]
impl crate::Resettable for I2cmDataiSpec {
    const RESET_VALUE: u8 = 0;
}
