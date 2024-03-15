#[doc = "Register `I2CM_SEGPTR` reader"]
pub type R = crate::R<I2cmSegptrSpec>;
#[doc = "Register `I2CM_SEGPTR` writer"]
pub type W = crate::W<I2cmSegptrSpec>;
#[doc = "Field `SEGPTR` reader - I2C DDC Segment Pointer Register"]
pub type SegptrR = crate::FieldReader;
#[doc = "Field `SEGPTR` writer - I2C DDC Segment Pointer Register"]
pub type SegptrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - I2C DDC Segment Pointer Register"]
    #[inline(always)]
    pub fn segptr(&self) -> SegptrR {
        SegptrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2C DDC Segment Pointer Register"]
    #[inline(always)]
    #[must_use]
    pub fn segptr(&mut self) -> SegptrW<I2cmSegptrSpec> {
        SegptrW::new(self, 0)
    }
}
#[doc = "I2C DDC Segment Pointer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cm_segptr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cm_segptr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmSegptrSpec;
impl crate::RegisterSpec for I2cmSegptrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2cm_segptr::R`](R) reader structure"]
impl crate::Readable for I2cmSegptrSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cm_segptr::W`](W) writer structure"]
impl crate::Writable for I2cmSegptrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets I2CM_SEGPTR to value 0"]
impl crate::Resettable for I2cmSegptrSpec {
    const RESET_VALUE: u8 = 0;
}
