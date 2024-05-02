#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `RIS_ISP_OFF` reader - isp output was disabled (vsynced) due to f_cnt\n\nreached or manual"]
pub type RisIspOffR = crate::BitReader;
#[doc = "Field `RIS_FRAME` reader - frame was completely put out"]
pub type RisFrameR = crate::BitReader;
#[doc = "Field `RIS_DATA_LOSS` reader - loss of data occurred within a line, processing failure"]
pub type RisDataLossR = crate::BitReader;
#[doc = "Field `RIS_PIC_SIZE_ERR` reader - pic size violation occurred, programming seems wrong"]
pub type RisPicSizeErrR = crate::BitReader;
#[doc = "Field `RIS_AWB_DONE` reader - White balancing measurement cycle is complete,\n\nresults can be read out"]
pub type RisAwbDoneR = crate::BitReader;
#[doc = "Field `RIS_FRAME_IN` reader - sampled input frame is complete"]
pub type RisFrameInR = crate::BitReader;
#[doc = "Field `RIS_V_START` reader - Start edge of v_sync"]
pub type RisVStartR = crate::BitReader;
#[doc = "Field `RIS_H_START` reader - Start edge of h_sync"]
pub type RisHStartR = crate::BitReader;
#[doc = "Field `RIS_FLASH_ON` reader - Flash light is switched on"]
pub type RisFlashOnR = crate::BitReader;
#[doc = "Field `RIS_FLASH_OFF` reader - Flash light is switched off"]
pub type RisFlashOffR = crate::BitReader;
#[doc = "Field `RIS_SHUTTER_ON` reader - Mechanical shutter is switched on"]
pub type RisShutterOnR = crate::BitReader;
#[doc = "Field `RIS_SHUTTER_OFF` reader - Mechanical shutter is switched off"]
pub type RisShutterOffR = crate::BitReader;
#[doc = "Field `RIS_AFM_SUM_OF` reader - Auto focus sum overflow"]
pub type RisAfmSumOfR = crate::BitReader;
#[doc = "Field `RIS_AFM_LUM_OF` reader - Auto focus luminance overflow"]
pub type RisAfmLumOfR = crate::BitReader;
#[doc = "Field `RIS_AFM_FIN` reader - AF measurement finished: this interrupt is set when\n\nthe first complete frame is calculated after enabling the AF\n\nmeasurement"]
pub type RisAfmFinR = crate::BitReader;
#[doc = "Field `RIS_FLASH_CAP` reader - Signaling captured frame"]
pub type RisFlashCapR = crate::BitReader;
#[doc = "Field `RIS_EXP_END` reader - Exposure measurement complete"]
pub type RisExpEndR = crate::BitReader;
#[doc = "Field `RIS_VSM_END` reader - VSM measurement complete"]
pub type RisVsmEndR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - isp output was disabled (vsynced) due to f_cnt\n\nreached or manual"]
    #[inline(always)]
    pub fn ris_isp_off(&self) -> RisIspOffR {
        RisIspOffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - frame was completely put out"]
    #[inline(always)]
    pub fn ris_frame(&self) -> RisFrameR {
        RisFrameR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - loss of data occurred within a line, processing failure"]
    #[inline(always)]
    pub fn ris_data_loss(&self) -> RisDataLossR {
        RisDataLossR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - pic size violation occurred, programming seems wrong"]
    #[inline(always)]
    pub fn ris_pic_size_err(&self) -> RisPicSizeErrR {
        RisPicSizeErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - White balancing measurement cycle is complete,\n\nresults can be read out"]
    #[inline(always)]
    pub fn ris_awb_done(&self) -> RisAwbDoneR {
        RisAwbDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - sampled input frame is complete"]
    #[inline(always)]
    pub fn ris_frame_in(&self) -> RisFrameInR {
        RisFrameInR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Start edge of v_sync"]
    #[inline(always)]
    pub fn ris_v_start(&self) -> RisVStartR {
        RisVStartR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Start edge of h_sync"]
    #[inline(always)]
    pub fn ris_h_start(&self) -> RisHStartR {
        RisHStartR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash light is switched on"]
    #[inline(always)]
    pub fn ris_flash_on(&self) -> RisFlashOnR {
        RisFlashOnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Flash light is switched off"]
    #[inline(always)]
    pub fn ris_flash_off(&self) -> RisFlashOffR {
        RisFlashOffR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mechanical shutter is switched on"]
    #[inline(always)]
    pub fn ris_shutter_on(&self) -> RisShutterOnR {
        RisShutterOnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mechanical shutter is switched off"]
    #[inline(always)]
    pub fn ris_shutter_off(&self) -> RisShutterOffR {
        RisShutterOffR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Auto focus sum overflow"]
    #[inline(always)]
    pub fn ris_afm_sum_of(&self) -> RisAfmSumOfR {
        RisAfmSumOfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Auto focus luminance overflow"]
    #[inline(always)]
    pub fn ris_afm_lum_of(&self) -> RisAfmLumOfR {
        RisAfmLumOfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AF measurement finished: this interrupt is set when\n\nthe first complete frame is calculated after enabling the AF\n\nmeasurement"]
    #[inline(always)]
    pub fn ris_afm_fin(&self) -> RisAfmFinR {
        RisAfmFinR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - Signaling captured frame"]
    #[inline(always)]
    pub fn ris_flash_cap(&self) -> RisFlashCapR {
        RisFlashCapR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Exposure measurement complete"]
    #[inline(always)]
    pub fn ris_exp_end(&self) -> RisExpEndR {
        RisExpEndR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - VSM measurement complete"]
    #[inline(always)]
    pub fn ris_vsm_end(&self) -> RisVsmEndR {
        RisVsmEndR::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0;
}
