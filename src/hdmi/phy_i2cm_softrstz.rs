#[doc = "Register `PHY_I2CM_SOFTRSTZ` reader"]
pub type R = crate::R<PhyI2cmSoftrstzSpec>;
#[doc = "Register `PHY_I2CM_SOFTRSTZ` writer"]
pub type W = crate::W<PhyI2cmSoftrstzSpec>;
#[doc = "Field `I2C_SOFTRSTZ` reader - I2C Master Software Reset. Active by writing a zero\n\nand auto cleared to one in the following cycle."]
pub type I2cSoftrstzR = crate::BitReader;
#[doc = "Field `I2C_SOFTRSTZ` writer - I2C Master Software Reset. Active by writing a zero\n\nand auto cleared to one in the following cycle."]
pub type I2cSoftrstzW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C Master Software Reset. Active by writing a zero\n\nand auto cleared to one in the following cycle."]
    #[inline(always)]
    pub fn i2c_softrstz(&self) -> I2cSoftrstzR {
        I2cSoftrstzR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Master Software Reset. Active by writing a zero\n\nand auto cleared to one in the following cycle."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_softrstz(&mut self) -> I2cSoftrstzW<PhyI2cmSoftrstzSpec> {
        I2cSoftrstzW::new(self, 0)
    }
}
#[doc = "PHY I2C SW reset control register\n\nThis register sets the I2C Master PHY software reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_softrstz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_softrstz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyI2cmSoftrstzSpec;
impl crate::RegisterSpec for PhyI2cmSoftrstzSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_i2cm_softrstz::R`](R) reader structure"]
impl crate::Readable for PhyI2cmSoftrstzSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_i2cm_softrstz::W`](W) writer structure"]
impl crate::Writable for PhyI2cmSoftrstzSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PHY_I2CM_SOFTRSTZ to value 0x01"]
impl crate::Resettable for PhyI2cmSoftrstzSpec {
    const RESET_VALUE: u8 = 0x01;
}
