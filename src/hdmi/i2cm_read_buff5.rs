#[doc = "Register `I2CM_READ_BUFF5` reader"]
pub type R = crate::R<I2cmReadBuff5Spec>;
#[doc = "Field `I2CM_READ_BUFF5` reader - Byte 5 of a I2C read buffer sequential read (from\n\naddress i2cm_address+5)"]
pub type I2cmReadBuff5R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Byte 5 of a I2C read buffer sequential read (from\n\naddress i2cm_address+5)"]
    #[inline(always)]
    pub fn i2cm_read_buff5(&self) -> I2cmReadBuff5R {
        I2cmReadBuff5R::new(self.bits)
    }
}
#[doc = "I2C Master Sequential Read Buffer Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_read_buff5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmReadBuff5Spec;
impl crate::RegisterSpec for I2cmReadBuff5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2cm_read_buff5::R`](R) reader structure"]
impl crate::Readable for I2cmReadBuff5Spec {}
#[doc = "`reset()` method sets I2CM_READ_BUFF5 to value 0"]
impl crate::Resettable for I2cmReadBuff5Spec {
    const RESET_VALUE: u8 = 0;
}
