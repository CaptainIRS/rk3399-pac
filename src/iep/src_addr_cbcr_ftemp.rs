#[doc = "Register `SRC_ADDR_CBCR_FTEMP` reader"]
pub type R = crate::R<SrcAddrCbcrFtempSpec>;
#[doc = "Register `SRC_ADDR_CBCR_FTEMP` writer"]
pub type W = crate::W<SrcAddrCbcrFtempSpec>;
#[doc = "Field `SRC_IMAGE_CBCR_MST_FTEMP` reader - Fraction part source image data CBCR start address in Memory"]
pub type SrcImageCbcrMstFtempR = crate::FieldReader<u32>;
#[doc = "Field `SRC_IMAGE_CBCR_MST_FTEMP` writer - Fraction part source image data CBCR start address in Memory"]
pub type SrcImageCbcrMstFtempW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Fraction part source image data CBCR start address in Memory"]
    #[inline(always)]
    pub fn src_image_cbcr_mst_ftemp(&self) -> SrcImageCbcrMstFtempR {
        SrcImageCbcrMstFtempR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Fraction part source image data CBCR start address in Memory"]
    #[inline(always)]
    #[must_use]
    pub fn src_image_cbcr_mst_ftemp(&mut self) -> SrcImageCbcrMstFtempW<SrcAddrCbcrFtempSpec> {
        SrcImageCbcrMstFtempW::new(self, 0)
    }
}
#[doc = "Start address of source image(CBCR fraction part)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr_cbcr_ftemp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr_cbcr_ftemp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcAddrCbcrFtempSpec;
impl crate::RegisterSpec for SrcAddrCbcrFtempSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_addr_cbcr_ftemp::R`](R) reader structure"]
impl crate::Readable for SrcAddrCbcrFtempSpec {}
#[doc = "`write(|w| ..)` method takes [`src_addr_cbcr_ftemp::W`](W) writer structure"]
impl crate::Writable for SrcAddrCbcrFtempSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC_ADDR_CBCR_FTEMP to value 0"]
impl crate::Resettable for SrcAddrCbcrFtempSpec {
    const RESET_VALUE: u32 = 0;
}
