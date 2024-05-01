#[doc = "Register `WIN0_SCL_OFFSET` reader"]
pub type R = crate::R<Win0SclOffsetSpec>;
#[doc = "Register `WIN0_SCL_OFFSET` writer"]
pub type W = crate::W<Win0SclOffsetSpec>;
#[doc = "Field `WIN0_HS_OFFSET_YRGB` reader - Y Horizontal scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
pub type Win0HsOffsetYrgbR = crate::FieldReader;
#[doc = "Field `WIN0_HS_OFFSET_YRGB` writer - Y Horizontal scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
pub type Win0HsOffsetYrgbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN0_HS_OFFSET_CBR` reader - Cbr Horizontal scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
pub type Win0HsOffsetCbrR = crate::FieldReader;
#[doc = "Field `WIN0_HS_OFFSET_CBR` writer - Cbr Horizontal scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
pub type Win0HsOffsetCbrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN0_VS_OFFSET_YRGB` reader - Y Vertical scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
pub type Win0VsOffsetYrgbR = crate::FieldReader;
#[doc = "Field `WIN0_VS_OFFSET_YRGB` writer - Y Vertical scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
pub type Win0VsOffsetYrgbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN0_VS_OFFSET_CBR` reader - Cbr Vertical scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
pub type Win0VsOffsetCbrR = crate::FieldReader;
#[doc = "Field `WIN0_VS_OFFSET_CBR` writer - Cbr Vertical scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
pub type Win0VsOffsetCbrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Y Horizontal scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
    #[inline(always)]
    pub fn win0_hs_offset_yrgb(&self) -> Win0HsOffsetYrgbR {
        Win0HsOffsetYrgbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Cbr Horizontal scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
    #[inline(always)]
    pub fn win0_hs_offset_cbr(&self) -> Win0HsOffsetCbrR {
        Win0HsOffsetCbrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Y Vertical scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
    #[inline(always)]
    pub fn win0_vs_offset_yrgb(&self) -> Win0VsOffsetYrgbR {
        Win0VsOffsetYrgbR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Cbr Vertical scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
    #[inline(always)]
    pub fn win0_vs_offset_cbr(&self) -> Win0VsOffsetCbrR {
        Win0VsOffsetCbrR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Y Horizontal scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
    #[inline(always)]
    #[must_use]
    pub fn win0_hs_offset_yrgb(&mut self) -> Win0HsOffsetYrgbW<Win0SclOffsetSpec> {
        Win0HsOffsetYrgbW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Cbr Horizontal scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
    #[inline(always)]
    #[must_use]
    pub fn win0_hs_offset_cbr(&mut self) -> Win0HsOffsetCbrW<Win0SclOffsetSpec> {
        Win0HsOffsetCbrW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Y Vertical scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
    #[inline(always)]
    #[must_use]
    pub fn win0_vs_offset_yrgb(&mut self) -> Win0VsOffsetYrgbW<Win0SclOffsetSpec> {
        Win0VsOffsetYrgbW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Cbr Vertical scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
    #[inline(always)]
    #[must_use]
    pub fn win0_vs_offset_cbr(&mut self) -> Win0VsOffsetCbrW<Win0SclOffsetSpec> {
        Win0VsOffsetCbrW::new(self, 24)
    }
}
#[doc = "Win0 scaling start point offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_scl_offset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_scl_offset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win0SclOffsetSpec;
impl crate::RegisterSpec for Win0SclOffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win0_scl_offset::R`](R) reader structure"]
impl crate::Readable for Win0SclOffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`win0_scl_offset::W`](W) writer structure"]
impl crate::Writable for Win0SclOffsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN0_SCL_OFFSET to value 0"]
impl crate::Resettable for Win0SclOffsetSpec {
    const RESET_VALUE: u32 = 0;
}
