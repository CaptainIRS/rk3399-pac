#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `ICR_ISP_OFF` writer - clear interrupt"]
pub type IcrIspOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICR_FRAME` writer - clear interrupt"]
pub type IcrFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICR_DATA_LOSS` writer - clear interrupt"]
pub type IcrDataLossW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICR_PIC_SIZE_ERR` writer - clear interrupt"]
pub type IcrPicSizeErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICR_AWB_DONE` writer - clear interrupt"]
pub type IcrAwbDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICR_FRAME_IN` writer - clear interrupt"]
pub type IcrFrameInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICR_V_START` writer - clear interrupt"]
pub type IcrVStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICR_H_START` writer - clear interrupt"]
pub type IcrHStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICR_FLASH_ON` writer - clear interrupt"]
pub type IcrFlashOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICR_FLASH_OFF` writer - clear interrupt"]
pub type IcrFlashOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICR_SHUTTER_ON` writer - clear interrupt"]
pub type IcrShutterOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICR_SHUTTER_OFF` writer - clear interrupt"]
pub type IcrShutterOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICR_AFM_SUM_OF` writer - clear interrupt"]
pub type IcrAfmSumOfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICR_AFM_LUM_OF` writer - clear interrupt"]
pub type IcrAfmLumOfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICR_AFM_FIN` writer - clear interrupt"]
pub type IcrAfmFinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICR_FLASH_CAP` writer - clear interrupt"]
pub type IcrFlashCapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICR_EXP_END` writer - clear interrupt"]
pub type IcrExpEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICR_VSM_END` writer - clear interrupt"]
pub type IcrVsmEndW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn icr_isp_off(&mut self) -> IcrIspOffW<IcrSpec> {
        IcrIspOffW::new(self, 0)
    }
    #[doc = "Bit 1 - clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn icr_frame(&mut self) -> IcrFrameW<IcrSpec> {
        IcrFrameW::new(self, 1)
    }
    #[doc = "Bit 2 - clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn icr_data_loss(&mut self) -> IcrDataLossW<IcrSpec> {
        IcrDataLossW::new(self, 2)
    }
    #[doc = "Bit 3 - clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn icr_pic_size_err(&mut self) -> IcrPicSizeErrW<IcrSpec> {
        IcrPicSizeErrW::new(self, 3)
    }
    #[doc = "Bit 4 - clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn icr_awb_done(&mut self) -> IcrAwbDoneW<IcrSpec> {
        IcrAwbDoneW::new(self, 4)
    }
    #[doc = "Bit 5 - clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn icr_frame_in(&mut self) -> IcrFrameInW<IcrSpec> {
        IcrFrameInW::new(self, 5)
    }
    #[doc = "Bit 6 - clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn icr_v_start(&mut self) -> IcrVStartW<IcrSpec> {
        IcrVStartW::new(self, 6)
    }
    #[doc = "Bit 7 - clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn icr_h_start(&mut self) -> IcrHStartW<IcrSpec> {
        IcrHStartW::new(self, 7)
    }
    #[doc = "Bit 8 - clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn icr_flash_on(&mut self) -> IcrFlashOnW<IcrSpec> {
        IcrFlashOnW::new(self, 8)
    }
    #[doc = "Bit 9 - clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn icr_flash_off(&mut self) -> IcrFlashOffW<IcrSpec> {
        IcrFlashOffW::new(self, 9)
    }
    #[doc = "Bit 10 - clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn icr_shutter_on(&mut self) -> IcrShutterOnW<IcrSpec> {
        IcrShutterOnW::new(self, 10)
    }
    #[doc = "Bit 11 - clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn icr_shutter_off(&mut self) -> IcrShutterOffW<IcrSpec> {
        IcrShutterOffW::new(self, 11)
    }
    #[doc = "Bit 12 - clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn icr_afm_sum_of(&mut self) -> IcrAfmSumOfW<IcrSpec> {
        IcrAfmSumOfW::new(self, 12)
    }
    #[doc = "Bit 13 - clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn icr_afm_lum_of(&mut self) -> IcrAfmLumOfW<IcrSpec> {
        IcrAfmLumOfW::new(self, 13)
    }
    #[doc = "Bit 14 - clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn icr_afm_fin(&mut self) -> IcrAfmFinW<IcrSpec> {
        IcrAfmFinW::new(self, 14)
    }
    #[doc = "Bit 17 - clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn icr_flash_cap(&mut self) -> IcrFlashCapW<IcrSpec> {
        IcrFlashCapW::new(self, 17)
    }
    #[doc = "Bit 18 - clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn icr_exp_end(&mut self) -> IcrExpEndW<IcrSpec> {
        IcrExpEndW::new(self, 18)
    }
    #[doc = "Bit 19 - clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn icr_vsm_end(&mut self) -> IcrVsmEndW<IcrSpec> {
        IcrVsmEndW::new(self, 19)
    }
}
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {
    const RESET_VALUE: u32 = 0;
}
