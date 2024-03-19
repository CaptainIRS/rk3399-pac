#[doc = "Register `FC_SCRAMBLER_CTRL` reader"]
pub type R = crate::R<FcScramblerCtrlSpec>;
#[doc = "Register `FC_SCRAMBLER_CTRL` writer"]
pub type W = crate::W<FcScramblerCtrlSpec>;
#[doc = "Field `SCRAMBLER_ON` reader - When set (1'b1), this field activates the HDMI 2.0\n\nscrambler feature. When disabled (1'b0) the\n\nscrambler feature is bypassed, placing Hdmi_tx in\n\nHDMI 1.4b compatible mode. To activate the\n\nscrambler feature, you must ensure that the quasi-\n\nstatic\n\nconfiguration bit fc_invidconf.HDCP_keepout is set\n\n(1'b1) at configuration time, before the required\n\nmc_swrstzreq.tmdsswrst_req reset request is\n\nissued.\n\nThis is field can be changed in runtime."]
pub type ScramblerOnR = crate::BitReader;
#[doc = "Field `SCRAMBLER_ON` writer - When set (1'b1), this field activates the HDMI 2.0\n\nscrambler feature. When disabled (1'b0) the\n\nscrambler feature is bypassed, placing Hdmi_tx in\n\nHDMI 1.4b compatible mode. To activate the\n\nscrambler feature, you must ensure that the quasi-\n\nstatic\n\nconfiguration bit fc_invidconf.HDCP_keepout is set\n\n(1'b1) at configuration time, before the required\n\nmc_swrstzreq.tmdsswrst_req reset request is\n\nissued.\n\nThis is field can be changed in runtime."]
pub type ScramblerOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCRAMBLER_UCP_LINE` reader - Debug register. When active (1'b1), the\n\nUnscrambled Control Period is generated after each\n\nactive video line (non-compliant behavior). This is\n\nquasi-static field which requires a a\n\nmc_swrstzreq.tmdsswrst_req reset request to be\n\nperformed after the change of this configuration bit."]
pub type ScramblerUcpLineR = crate::BitReader;
#[doc = "Field `SCRAMBLER_UCP_LINE` writer - Debug register. When active (1'b1), the\n\nUnscrambled Control Period is generated after each\n\nactive video line (non-compliant behavior). This is\n\nquasi-static field which requires a a\n\nmc_swrstzreq.tmdsswrst_req reset request to be\n\nperformed after the change of this configuration bit."]
pub type ScramblerUcpLineW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set (1'b1), this field activates the HDMI 2.0\n\nscrambler feature. When disabled (1'b0) the\n\nscrambler feature is bypassed, placing Hdmi_tx in\n\nHDMI 1.4b compatible mode. To activate the\n\nscrambler feature, you must ensure that the quasi-\n\nstatic\n\nconfiguration bit fc_invidconf.HDCP_keepout is set\n\n(1'b1) at configuration time, before the required\n\nmc_swrstzreq.tmdsswrst_req reset request is\n\nissued.\n\nThis is field can be changed in runtime."]
    #[inline(always)]
    pub fn scrambler_on(&self) -> ScramblerOnR {
        ScramblerOnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Debug register. When active (1'b1), the\n\nUnscrambled Control Period is generated after each\n\nactive video line (non-compliant behavior). This is\n\nquasi-static field which requires a a\n\nmc_swrstzreq.tmdsswrst_req reset request to be\n\nperformed after the change of this configuration bit."]
    #[inline(always)]
    pub fn scrambler_ucp_line(&self) -> ScramblerUcpLineR {
        ScramblerUcpLineR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set (1'b1), this field activates the HDMI 2.0\n\nscrambler feature. When disabled (1'b0) the\n\nscrambler feature is bypassed, placing Hdmi_tx in\n\nHDMI 1.4b compatible mode. To activate the\n\nscrambler feature, you must ensure that the quasi-\n\nstatic\n\nconfiguration bit fc_invidconf.HDCP_keepout is set\n\n(1'b1) at configuration time, before the required\n\nmc_swrstzreq.tmdsswrst_req reset request is\n\nissued.\n\nThis is field can be changed in runtime."]
    #[inline(always)]
    #[must_use]
    pub fn scrambler_on(&mut self) -> ScramblerOnW<FcScramblerCtrlSpec> {
        ScramblerOnW::new(self, 0)
    }
    #[doc = "Bit 4 - Debug register. When active (1'b1), the\n\nUnscrambled Control Period is generated after each\n\nactive video line (non-compliant behavior). This is\n\nquasi-static field which requires a a\n\nmc_swrstzreq.tmdsswrst_req reset request to be\n\nperformed after the change of this configuration bit."]
    #[inline(always)]
    #[must_use]
    pub fn scrambler_ucp_line(&mut self) -> ScramblerUcpLineW<FcScramblerCtrlSpec> {
        ScramblerUcpLineW::new(self, 4)
    }
}
#[doc = "Frame Composer Scrambler Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_scrambler_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_scrambler_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcScramblerCtrlSpec;
impl crate::RegisterSpec for FcScramblerCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_scrambler_ctrl::R`](R) reader structure"]
impl crate::Readable for FcScramblerCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_scrambler_ctrl::W`](W) writer structure"]
impl crate::Writable for FcScramblerCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_SCRAMBLER_CTRL to value 0"]
impl crate::Resettable for FcScramblerCtrlSpec {
    const RESET_VALUE: u8 = 0;
}
