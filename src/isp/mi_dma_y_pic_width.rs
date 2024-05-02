#[doc = "Register `MI_DMA_Y_PIC_WIDTH` reader"]
pub type R = crate::R<MiDmaYPicWidthSpec>;
#[doc = "Register `MI_DMA_Y_PIC_WIDTH` writer"]
pub type W = crate::W<MiDmaYPicWidthSpec>;
#[doc = "Field `dma_y_pic_width` reader - Image width of the Y component in pixel.\n\nFor YCbCr 4:2:x the image width must be a multiple of\n\n2.\n\nIn planar mode the image width of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\nhalf for 4:2:x and the same size for 4:4:4. In semi planar\n\n4:2:x mode the image width of the Cb component (which\n\nincludes Cr) is assumed the same size. In interleave mode\n\nno Cb/Cr image width is used.\n\n"]
pub type DmaYPicWidthR = crate::FieldReader<u16>;
#[doc = "Field `dma_y_pic_width` writer - Image width of the Y component in pixel.\n\nFor YCbCr 4:2:x the image width must be a multiple of\n\n2.\n\nIn planar mode the image width of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\nhalf for 4:2:x and the same size for 4:4:4. In semi planar\n\n4:2:x mode the image width of the Cb component (which\n\nincludes Cr) is assumed the same size. In interleave mode\n\nno Cb/Cr image width is used.\n\n"]
pub type DmaYPicWidthW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Image width of the Y component in pixel.\n\nFor YCbCr 4:2:x the image width must be a multiple of\n\n2.\n\nIn planar mode the image width of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\nhalf for 4:2:x and the same size for 4:4:4. In semi planar\n\n4:2:x mode the image width of the Cb component (which\n\nincludes Cr) is assumed the same size. In interleave mode\n\nno Cb/Cr image width is used.\n\n"]
    #[inline(always)]
    pub fn dma_y_pic_width(&self) -> DmaYPicWidthR {
        DmaYPicWidthR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Image width of the Y component in pixel.\n\nFor YCbCr 4:2:x the image width must be a multiple of\n\n2.\n\nIn planar mode the image width of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\nhalf for 4:2:x and the same size for 4:4:4. In semi planar\n\n4:2:x mode the image width of the Cb component (which\n\nincludes Cr) is assumed the same size. In interleave mode\n\nno Cb/Cr image width is used.\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn dma_y_pic_width(&mut self) -> DmaYPicWidthW<MiDmaYPicWidthSpec> {
        DmaYPicWidthW::new(self, 0)
    }
}
#[doc = "Y component image width\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_dma_y_pic_width::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_dma_y_pic_width::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiDmaYPicWidthSpec;
impl crate::RegisterSpec for MiDmaYPicWidthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_dma_y_pic_width::R`](R) reader structure"]
impl crate::Readable for MiDmaYPicWidthSpec {}
#[doc = "`write(|w| ..)` method takes [`mi_dma_y_pic_width::W`](W) writer structure"]
impl crate::Writable for MiDmaYPicWidthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_DMA_Y_PIC_WIDTH to value 0"]
impl crate::Resettable for MiDmaYPicWidthSpec {
    const RESET_VALUE: u32 = 0;
}
