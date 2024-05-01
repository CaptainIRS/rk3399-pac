#[doc = "Register `DST_ADDR_Y_ITEMP` reader"]
pub type R = crate::R<DstAddrYItempSpec>;
#[doc = "Register `DST_ADDR_Y_ITEMP` writer"]
pub type W = crate::W<DstAddrYItempSpec>;
#[doc = "Field `DST_IMAGE_Y_MST_ITEMP` reader - Interger part destination image data Y start address in Memory"]
pub type DstImageYMstItempR = crate::FieldReader<u32>;
#[doc = "Field `DST_IMAGE_Y_MST_ITEMP` writer - Interger part destination image data Y start address in Memory"]
pub type DstImageYMstItempW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interger part destination image data Y start address in Memory"]
    #[inline(always)]
    pub fn dst_image_y_mst_itemp(&self) -> DstImageYMstItempR {
        DstImageYMstItempR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interger part destination image data Y start address in Memory"]
    #[inline(always)]
    #[must_use]
    pub fn dst_image_y_mst_itemp(&mut self) -> DstImageYMstItempW<DstAddrYItempSpec> {
        DstImageYMstItempW::new(self, 0)
    }
}
#[doc = "Start address of destination image(Y integer part)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr_y_itemp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr_y_itemp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstAddrYItempSpec;
impl crate::RegisterSpec for DstAddrYItempSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_addr_y_itemp::R`](R) reader structure"]
impl crate::Readable for DstAddrYItempSpec {}
#[doc = "`write(|w| ..)` method takes [`dst_addr_y_itemp::W`](W) writer structure"]
impl crate::Writable for DstAddrYItempSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DST_ADDR_Y_ITEMP to value 0"]
impl crate::Resettable for DstAddrYItempSpec {
    const RESET_VALUE: u32 = 0;
}
