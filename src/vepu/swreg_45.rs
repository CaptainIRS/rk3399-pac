#[doc = "Register `SWREG_45` reader"]
pub type R = crate::R<Swreg45Spec>;
#[doc = "Register `SWREG_45` writer"]
pub type W = crate::W<Swreg45Spec>;
#[doc = "Field `INTRA_SLICE_BMP1` reader - Intra slice bitmap for slices32 to slices63\n\nbit0 : slices32\n\nbit1 : slices33\n\nbit2 : slices34\n\n......\n\nbit31 : slices63"]
pub type IntraSliceBmp1R = crate::FieldReader<u32>;
#[doc = "Field `INTRA_SLICE_BMP1` writer - Intra slice bitmap for slices32 to slices63\n\nbit0 : slices32\n\nbit1 : slices33\n\nbit2 : slices34\n\n......\n\nbit31 : slices63"]
pub type IntraSliceBmp1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Intra slice bitmap for slices32 to slices63\n\nbit0 : slices32\n\nbit1 : slices33\n\nbit2 : slices34\n\n......\n\nbit31 : slices63"]
    #[inline(always)]
    pub fn intra_slice_bmp1(&self) -> IntraSliceBmp1R {
        IntraSliceBmp1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Intra slice bitmap for slices32 to slices63\n\nbit0 : slices32\n\nbit1 : slices33\n\nbit2 : slices34\n\n......\n\nbit31 : slices63"]
    #[inline(always)]
    #[must_use]
    pub fn intra_slice_bmp1(&mut self) -> IntraSliceBmp1W<Swreg45Spec> {
        IntraSliceBmp1W::new(self, 0)
    }
}
#[doc = "Intra slice bitmap\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_45::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_45::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg45Spec;
impl crate::RegisterSpec for Swreg45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_45::R`](R) reader structure"]
impl crate::Readable for Swreg45Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_45::W`](W) writer structure"]
impl crate::Writable for Swreg45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_45 to value 0"]
impl crate::Resettable for Swreg45Spec {
    const RESET_VALUE: u32 = 0;
}
