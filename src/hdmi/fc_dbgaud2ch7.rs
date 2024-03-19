#[doc = "Register `FC_DBGAUD2CH7` reader"]
pub type R = crate::R<FcDbgaud2ch7Spec>;
#[doc = "Register `FC_DBGAUD2CH7` writer"]
pub type W = crate::W<FcDbgaud2ch7Spec>;
#[doc = "Field `FC_DBGAUD2CH7` reader - Frame Composer Audio Data Channel 7 Register 2"]
pub type FcDbgaud2ch7R = crate::FieldReader;
#[doc = "Field `FC_DBGAUD2CH7` writer - Frame Composer Audio Data Channel 7 Register 2"]
pub type FcDbgaud2ch7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer Audio Data Channel 7 Register 2"]
    #[inline(always)]
    pub fn fc_dbgaud2ch7(&self) -> FcDbgaud2ch7R {
        FcDbgaud2ch7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer Audio Data Channel 7 Register 2"]
    #[inline(always)]
    #[must_use]
    pub fn fc_dbgaud2ch7(&mut self) -> FcDbgaud2ch7W<FcDbgaud2ch7Spec> {
        FcDbgaud2ch7W::new(self, 0)
    }
}
#[doc = "Frame Composer Audio Data Channel 7 Register 2\n\nConfigures the audio fixed data to be used in channel 7 when in fixed audio selection.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud2ch7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud2ch7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcDbgaud2ch7Spec;
impl crate::RegisterSpec for FcDbgaud2ch7Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_dbgaud2ch7::R`](R) reader structure"]
impl crate::Readable for FcDbgaud2ch7Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_dbgaud2ch7::W`](W) writer structure"]
impl crate::Writable for FcDbgaud2ch7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_DBGAUD2CH7 to value 0"]
impl crate::Resettable for FcDbgaud2ch7Spec {
    const RESET_VALUE: u8 = 0;
}
