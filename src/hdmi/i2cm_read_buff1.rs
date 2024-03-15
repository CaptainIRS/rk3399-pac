#[doc = "Register `I2CM_READ_BUFF1` reader"]
pub type R = crate::R<I2cmReadBuff1Spec>;
#[doc = "Field `I2CM_READ_BUFF1` reader - Byte 1 of a I2C read buffer sequential read (from address i2cm_address+1)"]
pub type I2cmReadBuff1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Byte 1 of a I2C read buffer sequential read (from address i2cm_address+1)"]
    #[inline(always)]
    pub fn i2cm_read_buff1(&self) -> I2cmReadBuff1R {
        I2cmReadBuff1R::new(self.bits)
    }
}
#[doc = "Byte 1 of a I2C read buffer sequential read (from address i2cm_address+1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_read_buff1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmReadBuff1Spec;
impl crate::RegisterSpec for I2cmReadBuff1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2cm_read_buff1::R`](R) reader structure"]
impl crate::Readable for I2cmReadBuff1Spec {}
#[doc = "`reset()` method sets I2CM_READ_BUFF1 to value 0"]
impl crate::Resettable for I2cmReadBuff1Spec {
    const RESET_VALUE: u8 = 0;
}
