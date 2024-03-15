#[doc = "Register `I2CM_READ_BUFF0` reader"]
pub type R = crate::R<I2cmReadBuff0Spec>;
#[doc = "Field `I2CM_READ_BUFF0` reader - Byte 0 of a I2C read buffer sequential read (from address i2cm_address)"]
pub type I2cmReadBuff0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Byte 0 of a I2C read buffer sequential read (from address i2cm_address)"]
    #[inline(always)]
    pub fn i2cm_read_buff0(&self) -> I2cmReadBuff0R {
        I2cmReadBuff0R::new(self.bits)
    }
}
#[doc = "Byte 0 of a I2C read buffer sequential read (from address i2cm_address)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_read_buff0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmReadBuff0Spec;
impl crate::RegisterSpec for I2cmReadBuff0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2cm_read_buff0::R`](R) reader structure"]
impl crate::Readable for I2cmReadBuff0Spec {}
#[doc = "`reset()` method sets I2CM_READ_BUFF0 to value 0"]
impl crate::Resettable for I2cmReadBuff0Spec {
    const RESET_VALUE: u8 = 0;
}
