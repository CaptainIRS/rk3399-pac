#[doc = "Register `SRC_ADDR_CBCR` reader"]
pub type R = crate::R<SrcAddrCbcrSpec>;
#[doc = "Register `SRC_ADDR_CBCR` writer"]
pub type W = crate::W<SrcAddrCbcrSpec>;
#[doc = "Field `SRC_IMAGE_CBCR_MST` reader - Source image data CbCr start address in Memory"]
pub type SrcImageCbcrMstR = crate::FieldReader<u32>;
#[doc = "Field `SRC_IMAGE_CBCR_MST` writer - Source image data CbCr start address in Memory"]
pub type SrcImageCbcrMstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source image data CbCr start address in Memory"]
    #[inline(always)]
    pub fn src_image_cbcr_mst(&self) -> SrcImageCbcrMstR {
        SrcImageCbcrMstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source image data CbCr start address in Memory"]
    #[inline(always)]
    #[must_use]
    pub fn src_image_cbcr_mst(&mut self) -> SrcImageCbcrMstW<SrcAddrCbcrSpec> {
        SrcImageCbcrMstW::new(self, 0)
    }
}
#[doc = "Start address of source image(Cb/Cr)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr_cbcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr_cbcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcAddrCbcrSpec;
impl crate::RegisterSpec for SrcAddrCbcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_addr_cbcr::R`](R) reader structure"]
impl crate::Readable for SrcAddrCbcrSpec {}
#[doc = "`write(|w| ..)` method takes [`src_addr_cbcr::W`](W) writer structure"]
impl crate::Writable for SrcAddrCbcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC_ADDR_CBCR to value 0"]
impl crate::Resettable for SrcAddrCbcrSpec {
    const RESET_VALUE: u32 = 0;
}
