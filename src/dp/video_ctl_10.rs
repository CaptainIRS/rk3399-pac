#[doc = "Register `VIDEO_CTL_10` reader"]
pub type R = crate::R<VideoCtl10Spec>;
#[doc = "Register `VIDEO_CTL_10` writer"]
pub type W = crate::W<VideoCtl10Spec>;
#[doc = "Slave mode HSYNC polarity configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlaveHsyncPCfg {
    #[doc = "1: High is active. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    B1 = 1,
    #[doc = "0: High is active. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    B0 = 0,
}
impl From<SlaveHsyncPCfg> for bool {
    #[inline(always)]
    fn from(variant: SlaveHsyncPCfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLAVE_HSYNC_P_CFG` reader - Slave mode HSYNC polarity configuration."]
pub type SlaveHsyncPCfgR = crate::BitReader<SlaveHsyncPCfg>;
impl SlaveHsyncPCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SlaveHsyncPCfg {
        match self.bits {
            true => SlaveHsyncPCfg::B1,
            false => SlaveHsyncPCfg::B0,
        }
    }
    #[doc = "High is active. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SlaveHsyncPCfg::B1
    }
    #[doc = "High is active. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SlaveHsyncPCfg::B0
    }
}
#[doc = "Field `SLAVE_HSYNC_P_CFG` writer - Slave mode HSYNC polarity configuration."]
pub type SlaveHsyncPCfgW<'a, REG> = crate::BitWriter1C<'a, REG, SlaveHsyncPCfg>;
impl<'a, REG> SlaveHsyncPCfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High is active. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SlaveHsyncPCfg::B1)
    }
    #[doc = "High is active. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SlaveHsyncPCfg::B0)
    }
}
#[doc = "Slave mode VSYNC polarity configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlaveVsyncPCfg {
    #[doc = "1: High is active. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    B1 = 1,
    #[doc = "0: High is active. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    B0 = 0,
}
impl From<SlaveVsyncPCfg> for bool {
    #[inline(always)]
    fn from(variant: SlaveVsyncPCfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLAVE_VSYNC_P_CFG` reader - Slave mode VSYNC polarity configuration."]
pub type SlaveVsyncPCfgR = crate::BitReader<SlaveVsyncPCfg>;
impl SlaveVsyncPCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SlaveVsyncPCfg {
        match self.bits {
            true => SlaveVsyncPCfg::B1,
            false => SlaveVsyncPCfg::B0,
        }
    }
    #[doc = "High is active. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SlaveVsyncPCfg::B1
    }
    #[doc = "High is active. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SlaveVsyncPCfg::B0
    }
}
#[doc = "Field `SLAVE_VSYNC_P_CFG` writer - Slave mode VSYNC polarity configuration."]
pub type SlaveVsyncPCfgW<'a, REG> = crate::BitWriter1C<'a, REG, SlaveVsyncPCfg>;
impl<'a, REG> SlaveVsyncPCfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High is active. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SlaveVsyncPCfg::B1)
    }
    #[doc = "High is active. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SlaveVsyncPCfg::B0)
    }
}
#[doc = "Field `SLAVE_I_SCAN_CFG` reader - Interlace scan mode configuration. 0: Progressive, 1: Interlace. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
pub type SlaveIScanCfgR = crate::BitReader;
#[doc = "Field `SLAVE_I_SCAN_CFG` writer - Interlace scan mode configuration. 0: Progressive, 1: Interlace. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
pub type SlaveIScanCfgW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Video format select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSel {
    #[doc = "1: Video format information from video_capture module. According to the configuration of this bit field, the values of video format status registers in Base + 0x008C~ 0x00D0 are determined, which are transferred as main stream attribute packet. Note that if BIST_EN is set to 1, F_SEL must be cleared to 0 although video format information comes from registers set by user."]
    B1 = 1,
    #[doc = "0: Video format information from video_capture module. According to the configuration of this bit field, the values of video format status registers in Base + 0x008C~ 0x00D0 are determined, which are transferred as main stream attribute packet. Note that if BIST_EN is set to 1, F_SEL must be cleared to 0 although video format information comes from registers set by user."]
    B0 = 0,
}
impl From<FSel> for bool {
    #[inline(always)]
    fn from(variant: FSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `F_SEL` reader - Video format select."]
pub type FSelR = crate::BitReader<FSel>;
impl FSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSel {
        match self.bits {
            true => FSel::B1,
            false => FSel::B0,
        }
    }
    #[doc = "Video format information from video_capture module. According to the configuration of this bit field, the values of video format status registers in Base + 0x008C~ 0x00D0 are determined, which are transferred as main stream attribute packet. Note that if BIST_EN is set to 1, F_SEL must be cleared to 0 although video format information comes from registers set by user."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FSel::B1
    }
    #[doc = "Video format information from video_capture module. According to the configuration of this bit field, the values of video format status registers in Base + 0x008C~ 0x00D0 are determined, which are transferred as main stream attribute packet. Note that if BIST_EN is set to 1, F_SEL must be cleared to 0 although video format information comes from registers set by user."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FSel::B0
    }
}
#[doc = "Field `F_SEL` writer - Video format select."]
pub type FSelW<'a, REG> = crate::BitWriter<'a, REG, FSel>;
impl<'a, REG> FSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Video format information from video_capture module. According to the configuration of this bit field, the values of video format status registers in Base + 0x008C~ 0x00D0 are determined, which are transferred as main stream attribute packet. Note that if BIST_EN is set to 1, F_SEL must be cleared to 0 although video format information comes from registers set by user."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(FSel::B1)
    }
    #[doc = "Video format information from video_capture module. According to the configuration of this bit field, the values of video format status registers in Base + 0x008C~ 0x00D0 are determined, which are transferred as main stream attribute packet. Note that if BIST_EN is set to 1, F_SEL must be cleared to 0 although video format information comes from registers set by user."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(FSel::B0)
    }
}
impl R {
    #[doc = "Bit 0 - Slave mode HSYNC polarity configuration."]
    #[inline(always)]
    pub fn slave_hsync_p_cfg(&self) -> SlaveHsyncPCfgR {
        SlaveHsyncPCfgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave mode VSYNC polarity configuration."]
    #[inline(always)]
    pub fn slave_vsync_p_cfg(&self) -> SlaveVsyncPCfgR {
        SlaveVsyncPCfgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interlace scan mode configuration. 0: Progressive, 1: Interlace. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    #[inline(always)]
    pub fn slave_i_scan_cfg(&self) -> SlaveIScanCfgR {
        SlaveIScanCfgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Video format select."]
    #[inline(always)]
    pub fn f_sel(&self) -> FSelR {
        FSelR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave mode HSYNC polarity configuration."]
    #[inline(always)]
    #[must_use]
    pub fn slave_hsync_p_cfg(&mut self) -> SlaveHsyncPCfgW<VideoCtl10Spec> {
        SlaveHsyncPCfgW::new(self, 0)
    }
    #[doc = "Bit 1 - Slave mode VSYNC polarity configuration."]
    #[inline(always)]
    #[must_use]
    pub fn slave_vsync_p_cfg(&mut self) -> SlaveVsyncPCfgW<VideoCtl10Spec> {
        SlaveVsyncPCfgW::new(self, 1)
    }
    #[doc = "Bit 2 - Interlace scan mode configuration. 0: Progressive, 1: Interlace. When F_SEL is 1, this value is sent in main stream attribute packet. When BIST_EN is 1, this bit field is used to specify the BIST video stream format."]
    #[inline(always)]
    #[must_use]
    pub fn slave_i_scan_cfg(&mut self) -> SlaveIScanCfgW<VideoCtl10Spec> {
        SlaveIScanCfgW::new(self, 2)
    }
    #[doc = "Bit 4 - Video format select."]
    #[inline(always)]
    #[must_use]
    pub fn f_sel(&mut self) -> FSelW<VideoCtl10Spec> {
        FSelW::new(self, 4)
    }
}
#[doc = "Video Control 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`video_ctl_10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`video_ctl_10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VideoCtl10Spec;
impl crate::RegisterSpec for VideoCtl10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`video_ctl_10::R`](R) reader structure"]
impl crate::Readable for VideoCtl10Spec {}
#[doc = "`write(|w| ..)` method takes [`video_ctl_10::W`](W) writer structure"]
impl crate::Writable for VideoCtl10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x07;
}
#[doc = "`reset()` method sets VIDEO_CTL_10 to value 0"]
impl crate::Resettable for VideoCtl10Spec {
    const RESET_VALUE: u32 = 0;
}
