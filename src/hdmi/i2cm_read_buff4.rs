#[doc = "Register `I2CM_READ_BUFF4` reader"]
pub type R = crate::R<I2cmReadBuff4Spec>;
#[doc = "Field `I2CM_READ_BUFF4` reader - Byte 4 of a I2C read buffer sequential read (from address i2cm_address+4)"]
pub type I2cmReadBuff4R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Byte 4 of a I2C read buffer sequential read (from address i2cm_address+4)"]
    #[inline(always)]
    pub fn i2cm_read_buff4(&self) -> I2cmReadBuff4R {
        I2cmReadBuff4R::new(self.bits)
    }
}
#[doc = "Byte 4 of a I2C read buffer sequential read (from address i2cm_address+4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_read_buff4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmReadBuff4Spec;
impl crate::RegisterSpec for I2cmReadBuff4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2cm_read_buff4::R`](R) reader structure"]
impl crate::Readable for I2cmReadBuff4Spec {}
#[doc = "`reset()` method sets I2CM_READ_BUFF4 to value 0"]
impl crate::Resettable for I2cmReadBuff4Spec {
    const RESET_VALUE: u8 = 0;
}
