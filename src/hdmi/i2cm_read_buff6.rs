#[doc = "Register `I2CM_READ_BUFF6` reader"]
pub type R = crate::R<I2cmReadBuff6Spec>;
#[doc = "Field `I2CM_READ_BUFF6` reader - Byte 6 of a I2C read buffer sequential read (from address i2cm_address+6)"]
pub type I2cmReadBuff6R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Byte 6 of a I2C read buffer sequential read (from address i2cm_address+6)"]
    #[inline(always)]
    pub fn i2cm_read_buff6(&self) -> I2cmReadBuff6R {
        I2cmReadBuff6R::new(self.bits)
    }
}
#[doc = "Byte 6 of a I2C read buffer sequential read (from address i2cm_address+6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_read_buff6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmReadBuff6Spec;
impl crate::RegisterSpec for I2cmReadBuff6Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2cm_read_buff6::R`](R) reader structure"]
impl crate::Readable for I2cmReadBuff6Spec {}
#[doc = "`reset()` method sets I2CM_READ_BUFF6 to value 0"]
impl crate::Resettable for I2cmReadBuff6Spec {
    const RESET_VALUE: u8 = 0;
}
