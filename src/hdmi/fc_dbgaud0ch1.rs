#[doc = "Register `FC_DBGAUD0CH1` reader"]
pub type R = crate::R<FcDbgaud0ch1Spec>;
#[doc = "Register `FC_DBGAUD0CH1` writer"]
pub type W = crate::W<FcDbgaud0ch1Spec>;
#[doc = "Field `FC_DBGAUD0CH1` reader - Frame Composer Audio Data Channel 1 Register 0"]
pub type FcDbgaud0ch1R = crate::FieldReader;
#[doc = "Field `FC_DBGAUD0CH1` writer - Frame Composer Audio Data Channel 1 Register 0"]
pub type FcDbgaud0ch1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer Audio Data Channel 1 Register 0"]
    #[inline(always)]
    pub fn fc_dbgaud0ch1(&self) -> FcDbgaud0ch1R {
        FcDbgaud0ch1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer Audio Data Channel 1 Register 0"]
    #[inline(always)]
    #[must_use]
    pub fn fc_dbgaud0ch1(&mut self) -> FcDbgaud0ch1W<FcDbgaud0ch1Spec> {
        FcDbgaud0ch1W::new(self, 0)
    }
}
#[doc = "Frame Composer Audio Data Channel 1 Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud0ch1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud0ch1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcDbgaud0ch1Spec;
impl crate::RegisterSpec for FcDbgaud0ch1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_dbgaud0ch1::R`](R) reader structure"]
impl crate::Readable for FcDbgaud0ch1Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_dbgaud0ch1::W`](W) writer structure"]
impl crate::Writable for FcDbgaud0ch1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_DBGAUD0CH1 to value 0"]
impl crate::Resettable for FcDbgaud0ch1Spec {
    const RESET_VALUE: u8 = 0;
}
