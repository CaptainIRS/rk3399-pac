#[doc = "Register `MI_DMA_Y_PIC_SIZE` reader"]
pub type R = crate::R<MiDmaYPicSizeSpec>;
#[doc = "Register `MI_DMA_Y_PIC_SIZE` writer"]
pub type W = crate::W<MiDmaYPicSizeSpec>;
#[doc = "Field `dma_y_pic_size` reader - Image size of the Y component in pixel which has to be\n\nthe Y line length multiplied by the Y image height (dma_y_llength * dma_y_pic_height).\n\nIn planar mode the image size of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\na quarter for 4:2:0, half for 4:2:2 and the same for 4:4:4.\n\nIn semi planar mode the image size of the Cb component\n\n(which includes Cr) is assumed half for 4:2:0 and the\n\nsame size for 4:2:2. In interleave mode no Cb/Cr image\n\nsize is used.\n\n"]
pub type DmaYPicSizeR = crate::FieldReader<u32>;
#[doc = "Field `dma_y_pic_size` writer - Image size of the Y component in pixel which has to be\n\nthe Y line length multiplied by the Y image height (dma_y_llength * dma_y_pic_height).\n\nIn planar mode the image size of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\na quarter for 4:2:0, half for 4:2:2 and the same for 4:4:4.\n\nIn semi planar mode the image size of the Cb component\n\n(which includes Cr) is assumed half for 4:2:0 and the\n\nsame size for 4:2:2. In interleave mode no Cb/Cr image\n\nsize is used.\n\n"]
pub type DmaYPicSizeW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Image size of the Y component in pixel which has to be\n\nthe Y line length multiplied by the Y image height (dma_y_llength * dma_y_pic_height).\n\nIn planar mode the image size of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\na quarter for 4:2:0, half for 4:2:2 and the same for 4:4:4.\n\nIn semi planar mode the image size of the Cb component\n\n(which includes Cr) is assumed half for 4:2:0 and the\n\nsame size for 4:2:2. In interleave mode no Cb/Cr image\n\nsize is used.\n\n"]
    #[inline(always)]
    pub fn dma_y_pic_size(&self) -> DmaYPicSizeR {
        DmaYPicSizeR::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - Image size of the Y component in pixel which has to be\n\nthe Y line length multiplied by the Y image height (dma_y_llength * dma_y_pic_height).\n\nIn planar mode the image size of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\na quarter for 4:2:0, half for 4:2:2 and the same for 4:4:4.\n\nIn semi planar mode the image size of the Cb component\n\n(which includes Cr) is assumed half for 4:2:0 and the\n\nsame size for 4:2:2. In interleave mode no Cb/Cr image\n\nsize is used.\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn dma_y_pic_size(&mut self) -> DmaYPicSizeW<MiDmaYPicSizeSpec> {
        DmaYPicSizeW::new(self, 0)
    }
}
#[doc = "Y component image size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_dma_y_pic_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_dma_y_pic_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiDmaYPicSizeSpec;
impl crate::RegisterSpec for MiDmaYPicSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_dma_y_pic_size::R`](R) reader structure"]
impl crate::Readable for MiDmaYPicSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`mi_dma_y_pic_size::W`](W) writer structure"]
impl crate::Writable for MiDmaYPicSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_DMA_Y_PIC_SIZE to value 0"]
impl crate::Resettable for MiDmaYPicSizeSpec {
    const RESET_VALUE: u32 = 0;
}
