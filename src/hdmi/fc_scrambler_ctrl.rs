#[doc = "Register `FC_SCRAMBLER_CTRL` reader"]
pub type R = crate::R<FcScramblerCtrlSpec>;
#[doc = "Register `FC_SCRAMBLER_CTRL` writer"]
pub type W = crate::W<FcScramblerCtrlSpec>;
#[doc = "Field `SCRAMBLER_ON` reader - When set (1'b1), this field activates the HDMI 2.0 scrambler feature. When disabled (1'b0) the scrambler feature is bypassed, placing Hdmi_tx in HDMI 1.4b compatible mode. To activate the scrambler feature, you must ensure that the quasi- static configuration bit fc_invidconf.HDCP_keepout is set (1'b1) at configuration time, before the required mc_swrstzreq.tmdsswrst_req reset request is issued. This is field can be changed in runtime."]
pub type ScramblerOnR = crate::BitReader;
#[doc = "Field `SCRAMBLER_ON` writer - When set (1'b1), this field activates the HDMI 2.0 scrambler feature. When disabled (1'b0) the scrambler feature is bypassed, placing Hdmi_tx in HDMI 1.4b compatible mode. To activate the scrambler feature, you must ensure that the quasi- static configuration bit fc_invidconf.HDCP_keepout is set (1'b1) at configuration time, before the required mc_swrstzreq.tmdsswrst_req reset request is issued. This is field can be changed in runtime."]
pub type ScramblerOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCRAMBLER_UCP_LINE` reader - Debug register. When active (1'b1), the Unscrambled Control Period is generated after each active video line (non-compliant behavior). This is quasi-static field which requires a a mc_swrstzreq.tmdsswrst_req reset request to be performed after the change of this configuration bit."]
pub type ScramblerUcpLineR = crate::BitReader;
#[doc = "Field `SCRAMBLER_UCP_LINE` writer - Debug register. When active (1'b1), the Unscrambled Control Period is generated after each active video line (non-compliant behavior). This is quasi-static field which requires a a mc_swrstzreq.tmdsswrst_req reset request to be performed after the change of this configuration bit."]
pub type ScramblerUcpLineW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set (1'b1), this field activates the HDMI 2.0 scrambler feature. When disabled (1'b0) the scrambler feature is bypassed, placing Hdmi_tx in HDMI 1.4b compatible mode. To activate the scrambler feature, you must ensure that the quasi- static configuration bit fc_invidconf.HDCP_keepout is set (1'b1) at configuration time, before the required mc_swrstzreq.tmdsswrst_req reset request is issued. This is field can be changed in runtime."]
    #[inline(always)]
    pub fn scrambler_on(&self) -> ScramblerOnR {
        ScramblerOnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Debug register. When active (1'b1), the Unscrambled Control Period is generated after each active video line (non-compliant behavior). This is quasi-static field which requires a a mc_swrstzreq.tmdsswrst_req reset request to be performed after the change of this configuration bit."]
    #[inline(always)]
    pub fn scrambler_ucp_line(&self) -> ScramblerUcpLineR {
        ScramblerUcpLineR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set (1'b1), this field activates the HDMI 2.0 scrambler feature. When disabled (1'b0) the scrambler feature is bypassed, placing Hdmi_tx in HDMI 1.4b compatible mode. To activate the scrambler feature, you must ensure that the quasi- static configuration bit fc_invidconf.HDCP_keepout is set (1'b1) at configuration time, before the required mc_swrstzreq.tmdsswrst_req reset request is issued. This is field can be changed in runtime."]
    #[inline(always)]
    #[must_use]
    pub fn scrambler_on(&mut self) -> ScramblerOnW<FcScramblerCtrlSpec> {
        ScramblerOnW::new(self, 0)
    }
    #[doc = "Bit 4 - Debug register. When active (1'b1), the Unscrambled Control Period is generated after each active video line (non-compliant behavior). This is quasi-static field which requires a a mc_swrstzreq.tmdsswrst_req reset request to be performed after the change of this configuration bit."]
    #[inline(always)]
    #[must_use]
    pub fn scrambler_ucp_line(&mut self) -> ScramblerUcpLineW<FcScramblerCtrlSpec> {
        ScramblerUcpLineW::new(self, 4)
    }
}
#[doc = "When set (1'b1), this field activates the HDMI 2.0 scrambler feature. When disabled (1'b0) the scrambler feature is bypassed, placing Hdmi_tx in HDMI 1.4b compatible mode. To activate the scrambler feature, you must ensure that the quasi- static configuration bit fc_invidconf.HDCP_keepout is set (1'b1) at configuration time, before the required mc_swrstzreq.tmdsswrst_req reset request is issued. This is field can be changed in runtime.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_scrambler_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_scrambler_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
