#[doc = "Register `I2CM_DATAO` reader"]
pub type R = crate::R<I2cmDataoSpec>;
#[doc = "Register `I2CM_DATAO` writer"]
pub type W = crate::W<I2cmDataoSpec>;
#[doc = "Field `DATAO` reader - Data to be written on register pointed by address\\[7:0\\]."]
pub type DataoR = crate::FieldReader;
#[doc = "Field `DATAO` writer - Data to be written on register pointed by address\\[7:0\\]."]
pub type DataoW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data to be written on register pointed by address\\[7:0\\]."]
    #[inline(always)]
    pub fn datao(&self) -> DataoR {
        DataoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data to be written on register pointed by address\\[7:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn datao(&mut self) -> DataoW<I2cmDataoSpec> {
        DataoW::new(self, 0)
    }
}
#[doc = "Data to be written on register pointed by address\\[7:0\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_datao::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_datao::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmDataoSpec;
impl crate::RegisterSpec for I2cmDataoSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2cm_datao::R`](R) reader structure"]
impl crate::Readable for I2cmDataoSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cm_datao::W`](W) writer structure"]
impl crate::Writable for I2cmDataoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets I2CM_DATAO to value 0"]
impl crate::Resettable for I2cmDataoSpec {
    const RESET_VALUE: u8 = 0;
}
