#[doc = "Register `SRC_ADDR_CBCR_ITEMP` reader"]
pub type R = crate::R<SrcAddrCbcrItempSpec>;
#[doc = "Register `SRC_ADDR_CBCR_ITEMP` writer"]
pub type W = crate::W<SrcAddrCbcrItempSpec>;
#[doc = "Field `SRC_IMAGE_CBCR_MST_CBCR_ITEMP` reader - Interger part source image data CBCR start address in Memory"]
pub type SrcImageCbcrMstCbcrItempR = crate::FieldReader<u32>;
#[doc = "Field `SRC_IMAGE_CBCR_MST_CBCR_ITEMP` writer - Interger part source image data CBCR start address in Memory"]
pub type SrcImageCbcrMstCbcrItempW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interger part source image data CBCR start address in Memory"]
    #[inline(always)]
    pub fn src_image_cbcr_mst_cbcr_itemp(&self) -> SrcImageCbcrMstCbcrItempR {
        SrcImageCbcrMstCbcrItempR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interger part source image data CBCR start address in Memory"]
    #[inline(always)]
    #[must_use]
    pub fn src_image_cbcr_mst_cbcr_itemp(
        &mut self,
    ) -> SrcImageCbcrMstCbcrItempW<SrcAddrCbcrItempSpec> {
        SrcImageCbcrMstCbcrItempW::new(self, 0)
    }
}
#[doc = "Start address of source image(CBCR integer part)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr_cbcr_itemp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr_cbcr_itemp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcAddrCbcrItempSpec;
impl crate::RegisterSpec for SrcAddrCbcrItempSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_addr_cbcr_itemp::R`](R) reader structure"]
impl crate::Readable for SrcAddrCbcrItempSpec {}
#[doc = "`write(|w| ..)` method takes [`src_addr_cbcr_itemp::W`](W) writer structure"]
impl crate::Writable for SrcAddrCbcrItempSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC_ADDR_CBCR_ITEMP to value 0"]
impl crate::Resettable for SrcAddrCbcrItempSpec {
    const RESET_VALUE: u32 = 0;
}
