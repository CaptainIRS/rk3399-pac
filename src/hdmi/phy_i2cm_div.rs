#[doc = "Register `PHY_I2CM_DIV` reader"]
pub type R = crate::R<PhyI2cmDivSpec>;
#[doc = "Register `PHY_I2CM_DIV` writer"]
pub type W = crate::W<PhyI2cmDivSpec>;
#[doc = "Field `SPARE` reader - Reserved as \"spare\" register with no associated\n\nfunctionality."]
pub type SpareR = crate::FieldReader;
#[doc = "Field `SPARE` writer - Reserved as \"spare\" register with no associated\n\nfunctionality."]
pub type SpareW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FAST_STD_MODE` reader - Sets the I2C Master to work in Fast Mode or Standard\n\nMode: 1: Fast Mode\n\n0: Standard Mode"]
pub type FastStdModeR = crate::BitReader;
#[doc = "Field `FAST_STD_MODE` writer - Sets the I2C Master to work in Fast Mode or Standard\n\nMode: 1: Fast Mode\n\n0: Standard Mode"]
pub type FastStdModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Reserved as \"spare\" register with no associated\n\nfunctionality."]
    #[inline(always)]
    pub fn spare(&self) -> SpareR {
        SpareR::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Sets the I2C Master to work in Fast Mode or Standard\n\nMode: 1: Fast Mode\n\n0: Standard Mode"]
    #[inline(always)]
    pub fn fast_std_mode(&self) -> FastStdModeR {
        FastStdModeR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Reserved as \"spare\" register with no associated\n\nfunctionality."]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SpareW<PhyI2cmDivSpec> {
        SpareW::new(self, 0)
    }
    #[doc = "Bit 3 - Sets the I2C Master to work in Fast Mode or Standard\n\nMode: 1: Fast Mode\n\n0: Standard Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fast_std_mode(&mut self) -> FastStdModeW<PhyI2cmDivSpec> {
        FastStdModeW::new(self, 3)
    }
}
#[doc = "PHY I2C Speed control Register\n\nThis register wets the I2C Master PHY to work in either Fast or Standard mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_i2cm_div::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_i2cm_div::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyI2cmDivSpec;
impl crate::RegisterSpec for PhyI2cmDivSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_i2cm_div::R`](R) reader structure"]
impl crate::Readable for PhyI2cmDivSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_i2cm_div::W`](W) writer structure"]
impl crate::Writable for PhyI2cmDivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PHY_I2CM_DIV to value 0x0b"]
impl crate::Resettable for PhyI2cmDivSpec {
    const RESET_VALUE: u8 = 0x0b;
}
