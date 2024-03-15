#[doc = "Register `I2CM_READ_BUFF3` reader"]
pub type R = crate::R<I2cmReadBuff3Spec>;
#[doc = "Field `I2CM_READ_BUFF3` reader - Byte 3 of a I2C read buffer sequential read (from address i2cm_address+3)"]
pub type I2cmReadBuff3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Byte 3 of a I2C read buffer sequential read (from address i2cm_address+3)"]
    #[inline(always)]
    pub fn i2cm_read_buff3(&self) -> I2cmReadBuff3R {
        I2cmReadBuff3R::new(self.bits)
    }
}
#[doc = "Byte 3 of a I2C read buffer sequential read (from address i2cm_address+3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_read_buff3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmReadBuff3Spec;
impl crate::RegisterSpec for I2cmReadBuff3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2cm_read_buff3::R`](R) reader structure"]
impl crate::Readable for I2cmReadBuff3Spec {}
#[doc = "`reset()` method sets I2CM_READ_BUFF3 to value 0"]
impl crate::Resettable for I2cmReadBuff3Spec {
    const RESET_VALUE: u8 = 0;
}
