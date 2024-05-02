#[doc = "Register `IMSC` reader"]
pub type R = crate::R<ImscSpec>;
#[doc = "Register `IMSC` writer"]
pub type W = crate::W<ImscSpec>;
#[doc = "Field `IMSC_ISP_OFF` reader - enable interrupt (1) or mask out (0)"]
pub type ImscIspOffR = crate::BitReader;
#[doc = "Field `IMSC_ISP_OFF` writer - enable interrupt (1) or mask out (0)"]
pub type ImscIspOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMSC_FRAME` reader - enable interrupt (1) or mask out (0)"]
pub type ImscFrameR = crate::BitReader;
#[doc = "Field `IMSC_FRAME` writer - enable interrupt (1) or mask out (0)"]
pub type ImscFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMSC_DATA_LOSS` reader - enable interrupt (1) or mask out (0)"]
pub type ImscDataLossR = crate::BitReader;
#[doc = "Field `IMSC_DATA_LOSS` writer - enable interrupt (1) or mask out (0)"]
pub type ImscDataLossW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMSC_PIC_SIZE_ERR` reader - enable interrupt (1) or mask out (0)"]
pub type ImscPicSizeErrR = crate::BitReader;
#[doc = "Field `IMSC_PIC_SIZE_ERR` writer - enable interrupt (1) or mask out (0)"]
pub type ImscPicSizeErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMSC_AWB_DONE` reader - enable interrupt (1) or mask out (0)"]
pub type ImscAwbDoneR = crate::BitReader;
#[doc = "Field `IMSC_AWB_DONE` writer - enable interrupt (1) or mask out (0)"]
pub type ImscAwbDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMSC_FRAME_IN` reader - enable interrupt (1) or mask out (0)"]
pub type ImscFrameInR = crate::BitReader;
#[doc = "Field `IMSC_FRAME_IN` writer - enable interrupt (1) or mask out (0)"]
pub type ImscFrameInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMSC_V_START` reader - enable interrupt (1) or mask out (0)"]
pub type ImscVStartR = crate::BitReader;
#[doc = "Field `IMSC_V_START` writer - enable interrupt (1) or mask out (0)"]
pub type ImscVStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMSC_H_START` reader - enable interrupt (1) or mask out (0)"]
pub type ImscHStartR = crate::BitReader;
#[doc = "Field `IMSC_H_START` writer - enable interrupt (1) or mask out (0)"]
pub type ImscHStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMSC_FLASH_ON` reader - enable interrupt (1) or mask out (0)"]
pub type ImscFlashOnR = crate::BitReader;
#[doc = "Field `IMSC_FLASH_ON` writer - enable interrupt (1) or mask out (0)"]
pub type ImscFlashOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMSC_FLASH_OFF` reader - enable interrupt (1) or mask out (0)"]
pub type ImscFlashOffR = crate::BitReader;
#[doc = "Field `IMSC_FLASH_OFF` writer - enable interrupt (1) or mask out (0)"]
pub type ImscFlashOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMSC_SHUTTER_ON` reader - enable interrupt (1) or mask out (0)"]
pub type ImscShutterOnR = crate::BitReader;
#[doc = "Field `IMSC_SHUTTER_ON` writer - enable interrupt (1) or mask out (0)"]
pub type ImscShutterOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMSC_SHUTTER_OFF` reader - enable interrupt (1) or mask out (0)"]
pub type ImscShutterOffR = crate::BitReader;
#[doc = "Field `IMSC_SHUTTER_OFF` writer - enable interrupt (1) or mask out (0)"]
pub type ImscShutterOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMSC_AFM_SUM_OF` reader - enable interrupt (1) or mask out (0)"]
pub type ImscAfmSumOfR = crate::BitReader;
#[doc = "Field `IMSC_AFM_SUM_OF` writer - enable interrupt (1) or mask out (0)"]
pub type ImscAfmSumOfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMSC_AFM_LUM_OF` reader - enable interrupt (1) or mask out (0)"]
pub type ImscAfmLumOfR = crate::BitReader;
#[doc = "Field `IMSC_AFM_LUM_OF` writer - enable interrupt (1) or mask out (0)"]
pub type ImscAfmLumOfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMSC_AFM_FIN` reader - enable interrupt (1) or mask out (0)"]
pub type ImscAfmFinR = crate::BitReader;
#[doc = "Field `IMSC_AFM_FIN` writer - enable interrupt (1) or mask out (0)"]
pub type ImscAfmFinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMSC_FLASH_CAP` reader - enable interrupt (1) or mask out (0)"]
pub type ImscFlashCapR = crate::BitReader;
#[doc = "Field `IMSC_FLASH_CAP` writer - enable interrupt (1) or mask out (0)"]
pub type ImscFlashCapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMSC_EXP_END` reader - enable interrupt (1) or mask out (0)"]
pub type ImscExpEndR = crate::BitReader;
#[doc = "Field `IMSC_EXP_END` writer - enable interrupt (1) or mask out (0)"]
pub type ImscExpEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMSC_VSM_END` reader - enable interrupt (1) or mask out (0)"]
pub type ImscVsmEndR = crate::BitReader;
#[doc = "Field `IMSC_VSM_END` writer - enable interrupt (1) or mask out (0)"]
pub type ImscVsmEndW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_isp_off(&self) -> ImscIspOffR {
        ImscIspOffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_frame(&self) -> ImscFrameR {
        ImscFrameR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_data_loss(&self) -> ImscDataLossR {
        ImscDataLossR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_pic_size_err(&self) -> ImscPicSizeErrR {
        ImscPicSizeErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_awb_done(&self) -> ImscAwbDoneR {
        ImscAwbDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_frame_in(&self) -> ImscFrameInR {
        ImscFrameInR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_v_start(&self) -> ImscVStartR {
        ImscVStartR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_h_start(&self) -> ImscHStartR {
        ImscHStartR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_flash_on(&self) -> ImscFlashOnR {
        ImscFlashOnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_flash_off(&self) -> ImscFlashOffR {
        ImscFlashOffR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_shutter_on(&self) -> ImscShutterOnR {
        ImscShutterOnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_shutter_off(&self) -> ImscShutterOffR {
        ImscShutterOffR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_afm_sum_of(&self) -> ImscAfmSumOfR {
        ImscAfmSumOfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_afm_lum_of(&self) -> ImscAfmLumOfR {
        ImscAfmLumOfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_afm_fin(&self) -> ImscAfmFinR {
        ImscAfmFinR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_flash_cap(&self) -> ImscFlashCapR {
        ImscFlashCapR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_exp_end(&self) -> ImscExpEndR {
        ImscExpEndR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    pub fn imsc_vsm_end(&self) -> ImscVsmEndR {
        ImscVsmEndR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_isp_off(&mut self) -> ImscIspOffW<ImscSpec> {
        ImscIspOffW::new(self, 0)
    }
    #[doc = "Bit 1 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_frame(&mut self) -> ImscFrameW<ImscSpec> {
        ImscFrameW::new(self, 1)
    }
    #[doc = "Bit 2 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_data_loss(&mut self) -> ImscDataLossW<ImscSpec> {
        ImscDataLossW::new(self, 2)
    }
    #[doc = "Bit 3 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_pic_size_err(&mut self) -> ImscPicSizeErrW<ImscSpec> {
        ImscPicSizeErrW::new(self, 3)
    }
    #[doc = "Bit 4 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_awb_done(&mut self) -> ImscAwbDoneW<ImscSpec> {
        ImscAwbDoneW::new(self, 4)
    }
    #[doc = "Bit 5 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_frame_in(&mut self) -> ImscFrameInW<ImscSpec> {
        ImscFrameInW::new(self, 5)
    }
    #[doc = "Bit 6 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_v_start(&mut self) -> ImscVStartW<ImscSpec> {
        ImscVStartW::new(self, 6)
    }
    #[doc = "Bit 7 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_h_start(&mut self) -> ImscHStartW<ImscSpec> {
        ImscHStartW::new(self, 7)
    }
    #[doc = "Bit 8 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_flash_on(&mut self) -> ImscFlashOnW<ImscSpec> {
        ImscFlashOnW::new(self, 8)
    }
    #[doc = "Bit 9 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_flash_off(&mut self) -> ImscFlashOffW<ImscSpec> {
        ImscFlashOffW::new(self, 9)
    }
    #[doc = "Bit 10 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_shutter_on(&mut self) -> ImscShutterOnW<ImscSpec> {
        ImscShutterOnW::new(self, 10)
    }
    #[doc = "Bit 11 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_shutter_off(&mut self) -> ImscShutterOffW<ImscSpec> {
        ImscShutterOffW::new(self, 11)
    }
    #[doc = "Bit 12 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_afm_sum_of(&mut self) -> ImscAfmSumOfW<ImscSpec> {
        ImscAfmSumOfW::new(self, 12)
    }
    #[doc = "Bit 13 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_afm_lum_of(&mut self) -> ImscAfmLumOfW<ImscSpec> {
        ImscAfmLumOfW::new(self, 13)
    }
    #[doc = "Bit 14 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_afm_fin(&mut self) -> ImscAfmFinW<ImscSpec> {
        ImscAfmFinW::new(self, 14)
    }
    #[doc = "Bit 17 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_flash_cap(&mut self) -> ImscFlashCapW<ImscSpec> {
        ImscFlashCapW::new(self, 17)
    }
    #[doc = "Bit 18 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_exp_end(&mut self) -> ImscExpEndW<ImscSpec> {
        ImscExpEndW::new(self, 18)
    }
    #[doc = "Bit 19 - enable interrupt (1) or mask out (0)"]
    #[inline(always)]
    #[must_use]
    pub fn imsc_vsm_end(&mut self) -> ImscVsmEndW<ImscSpec> {
        ImscVsmEndW::new(self, 19)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imsc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imsc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImscSpec;
impl crate::RegisterSpec for ImscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imsc::R`](R) reader structure"]
impl crate::Readable for ImscSpec {}
#[doc = "`write(|w| ..)` method takes [`imsc::W`](W) writer structure"]
impl crate::Writable for ImscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMSC to value 0"]
impl crate::Resettable for ImscSpec {
    const RESET_VALUE: u32 = 0;
}
