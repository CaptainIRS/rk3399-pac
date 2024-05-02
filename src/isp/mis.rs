#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Field `MIS_ISP_OFF` reader - isp was turned off (vsynced) due to f_cnt reached or\n\nmanual"]
pub type MisIspOffR = crate::BitReader;
#[doc = "Field `MIS_FRAME` reader - frame was completely put out"]
pub type MisFrameR = crate::BitReader;
#[doc = "Field `MIS_DATA_LOSS` reader - loss of data occurred within a line, processing failure"]
pub type MisDataLossR = crate::BitReader;
#[doc = "Field `MIS_PIC_SIZE_ERR` reader - pic size violation occurred, programming seems wrong"]
pub type MisPicSizeErrR = crate::BitReader;
#[doc = "Field `MIS_AWB_DONE` reader - White balancing measurement cycle is complete,\n\nresults can be read out"]
pub type MisAwbDoneR = crate::BitReader;
#[doc = "Field `MIS_FRAME_IN` reader - sampled input frame is complete"]
pub type MisFrameInR = crate::BitReader;
#[doc = "Field `MIS_V_START` reader - Start edge of v_sync"]
pub type MisVStartR = crate::BitReader;
#[doc = "Field `MIS_H_START` reader - Start edge of h_sync"]
pub type MisHStartR = crate::BitReader;
#[doc = "Field `MIS_FLASH_ON` reader - Flash light is switched on"]
pub type MisFlashOnR = crate::BitReader;
#[doc = "Field `MIS_FLASH_OFF` reader - Flash light is switched off"]
pub type MisFlashOffR = crate::BitReader;
#[doc = "Field `MIS_SHUTTER_ON` reader - Mechanical shutter is switched on"]
pub type MisShutterOnR = crate::BitReader;
#[doc = "Field `MIS_SHUTTER_OFF` reader - Mechanical shutter is switched off"]
pub type MisShutterOffR = crate::BitReader;
#[doc = "Field `MIS_AFM_SUM_OF` reader - Sum overflow"]
pub type MisAfmSumOfR = crate::BitReader;
#[doc = "Field `MIS_AFM_LUM_OF` reader - Luminance overflow"]
pub type MisAfmLumOfR = crate::BitReader;
#[doc = "Field `MIS_AFM_FIN` reader - AF measurement finished: this interrupt is set when\n\nthe first complete frame is calculated after enabling the AF\n\nmeasurement"]
pub type MisAfmFinR = crate::BitReader;
#[doc = "Field `MIS_FLASH_CAP` reader - Captured is frame is detected"]
pub type MisFlashCapR = crate::BitReader;
#[doc = "Field `MIS_EXP_END` reader - Exposure measurement complete"]
pub type MisExpEndR = crate::BitReader;
#[doc = "Field `MIS_VSM_END` reader - VSM measurement complete"]
pub type MisVsmEndR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - isp was turned off (vsynced) due to f_cnt reached or\n\nmanual"]
    #[inline(always)]
    pub fn mis_isp_off(&self) -> MisIspOffR {
        MisIspOffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - frame was completely put out"]
    #[inline(always)]
    pub fn mis_frame(&self) -> MisFrameR {
        MisFrameR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - loss of data occurred within a line, processing failure"]
    #[inline(always)]
    pub fn mis_data_loss(&self) -> MisDataLossR {
        MisDataLossR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - pic size violation occurred, programming seems wrong"]
    #[inline(always)]
    pub fn mis_pic_size_err(&self) -> MisPicSizeErrR {
        MisPicSizeErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - White balancing measurement cycle is complete,\n\nresults can be read out"]
    #[inline(always)]
    pub fn mis_awb_done(&self) -> MisAwbDoneR {
        MisAwbDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - sampled input frame is complete"]
    #[inline(always)]
    pub fn mis_frame_in(&self) -> MisFrameInR {
        MisFrameInR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Start edge of v_sync"]
    #[inline(always)]
    pub fn mis_v_start(&self) -> MisVStartR {
        MisVStartR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Start edge of h_sync"]
    #[inline(always)]
    pub fn mis_h_start(&self) -> MisHStartR {
        MisHStartR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash light is switched on"]
    #[inline(always)]
    pub fn mis_flash_on(&self) -> MisFlashOnR {
        MisFlashOnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Flash light is switched off"]
    #[inline(always)]
    pub fn mis_flash_off(&self) -> MisFlashOffR {
        MisFlashOffR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mechanical shutter is switched on"]
    #[inline(always)]
    pub fn mis_shutter_on(&self) -> MisShutterOnR {
        MisShutterOnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mechanical shutter is switched off"]
    #[inline(always)]
    pub fn mis_shutter_off(&self) -> MisShutterOffR {
        MisShutterOffR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Sum overflow"]
    #[inline(always)]
    pub fn mis_afm_sum_of(&self) -> MisAfmSumOfR {
        MisAfmSumOfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Luminance overflow"]
    #[inline(always)]
    pub fn mis_afm_lum_of(&self) -> MisAfmLumOfR {
        MisAfmLumOfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AF measurement finished: this interrupt is set when\n\nthe first complete frame is calculated after enabling the AF\n\nmeasurement"]
    #[inline(always)]
    pub fn mis_afm_fin(&self) -> MisAfmFinR {
        MisAfmFinR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - Captured is frame is detected"]
    #[inline(always)]
    pub fn mis_flash_cap(&self) -> MisFlashCapR {
        MisFlashCapR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Exposure measurement complete"]
    #[inline(always)]
    pub fn mis_exp_end(&self) -> MisExpEndR {
        MisExpEndR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - VSM measurement complete"]
    #[inline(always)]
    pub fn mis_vsm_end(&self) -> MisVsmEndR {
        MisVsmEndR::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {
    const RESET_VALUE: u32 = 0;
}
