#[doc = "Register `DST_ACT_INFO` reader"]
pub type R = crate::R<DstActInfoSpec>;
#[doc = "Register `DST_ACT_INFO` writer"]
pub type W = crate::W<DstActInfoSpec>;
#[doc = "Field `SW_DST_ACT_WIDTH` reader - Destination image active width"]
pub type SwDstActWidthR = crate::FieldReader<u16>;
#[doc = "Field `SW_DST_ACT_WIDTH` writer - Destination image active width"]
pub type SwDstActWidthW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SW_DST_ACT_HEIGHT` reader - Destination image active height"]
pub type SwDstActHeightR = crate::FieldReader<u16>;
#[doc = "Field `SW_DST_ACT_HEIGHT` writer - Destination image active height"]
pub type SwDstActHeightW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Destination image active width"]
    #[inline(always)]
    pub fn sw_dst_act_width(&self) -> SwDstActWidthR {
        SwDstActWidthR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Destination image active height"]
    #[inline(always)]
    pub fn sw_dst_act_height(&self) -> SwDstActHeightR {
        SwDstActHeightR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Destination image active width"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_act_width(&mut self) -> SwDstActWidthW<DstActInfoSpec> {
        SwDstActWidthW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Destination image active height"]
    #[inline(always)]
    #[must_use]
    pub fn sw_dst_act_height(&mut self) -> SwDstActHeightW<DstActInfoSpec> {
        SwDstActHeightW::new(self, 16)
    }
}
#[doc = "RGA destination image active width/height register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_act_info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_act_info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstActInfoSpec;
impl crate::RegisterSpec for DstActInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_act_info::R`](R) reader structure"]
impl crate::Readable for DstActInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`dst_act_info::W`](W) writer structure"]
impl crate::Writable for DstActInfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DST_ACT_INFO to value 0"]
impl crate::Resettable for DstActInfoSpec {
    const RESET_VALUE: u32 = 0;
}
