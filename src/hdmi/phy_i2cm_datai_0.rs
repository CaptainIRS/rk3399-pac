#[doc = "Register `PHY_I2CM_DATAI_0` reader"]
pub type R = crate::R<PhyI2cmDatai0Spec>;
#[doc = "Field `DATAI` reader - Data LSB (datai\\[7:0\\]) read from register pointed by\n\nphy_i2cm_address\\[7:0\\]."]
pub type DataiR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Data LSB (datai\\[7:0\\]) read from register pointed by\n\nphy_i2cm_address\\[7:0\\]."]
    #[inline(always)]
    pub fn datai(&self) -> DataiR {
        DataiR::new(self.bits)
    }
}
#[doc = "PHY I2C Data Read Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_datai_0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyI2cmDatai0Spec;
impl crate::RegisterSpec for PhyI2cmDatai0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_i2cm_datai_0::R`](R) reader structure"]
impl crate::Readable for PhyI2cmDatai0Spec {}
#[doc = "`reset()` method sets PHY_I2CM_DATAI_0 to value 0"]
impl crate::Resettable for PhyI2cmDatai0Spec {
    const RESET_VALUE: u8 = 0;
}
