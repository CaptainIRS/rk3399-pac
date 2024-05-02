#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ISP_ENABLE` reader - 1: ISP data output enabled 0: ISP data output\n\ndisabled\n\nControls output formatter frame synchronously, if isp_gen_cfg_upd is used to activate this bit. For\n\nimmediate update isp_cfg_upd must be used."]
pub type IspEnableR = crate::BitReader;
#[doc = "Field `ISP_ENABLE` writer - 1: ISP data output enabled 0: ISP data output\n\ndisabled\n\nControls output formatter frame synchronously, if isp_gen_cfg_upd is used to activate this bit. For\n\nimmediate update isp_cfg_upd must be used."]
pub type IspEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISP_MODE` reader - 000 - RAW picture with BT.601 sync (ISP bypass) 001\n\n- ITU-R BT.656 (YUV with embedded sync) 010 - ITU-R BT.601 (YUV input with H and Vsync\n\nsignals) 011 - Bayer RGB processing with H and Vsync\n\nsignals\n\n- data mode (ISP bypass, sync signals interpreted as\n\ndata enable)\n\n- Bayer RGB processing with BT.656 synchronization 110 - RAW picture with ITU-R BT.656 synchronization\n\n(ISP bypass) 111 - reserved\n\nSide effect:\n\nIf RAW, BT.601, BT.656, or data mode is selected,\n\nthe clock of the ISP SRAMs (ISP line buffer, Lens Shading,\n\nBad Pixel) is switched off. Only in Bayer RGB mode the\n\nclock to the SRAMs is enabled. This further reduces power\n\nconsumption."]
pub type IspModeR = crate::FieldReader;
#[doc = "Field `ISP_MODE` writer - 000 - RAW picture with BT.601 sync (ISP bypass) 001\n\n- ITU-R BT.656 (YUV with embedded sync) 010 - ITU-R BT.601 (YUV input with H and Vsync\n\nsignals) 011 - Bayer RGB processing with H and Vsync\n\nsignals\n\n- data mode (ISP bypass, sync signals interpreted as\n\ndata enable)\n\n- Bayer RGB processing with BT.656 synchronization 110 - RAW picture with ITU-R BT.656 synchronization\n\n(ISP bypass) 111 - reserved\n\nSide effect:\n\nIf RAW, BT.601, BT.656, or data mode is selected,\n\nthe clock of the ISP SRAMs (ISP line buffer, Lens Shading,\n\nBad Pixel) is switched off. Only in Bayer RGB mode the\n\nclock to the SRAMs is enabled. This further reduces power\n\nconsumption."]
pub type IspModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ISP_INFORM_ENABLE` reader - 1: input formatter enabled 0: input formatter disabled\n\nThe ISP input formatter is enabled or disabled by this bit\n\nimmediately, but always starts or stops acquisition frame\n\nsynchronously.\n\n"]
pub type IspInformEnableR = crate::BitReader;
#[doc = "Field `ISP_INFORM_ENABLE` writer - 1: input formatter enabled 0: input formatter disabled\n\nThe ISP input formatter is enabled or disabled by this bit\n\nimmediately, but always starts or stops acquisition frame\n\nsynchronously.\n\n"]
pub type IspInformEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISP_GAMMA_IN_ENABLE` reader - Sensor De-gamma ON/OFF"]
pub type IspGammaInEnableR = crate::BitReader;
#[doc = "Field `ISP_GAMMA_IN_ENABLE` writer - Sensor De-gamma ON/OFF"]
pub type IspGammaInEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISP_AWB_ENABLE` reader - auto white balance ON/OFF"]
pub type IspAwbEnableR = crate::BitReader;
#[doc = "Field `ISP_AWB_ENABLE` writer - auto white balance ON/OFF"]
pub type IspAwbEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISP_CFG_UPD` reader - 1: immediately configure (update) shadow registers,\n\nwrite only"]
pub type IspCfgUpdR = crate::BitReader;
#[doc = "Field `ISP_CFG_UPD` writer - 1: immediately configure (update) shadow registers,\n\nwrite only"]
pub type IspCfgUpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISP_GEN_CFG_UPD` reader - 1: generate frame synchronous configuration signal at\n\nthe output of ISP for shadow registers of the following\n\nprocessing modules, write only"]
pub type IspGenCfgUpdR = crate::BitReader;
#[doc = "Field `ISP_GEN_CFG_UPD` writer - 1: generate frame synchronous configuration signal at\n\nthe output of ISP for shadow registers of the following\n\nprocessing modules, write only"]
pub type IspGenCfgUpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IspFlashMode {
    #[doc = "0: sensor interface works independently from flash control unit"]
    B0 = 0,
    #[doc = "1: one frame is captured when signaled by flash control unit"]
    B1 = 1,
}
impl From<IspFlashMode> for bool {
    #[inline(always)]
    fn from(variant: IspFlashMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISP_FLASH_MODE` reader - "]
pub type IspFlashModeR = crate::BitReader<IspFlashMode>;
impl IspFlashModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IspFlashMode {
        match self.bits {
            false => IspFlashMode::B0,
            true => IspFlashMode::B1,
        }
    }
    #[doc = "sensor interface works independently from flash control unit"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IspFlashMode::B0
    }
    #[doc = "one frame is captured when signaled by flash control unit"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IspFlashMode::B1
    }
}
#[doc = "Field `ISP_FLASH_MODE` writer - "]
pub type IspFlashModeW<'a, REG> = crate::BitWriter<'a, REG, IspFlashMode>;
impl<'a, REG> IspFlashModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "sensor interface works independently from flash control unit"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IspFlashMode::B0)
    }
    #[doc = "one frame is captured when signaled by flash control unit"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IspFlashMode::B1)
    }
}
#[doc = "Color Space Matrix luminance clipping range for ISP\n\noutput\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IspCsmYRange {
    #[doc = "0: Y range 64..940 (16..235) according to ITU-R BT.601 standard"]
    B0 = 0,
    #[doc = "1: full Y range 0..1023 ( 0..255) Numbers in brackets are for 8 bit resolution. This bit also configures the YCbCr sequence align block accordingly."]
    B1 = 1,
}
impl From<IspCsmYRange> for bool {
    #[inline(always)]
    fn from(variant: IspCsmYRange) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISP_CSM_Y_RANGE` reader - Color Space Matrix luminance clipping range for ISP\n\noutput"]
pub type IspCsmYRangeR = crate::BitReader<IspCsmYRange>;
impl IspCsmYRangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IspCsmYRange {
        match self.bits {
            false => IspCsmYRange::B0,
            true => IspCsmYRange::B1,
        }
    }
    #[doc = "Y range 64..940 (16..235) according to ITU-R BT.601 standard"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IspCsmYRange::B0
    }
    #[doc = "full Y range 0..1023 ( 0..255) Numbers in brackets are for 8 bit resolution. This bit also configures the YCbCr sequence align block accordingly."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IspCsmYRange::B1
    }
}
#[doc = "Field `ISP_CSM_Y_RANGE` writer - Color Space Matrix luminance clipping range for ISP\n\noutput"]
pub type IspCsmYRangeW<'a, REG> = crate::BitWriter<'a, REG, IspCsmYRange>;
impl<'a, REG> IspCsmYRangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Y range 64..940 (16..235) according to ITU-R BT.601 standard"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IspCsmYRange::B0)
    }
    #[doc = "full Y range 0..1023 ( 0..255) Numbers in brackets are for 8 bit resolution. This bit also configures the YCbCr sequence align block accordingly."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IspCsmYRange::B1)
    }
}
#[doc = "Color Space Matrix chrominance clipping range for ISP\n\noutput\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IspCsmCRange {
    #[doc = "0: CbCr range 64..960 (16..240) according to ITU-R BT.601 standard"]
    B0 = 0,
    #[doc = "1: full UV range 0..1023 ( 0..255) Numbers in brackets are for 8 bit resolution. This bit also configures the YCbCr sequence align block accordingly."]
    B1 = 1,
}
impl From<IspCsmCRange> for bool {
    #[inline(always)]
    fn from(variant: IspCsmCRange) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISP_CSM_C_RANGE` reader - Color Space Matrix chrominance clipping range for ISP\n\noutput"]
pub type IspCsmCRangeR = crate::BitReader<IspCsmCRange>;
impl IspCsmCRangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IspCsmCRange {
        match self.bits {
            false => IspCsmCRange::B0,
            true => IspCsmCRange::B1,
        }
    }
    #[doc = "CbCr range 64..960 (16..240) according to ITU-R BT.601 standard"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IspCsmCRange::B0
    }
    #[doc = "full UV range 0..1023 ( 0..255) Numbers in brackets are for 8 bit resolution. This bit also configures the YCbCr sequence align block accordingly."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IspCsmCRange::B1
    }
}
#[doc = "Field `ISP_CSM_C_RANGE` writer - Color Space Matrix chrominance clipping range for ISP\n\noutput"]
pub type IspCsmCRangeW<'a, REG> = crate::BitWriter<'a, REG, IspCsmCRange>;
impl<'a, REG> IspCsmCRangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CbCr range 64..960 (16..240) according to ITU-R BT.601 standard"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IspCsmCRange::B0)
    }
    #[doc = "full UV range 0..1023 ( 0..255) Numbers in brackets are for 8 bit resolution. This bit also configures the YCbCr sequence align block accordingly."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IspCsmCRange::B1)
    }
}
impl R {
    #[doc = "Bit 0 - 1: ISP data output enabled 0: ISP data output\n\ndisabled\n\nControls output formatter frame synchronously, if isp_gen_cfg_upd is used to activate this bit. For\n\nimmediate update isp_cfg_upd must be used."]
    #[inline(always)]
    pub fn isp_enable(&self) -> IspEnableR {
        IspEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 000 - RAW picture with BT.601 sync (ISP bypass) 001\n\n- ITU-R BT.656 (YUV with embedded sync) 010 - ITU-R BT.601 (YUV input with H and Vsync\n\nsignals) 011 - Bayer RGB processing with H and Vsync\n\nsignals\n\n- data mode (ISP bypass, sync signals interpreted as\n\ndata enable)\n\n- Bayer RGB processing with BT.656 synchronization 110 - RAW picture with ITU-R BT.656 synchronization\n\n(ISP bypass) 111 - reserved\n\nSide effect:\n\nIf RAW, BT.601, BT.656, or data mode is selected,\n\nthe clock of the ISP SRAMs (ISP line buffer, Lens Shading,\n\nBad Pixel) is switched off. Only in Bayer RGB mode the\n\nclock to the SRAMs is enabled. This further reduces power\n\nconsumption."]
    #[inline(always)]
    pub fn isp_mode(&self) -> IspModeR {
        IspModeR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - 1: input formatter enabled 0: input formatter disabled\n\nThe ISP input formatter is enabled or disabled by this bit\n\nimmediately, but always starts or stops acquisition frame\n\nsynchronously.\n\n"]
    #[inline(always)]
    pub fn isp_inform_enable(&self) -> IspInformEnableR {
        IspInformEnableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Sensor De-gamma ON/OFF"]
    #[inline(always)]
    pub fn isp_gamma_in_enable(&self) -> IspGammaInEnableR {
        IspGammaInEnableR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - auto white balance ON/OFF"]
    #[inline(always)]
    pub fn isp_awb_enable(&self) -> IspAwbEnableR {
        IspAwbEnableR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: immediately configure (update) shadow registers,\n\nwrite only"]
    #[inline(always)]
    pub fn isp_cfg_upd(&self) -> IspCfgUpdR {
        IspCfgUpdR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: generate frame synchronous configuration signal at\n\nthe output of ISP for shadow registers of the following\n\nprocessing modules, write only"]
    #[inline(always)]
    pub fn isp_gen_cfg_upd(&self) -> IspGenCfgUpdR {
        IspGenCfgUpdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn isp_flash_mode(&self) -> IspFlashModeR {
        IspFlashModeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Color Space Matrix luminance clipping range for ISP\n\noutput"]
    #[inline(always)]
    pub fn isp_csm_y_range(&self) -> IspCsmYRangeR {
        IspCsmYRangeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Color Space Matrix chrominance clipping range for ISP\n\noutput"]
    #[inline(always)]
    pub fn isp_csm_c_range(&self) -> IspCsmCRangeR {
        IspCsmCRangeR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1: ISP data output enabled 0: ISP data output\n\ndisabled\n\nControls output formatter frame synchronously, if isp_gen_cfg_upd is used to activate this bit. For\n\nimmediate update isp_cfg_upd must be used."]
    #[inline(always)]
    #[must_use]
    pub fn isp_enable(&mut self) -> IspEnableW<CtrlSpec> {
        IspEnableW::new(self, 0)
    }
    #[doc = "Bits 1:3 - 000 - RAW picture with BT.601 sync (ISP bypass) 001\n\n- ITU-R BT.656 (YUV with embedded sync) 010 - ITU-R BT.601 (YUV input with H and Vsync\n\nsignals) 011 - Bayer RGB processing with H and Vsync\n\nsignals\n\n- data mode (ISP bypass, sync signals interpreted as\n\ndata enable)\n\n- Bayer RGB processing with BT.656 synchronization 110 - RAW picture with ITU-R BT.656 synchronization\n\n(ISP bypass) 111 - reserved\n\nSide effect:\n\nIf RAW, BT.601, BT.656, or data mode is selected,\n\nthe clock of the ISP SRAMs (ISP line buffer, Lens Shading,\n\nBad Pixel) is switched off. Only in Bayer RGB mode the\n\nclock to the SRAMs is enabled. This further reduces power\n\nconsumption."]
    #[inline(always)]
    #[must_use]
    pub fn isp_mode(&mut self) -> IspModeW<CtrlSpec> {
        IspModeW::new(self, 1)
    }
    #[doc = "Bit 4 - 1: input formatter enabled 0: input formatter disabled\n\nThe ISP input formatter is enabled or disabled by this bit\n\nimmediately, but always starts or stops acquisition frame\n\nsynchronously.\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn isp_inform_enable(&mut self) -> IspInformEnableW<CtrlSpec> {
        IspInformEnableW::new(self, 4)
    }
    #[doc = "Bit 6 - Sensor De-gamma ON/OFF"]
    #[inline(always)]
    #[must_use]
    pub fn isp_gamma_in_enable(&mut self) -> IspGammaInEnableW<CtrlSpec> {
        IspGammaInEnableW::new(self, 6)
    }
    #[doc = "Bit 7 - auto white balance ON/OFF"]
    #[inline(always)]
    #[must_use]
    pub fn isp_awb_enable(&mut self) -> IspAwbEnableW<CtrlSpec> {
        IspAwbEnableW::new(self, 7)
    }
    #[doc = "Bit 9 - 1: immediately configure (update) shadow registers,\n\nwrite only"]
    #[inline(always)]
    #[must_use]
    pub fn isp_cfg_upd(&mut self) -> IspCfgUpdW<CtrlSpec> {
        IspCfgUpdW::new(self, 9)
    }
    #[doc = "Bit 10 - 1: generate frame synchronous configuration signal at\n\nthe output of ISP for shadow registers of the following\n\nprocessing modules, write only"]
    #[inline(always)]
    #[must_use]
    pub fn isp_gen_cfg_upd(&mut self) -> IspGenCfgUpdW<CtrlSpec> {
        IspGenCfgUpdW::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn isp_flash_mode(&mut self) -> IspFlashModeW<CtrlSpec> {
        IspFlashModeW::new(self, 12)
    }
    #[doc = "Bit 13 - Color Space Matrix luminance clipping range for ISP\n\noutput"]
    #[inline(always)]
    #[must_use]
    pub fn isp_csm_y_range(&mut self) -> IspCsmYRangeW<CtrlSpec> {
        IspCsmYRangeW::new(self, 13)
    }
    #[doc = "Bit 14 - Color Space Matrix chrominance clipping range for ISP\n\noutput"]
    #[inline(always)]
    #[must_use]
    pub fn isp_csm_c_range(&mut self) -> IspCsmCRangeW<CtrlSpec> {
        IspCsmCRangeW::new(self, 14)
    }
}
#[doc = "global control register\n\nNote: partly write-only \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
