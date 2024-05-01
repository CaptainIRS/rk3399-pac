#[doc = "Register `SRC_BASE3` reader"]
pub type R = crate::R<SrcBase3Spec>;
#[doc = "Register `SRC_BASE3` writer"]
pub type W = crate::W<SrcBase3Spec>;
#[doc = "Field `SW_SRC_BASE3` reader - source image 1 RGB base address\n\n(source bitblt mode1)"]
pub type SwSrcBase3R = crate::BitReader;
#[doc = "Field `SW_SRC_BASE3` writer - source image 1 RGB base address\n\n(source bitblt mode1)"]
pub type SwSrcBase3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - source image 1 RGB base address\n\n(source bitblt mode1)"]
    #[inline(always)]
    pub fn sw_src_base3(&self) -> SwSrcBase3R {
        SwSrcBase3R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - source image 1 RGB base address\n\n(source bitblt mode1)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_base3(&mut self) -> SwSrcBase3W<SrcBase3Spec> {
        SwSrcBase3W::new(self, 0)
    }
}
#[doc = "RGA source image 1 base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_base3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_base3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcBase3Spec;
impl crate::RegisterSpec for SrcBase3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_base3::R`](R) reader structure"]
impl crate::Readable for SrcBase3Spec {}
#[doc = "`write(|w| ..)` method takes [`src_base3::W`](W) writer structure"]
impl crate::Writable for SrcBase3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC_BASE3 to value 0"]
impl crate::Resettable for SrcBase3Spec {
    const RESET_VALUE: u32 = 0;
}
