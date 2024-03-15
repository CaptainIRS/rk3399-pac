#[doc = "Register `PHY_I2CM_DATAO_0` reader"]
pub type R = crate::R<PhyI2cmDatao0Spec>;
#[doc = "Register `PHY_I2CM_DATAO_0` writer"]
pub type W = crate::W<PhyI2cmDatao0Spec>;
#[doc = "Field `DATAO` reader - Data LSB (datao\\[7:0\\]) to be written on register pointed by phy_i2cm_address \\[7:0\\]."]
pub type DataoR = crate::FieldReader;
#[doc = "Field `DATAO` writer - Data LSB (datao\\[7:0\\]) to be written on register pointed by phy_i2cm_address \\[7:0\\]."]
pub type DataoW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data LSB (datao\\[7:0\\]) to be written on register pointed by phy_i2cm_address \\[7:0\\]."]
    #[inline(always)]
    pub fn datao(&self) -> DataoR {
        DataoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data LSB (datao\\[7:0\\]) to be written on register pointed by phy_i2cm_address \\[7:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn datao(&mut self) -> DataoW<PhyI2cmDatao0Spec> {
        DataoW::new(self, 0)
    }
}
#[doc = "Data LSB (datao\\[7:0\\]) to be written on register pointed by phy_i2cm_address \\[7:0\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_datao_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_datao_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyI2cmDatao0Spec;
impl crate::RegisterSpec for PhyI2cmDatao0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_i2cm_datao_0::R`](R) reader structure"]
impl crate::Readable for PhyI2cmDatao0Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_i2cm_datao_0::W`](W) writer structure"]
impl crate::Writable for PhyI2cmDatao0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PHY_I2CM_DATAO_0 to value 0"]
impl crate::Resettable for PhyI2cmDatao0Spec {
    const RESET_VALUE: u8 = 0;
}
