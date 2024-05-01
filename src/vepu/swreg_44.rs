#[doc = "Register `SWREG_44` reader"]
pub type R = crate::R<Swreg44Spec>;
#[doc = "Register `SWREG_44` writer"]
pub type W = crate::W<Swreg44Spec>;
#[doc = "Field `INTRA_SLICE_BMP0` reader - Intra slice bitmap for slices0 to slices31\n\nbit0 : slices0\n\nbit1 : slices1\n\nbit2 : slices2\n\n......\n\nbit31 : slices31"]
pub type IntraSliceBmp0R = crate::FieldReader<u32>;
#[doc = "Field `INTRA_SLICE_BMP0` writer - Intra slice bitmap for slices0 to slices31\n\nbit0 : slices0\n\nbit1 : slices1\n\nbit2 : slices2\n\n......\n\nbit31 : slices31"]
pub type IntraSliceBmp0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Intra slice bitmap for slices0 to slices31\n\nbit0 : slices0\n\nbit1 : slices1\n\nbit2 : slices2\n\n......\n\nbit31 : slices31"]
    #[inline(always)]
    pub fn intra_slice_bmp0(&self) -> IntraSliceBmp0R {
        IntraSliceBmp0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Intra slice bitmap for slices0 to slices31\n\nbit0 : slices0\n\nbit1 : slices1\n\nbit2 : slices2\n\n......\n\nbit31 : slices31"]
    #[inline(always)]
    #[must_use]
    pub fn intra_slice_bmp0(&mut self) -> IntraSliceBmp0W<Swreg44Spec> {
        IntraSliceBmp0W::new(self, 0)
    }
}
#[doc = "Intra slice bitmap\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_44::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_44::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg44Spec;
impl crate::RegisterSpec for Swreg44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_44::R`](R) reader structure"]
impl crate::Readable for Swreg44Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_44::W`](W) writer structure"]
impl crate::Writable for Swreg44Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_44 to value 0"]
impl crate::Resettable for Swreg44Spec {
    const RESET_VALUE: u32 = 0;
}
