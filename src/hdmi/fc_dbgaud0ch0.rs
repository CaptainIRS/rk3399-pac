#[doc = "Register `FC_DBGAUD0CH0` reader"]
pub type R = crate::R<FcDbgaud0ch0Spec>;
#[doc = "Register `FC_DBGAUD0CH0` writer"]
pub type W = crate::W<FcDbgaud0ch0Spec>;
#[doc = "Field `FC_DBGAUD0CH0` reader - Frame Composer Audio Data Channel 0 Register 0"]
pub type FcDbgaud0ch0R = crate::FieldReader;
#[doc = "Field `FC_DBGAUD0CH0` writer - Frame Composer Audio Data Channel 0 Register 0"]
pub type FcDbgaud0ch0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer Audio Data Channel 0 Register 0"]
    #[inline(always)]
    pub fn fc_dbgaud0ch0(&self) -> FcDbgaud0ch0R {
        FcDbgaud0ch0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer Audio Data Channel 0 Register 0"]
    #[inline(always)]
    #[must_use]
    pub fn fc_dbgaud0ch0(&mut self) -> FcDbgaud0ch0W<FcDbgaud0ch0Spec> {
        FcDbgaud0ch0W::new(self, 0)
    }
}
#[doc = "Frame Composer Audio Data Channel 0 Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud0ch0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud0ch0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcDbgaud0ch0Spec;
impl crate::RegisterSpec for FcDbgaud0ch0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_dbgaud0ch0::R`](R) reader structure"]
impl crate::Readable for FcDbgaud0ch0Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_dbgaud0ch0::W`](W) writer structure"]
impl crate::Writable for FcDbgaud0ch0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_DBGAUD0CH0 to value 0"]
impl crate::Resettable for FcDbgaud0ch0Spec {
    const RESET_VALUE: u8 = 0;
}
