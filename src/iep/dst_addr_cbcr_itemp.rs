#[doc = "Register `DST_ADDR_CBCR_ITEMP` reader"]
pub type R = crate::R<DstAddrCbcrItempSpec>;
#[doc = "Register `DST_ADDR_CBCR_ITEMP` writer"]
pub type W = crate::W<DstAddrCbcrItempSpec>;
#[doc = "Field `DST_IMAGE_CBCR_MST_ITEMP` reader - Int part destination image data CBCR start address in Memory"]
pub type DstImageCbcrMstItempR = crate::FieldReader<u32>;
#[doc = "Field `DST_IMAGE_CBCR_MST_ITEMP` writer - Int part destination image data CBCR start address in Memory"]
pub type DstImageCbcrMstItempW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Int part destination image data CBCR start address in Memory"]
    #[inline(always)]
    pub fn dst_image_cbcr_mst_itemp(&self) -> DstImageCbcrMstItempR {
        DstImageCbcrMstItempR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Int part destination image data CBCR start address in Memory"]
    #[inline(always)]
    #[must_use]
    pub fn dst_image_cbcr_mst_itemp(&mut self) -> DstImageCbcrMstItempW<DstAddrCbcrItempSpec> {
        DstImageCbcrMstItempW::new(self, 0)
    }
}
#[doc = "Start address of destination image(CBCR integer part)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr_cbcr_itemp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr_cbcr_itemp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstAddrCbcrItempSpec;
impl crate::RegisterSpec for DstAddrCbcrItempSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_addr_cbcr_itemp::R`](R) reader structure"]
impl crate::Readable for DstAddrCbcrItempSpec {}
#[doc = "`write(|w| ..)` method takes [`dst_addr_cbcr_itemp::W`](W) writer structure"]
impl crate::Writable for DstAddrCbcrItempSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DST_ADDR_CBCR_ITEMP to value 0"]
impl crate::Resettable for DstAddrCbcrItempSpec {
    const RESET_VALUE: u32 = 0;
}
