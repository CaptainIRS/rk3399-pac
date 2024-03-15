#[doc = "Register `FC_DBGAUD1CH4` reader"]
pub type R = crate::R<FcDbgaud1ch4Spec>;
#[doc = "Register `FC_DBGAUD1CH4` writer"]
pub type W = crate::W<FcDbgaud1ch4Spec>;
#[doc = "Field `FC_DBGAUD1CH4` reader - Frame Composer Audio Data Channel 4 Register 1"]
pub type FcDbgaud1ch4R = crate::FieldReader;
#[doc = "Field `FC_DBGAUD1CH4` writer - Frame Composer Audio Data Channel 4 Register 1"]
pub type FcDbgaud1ch4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer Audio Data Channel 4 Register 1"]
    #[inline(always)]
    pub fn fc_dbgaud1ch4(&self) -> FcDbgaud1ch4R {
        FcDbgaud1ch4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer Audio Data Channel 4 Register 1"]
    #[inline(always)]
    #[must_use]
    pub fn fc_dbgaud1ch4(&mut self) -> FcDbgaud1ch4W<FcDbgaud1ch4Spec> {
        FcDbgaud1ch4W::new(self, 0)
    }
}
#[doc = "Frame Composer Audio Data Channel 4 Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud1ch4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud1ch4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcDbgaud1ch4Spec;
impl crate::RegisterSpec for FcDbgaud1ch4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_dbgaud1ch4::R`](R) reader structure"]
impl crate::Readable for FcDbgaud1ch4Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_dbgaud1ch4::W`](W) writer structure"]
impl crate::Writable for FcDbgaud1ch4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_DBGAUD1CH4 to value 0"]
impl crate::Resettable for FcDbgaud1ch4Spec {
    const RESET_VALUE: u8 = 0;
}
