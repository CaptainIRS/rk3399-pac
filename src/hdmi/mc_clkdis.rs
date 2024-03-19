#[doc = "Register `MC_CLKDIS` reader"]
pub type R = crate::R<McClkdisSpec>;
#[doc = "Register `MC_CLKDIS` writer"]
pub type W = crate::W<McClkdisSpec>;
#[doc = "Field `AUDCLK_DISABLE` reader - Audio Sampler clock synchronous disable signal."]
pub type AudclkDisableR = crate::BitReader;
#[doc = "Field `AUDCLK_DISABLE` writer - Audio Sampler clock synchronous disable signal."]
pub type AudclkDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSCCLK_DISABLE` reader - Color Space Converter clock synchronous disable\n\nsignal."]
pub type CscclkDisableR = crate::BitReader;
#[doc = "Field `CSCCLK_DISABLE` writer - Color Space Converter clock synchronous disable\n\nsignal."]
pub type CscclkDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CECCLK_DISABLE` reader - CEC Engine clock synchronous disable signal."]
pub type CecclkDisableR = crate::BitReader;
#[doc = "Field `CECCLK_DISABLE` writer - CEC Engine clock synchronous disable signal."]
pub type CecclkDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDCPCLK_DISABLE` reader - HDCP clock synchronous disable signal. When active\n\n(1b), simultaneously bypasses HDCP."]
pub type HdcpclkDisableR = crate::BitReader;
#[doc = "Field `HDCPCLK_DISABLE` writer - HDCP clock synchronous disable signal. When active\n\n(1b), simultaneously bypasses HDCP."]
pub type HdcpclkDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H22SCLK_DISABLE` reader - HDCP22 clock synchronous disable signal. When active\n\n(1b), simultaneously bypasses HDCP22."]
pub type H22sclkDisableR = crate::BitReader;
#[doc = "Field `H22SCLK_DISABLE` writer - HDCP22 clock synchronous disable signal. When active\n\n(1b), simultaneously bypasses HDCP22."]
pub type H22sclkDisableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Audio Sampler clock synchronous disable signal."]
    #[inline(always)]
    pub fn audclk_disable(&self) -> AudclkDisableR {
        AudclkDisableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Color Space Converter clock synchronous disable\n\nsignal."]
    #[inline(always)]
    pub fn cscclk_disable(&self) -> CscclkDisableR {
        CscclkDisableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CEC Engine clock synchronous disable signal."]
    #[inline(always)]
    pub fn cecclk_disable(&self) -> CecclkDisableR {
        CecclkDisableR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HDCP clock synchronous disable signal. When active\n\n(1b), simultaneously bypasses HDCP."]
    #[inline(always)]
    pub fn hdcpclk_disable(&self) -> HdcpclkDisableR {
        HdcpclkDisableR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HDCP22 clock synchronous disable signal. When active\n\n(1b), simultaneously bypasses HDCP22."]
    #[inline(always)]
    pub fn h22sclk_disable(&self) -> H22sclkDisableR {
        H22sclkDisableR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Audio Sampler clock synchronous disable signal."]
    #[inline(always)]
    #[must_use]
    pub fn audclk_disable(&mut self) -> AudclkDisableW<McClkdisSpec> {
        AudclkDisableW::new(self, 3)
    }
    #[doc = "Bit 4 - Color Space Converter clock synchronous disable\n\nsignal."]
    #[inline(always)]
    #[must_use]
    pub fn cscclk_disable(&mut self) -> CscclkDisableW<McClkdisSpec> {
        CscclkDisableW::new(self, 4)
    }
    #[doc = "Bit 5 - CEC Engine clock synchronous disable signal."]
    #[inline(always)]
    #[must_use]
    pub fn cecclk_disable(&mut self) -> CecclkDisableW<McClkdisSpec> {
        CecclkDisableW::new(self, 5)
    }
    #[doc = "Bit 6 - HDCP clock synchronous disable signal. When active\n\n(1b), simultaneously bypasses HDCP."]
    #[inline(always)]
    #[must_use]
    pub fn hdcpclk_disable(&mut self) -> HdcpclkDisableW<McClkdisSpec> {
        HdcpclkDisableW::new(self, 6)
    }
    #[doc = "Bit 7 - HDCP22 clock synchronous disable signal. When active\n\n(1b), simultaneously bypasses HDCP22."]
    #[inline(always)]
    #[must_use]
    pub fn h22sclk_disable(&mut self) -> H22sclkDisableW<McClkdisSpec> {
        H22sclkDisableW::new(self, 7)
    }
}
#[doc = "Main Controller Synchronous Clock Domain Disable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mc_clkdis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mc_clkdis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McClkdisSpec;
impl crate::RegisterSpec for McClkdisSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mc_clkdis::R`](R) reader structure"]
impl crate::Readable for McClkdisSpec {}
#[doc = "`write(|w| ..)` method takes [`mc_clkdis::W`](W) writer structure"]
impl crate::Writable for McClkdisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MC_CLKDIS to value 0"]
impl crate::Resettable for McClkdisSpec {
    const RESET_VALUE: u8 = 0;
}
