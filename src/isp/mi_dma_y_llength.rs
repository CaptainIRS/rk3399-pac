#[doc = "Register `MI_DMA_Y_LLENGTH` reader"]
pub type R = crate::R<MiDmaYLlengthSpec>;
#[doc = "Register `MI_DMA_Y_LLENGTH` writer"]
pub type W = crate::W<MiDmaYLlengthSpec>;
#[doc = "Field `dma_y_llength` reader - Line length of the Y component of the original image in\n\nmemory For an uncropped image, where lines follow each\n\nother without offset (no line stride), line length must\n\nmatch image width.\n\nFor Y component the line length in 4:2:x planar mode\n\nmust be a multiple of 8, for all other component modes a\n\nmultiple of 4.\n\nIn planar mode the line length of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\nhalf for 4:2:x and the same size for 4:4:4. In semi planar\n\n4:2:x mode the line length of the Cb component (which\n\nincludes Cr) is assumed the same size. In interleave mode\n\nno Cb/Cr line length is used."]
pub type DmaYLlengthR = crate::FieldReader<u16>;
#[doc = "Field `dma_y_llength` writer - Line length of the Y component of the original image in\n\nmemory For an uncropped image, where lines follow each\n\nother without offset (no line stride), line length must\n\nmatch image width.\n\nFor Y component the line length in 4:2:x planar mode\n\nmust be a multiple of 8, for all other component modes a\n\nmultiple of 4.\n\nIn planar mode the line length of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\nhalf for 4:2:x and the same size for 4:4:4. In semi planar\n\n4:2:x mode the line length of the Cb component (which\n\nincludes Cr) is assumed the same size. In interleave mode\n\nno Cb/Cr line length is used."]
pub type DmaYLlengthW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Line length of the Y component of the original image in\n\nmemory For an uncropped image, where lines follow each\n\nother without offset (no line stride), line length must\n\nmatch image width.\n\nFor Y component the line length in 4:2:x planar mode\n\nmust be a multiple of 8, for all other component modes a\n\nmultiple of 4.\n\nIn planar mode the line length of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\nhalf for 4:2:x and the same size for 4:4:4. In semi planar\n\n4:2:x mode the line length of the Cb component (which\n\nincludes Cr) is assumed the same size. In interleave mode\n\nno Cb/Cr line length is used."]
    #[inline(always)]
    pub fn dma_y_llength(&self) -> DmaYLlengthR {
        DmaYLlengthR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Line length of the Y component of the original image in\n\nmemory For an uncropped image, where lines follow each\n\nother without offset (no line stride), line length must\n\nmatch image width.\n\nFor Y component the line length in 4:2:x planar mode\n\nmust be a multiple of 8, for all other component modes a\n\nmultiple of 4.\n\nIn planar mode the line length of the Cb and Cr\n\ncomponent is assumed according to the YCbCr format, i.e.\n\nhalf for 4:2:x and the same size for 4:4:4. In semi planar\n\n4:2:x mode the line length of the Cb component (which\n\nincludes Cr) is assumed the same size. In interleave mode\n\nno Cb/Cr line length is used."]
    #[inline(always)]
    #[must_use]
    pub fn dma_y_llength(&mut self) -> DmaYLlengthW<MiDmaYLlengthSpec> {
        DmaYLlengthW::new(self, 0)
    }
}
#[doc = "Y component original line length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_dma_y_llength::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_dma_y_llength::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiDmaYLlengthSpec;
impl crate::RegisterSpec for MiDmaYLlengthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_dma_y_llength::R`](R) reader structure"]
impl crate::Readable for MiDmaYLlengthSpec {}
#[doc = "`write(|w| ..)` method takes [`mi_dma_y_llength::W`](W) writer structure"]
impl crate::Writable for MiDmaYLlengthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_DMA_Y_LLENGTH to value 0"]
impl crate::Resettable for MiDmaYLlengthSpec {
    const RESET_VALUE: u32 = 0;
}
