#[doc = "Register `DST_ADDR_CBCR` reader"]
pub type R = crate::R<DstAddrCbcrSpec>;
#[doc = "Register `DST_ADDR_CBCR` writer"]
pub type W = crate::W<DstAddrCbcrSpec>;
#[doc = "Field `DST_IMAGE_CBCR_MST` reader - Destination image data CBCR start address in Memory"]
pub type DstImageCbcrMstR = crate::FieldReader<u32>;
#[doc = "Field `DST_IMAGE_CBCR_MST` writer - Destination image data CBCR start address in Memory"]
pub type DstImageCbcrMstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination image data CBCR start address in Memory"]
    #[inline(always)]
    pub fn dst_image_cbcr_mst(&self) -> DstImageCbcrMstR {
        DstImageCbcrMstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination image data CBCR start address in Memory"]
    #[inline(always)]
    #[must_use]
    pub fn dst_image_cbcr_mst(&mut self) -> DstImageCbcrMstW<DstAddrCbcrSpec> {
        DstImageCbcrMstW::new(self, 0)
    }
}
#[doc = "Start address of destination image(Cb/Cr)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr_cbcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr_cbcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstAddrCbcrSpec;
impl crate::RegisterSpec for DstAddrCbcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_addr_cbcr::R`](R) reader structure"]
impl crate::Readable for DstAddrCbcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dst_addr_cbcr::W`](W) writer structure"]
impl crate::Writable for DstAddrCbcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DST_ADDR_CBCR to value 0"]
impl crate::Resettable for DstAddrCbcrSpec {
    const RESET_VALUE: u32 = 0;
}
