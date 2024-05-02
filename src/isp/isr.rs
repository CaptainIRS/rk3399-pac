#[doc = "Register `ISR` writer"]
pub type W = crate::W<IsrSpec>;
#[doc = "Field `ISR_ISP_OFF` writer - set interrupt"]
pub type IsrIspOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_FRAME` writer - set interrupt"]
pub type IsrFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_DATA_LOSS` writer - set interrupt"]
pub type IsrDataLossW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_PIC_SIZE_ERR` writer - set interrupt"]
pub type IsrPicSizeErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_AWB_DONE` writer - set interrupt"]
pub type IsrAwbDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_FRAME_IN` writer - set interrupt"]
pub type IsrFrameInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_V_START` writer - set interrupt"]
pub type IsrVStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_H_START` writer - set interrupt"]
pub type IsrHStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_FLASH_ON` writer - set interrupt"]
pub type IsrFlashOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_FLASH_OFF` writer - set interrupt"]
pub type IsrFlashOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_SHUTTER_ON` writer - set interrupt"]
pub type IsrShutterOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_SHUTTER_OFF` writer - set interrupt"]
pub type IsrShutterOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_AFM_SUM_OF` writer - set interrupt"]
pub type IsrAfmSumOfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_AFM_LUM_OF` writer - set interrupt"]
pub type IsrAfmLumOfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_AFM_FIN` writer - set interrupt"]
pub type IsrAfmFinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_FLASH_CAP` writer - set interrupt"]
pub type IsrFlashCapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_EXP_END` writer - set interrupt"]
pub type IsrExpEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISR_VSM_END` writer - set interrupt"]
pub type IsrVsmEndW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - set interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isr_isp_off(&mut self) -> IsrIspOffW<IsrSpec> {
        IsrIspOffW::new(self, 0)
    }
    #[doc = "Bit 1 - set interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isr_frame(&mut self) -> IsrFrameW<IsrSpec> {
        IsrFrameW::new(self, 1)
    }
    #[doc = "Bit 2 - set interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isr_data_loss(&mut self) -> IsrDataLossW<IsrSpec> {
        IsrDataLossW::new(self, 2)
    }
    #[doc = "Bit 3 - set interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isr_pic_size_err(&mut self) -> IsrPicSizeErrW<IsrSpec> {
        IsrPicSizeErrW::new(self, 3)
    }
    #[doc = "Bit 4 - set interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isr_awb_done(&mut self) -> IsrAwbDoneW<IsrSpec> {
        IsrAwbDoneW::new(self, 4)
    }
    #[doc = "Bit 5 - set interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isr_frame_in(&mut self) -> IsrFrameInW<IsrSpec> {
        IsrFrameInW::new(self, 5)
    }
    #[doc = "Bit 6 - set interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isr_v_start(&mut self) -> IsrVStartW<IsrSpec> {
        IsrVStartW::new(self, 6)
    }
    #[doc = "Bit 7 - set interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isr_h_start(&mut self) -> IsrHStartW<IsrSpec> {
        IsrHStartW::new(self, 7)
    }
    #[doc = "Bit 8 - set interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isr_flash_on(&mut self) -> IsrFlashOnW<IsrSpec> {
        IsrFlashOnW::new(self, 8)
    }
    #[doc = "Bit 9 - set interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isr_flash_off(&mut self) -> IsrFlashOffW<IsrSpec> {
        IsrFlashOffW::new(self, 9)
    }
    #[doc = "Bit 10 - set interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isr_shutter_on(&mut self) -> IsrShutterOnW<IsrSpec> {
        IsrShutterOnW::new(self, 10)
    }
    #[doc = "Bit 11 - set interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isr_shutter_off(&mut self) -> IsrShutterOffW<IsrSpec> {
        IsrShutterOffW::new(self, 11)
    }
    #[doc = "Bit 12 - set interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isr_afm_sum_of(&mut self) -> IsrAfmSumOfW<IsrSpec> {
        IsrAfmSumOfW::new(self, 12)
    }
    #[doc = "Bit 13 - set interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isr_afm_lum_of(&mut self) -> IsrAfmLumOfW<IsrSpec> {
        IsrAfmLumOfW::new(self, 13)
    }
    #[doc = "Bit 14 - set interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isr_afm_fin(&mut self) -> IsrAfmFinW<IsrSpec> {
        IsrAfmFinW::new(self, 14)
    }
    #[doc = "Bit 17 - set interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isr_flash_cap(&mut self) -> IsrFlashCapW<IsrSpec> {
        IsrFlashCapW::new(self, 17)
    }
    #[doc = "Bit 18 - set interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isr_exp_end(&mut self) -> IsrExpEndW<IsrSpec> {
        IsrExpEndW::new(self, 18)
    }
    #[doc = "Bit 19 - set interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isr_vsm_end(&mut self) -> IsrVsmEndW<IsrSpec> {
        IsrVsmEndW::new(self, 19)
    }
}
#[doc = "Interrupt set register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for IsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {
    const RESET_VALUE: u32 = 0;
}
