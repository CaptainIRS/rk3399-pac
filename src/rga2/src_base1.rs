#[doc = "Register `SRC_BASE1` reader"]
pub type R = crate::R<SrcBase1Spec>;
#[doc = "Register `SRC_BASE1` writer"]
pub type W = crate::W<SrcBase1Spec>;
#[doc = "Field `SW_SRC_BASE1` reader - source image Cb base address (YUV422/420-P)\n\nsource image Cb/Cr base address (YU,V422/420-SP)"]
pub type SwSrcBase1R = crate::FieldReader<u32>;
#[doc = "Field `SW_SRC_BASE1` writer - source image Cb base address (YUV422/420-P)\n\nsource image Cb/Cr base address (YU,V422/420-SP)"]
pub type SwSrcBase1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - source image Cb base address (YUV422/420-P)\n\nsource image Cb/Cr base address (YU,V422/420-SP)"]
    #[inline(always)]
    pub fn sw_src_base1(&self) -> SwSrcBase1R {
        SwSrcBase1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - source image Cb base address (YUV422/420-P)\n\nsource image Cb/Cr base address (YU,V422/420-SP)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_base1(&mut self) -> SwSrcBase1W<SrcBase1Spec> {
        SwSrcBase1W::new(self, 0)
    }
}
#[doc = "RGA source image Cb/Cbr base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_base1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_base1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcBase1Spec;
impl crate::RegisterSpec for SrcBase1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_base1::R`](R) reader structure"]
impl crate::Readable for SrcBase1Spec {}
#[doc = "`write(|w| ..)` method takes [`src_base1::W`](W) writer structure"]
impl crate::Writable for SrcBase1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC_BASE1 to value 0"]
impl crate::Resettable for SrcBase1Spec {
    const RESET_VALUE: u32 = 0;
}
