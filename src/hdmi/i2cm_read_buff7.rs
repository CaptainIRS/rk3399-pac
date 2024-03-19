#[doc = "Register `I2CM_READ_BUFF7` reader"]
pub type R = crate::R<I2cmReadBuff7Spec>;
#[doc = "Field `I2CM_READ_BUFF7` reader - Byte 7 of a I2C read buffer sequential read (from\n\naddress i2cm_address+7)"]
pub type I2cmReadBuff7R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Byte 7 of a I2C read buffer sequential read (from\n\naddress i2cm_address+7)"]
    #[inline(always)]
    pub fn i2cm_read_buff7(&self) -> I2cmReadBuff7R {
        I2cmReadBuff7R::new(self.bits)
    }
}
#[doc = "I2C Master Sequential Read Buffer Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_read_buff7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmReadBuff7Spec;
impl crate::RegisterSpec for I2cmReadBuff7Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2cm_read_buff7::R`](R) reader structure"]
impl crate::Readable for I2cmReadBuff7Spec {}
#[doc = "`reset()` method sets I2CM_READ_BUFF7 to value 0"]
impl crate::Resettable for I2cmReadBuff7Spec {
    const RESET_VALUE: u8 = 0;
}
