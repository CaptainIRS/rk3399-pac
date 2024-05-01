#[doc = "Register `WIN1_SCL_FACTOR_YRGB` reader"]
pub type R = crate::R<Win1SclFactorYrgbSpec>;
#[doc = "Register `WIN1_SCL_FACTOR_YRGB` writer"]
pub type W = crate::W<Win1SclFactorYrgbSpec>;
#[doc = "Field `WIN1_HS_FACTOR_YRGB` reader - Win1 YRGB horizontal scaling factor:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[15:0\\])\n\n/(LCDC_WIN1_DSP_INFO\\[15:0\\]))*2^12"]
pub type Win1HsFactorYrgbR = crate::FieldReader<u16>;
#[doc = "Field `WIN1_HS_FACTOR_YRGB` writer - Win1 YRGB horizontal scaling factor:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[15:0\\])\n\n/(LCDC_WIN1_DSP_INFO\\[15:0\\]))*2^12"]
pub type Win1HsFactorYrgbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WIN1_VS_FACTOR_YRGB` reader - Win1 YRGB vertical scaling factor:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[31:16\\])\n\n/(LCDC_WIN1_DSP_INFO\\[31:16\\]))*2^12"]
pub type Win1VsFactorYrgbR = crate::FieldReader<u16>;
#[doc = "Field `WIN1_VS_FACTOR_YRGB` writer - Win1 YRGB vertical scaling factor:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[31:16\\])\n\n/(LCDC_WIN1_DSP_INFO\\[31:16\\]))*2^12"]
pub type Win1VsFactorYrgbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Win1 YRGB horizontal scaling factor:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[15:0\\])\n\n/(LCDC_WIN1_DSP_INFO\\[15:0\\]))*2^12"]
    #[inline(always)]
    pub fn win1_hs_factor_yrgb(&self) -> Win1HsFactorYrgbR {
        Win1HsFactorYrgbR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Win1 YRGB vertical scaling factor:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[31:16\\])\n\n/(LCDC_WIN1_DSP_INFO\\[31:16\\]))*2^12"]
    #[inline(always)]
    pub fn win1_vs_factor_yrgb(&self) -> Win1VsFactorYrgbR {
        Win1VsFactorYrgbR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Win1 YRGB horizontal scaling factor:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[15:0\\])\n\n/(LCDC_WIN1_DSP_INFO\\[15:0\\]))*2^12"]
    #[inline(always)]
    #[must_use]
    pub fn win1_hs_factor_yrgb(&mut self) -> Win1HsFactorYrgbW<Win1SclFactorYrgbSpec> {
        Win1HsFactorYrgbW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Win1 YRGB vertical scaling factor:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[31:16\\])\n\n/(LCDC_WIN1_DSP_INFO\\[31:16\\]))*2^12"]
    #[inline(always)]
    #[must_use]
    pub fn win1_vs_factor_yrgb(&mut self) -> Win1VsFactorYrgbW<Win1SclFactorYrgbSpec> {
        Win1VsFactorYrgbW::new(self, 16)
    }
}
#[doc = "Win1 YRGB scaling factor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_scl_factor_yrgb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_scl_factor_yrgb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win1SclFactorYrgbSpec;
impl crate::RegisterSpec for Win1SclFactorYrgbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win1_scl_factor_yrgb::R`](R) reader structure"]
impl crate::Readable for Win1SclFactorYrgbSpec {}
#[doc = "`write(|w| ..)` method takes [`win1_scl_factor_yrgb::W`](W) writer structure"]
impl crate::Writable for Win1SclFactorYrgbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN1_SCL_FACTOR_YRGB to value 0x1000_1000"]
impl crate::Resettable for Win1SclFactorYrgbSpec {
    const RESET_VALUE: u32 = 0x1000_1000;
}
