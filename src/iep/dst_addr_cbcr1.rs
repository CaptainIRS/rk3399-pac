#[doc = "Register `DST_ADDR_CBCR1` reader"]
pub type R = crate::R<DstAddrCbcr1Spec>;
#[doc = "Register `DST_ADDR_CBCR1` writer"]
pub type W = crate::W<DstAddrCbcr1Spec>;
#[doc = "Field `DST_IMAGE_CBCR_MST` reader - Destination image data CbCr start address in Memory"]
pub type DstImageCbcrMstR = crate::FieldReader<u32>;
#[doc = "Field `DST_IMAGE_CBCR_MST` writer - Destination image data CbCr start address in Memory"]
pub type DstImageCbcrMstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination image data CbCr start address in Memory"]
    #[inline(always)]
    pub fn dst_image_cbcr_mst(&self) -> DstImageCbcrMstR {
        DstImageCbcrMstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination image data CbCr start address in Memory"]
    #[inline(always)]
    #[must_use]
    pub fn dst_image_cbcr_mst(&mut self) -> DstImageCbcrMstW<DstAddrCbcr1Spec> {
        DstImageCbcrMstW::new(self, 0)
    }
}
#[doc = "Start address of destination image(Cb/Cr)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr_cbcr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr_cbcr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstAddrCbcr1Spec;
impl crate::RegisterSpec for DstAddrCbcr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_addr_cbcr1::R`](R) reader structure"]
impl crate::Readable for DstAddrCbcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`dst_addr_cbcr1::W`](W) writer structure"]
impl crate::Writable for DstAddrCbcr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DST_ADDR_CBCR1 to value 0"]
impl crate::Resettable for DstAddrCbcr1Spec {
    const RESET_VALUE: u32 = 0;
}
