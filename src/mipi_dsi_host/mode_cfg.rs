#[doc = "Register `MODE_CFG` reader"]
pub type R = crate::R<ModeCfgSpec>;
#[doc = "Register `MODE_CFG` writer"]
pub type W = crate::W<ModeCfgSpec>;
#[doc = "Field `CMD_VIDEO_MODE` reader - Field0000 Abstract\n\nThis bit configures the operation mode:\n\n■0: Video mode\n\n■1: Command mode"]
pub type CmdVideoModeR = crate::BitReader;
#[doc = "Field `CMD_VIDEO_MODE` writer - Field0000 Abstract\n\nThis bit configures the operation mode:\n\n■0: Video mode\n\n■1: Command mode"]
pub type CmdVideoModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Field0000 Abstract\n\nThis bit configures the operation mode:\n\n■0: Video mode\n\n■1: Command mode"]
    #[inline(always)]
    pub fn cmd_video_mode(&self) -> CmdVideoModeR {
        CmdVideoModeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Field0000 Abstract\n\nThis bit configures the operation mode:\n\n■0: Video mode\n\n■1: Command mode"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_video_mode(&mut self) -> CmdVideoModeW<ModeCfgSpec> {
        CmdVideoModeW::new(self, 0)
    }
}
#[doc = "Register0000 Abstract\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeCfgSpec;
impl crate::RegisterSpec for ModeCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode_cfg::R`](R) reader structure"]
impl crate::Readable for ModeCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`mode_cfg::W`](W) writer structure"]
impl crate::Writable for ModeCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODE_CFG to value 0x01"]
impl crate::Resettable for ModeCfgSpec {
    const RESET_VALUE: u32 = 0x01;
}
