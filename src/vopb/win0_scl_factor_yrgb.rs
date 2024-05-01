#[doc = "Register `WIN0_SCL_FACTOR_YRGB` reader"]
pub type R = crate::R<Win0SclFactorYrgbSpec>;
#[doc = "Register `WIN0_SCL_FACTOR_YRGB` writer"]
pub type W = crate::W<Win0SclFactorYrgbSpec>;
#[doc = "Field `WIN0_HS_FACTOR_YRGB` reader - Win0 YRGB horizontal scaling factor:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[15:0\\])\n\n/(LCDC_WIN0_DSP_INFO\\[15:0\\]))*2^12"]
pub type Win0HsFactorYrgbR = crate::FieldReader<u16>;
#[doc = "Field `WIN0_HS_FACTOR_YRGB` writer - Win0 YRGB horizontal scaling factor:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[15:0\\])\n\n/(LCDC_WIN0_DSP_INFO\\[15:0\\]))*2^12"]
pub type Win0HsFactorYrgbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WIN0_VS_FACTOR_YRGB` reader - Win0 YRGB vertical scaling factor:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[31:16\\])\n\n/(LCDC_WIN0_DSP_INFO\\[31:16\\]))*2^12"]
pub type Win0VsFactorYrgbR = crate::FieldReader<u16>;
#[doc = "Field `WIN0_VS_FACTOR_YRGB` writer - Win0 YRGB vertical scaling factor:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[31:16\\])\n\n/(LCDC_WIN0_DSP_INFO\\[31:16\\]))*2^12"]
pub type Win0VsFactorYrgbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Win0 YRGB horizontal scaling factor:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[15:0\\])\n\n/(LCDC_WIN0_DSP_INFO\\[15:0\\]))*2^12"]
    #[inline(always)]
    pub fn win0_hs_factor_yrgb(&self) -> Win0HsFactorYrgbR {
        Win0HsFactorYrgbR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Win0 YRGB vertical scaling factor:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[31:16\\])\n\n/(LCDC_WIN0_DSP_INFO\\[31:16\\]))*2^12"]
    #[inline(always)]
    pub fn win0_vs_factor_yrgb(&self) -> Win0VsFactorYrgbR {
        Win0VsFactorYrgbR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Win0 YRGB horizontal scaling factor:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[15:0\\])\n\n/(LCDC_WIN0_DSP_INFO\\[15:0\\]))*2^12"]
    #[inline(always)]
    #[must_use]
    pub fn win0_hs_factor_yrgb(&mut self) -> Win0HsFactorYrgbW<Win0SclFactorYrgbSpec> {
        Win0HsFactorYrgbW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Win0 YRGB vertical scaling factor:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[31:16\\])\n\n/(LCDC_WIN0_DSP_INFO\\[31:16\\]))*2^12"]
    #[inline(always)]
    #[must_use]
    pub fn win0_vs_factor_yrgb(&mut self) -> Win0VsFactorYrgbW<Win0SclFactorYrgbSpec> {
        Win0VsFactorYrgbW::new(self, 16)
    }
}
#[doc = "Win0 YRGB scaling factor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_scl_factor_yrgb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_scl_factor_yrgb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win0SclFactorYrgbSpec;
impl crate::RegisterSpec for Win0SclFactorYrgbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win0_scl_factor_yrgb::R`](R) reader structure"]
impl crate::Readable for Win0SclFactorYrgbSpec {}
#[doc = "`write(|w| ..)` method takes [`win0_scl_factor_yrgb::W`](W) writer structure"]
impl crate::Writable for Win0SclFactorYrgbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN0_SCL_FACTOR_YRGB to value 0x1000_1000"]
impl crate::Resettable for Win0SclFactorYrgbSpec {
    const RESET_VALUE: u32 = 0x1000_1000;
}
