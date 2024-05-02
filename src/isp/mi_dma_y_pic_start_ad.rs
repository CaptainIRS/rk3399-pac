#[doc = "Register `MI_DMA_Y_PIC_START_AD` reader"]
pub type R = crate::R<MiDmaYPicStartAdSpec>;
#[doc = "Register `MI_DMA_Y_PIC_START_AD` writer"]
pub type W = crate::W<MiDmaYPicStartAdSpec>;
#[doc = "Field `dma_y_pic_start_ad` reader - Image start address of the y component"]
pub type DmaYPicStartAdR = crate::FieldReader<u32>;
#[doc = "Field `dma_y_pic_start_ad` writer - Image start address of the y component"]
pub type DmaYPicStartAdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Image start address of the y component"]
    #[inline(always)]
    pub fn dma_y_pic_start_ad(&self) -> DmaYPicStartAdR {
        DmaYPicStartAdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Image start address of the y component"]
    #[inline(always)]
    #[must_use]
    pub fn dma_y_pic_start_ad(&mut self) -> DmaYPicStartAdW<MiDmaYPicStartAdSpec> {
        DmaYPicStartAdW::new(self, 0)
    }
}
#[doc = "Y component image start address\n\nNote: Must be multiple of 4 in interleaved mode. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_dma_y_pic_start_ad::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_dma_y_pic_start_ad::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiDmaYPicStartAdSpec;
impl crate::RegisterSpec for MiDmaYPicStartAdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_dma_y_pic_start_ad::R`](R) reader structure"]
impl crate::Readable for MiDmaYPicStartAdSpec {}
#[doc = "`write(|w| ..)` method takes [`mi_dma_y_pic_start_ad::W`](W) writer structure"]
impl crate::Writable for MiDmaYPicStartAdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_DMA_Y_PIC_START_AD to value 0"]
impl crate::Resettable for MiDmaYPicStartAdSpec {
    const RESET_VALUE: u32 = 0;
}
