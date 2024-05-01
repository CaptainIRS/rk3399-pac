#[doc = "Register `WIN1_SCL_OFFSET` reader"]
pub type R = crate::R<Win1SclOffsetSpec>;
#[doc = "Register `WIN1_SCL_OFFSET` writer"]
pub type W = crate::W<Win1SclOffsetSpec>;
#[doc = "Field `WIN1_HS_OFFSET_YRGB` reader - Y Horizontal scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
pub type Win1HsOffsetYrgbR = crate::FieldReader;
#[doc = "Field `WIN1_HS_OFFSET_YRGB` writer - Y Horizontal scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
pub type Win1HsOffsetYrgbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN1_HS_OFFSET_CBR` reader - Cbr Horizontal scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
pub type Win1HsOffsetCbrR = crate::FieldReader;
#[doc = "Field `WIN1_HS_OFFSET_CBR` writer - Cbr Horizontal scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
pub type Win1HsOffsetCbrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN1_VS_OFFSET_YRGB` reader - Y Vertical scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
pub type Win1VsOffsetYrgbR = crate::FieldReader;
#[doc = "Field `WIN1_VS_OFFSET_YRGB` writer - Y Vertical scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
pub type Win1VsOffsetYrgbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN1_VS_OFFSET_CBR` reader - Cbr Vertical scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
pub type Win1VsOffsetCbrR = crate::FieldReader;
#[doc = "Field `WIN1_VS_OFFSET_CBR` writer - Cbr Vertical scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
pub type Win1VsOffsetCbrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Y Horizontal scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
    #[inline(always)]
    pub fn win1_hs_offset_yrgb(&self) -> Win1HsOffsetYrgbR {
        Win1HsOffsetYrgbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Cbr Horizontal scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
    #[inline(always)]
    pub fn win1_hs_offset_cbr(&self) -> Win1HsOffsetCbrR {
        Win1HsOffsetCbrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Y Vertical scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
    #[inline(always)]
    pub fn win1_vs_offset_yrgb(&self) -> Win1VsOffsetYrgbR {
        Win1VsOffsetYrgbR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Cbr Vertical scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
    #[inline(always)]
    pub fn win1_vs_offset_cbr(&self) -> Win1VsOffsetCbrR {
        Win1VsOffsetCbrR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Y Horizontal scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
    #[inline(always)]
    #[must_use]
    pub fn win1_hs_offset_yrgb(&mut self) -> Win1HsOffsetYrgbW<Win1SclOffsetSpec> {
        Win1HsOffsetYrgbW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Cbr Horizontal scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
    #[inline(always)]
    #[must_use]
    pub fn win1_hs_offset_cbr(&mut self) -> Win1HsOffsetCbrW<Win1SclOffsetSpec> {
        Win1HsOffsetCbrW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Y Vertical scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
    #[inline(always)]
    #[must_use]
    pub fn win1_vs_offset_yrgb(&mut self) -> Win1VsOffsetYrgbW<Win1SclOffsetSpec> {
        Win1VsOffsetYrgbW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Cbr Vertical scaling start point offset\n\n(0x00~0xff)/0x100 = 0~0.99"]
    #[inline(always)]
    #[must_use]
    pub fn win1_vs_offset_cbr(&mut self) -> Win1VsOffsetCbrW<Win1SclOffsetSpec> {
        Win1VsOffsetCbrW::new(self, 24)
    }
}
#[doc = "Win1 scaling start point offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_scl_offset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_scl_offset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win1SclOffsetSpec;
impl crate::RegisterSpec for Win1SclOffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win1_scl_offset::R`](R) reader structure"]
impl crate::Readable for Win1SclOffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`win1_scl_offset::W`](W) writer structure"]
impl crate::Writable for Win1SclOffsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN1_SCL_OFFSET to value 0"]
impl crate::Resettable for Win1SclOffsetSpec {
    const RESET_VALUE: u32 = 0;
}
