#[doc = "Register `WIN0_SCL_FACTOR_CBR` reader"]
pub type R = crate::R<Win0SclFactorCbrSpec>;
#[doc = "Register `WIN0_SCL_FACTOR_CBR` writer"]
pub type W = crate::W<Win0SclFactorCbrSpec>;
#[doc = "Field `WIN0_HS_FACTOR_CBR` reader - Win0 CBR horizontal scaling factor:\n\nYCbCr422,YCbCr420:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[15:0\\]/2)\n\n/(LCDC_WIN0_DSP_INFO\\[15:0\\]))*2^12\n\nYCbCr444:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[15:0\\])\n\n/(LCDC_WIN0_DSP_INFO\\[15:0\\]))*2^12"]
pub type Win0HsFactorCbrR = crate::FieldReader<u16>;
#[doc = "Field `WIN0_HS_FACTOR_CBR` writer - Win0 CBR horizontal scaling factor:\n\nYCbCr422,YCbCr420:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[15:0\\]/2)\n\n/(LCDC_WIN0_DSP_INFO\\[15:0\\]))*2^12\n\nYCbCr444:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[15:0\\])\n\n/(LCDC_WIN0_DSP_INFO\\[15:0\\]))*2^12"]
pub type Win0HsFactorCbrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WIN0_VS_FACTOR_CBR` reader - Win0 CBR vertical scaling factor:\n\nYCbCr420:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[31:16\\]/ 2)\n\n/(LCDC_WIN0_DSP_INFO\\[31:16\\]
))*2^12\n\nYCbCr422,YCbCr444:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[31:16\\])\n\n/(LCDC_WIN0_DSP_INFO\\[31:16\\]
))*2^12"]
pub type Win0VsFactorCbrR = crate::FieldReader<u16>;
#[doc = "Field `WIN0_VS_FACTOR_CBR` writer - Win0 CBR vertical scaling factor:\n\nYCbCr420:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[31:16\\]/ 2)\n\n/(LCDC_WIN0_DSP_INFO\\[31:16\\]
))*2^12\n\nYCbCr422,YCbCr444:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[31:16\\])\n\n/(LCDC_WIN0_DSP_INFO\\[31:16\\]
))*2^12"]
pub type Win0VsFactorCbrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Win0 CBR horizontal scaling factor:\n\nYCbCr422,YCbCr420:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[15:0\\]/2)\n\n/(LCDC_WIN0_DSP_INFO\\[15:0\\]))*2^12\n\nYCbCr444:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[15:0\\])\n\n/(LCDC_WIN0_DSP_INFO\\[15:0\\]))*2^12"]
    #[inline(always)]
    pub fn win0_hs_factor_cbr(&self) -> Win0HsFactorCbrR {
        Win0HsFactorCbrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Win0 CBR vertical scaling factor:\n\nYCbCr420:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[31:16\\]/ 2)\n\n/(LCDC_WIN0_DSP_INFO\\[31:16\\]
))*2^12\n\nYCbCr422,YCbCr444:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[31:16\\])\n\n/(LCDC_WIN0_DSP_INFO\\[31:16\\]
))*2^12"]
    #[inline(always)]
    pub fn win0_vs_factor_cbr(&self) -> Win0VsFactorCbrR {
        Win0VsFactorCbrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Win0 CBR horizontal scaling factor:\n\nYCbCr422,YCbCr420:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[15:0\\]/2)\n\n/(LCDC_WIN0_DSP_INFO\\[15:0\\]))*2^12\n\nYCbCr444:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[15:0\\])\n\n/(LCDC_WIN0_DSP_INFO\\[15:0\\]))*2^12"]
    #[inline(always)]
    #[must_use]
    pub fn win0_hs_factor_cbr(&mut self) -> Win0HsFactorCbrW<Win0SclFactorCbrSpec> {
        Win0HsFactorCbrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Win0 CBR vertical scaling factor:\n\nYCbCr420:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[31:16\\]/ 2)\n\n/(LCDC_WIN0_DSP_INFO\\[31:16\\]
))*2^12\n\nYCbCr422,YCbCr444:\n\nfactor=((LCDC_WIN0_ACT_INFO\\[31:16\\])\n\n/(LCDC_WIN0_DSP_INFO\\[31:16\\]
))*2^12"]
    #[inline(always)]
    #[must_use]
    pub fn win0_vs_factor_cbr(&mut self) -> Win0VsFactorCbrW<Win0SclFactorCbrSpec> {
        Win0VsFactorCbrW::new(self, 16)
    }
}
#[doc = "Win0 Cbr scaling factor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_scl_factor_cbr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_scl_factor_cbr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win0SclFactorCbrSpec;
impl crate::RegisterSpec for Win0SclFactorCbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win0_scl_factor_cbr::R`](R) reader structure"]
impl crate::Readable for Win0SclFactorCbrSpec {}
#[doc = "`write(|w| ..)` method takes [`win0_scl_factor_cbr::W`](W) writer structure"]
impl crate::Writable for Win0SclFactorCbrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN0_SCL_FACTOR_CBR to value 0x1000_1000"]
impl crate::Resettable for Win0SclFactorCbrSpec {
    const RESET_VALUE: u32 = 0x1000_1000;
}
