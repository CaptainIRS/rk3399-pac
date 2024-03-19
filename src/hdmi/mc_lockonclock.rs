#[doc = "Register `MC_LOCKONCLOCK` reader"]
pub type R = crate::R<McLockonclockSpec>;
#[doc = "Register `MC_LOCKONCLOCK` writer"]
pub type W = crate::W<McLockonclockSpec>;
#[doc = "Field `CECCLK` reader - CEC clock status. Indicates that the clock is present in\n\nthe system. Cleared by WR 1 to this position."]
pub type CecclkR = crate::BitReader;
#[doc = "Field `CECCLK` writer - CEC clock status. Indicates that the clock is present in\n\nthe system. Cleared by WR 1 to this position."]
pub type CecclkW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AUDIOSPDIFCLK` reader - SPDIF clock status. Indicates that the clock is present in\n\nthe system. Cleared by WR 1 to this position."]
pub type AudiospdifclkR = crate::BitReader;
#[doc = "Field `AUDIOSPDIFCLK` writer - SPDIF clock status. Indicates that the clock is present in\n\nthe system. Cleared by WR 1 to this position."]
pub type AudiospdifclkW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `I2SCLK` reader - I2S clock status. Indicates that the clock is present in the\n\nsystem. Cleared by WR 1 to this position."]
pub type I2sclkR = crate::BitReader;
#[doc = "Field `I2SCLK` writer - I2S clock status. Indicates that the clock is present in the\n\nsystem. Cleared by WR 1 to this position."]
pub type I2sclkW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PREPCLK` reader - Pixel Repetition clock status. Indicates that the clock is\n\npresent in the system. Cleared by WR 1 to this position."]
pub type PrepclkR = crate::BitReader;
#[doc = "Field `PREPCLK` writer - Pixel Repetition clock status. Indicates that the clock is\n\npresent in the system. Cleared by WR 1 to this position."]
pub type PrepclkW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TCLK` reader - TMDS clock status. Indicates that the clock is present in\n\nthe system. Cleared by WR 1 to this position."]
pub type TclkR = crate::BitReader;
#[doc = "Field `TCLK` writer - TMDS clock status. Indicates that the clock is present in\n\nthe system. Cleared by WR 1 to this position."]
pub type TclkW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PCLK` reader - Pixel clock status. Indicates that the clock is present in\n\nthe system. Cleared by WR 1 to this position."]
pub type PclkR = crate::BitReader;
#[doc = "Field `PCLK` writer - Pixel clock status. Indicates that the clock is present in\n\nthe system. Cleared by WR 1 to this position."]
pub type PclkW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IGPACLK` reader - GPAUD interface clock status. This bit is enabled when\n\nthe Generic Parallel Audio (GPAUD) interface is enabled\n\n(AUDIO_IF = 6). Otherwise, this bit returns zero.\n\nThis bit indicates the clock is present in the system. It is\n\ncleared by writing 1 to this bit."]
pub type IgpaclkR = crate::BitReader;
#[doc = "Field `IGPACLK` writer - GPAUD interface clock status. This bit is enabled when\n\nthe Generic Parallel Audio (GPAUD) interface is enabled\n\n(AUDIO_IF = 6). Otherwise, this bit returns zero.\n\nThis bit indicates the clock is present in the system. It is\n\ncleared by writing 1 to this bit."]
pub type IgpaclkW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - CEC clock status. Indicates that the clock is present in\n\nthe system. Cleared by WR 1 to this position."]
    #[inline(always)]
    pub fn cecclk(&self) -> CecclkR {
        CecclkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SPDIF clock status. Indicates that the clock is present in\n\nthe system. Cleared by WR 1 to this position."]
    #[inline(always)]
    pub fn audiospdifclk(&self) -> AudiospdifclkR {
        AudiospdifclkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2S clock status. Indicates that the clock is present in the\n\nsystem. Cleared by WR 1 to this position."]
    #[inline(always)]
    pub fn i2sclk(&self) -> I2sclkR {
        I2sclkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pixel Repetition clock status. Indicates that the clock is\n\npresent in the system. Cleared by WR 1 to this position."]
    #[inline(always)]
    pub fn prepclk(&self) -> PrepclkR {
        PrepclkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TMDS clock status. Indicates that the clock is present in\n\nthe system. Cleared by WR 1 to this position."]
    #[inline(always)]
    pub fn tclk(&self) -> TclkR {
        TclkR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pixel clock status. Indicates that the clock is present in\n\nthe system. Cleared by WR 1 to this position."]
    #[inline(always)]
    pub fn pclk(&self) -> PclkR {
        PclkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPAUD interface clock status. This bit is enabled when\n\nthe Generic Parallel Audio (GPAUD) interface is enabled\n\n(AUDIO_IF = 6). Otherwise, this bit returns zero.\n\nThis bit indicates the clock is present in the system. It is\n\ncleared by writing 1 to this bit."]
    #[inline(always)]
    pub fn igpaclk(&self) -> IgpaclkR {
        IgpaclkR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CEC clock status. Indicates that the clock is present in\n\nthe system. Cleared by WR 1 to this position."]
    #[inline(always)]
    #[must_use]
    pub fn cecclk(&mut self) -> CecclkW<McLockonclockSpec> {
        CecclkW::new(self, 0)
    }
    #[doc = "Bit 2 - SPDIF clock status. Indicates that the clock is present in\n\nthe system. Cleared by WR 1 to this position."]
    #[inline(always)]
    #[must_use]
    pub fn audiospdifclk(&mut self) -> AudiospdifclkW<McLockonclockSpec> {
        AudiospdifclkW::new(self, 2)
    }
    #[doc = "Bit 3 - I2S clock status. Indicates that the clock is present in the\n\nsystem. Cleared by WR 1 to this position."]
    #[inline(always)]
    #[must_use]
    pub fn i2sclk(&mut self) -> I2sclkW<McLockonclockSpec> {
        I2sclkW::new(self, 3)
    }
    #[doc = "Bit 4 - Pixel Repetition clock status. Indicates that the clock is\n\npresent in the system. Cleared by WR 1 to this position."]
    #[inline(always)]
    #[must_use]
    pub fn prepclk(&mut self) -> PrepclkW<McLockonclockSpec> {
        PrepclkW::new(self, 4)
    }
    #[doc = "Bit 5 - TMDS clock status. Indicates that the clock is present in\n\nthe system. Cleared by WR 1 to this position."]
    #[inline(always)]
    #[must_use]
    pub fn tclk(&mut self) -> TclkW<McLockonclockSpec> {
        TclkW::new(self, 5)
    }
    #[doc = "Bit 6 - Pixel clock status. Indicates that the clock is present in\n\nthe system. Cleared by WR 1 to this position."]
    #[inline(always)]
    #[must_use]
    pub fn pclk(&mut self) -> PclkW<McLockonclockSpec> {
        PclkW::new(self, 6)
    }
    #[doc = "Bit 7 - GPAUD interface clock status. This bit is enabled when\n\nthe Generic Parallel Audio (GPAUD) interface is enabled\n\n(AUDIO_IF = 6). Otherwise, this bit returns zero.\n\nThis bit indicates the clock is present in the system. It is\n\ncleared by writing 1 to this bit."]
    #[inline(always)]
    #[must_use]
    pub fn igpaclk(&mut self) -> IgpaclkW<McLockonclockSpec> {
        IgpaclkW::new(self, 7)
    }
}
#[doc = "Main Controller Clock Present Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mc_lockonclock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mc_lockonclock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McLockonclockSpec;
impl crate::RegisterSpec for McLockonclockSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mc_lockonclock::R`](R) reader structure"]
impl crate::Readable for McLockonclockSpec {}
#[doc = "`write(|w| ..)` method takes [`mc_lockonclock::W`](W) writer structure"]
impl crate::Writable for McLockonclockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0xfd;
}
#[doc = "`reset()` method sets MC_LOCKONCLOCK to value 0"]
impl crate::Resettable for McLockonclockSpec {
    const RESET_VALUE: u8 = 0;
}
