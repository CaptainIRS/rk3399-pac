#[doc = "Register `I2CM_DIV` reader"]
pub type R = crate::R<I2cmDivSpec>;
#[doc = "Register `I2CM_DIV` writer"]
pub type W = crate::W<I2cmDivSpec>;
#[doc = "Field `SPARE` reader - Reserved as \"spare\" bit with no associated functionality."]
pub type SpareR = crate::FieldReader;
#[doc = "Field `SPARE` writer - Reserved as \"spare\" bit with no associated functionality."]
pub type SpareW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FAST_STD_MODE` reader - Sets the I2C Master to work in Fast Mode or Standard Mode: 1: Fast Mode 0: Standard Mode"]
pub type FastStdModeR = crate::BitReader;
#[doc = "Field `FAST_STD_MODE` writer - Sets the I2C Master to work in Fast Mode or Standard Mode: 1: Fast Mode 0: Standard Mode"]
pub type FastStdModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Reserved as \"spare\" bit with no associated functionality."]
    #[inline(always)]
    pub fn spare(&self) -> SpareR {
        SpareR::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Sets the I2C Master to work in Fast Mode or Standard Mode: 1: Fast Mode 0: Standard Mode"]
    #[inline(always)]
    pub fn fast_std_mode(&self) -> FastStdModeR {
        FastStdModeR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Reserved as \"spare\" bit with no associated functionality."]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SpareW<I2cmDivSpec> {
        SpareW::new(self, 0)
    }
    #[doc = "Bit 3 - Sets the I2C Master to work in Fast Mode or Standard Mode: 1: Fast Mode 0: Standard Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fast_std_mode(&mut self) -> FastStdModeW<I2cmDivSpec> {
        FastStdModeW::new(self, 3)
    }
}
#[doc = "Reserved as \"spare\" bit with no associated functionality.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_div::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_div::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmDivSpec;
impl crate::RegisterSpec for I2cmDivSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2cm_div::R`](R) reader structure"]
impl crate::Readable for I2cmDivSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cm_div::W`](W) writer structure"]
impl crate::Writable for I2cmDivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets I2CM_DIV to value 0x0b"]
impl crate::Resettable for I2cmDivSpec {
    const RESET_VALUE: u8 = 0x0b;
}
