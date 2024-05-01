#[doc = "Register `SRC_ADDR_CR` reader"]
pub type R = crate::R<SrcAddrCrSpec>;
#[doc = "Register `SRC_ADDR_CR` writer"]
pub type W = crate::W<SrcAddrCrSpec>;
#[doc = "Field `SRC_IMAGE_CR_MST` reader - Source image data Cr start address in Memory"]
pub type SrcImageCrMstR = crate::FieldReader<u32>;
#[doc = "Field `SRC_IMAGE_CR_MST` writer - Source image data Cr start address in Memory"]
pub type SrcImageCrMstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source image data Cr start address in Memory"]
    #[inline(always)]
    pub fn src_image_cr_mst(&self) -> SrcImageCrMstR {
        SrcImageCrMstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source image data Cr start address in Memory"]
    #[inline(always)]
    #[must_use]
    pub fn src_image_cr_mst(&mut self) -> SrcImageCrMstW<SrcAddrCrSpec> {
        SrcImageCrMstW::new(self, 0)
    }
}
#[doc = "Start address of source image(Cr)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcAddrCrSpec;
impl crate::RegisterSpec for SrcAddrCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_addr_cr::R`](R) reader structure"]
impl crate::Readable for SrcAddrCrSpec {}
#[doc = "`write(|w| ..)` method takes [`src_addr_cr::W`](W) writer structure"]
impl crate::Writable for SrcAddrCrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC_ADDR_CR to value 0"]
impl crate::Resettable for SrcAddrCrSpec {
    const RESET_VALUE: u32 = 0;
}
