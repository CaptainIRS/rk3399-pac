#[doc = "Register `I2CM_READ_BUFF2` reader"]
pub type R = crate::R<I2cmReadBuff2Spec>;
#[doc = "Field `I2CM_READ_BUFF2` reader - Byte 2 of a I2C read buffer sequential read (from address i2cm_address+2)"]
pub type I2cmReadBuff2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Byte 2 of a I2C read buffer sequential read (from address i2cm_address+2)"]
    #[inline(always)]
    pub fn i2cm_read_buff2(&self) -> I2cmReadBuff2R {
        I2cmReadBuff2R::new(self.bits)
    }
}
#[doc = "Byte 2 of a I2C read buffer sequential read (from address i2cm_address+2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_read_buff2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmReadBuff2Spec;
impl crate::RegisterSpec for I2cmReadBuff2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2cm_read_buff2::R`](R) reader structure"]
impl crate::Readable for I2cmReadBuff2Spec {}
#[doc = "`reset()` method sets I2CM_READ_BUFF2 to value 0"]
impl crate::Resettable for I2cmReadBuff2Spec {
    const RESET_VALUE: u8 = 0;
}
