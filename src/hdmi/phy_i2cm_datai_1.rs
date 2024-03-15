#[doc = "Register `PHY_I2CM_DATAI_1` reader"]
pub type R = crate::R<PhyI2cmDatai1Spec>;
#[doc = "Field `DATAI` reader - Data MSB (datai\\[15:8\\]) read from register pointed by phy_i2cm_address\\[7:0\\]."]
pub type DataiR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Data MSB (datai\\[15:8\\]) read from register pointed by phy_i2cm_address\\[7:0\\]."]
    #[inline(always)]
    pub fn datai(&self) -> DataiR {
        DataiR::new(self.bits)
    }
}
#[doc = "Data MSB (datai\\[15:8\\]) read from register pointed by phy_i2cm_address\\[7:0\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_datai_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyI2cmDatai1Spec;
impl crate::RegisterSpec for PhyI2cmDatai1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_i2cm_datai_1::R`](R) reader structure"]
impl crate::Readable for PhyI2cmDatai1Spec {}
#[doc = "`reset()` method sets PHY_I2CM_DATAI_1 to value 0"]
impl crate::Resettable for PhyI2cmDatai1Spec {
    const RESET_VALUE: u8 = 0;
}
