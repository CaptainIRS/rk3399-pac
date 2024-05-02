#[doc = "Register `EXP_CTRL` reader"]
pub type R = crate::R<ExpCtrlSpec>;
#[doc = "Register `EXP_CTRL` writer"]
pub type W = crate::W<ExpCtrlSpec>;
#[doc = "Field `exp_start` reader - '1' start measuring a frame. The exp block will reset\n\nthis bit and halt after completing one frame, if bit\n\n'autostop' is set to '1'.\n\n"]
pub type ExpStartR = crate::BitReader;
#[doc = "Field `exp_start` writer - '1' start measuring a frame. The exp block will reset\n\nthis bit and halt after completing one frame, if bit\n\n'autostop' is set to '1'.\n\n"]
pub type ExpStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `autostop` reader - '1' stop measuring after a complete frame '0'\n\ncontinous measurement"]
pub type AutostopR = crate::BitReader;
#[doc = "Field `autostop` writer - '1' stop measuring after a complete frame '0'\n\ncontinous measurement"]
pub type AutostopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `exp_meas_mode` reader - '1' luminance calculation according to Y=(R+G+B) x\n\n0.332 (85/256)\n\n'0' luminance calculation according to\n\nY=16+0.25R+0.5G+0.1094B"]
pub type ExpMeasModeR = crate::BitReader;
#[doc = "Field `exp_meas_mode` writer - '1' luminance calculation according to Y=(R+G+B) x\n\n0.332 (85/256)\n\n'0' luminance calculation according to\n\nY=16+0.25R+0.5G+0.1094B"]
pub type ExpMeasModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - '1' start measuring a frame. The exp block will reset\n\nthis bit and halt after completing one frame, if bit\n\n'autostop' is set to '1'.\n\n"]
    #[inline(always)]
    pub fn exp_start(&self) -> ExpStartR {
        ExpStartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - '1' stop measuring after a complete frame '0'\n\ncontinous measurement"]
    #[inline(always)]
    pub fn autostop(&self) -> AutostopR {
        AutostopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 31 - '1' luminance calculation according to Y=(R+G+B) x\n\n0.332 (85/256)\n\n'0' luminance calculation according to\n\nY=16+0.25R+0.5G+0.1094B"]
    #[inline(always)]
    pub fn exp_meas_mode(&self) -> ExpMeasModeR {
        ExpMeasModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - '1' start measuring a frame. The exp block will reset\n\nthis bit and halt after completing one frame, if bit\n\n'autostop' is set to '1'.\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn exp_start(&mut self) -> ExpStartW<ExpCtrlSpec> {
        ExpStartW::new(self, 0)
    }
    #[doc = "Bit 1 - '1' stop measuring after a complete frame '0'\n\ncontinous measurement"]
    #[inline(always)]
    #[must_use]
    pub fn autostop(&mut self) -> AutostopW<ExpCtrlSpec> {
        AutostopW::new(self, 1)
    }
    #[doc = "Bit 31 - '1' luminance calculation according to Y=(R+G+B) x\n\n0.332 (85/256)\n\n'0' luminance calculation according to\n\nY=16+0.25R+0.5G+0.1094B"]
    #[inline(always)]
    #[must_use]
    pub fn exp_meas_mode(&mut self) -> ExpMeasModeW<ExpCtrlSpec> {
        ExpMeasModeW::new(self, 31)
    }
}
#[doc = "Exposure control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exp_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exp_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExpCtrlSpec;
impl crate::RegisterSpec for ExpCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exp_ctrl::R`](R) reader structure"]
impl crate::Readable for ExpCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`exp_ctrl::W`](W) writer structure"]
impl crate::Writable for ExpCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXP_CTRL to value 0"]
impl crate::Resettable for ExpCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
