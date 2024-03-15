#[doc = "Register `FC_DBGAUD2CH6` reader"]
pub type R = crate::R<FcDbgaud2ch6Spec>;
#[doc = "Register `FC_DBGAUD2CH6` writer"]
pub type W = crate::W<FcDbgaud2ch6Spec>;
#[doc = "Field `FC_DBGAUD2CH6` reader - Frame Composer Audio Data Channel 6 Register 2"]
pub type FcDbgaud2ch6R = crate::FieldReader;
#[doc = "Field `FC_DBGAUD2CH6` writer - Frame Composer Audio Data Channel 6 Register 2"]
pub type FcDbgaud2ch6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer Audio Data Channel 6 Register 2"]
    #[inline(always)]
    pub fn fc_dbgaud2ch6(&self) -> FcDbgaud2ch6R {
        FcDbgaud2ch6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer Audio Data Channel 6 Register 2"]
    #[inline(always)]
    #[must_use]
    pub fn fc_dbgaud2ch6(&mut self) -> FcDbgaud2ch6W<FcDbgaud2ch6Spec> {
        FcDbgaud2ch6W::new(self, 0)
    }
}
#[doc = "Frame Composer Audio Data Channel 6 Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud2ch6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud2ch6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcDbgaud2ch6Spec;
impl crate::RegisterSpec for FcDbgaud2ch6Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_dbgaud2ch6::R`](R) reader structure"]
impl crate::Readable for FcDbgaud2ch6Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_dbgaud2ch6::W`](W) writer structure"]
impl crate::Writable for FcDbgaud2ch6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_DBGAUD2CH6 to value 0"]
impl crate::Resettable for FcDbgaud2ch6Spec {
    const RESET_VALUE: u8 = 0;
}
