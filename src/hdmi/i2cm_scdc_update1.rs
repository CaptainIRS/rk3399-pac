#[doc = "Register `I2CM_SCDC_UPDATE1` reader"]
pub type R = crate::R<I2cmScdcUpdate1Spec>;
#[doc = "Field `I2CM_SCDC_UPDATE1` reader - Byte 1 of a SCDC I2C update sequential read"]
pub type I2cmScdcUpdate1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Byte 1 of a SCDC I2C update sequential read"]
    #[inline(always)]
    pub fn i2cm_scdc_update1(&self) -> I2cmScdcUpdate1R {
        I2cmScdcUpdate1R::new(self.bits)
    }
}
#[doc = "I2C SCDC Read Update Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_scdc_update1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmScdcUpdate1Spec;
impl crate::RegisterSpec for I2cmScdcUpdate1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2cm_scdc_update1::R`](R) reader structure"]
impl crate::Readable for I2cmScdcUpdate1Spec {}
#[doc = "`reset()` method sets I2CM_SCDC_UPDATE1 to value 0"]
impl crate::Resettable for I2cmScdcUpdate1Spec {
    const RESET_VALUE: u8 = 0;
}
