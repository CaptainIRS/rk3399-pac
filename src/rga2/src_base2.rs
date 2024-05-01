#[doc = "Register `SRC_BASE2` reader"]
pub type R = crate::R<SrcBase2Spec>;
#[doc = "Register `SRC_BASE2` writer"]
pub type W = crate::W<SrcBase2Spec>;
#[doc = "Field `SW_SRC_BASE2` reader - source image Cr base address (YUV422/420-P)"]
pub type SwSrcBase2R = crate::BitReader;
#[doc = "Field `SW_SRC_BASE2` writer - source image Cr base address (YUV422/420-P)"]
pub type SwSrcBase2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - source image Cr base address (YUV422/420-P)"]
    #[inline(always)]
    pub fn sw_src_base2(&self) -> SwSrcBase2R {
        SwSrcBase2R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - source image Cr base address (YUV422/420-P)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_base2(&mut self) -> SwSrcBase2W<SrcBase2Spec> {
        SwSrcBase2W::new(self, 0)
    }
}
#[doc = "RGA source image Cr base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_base2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_base2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcBase2Spec;
impl crate::RegisterSpec for SrcBase2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_base2::R`](R) reader structure"]
impl crate::Readable for SrcBase2Spec {}
#[doc = "`write(|w| ..)` method takes [`src_base2::W`](W) writer structure"]
impl crate::Writable for SrcBase2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC_BASE2 to value 0"]
impl crate::Resettable for SrcBase2Spec {
    const RESET_VALUE: u32 = 0;
}
