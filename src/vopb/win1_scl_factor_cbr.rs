#[doc = "Register `WIN1_SCL_FACTOR_CBR` reader"]
pub type R = crate::R<Win1SclFactorCbrSpec>;
#[doc = "Register `WIN1_SCL_FACTOR_CBR` writer"]
pub type W = crate::W<Win1SclFactorCbrSpec>;
#[doc = "Field `WIN1_HS_FACTOR_CBR` reader - Win1 Cbr horizontal scaling factor:\n\nYCbCr422,YCbCr420:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[15:0\\]/2)\n\n/(LCDC_WIN1_DSP_INFO\\[15:0\\]))*2^12\n\nYCbCr444:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[15:0\\])\n\n/(LCDC_WIN1_DSP_INFO\\[15:0\\]))*2^12"]
pub type Win1HsFactorCbrR = crate::FieldReader<u16>;
#[doc = "Field `WIN1_HS_FACTOR_CBR` writer - Win1 Cbr horizontal scaling factor:\n\nYCbCr422,YCbCr420:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[15:0\\]/2)\n\n/(LCDC_WIN1_DSP_INFO\\[15:0\\]))*2^12\n\nYCbCr444:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[15:0\\])\n\n/(LCDC_WIN1_DSP_INFO\\[15:0\\]))*2^12"]
pub type Win1HsFactorCbrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WIN1_VS_FACTOR_CBR` reader - Win1 CBR vertical scaling factor:\n\nYCbCr420:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[31:16\\]/ 2)\n\n/(LCDC_WIN1_DSP_INFO\\[31:16\\]
))*2^12\n\nYCbCr422,YCbCr444:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[31:16\\])\n\n/(LCDC_WIN1_DSP_INFO\\[31:16\\]
))*2^12"]
pub type Win1VsFactorCbrR = crate::FieldReader<u16>;
#[doc = "Field `WIN1_VS_FACTOR_CBR` writer - Win1 CBR vertical scaling factor:\n\nYCbCr420:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[31:16\\]/ 2)\n\n/(LCDC_WIN1_DSP_INFO\\[31:16\\]
))*2^12\n\nYCbCr422,YCbCr444:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[31:16\\])\n\n/(LCDC_WIN1_DSP_INFO\\[31:16\\]
))*2^12"]
pub type Win1VsFactorCbrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Win1 Cbr horizontal scaling factor:\n\nYCbCr422,YCbCr420:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[15:0\\]/2)\n\n/(LCDC_WIN1_DSP_INFO\\[15:0\\]))*2^12\n\nYCbCr444:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[15:0\\])\n\n/(LCDC_WIN1_DSP_INFO\\[15:0\\]))*2^12"]
    #[inline(always)]
    pub fn win1_hs_factor_cbr(&self) -> Win1HsFactorCbrR {
        Win1HsFactorCbrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Win1 CBR vertical scaling factor:\n\nYCbCr420:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[31:16\\]/ 2)\n\n/(LCDC_WIN1_DSP_INFO\\[31:16\\]
))*2^12\n\nYCbCr422,YCbCr444:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[31:16\\])\n\n/(LCDC_WIN1_DSP_INFO\\[31:16\\]
))*2^12"]
    #[inline(always)]
    pub fn win1_vs_factor_cbr(&self) -> Win1VsFactorCbrR {
        Win1VsFactorCbrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Win1 Cbr horizontal scaling factor:\n\nYCbCr422,YCbCr420:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[15:0\\]/2)\n\n/(LCDC_WIN1_DSP_INFO\\[15:0\\]))*2^12\n\nYCbCr444:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[15:0\\])\n\n/(LCDC_WIN1_DSP_INFO\\[15:0\\]))*2^12"]
    #[inline(always)]
    #[must_use]
    pub fn win1_hs_factor_cbr(&mut self) -> Win1HsFactorCbrW<Win1SclFactorCbrSpec> {
        Win1HsFactorCbrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Win1 CBR vertical scaling factor:\n\nYCbCr420:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[31:16\\]/ 2)\n\n/(LCDC_WIN1_DSP_INFO\\[31:16\\]
))*2^12\n\nYCbCr422,YCbCr444:\n\nfactor=((LCDC_WIN1_ACT_INFO\\[31:16\\])\n\n/(LCDC_WIN1_DSP_INFO\\[31:16\\]
))*2^12"]
    #[inline(always)]
    #[must_use]
    pub fn win1_vs_factor_cbr(&mut self) -> Win1VsFactorCbrW<Win1SclFactorCbrSpec> {
        Win1VsFactorCbrW::new(self, 16)
    }
}
#[doc = "Win1 Cbr scaling factor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_scl_factor_cbr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_scl_factor_cbr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win1SclFactorCbrSpec;
impl crate::RegisterSpec for Win1SclFactorCbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win1_scl_factor_cbr::R`](R) reader structure"]
impl crate::Readable for Win1SclFactorCbrSpec {}
#[doc = "`write(|w| ..)` method takes [`win1_scl_factor_cbr::W`](W) writer structure"]
impl crate::Writable for Win1SclFactorCbrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN1_SCL_FACTOR_CBR to value 0x1000_1000"]
impl crate::Resettable for Win1SclFactorCbrSpec {
    const RESET_VALUE: u32 = 0x1000_1000;
}
