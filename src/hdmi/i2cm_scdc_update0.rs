#[doc = "Register `I2CM_SCDC_UPDATE0` reader"]
pub type R = crate::R<I2cmScdcUpdate0Spec>;
#[doc = "Field `I2CM_SCDC_UPDATE0` reader - Byte 0 of a SCDC I2C update sequential read"]
pub type I2cmScdcUpdate0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Byte 0 of a SCDC I2C update sequential read"]
    #[inline(always)]
    pub fn i2cm_scdc_update0(&self) -> I2cmScdcUpdate0R {
        I2cmScdcUpdate0R::new(self.bits)
    }
}
#[doc = "Byte 0 of a SCDC I2C update sequential read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_scdc_update0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmScdcUpdate0Spec;
impl crate::RegisterSpec for I2cmScdcUpdate0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2cm_scdc_update0::R`](R) reader structure"]
impl crate::Readable for I2cmScdcUpdate0Spec {}
#[doc = "`reset()` method sets I2CM_SCDC_UPDATE0 to value 0"]
impl crate::Resettable for I2cmScdcUpdate0Spec {
    const RESET_VALUE: u8 = 0;
}
