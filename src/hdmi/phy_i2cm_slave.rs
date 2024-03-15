#[doc = "Register `PHY_I2CM_SLAVE` reader"]
pub type R = crate::R<PhyI2cmSlaveSpec>;
#[doc = "Register `PHY_I2CM_SLAVE` writer"]
pub type W = crate::W<PhyI2cmSlaveSpec>;
#[doc = "Field `SLAVEADDR` reader - Slave address to be sent during read and write operations. PHY Gen2 slave address: 7'h69 HEAC PHY slave address: 7'h49"]
pub type SlaveaddrR = crate::FieldReader;
#[doc = "Field `SLAVEADDR` writer - Slave address to be sent during read and write operations. PHY Gen2 slave address: 7'h69 HEAC PHY slave address: 7'h49"]
pub type SlaveaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Slave address to be sent during read and write operations. PHY Gen2 slave address: 7'h69 HEAC PHY slave address: 7'h49"]
    #[inline(always)]
    pub fn slaveaddr(&self) -> SlaveaddrR {
        SlaveaddrR::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Slave address to be sent during read and write operations. PHY Gen2 slave address: 7'h69 HEAC PHY slave address: 7'h49"]
    #[inline(always)]
    #[must_use]
    pub fn slaveaddr(&mut self) -> SlaveaddrW<PhyI2cmSlaveSpec> {
        SlaveaddrW::new(self, 0)
    }
}
#[doc = "Slave address to be sent during read and write operations. PHY Gen2 slave address: 7'h69 HEAC PHY slave address: 7'h49\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_slave::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_slave::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyI2cmSlaveSpec;
impl crate::RegisterSpec for PhyI2cmSlaveSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_i2cm_slave::R`](R) reader structure"]
impl crate::Readable for PhyI2cmSlaveSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_i2cm_slave::W`](W) writer structure"]
impl crate::Writable for PhyI2cmSlaveSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PHY_I2CM_SLAVE to value 0"]
impl crate::Resettable for PhyI2cmSlaveSpec {
    const RESET_VALUE: u8 = 0;
}
