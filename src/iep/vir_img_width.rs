#[doc = "Register `VIR_IMG_WIDTH` reader"]
pub type R = crate::R<VirImgWidthSpec>;
#[doc = "Register `VIR_IMG_WIDTH` writer"]
pub type W = crate::W<VirImgWidthSpec>;
#[doc = "Field `SRC_VIR_IMAGE_WIDTH` reader - Source virtual image width"]
pub type SrcVirImageWidthR = crate::FieldReader<u16>;
#[doc = "Field `SRC_VIR_IMAGE_WIDTH` writer - Source virtual image width"]
pub type SrcVirImageWidthW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DST_VIR_IMAGE_WIDTH` reader - Destination virtual image width"]
pub type DstVirImageWidthR = crate::FieldReader<u16>;
#[doc = "Field `DST_VIR_IMAGE_WIDTH` writer - Destination virtual image width"]
pub type DstVirImageWidthW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Source virtual image width"]
    #[inline(always)]
    pub fn src_vir_image_width(&self) -> SrcVirImageWidthR {
        SrcVirImageWidthR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Destination virtual image width"]
    #[inline(always)]
    pub fn dst_vir_image_width(&self) -> DstVirImageWidthR {
        DstVirImageWidthR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Source virtual image width"]
    #[inline(always)]
    #[must_use]
    pub fn src_vir_image_width(&mut self) -> SrcVirImageWidthW<VirImgWidthSpec> {
        SrcVirImageWidthW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Destination virtual image width"]
    #[inline(always)]
    #[must_use]
    pub fn dst_vir_image_width(&mut self) -> DstVirImageWidthW<VirImgWidthSpec> {
        DstVirImageWidthW::new(self, 16)
    }
}
#[doc = "Image virtual width\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vir_img_width::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vir_img_width::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VirImgWidthSpec;
impl crate::RegisterSpec for VirImgWidthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vir_img_width::R`](R) reader structure"]
impl crate::Readable for VirImgWidthSpec {}
#[doc = "`write(|w| ..)` method takes [`vir_img_width::W`](W) writer structure"]
impl crate::Writable for VirImgWidthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VIR_IMG_WIDTH to value 0x0140_0140"]
impl crate::Resettable for VirImgWidthSpec {
    const RESET_VALUE: u32 = 0x0140_0140;
}
