#[doc = "Register `SWREG_108_REUSE` reader"]
pub type R = crate::R<Swreg108ReuseSpec>;
#[doc = "Register `SWREG_108_REUSE` writer"]
pub type W = crate::W<Swreg108ReuseSpec>;
#[doc = "Field `INTRA_SLICE_BMP2` reader - Field0000 Abstract\n\nbit0 : slices64\n\nbit1 : slices65\n\nbit2 : slices66\n\n......\n\nbit31 : slices95"]
pub type IntraSliceBmp2R = crate::FieldReader<u32>;
#[doc = "Field `INTRA_SLICE_BMP2` writer - Field0000 Abstract\n\nbit0 : slices64\n\nbit1 : slices65\n\nbit2 : slices66\n\n......\n\nbit31 : slices95"]
pub type IntraSliceBmp2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Field0000 Abstract\n\nbit0 : slices64\n\nbit1 : slices65\n\nbit2 : slices66\n\n......\n\nbit31 : slices95"]
    #[inline(always)]
    pub fn intra_slice_bmp2(&self) -> IntraSliceBmp2R {
        IntraSliceBmp2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Field0000 Abstract\n\nbit0 : slices64\n\nbit1 : slices65\n\nbit2 : slices66\n\n......\n\nbit31 : slices95"]
    #[inline(always)]
    #[must_use]
    pub fn intra_slice_bmp2(&mut self) -> IntraSliceBmp2W<Swreg108ReuseSpec> {
        IntraSliceBmp2W::new(self, 0)
    }
}
#[doc = "intra_slice_bmp2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_108_reuse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_108_reuse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg108ReuseSpec;
impl crate::RegisterSpec for Swreg108ReuseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_108_reuse::R`](R) reader structure"]
impl crate::Readable for Swreg108ReuseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg_108_reuse::W`](W) writer structure"]
impl crate::Writable for Swreg108ReuseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_108_REUSE to value 0"]
impl crate::Resettable for Swreg108ReuseSpec {
    const RESET_VALUE: u32 = 0;
}
