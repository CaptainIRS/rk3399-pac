#[doc = "Register `SRC_ADDR_Y_ITEMP` reader"]
pub type R = crate::R<SrcAddrYItempSpec>;
#[doc = "Register `SRC_ADDR_Y_ITEMP` writer"]
pub type W = crate::W<SrcAddrYItempSpec>;
#[doc = "Field `SRC_IMAGE_Y_MST_ITEMP` reader - Interger part source image data Y start address in Memory"]
pub type SrcImageYMstItempR = crate::FieldReader<u32>;
#[doc = "Field `SRC_IMAGE_Y_MST_ITEMP` writer - Interger part source image data Y start address in Memory"]
pub type SrcImageYMstItempW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interger part source image data Y start address in Memory"]
    #[inline(always)]
    pub fn src_image_y_mst_itemp(&self) -> SrcImageYMstItempR {
        SrcImageYMstItempR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interger part source image data Y start address in Memory"]
    #[inline(always)]
    #[must_use]
    pub fn src_image_y_mst_itemp(&mut self) -> SrcImageYMstItempW<SrcAddrYItempSpec> {
        SrcImageYMstItempW::new(self, 0)
    }
}
#[doc = "Start address of source image(Y integer part)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr_y_itemp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr_y_itemp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcAddrYItempSpec;
impl crate::RegisterSpec for SrcAddrYItempSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_addr_y_itemp::R`](R) reader structure"]
impl crate::Readable for SrcAddrYItempSpec {}
#[doc = "`write(|w| ..)` method takes [`src_addr_y_itemp::W`](W) writer structure"]
impl crate::Writable for SrcAddrYItempSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC_ADDR_Y_ITEMP to value 0"]
impl crate::Resettable for SrcAddrYItempSpec {
    const RESET_VALUE: u32 = 0;
}
