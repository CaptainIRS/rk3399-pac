#[doc = "Register `FC_DBGAUD2CH0` reader"]
pub type R = crate::R<FcDbgaud2ch0Spec>;
#[doc = "Register `FC_DBGAUD2CH0` writer"]
pub type W = crate::W<FcDbgaud2ch0Spec>;
#[doc = "Field `FC_DBGAUD2CH0` reader - Frame Composer Audio Data Channel 0 Register 2"]
pub type FcDbgaud2ch0R = crate::FieldReader;
#[doc = "Field `FC_DBGAUD2CH0` writer - Frame Composer Audio Data Channel 0 Register 2"]
pub type FcDbgaud2ch0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Frame Composer Audio Data Channel 0 Register 2"]
    #[inline(always)]
    pub fn fc_dbgaud2ch0(&self) -> FcDbgaud2ch0R {
        FcDbgaud2ch0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Composer Audio Data Channel 0 Register 2"]
    #[inline(always)]
    #[must_use]
    pub fn fc_dbgaud2ch0(&mut self) -> FcDbgaud2ch0W<FcDbgaud2ch0Spec> {
        FcDbgaud2ch0W::new(self, 0)
    }
}
#[doc = "Frame Composer Audio Data Channel 0 Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_dbgaud2ch0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_dbgaud2ch0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcDbgaud2ch0Spec;
impl crate::RegisterSpec for FcDbgaud2ch0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_dbgaud2ch0::R`](R) reader structure"]
impl crate::Readable for FcDbgaud2ch0Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_dbgaud2ch0::W`](W) writer structure"]
impl crate::Writable for FcDbgaud2ch0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_DBGAUD2CH0 to value 0"]
impl crate::Resettable for FcDbgaud2ch0Spec {
    const RESET_VALUE: u8 = 0;
}
