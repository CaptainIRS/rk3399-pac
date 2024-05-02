#[doc = "Register `MI_DMA_CB_PIC_START_AD` reader"]
pub type R = crate::R<MiDmaCbPicStartAdSpec>;
#[doc = "Register `MI_DMA_CB_PIC_START_AD` writer"]
pub type W = crate::W<MiDmaCbPicStartAdSpec>;
#[doc = "Field `dma_cb_pic_start_ad` reader - Image start address of the Cb component"]
pub type DmaCbPicStartAdR = crate::FieldReader<u32>;
#[doc = "Field `dma_cb_pic_start_ad` writer - Image start address of the Cb component"]
pub type DmaCbPicStartAdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Image start address of the Cb component"]
    #[inline(always)]
    pub fn dma_cb_pic_start_ad(&self) -> DmaCbPicStartAdR {
        DmaCbPicStartAdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Image start address of the Cb component"]
    #[inline(always)]
    #[must_use]
    pub fn dma_cb_pic_start_ad(&mut self) -> DmaCbPicStartAdW<MiDmaCbPicStartAdSpec> {
        DmaCbPicStartAdW::new(self, 0)
    }
}
#[doc = "Cb component image start address\n\nNote: Must be multiple of 2 in semi-planar mode. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_dma_cb_pic_start_ad::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mi_dma_cb_pic_start_ad::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiDmaCbPicStartAdSpec;
impl crate::RegisterSpec for MiDmaCbPicStartAdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_dma_cb_pic_start_ad::R`](R) reader structure"]
impl crate::Readable for MiDmaCbPicStartAdSpec {}
#[doc = "`write(|w| ..)` method takes [`mi_dma_cb_pic_start_ad::W`](W) writer structure"]
impl crate::Writable for MiDmaCbPicStartAdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MI_DMA_CB_PIC_START_AD to value 0"]
impl crate::Resettable for MiDmaCbPicStartAdSpec {
    const RESET_VALUE: u32 = 0;
}
