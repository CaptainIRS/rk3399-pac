#[doc = "Register `SWREG28_VP9_CPRHEADER_CONFIG` reader"]
pub type R = crate::R<Swreg28Vp9CprheaderConfigSpec>;
#[doc = "Register `SWREG28_VP9_CPRHEADER_CONFIG` writer"]
pub type W = crate::W<Swreg28Vp9CprheaderConfigSpec>;
#[doc = "Field `SW_VP9_TX_MODE` reader - tx_mode\n\ntx_mode specifies frame transform mode.\n\nONLY_4X4 = 0, // only 4x4 transform used\n\nALLOW_8X8 = 1, // allow block transform size up\n\nto 8x8\n\nALLOW_16X16 = 2, // allow block transform size\n\nup to 16x16\n\nALLOW_32X32 = 3, // allow block transform size\n\nup to 32x32\n\nTX_MODE_SELECT = 4, // transform specified for\n\neach block"]
pub type SwVp9TxModeR = crate::FieldReader;
#[doc = "Field `SW_VP9_TX_MODE` writer - tx_mode\n\ntx_mode specifies frame transform mode.\n\nONLY_4X4 = 0, // only 4x4 transform used\n\nALLOW_8X8 = 1, // allow block transform size up\n\nto 8x8\n\nALLOW_16X16 = 2, // allow block transform size\n\nup to 16x16\n\nALLOW_32X32 = 3, // allow block transform size\n\nup to 32x32\n\nTX_MODE_SELECT = 4, // transform specified for\n\neach block"]
pub type SwVp9TxModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_VP9_FRAME_REFERENCE_MODE` reader - frame_reference_mode\n\nframe_reference_mode specifies frame reference mode.\n\nSINGLE_REFERENCE= 0,\n\nCOMPOUND_REFERENCE= 1,\n\nREFERENCE_MODE_SELECT= 2,\n\nREFERENCE_MODES= 3,\n\nWhen frame_reference_mode_flag0 is not present ,it equal to 0 by\n\ndefault.\n\nWhen frame_reference_mode_flag1 is not present ,it equal to 0 by\n\ndefault.\n\nframe_reference_mode = frame_reference_mode_flag0 == 0 ?\n\nframe_reference_mode_flag1 == 0 ?\n\nREFERENCE_MODE_SELECT : COMPOUND_REFERENCE) :\n\nSINGLE_REFERENCE"]
pub type SwVp9FrameReferenceModeR = crate::FieldReader;
#[doc = "Field `SW_VP9_FRAME_REFERENCE_MODE` writer - frame_reference_mode\n\nframe_reference_mode specifies frame reference mode.\n\nSINGLE_REFERENCE= 0,\n\nCOMPOUND_REFERENCE= 1,\n\nREFERENCE_MODE_SELECT= 2,\n\nREFERENCE_MODES= 3,\n\nWhen frame_reference_mode_flag0 is not present ,it equal to 0 by\n\ndefault.\n\nWhen frame_reference_mode_flag1 is not present ,it equal to 0 by\n\ndefault.\n\nframe_reference_mode = frame_reference_mode_flag0 == 0 ?\n\nframe_reference_mode_flag1 == 0 ?\n\nREFERENCE_MODE_SELECT : COMPOUND_REFERENCE) :\n\nSINGLE_REFERENCE"]
pub type SwVp9FrameReferenceModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - tx_mode\n\ntx_mode specifies frame transform mode.\n\nONLY_4X4 = 0, // only 4x4 transform used\n\nALLOW_8X8 = 1, // allow block transform size up\n\nto 8x8\n\nALLOW_16X16 = 2, // allow block transform size\n\nup to 16x16\n\nALLOW_32X32 = 3, // allow block transform size\n\nup to 32x32\n\nTX_MODE_SELECT = 4, // transform specified for\n\neach block"]
    #[inline(always)]
    pub fn sw_vp9_tx_mode(&self) -> SwVp9TxModeR {
        SwVp9TxModeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - frame_reference_mode\n\nframe_reference_mode specifies frame reference mode.\n\nSINGLE_REFERENCE= 0,\n\nCOMPOUND_REFERENCE= 1,\n\nREFERENCE_MODE_SELECT= 2,\n\nREFERENCE_MODES= 3,\n\nWhen frame_reference_mode_flag0 is not present ,it equal to 0 by\n\ndefault.\n\nWhen frame_reference_mode_flag1 is not present ,it equal to 0 by\n\ndefault.\n\nframe_reference_mode = frame_reference_mode_flag0 == 0 ?\n\nframe_reference_mode_flag1 == 0 ?\n\nREFERENCE_MODE_SELECT : COMPOUND_REFERENCE) :\n\nSINGLE_REFERENCE"]
    #[inline(always)]
    pub fn sw_vp9_frame_reference_mode(&self) -> SwVp9FrameReferenceModeR {
        SwVp9FrameReferenceModeR::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - tx_mode\n\ntx_mode specifies frame transform mode.\n\nONLY_4X4 = 0, // only 4x4 transform used\n\nALLOW_8X8 = 1, // allow block transform size up\n\nto 8x8\n\nALLOW_16X16 = 2, // allow block transform size\n\nup to 16x16\n\nALLOW_32X32 = 3, // allow block transform size\n\nup to 32x32\n\nTX_MODE_SELECT = 4, // transform specified for\n\neach block"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_tx_mode(&mut self) -> SwVp9TxModeW<Swreg28Vp9CprheaderConfigSpec> {
        SwVp9TxModeW::new(self, 0)
    }
    #[doc = "Bits 3:4 - frame_reference_mode\n\nframe_reference_mode specifies frame reference mode.\n\nSINGLE_REFERENCE= 0,\n\nCOMPOUND_REFERENCE= 1,\n\nREFERENCE_MODE_SELECT= 2,\n\nREFERENCE_MODES= 3,\n\nWhen frame_reference_mode_flag0 is not present ,it equal to 0 by\n\ndefault.\n\nWhen frame_reference_mode_flag1 is not present ,it equal to 0 by\n\ndefault.\n\nframe_reference_mode = frame_reference_mode_flag0 == 0 ?\n\nframe_reference_mode_flag1 == 0 ?\n\nREFERENCE_MODE_SELECT : COMPOUND_REFERENCE) :\n\nSINGLE_REFERENCE"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_frame_reference_mode(
        &mut self,
    ) -> SwVp9FrameReferenceModeW<Swreg28Vp9CprheaderConfigSpec> {
        SwVp9FrameReferenceModeW::new(self, 3)
    }
}
#[doc = "vp9 compressed header config info\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg28_vp9_cprheader_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg28_vp9_cprheader_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg28Vp9CprheaderConfigSpec;
impl crate::RegisterSpec for Swreg28Vp9CprheaderConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg28_vp9_cprheader_config::R`](R) reader structure"]
impl crate::Readable for Swreg28Vp9CprheaderConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg28_vp9_cprheader_config::W`](W) writer structure"]
impl crate::Writable for Swreg28Vp9CprheaderConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG28_VP9_CPRHEADER_CONFIG to value 0"]
impl crate::Resettable for Swreg28Vp9CprheaderConfigSpec {
    const RESET_VALUE: u32 = 0;
}
