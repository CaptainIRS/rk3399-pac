#[doc = "Register `I2CM_SLAVE` reader"]
pub type R = crate::R<I2cmSlaveSpec>;
#[doc = "Register `I2CM_SLAVE` writer"]
pub type W = crate::W<I2cmSlaveSpec>;
#[doc = "Field `SLAVEADDR` reader - Slave address to be sent during read and write normal\n\noperations."]
pub type SlaveaddrR = crate::FieldReader;
#[doc = "Field `SLAVEADDR` writer - Slave address to be sent during read and write normal\n\noperations."]
pub type SlaveaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Slave address to be sent during read and write normal\n\noperations."]
    #[inline(always)]
    pub fn slaveaddr(&self) -> SlaveaddrR {
        SlaveaddrR::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Slave address to be sent during read and write normal\n\noperations."]
    #[inline(always)]
    #[must_use]
    pub fn slaveaddr(&mut self) -> SlaveaddrW<I2cmSlaveSpec> {
        SlaveaddrW::new(self, 0)
    }
}
#[doc = "I2C DDC Slave address Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_slave::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_slave::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmSlaveSpec;
impl crate::RegisterSpec for I2cmSlaveSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2cm_slave::R`](R) reader structure"]
impl crate::Readable for I2cmSlaveSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cm_slave::W`](W) writer structure"]
impl crate::Writable for I2cmSlaveSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets I2CM_SLAVE to value 0"]
impl crate::Resettable for I2cmSlaveSpec {
    const RESET_VALUE: u8 = 0;
}
