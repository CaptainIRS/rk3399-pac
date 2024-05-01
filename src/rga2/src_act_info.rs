#[doc = "Register `SRC_ACT_INFO` reader"]
pub type R = crate::R<SrcActInfoSpec>;
#[doc = "Register `SRC_ACT_INFO` writer"]
pub type W = crate::W<SrcActInfoSpec>;
#[doc = "Field `SW_SRC_ACT_WIDTH` reader - source image active width"]
pub type SwSrcActWidthR = crate::FieldReader<u16>;
#[doc = "Field `SW_SRC_ACT_WIDTH` writer - source image active width"]
pub type SwSrcActWidthW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `SW_SRC_ACT_HEIGHT` reader - source image active height"]
pub type SwSrcActHeightR = crate::FieldReader<u16>;
#[doc = "Field `SW_SRC_ACT_HEIGHT` writer - source image active height"]
pub type SwSrcActHeightW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - source image active width"]
    #[inline(always)]
    pub fn sw_src_act_width(&self) -> SwSrcActWidthR {
        SwSrcActWidthR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - source image active height"]
    #[inline(always)]
    pub fn sw_src_act_height(&self) -> SwSrcActHeightR {
        SwSrcActHeightR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - source image active width"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_act_width(&mut self) -> SwSrcActWidthW<SrcActInfoSpec> {
        SwSrcActWidthW::new(self, 0)
    }
    #[doc = "Bits 16:28 - source image active height"]
    #[inline(always)]
    #[must_use]
    pub fn sw_src_act_height(&mut self) -> SwSrcActHeightW<SrcActInfoSpec> {
        SwSrcActHeightW::new(self, 16)
    }
}
#[doc = "RGA source image active width/height register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_act_info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_act_info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcActInfoSpec;
impl crate::RegisterSpec for SrcActInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_act_info::R`](R) reader structure"]
impl crate::Readable for SrcActInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`src_act_info::W`](W) writer structure"]
impl crate::Writable for SrcActInfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRC_ACT_INFO to value 0"]
impl crate::Resettable for SrcActInfoSpec {
    const RESET_VALUE: u32 = 0;
}
