#[doc = "Register `PCIE_PF_LINK_CONTROL_AND_STATUS_2` reader"]
pub type R = crate::R<PciePfLinkControlAndStatus2Spec>;
#[doc = "Register `PCIE_PF_LINK_CONTROL_AND_STATUS_2` writer"]
pub type W = crate::W<PciePfLinkControlAndStatus2Spec>;
#[doc = "Field `TLS` reader - Target Link Speed \\[TLS\\]\n\nFor an upstream component, this\n\nfield sets an upper limit on Link\n\noperational speed during\n\nreconfiguration. Additionally for both\n\nupstream and downstream\n\ncomponents, this field sets the\n\ntarget speed when the software\n\nforces the link into Compliance\n\nmode by setting the Enter\n\nCompliance bit in this register (0001\n\n= 2.5 GT/ s, 0010 = 5 GT/s, 0100 =\n\n8 GT/s). The default value of this\n\nfield is 0001 (2.5 GT/s) when the\n\nPCIE_GENERATION_SEL strap pins\n\nof the core are set to 0, 0010 (5\n\nGT/s) when the strap is set to 1.\n\nThese bits are STICKY."]
pub type TlsR = crate::FieldReader;
#[doc = "Field `TLS` writer - Target Link Speed \\[TLS\\]\n\nFor an upstream component, this\n\nfield sets an upper limit on Link\n\noperational speed during\n\nreconfiguration. Additionally for both\n\nupstream and downstream\n\ncomponents, this field sets the\n\ntarget speed when the software\n\nforces the link into Compliance\n\nmode by setting the Enter\n\nCompliance bit in this register (0001\n\n= 2.5 GT/ s, 0010 = 5 GT/s, 0100 =\n\n8 GT/s). The default value of this\n\nfield is 0001 (2.5 GT/s) when the\n\nPCIE_GENERATION_SEL strap pins\n\nof the core are set to 0, 0010 (5\n\nGT/s) when the strap is set to 1.\n\nThese bits are STICKY."]
pub type TlsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EC` reader - Enter Compliance \\[EC\\]\n\ninto the Compliance mode. The\n\ntarget speed for the Compliance\n\nmode is determined by the Target\n\nLink Speed field of this register.\n\nSTICKY."]
pub type EcR = crate::BitReader;
#[doc = "Field `EC` writer - Enter Compliance \\[EC\\]\n\ninto the Compliance mode. The\n\ntarget speed for the Compliance\n\nmode is determined by the Target\n\nLink Speed field of this register.\n\nSTICKY."]
pub type EcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASD` reader - Hardware Autonomous Speed Disable \\[HASD\\]\n\nWhen this bit is set, the LTSSM is\n\nprevented from changing the\n\noperating speed of the link, other\n\nthan reducing the speed to correct\n\nunreliable operation of the link.\n\nSTICKY"]
pub type HasdR = crate::BitReader;
#[doc = "Field `HASD` writer - Hardware Autonomous Speed Disable \\[HASD\\]\n\nWhen this bit is set, the LTSSM is\n\nprevented from changing the\n\noperating speed of the link, other\n\nthan reducing the speed to correct\n\nunreliable operation of the link.\n\nSTICKY"]
pub type HasdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDE` reader - Selectable De- emphasis \\[SDE\\]\n\nThis bit selects the de-emphasis\n\nlevel when the core is operating at 5\n\nGT/s (0 = -6 dB, 1 = -3.5 dB). This\n\nis reserved for Endpoints."]
pub type SdeR = crate::BitReader;
#[doc = "Field `TM` reader - Transmit Margin \\[TM\\]\n\nThis field is intended for debug and\n\ncompliance testing purposes only. It\n\ncontrols the non-de- emphasized\n\nvoltage level at the transmitter\n\noutputs. Its encodings are: 000:\n\nNormal operating range. 001: 800 -\n\n1200 mV for full swing and 400 -\n\n700 mV for half swing. 010 - 111:\n\nSee PCI Express Base Specification\n\n2.0. This field is reset to 0 when the\n\nLTSSM enters the Polling\n\nConfiguration substate during link\n\ntraining. STICKY."]
pub type TmR = crate::FieldReader;
#[doc = "Field `TM` writer - Transmit Margin \\[TM\\]\n\nThis field is intended for debug and\n\ncompliance testing purposes only. It\n\ncontrols the non-de- emphasized\n\nvoltage level at the transmitter\n\noutputs. Its encodings are: 000:\n\nNormal operating range. 001: 800 -\n\n1200 mV for full swing and 400 -\n\n700 mV for half swing. 010 - 111:\n\nSee PCI Express Base Specification\n\n2.0. This field is reset to 0 when the\n\nLTSSM enters the Polling\n\nConfiguration substate during link\n\ntraining. STICKY."]
pub type TmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EMC` reader - Enter Modified Compliance \\[EMC\\]\n\nThis field is intended for debug and\n\ncompliance testing purposes only. If\n\nthis bit is set to 1, the device will\n\ntransmit the Modified Compliance\n\nPattern when the LTSSM enters the\n\nPolling. Compliance substate.\n\nSTICKY."]
pub type EmcR = crate::BitReader;
#[doc = "Field `EMC` writer - Enter Modified Compliance \\[EMC\\]\n\nThis field is intended for debug and\n\ncompliance testing purposes only. If\n\nthis bit is set to 1, the device will\n\ntransmit the Modified Compliance\n\nPattern when the LTSSM enters the\n\nPolling. Compliance substate.\n\nSTICKY."]
pub type EmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS` reader - Compliance SOS \\[CS\\]\n\nWhen this bit is set to 1, the device\n\nwill transmit SKP ordered sets\n\nbetween compliance patterns.\n\nSTICKY."]
pub type CsR = crate::BitReader;
#[doc = "Field `CS` writer - Compliance SOS \\[CS\\]\n\nWhen this bit is set to 1, the device\n\nwill transmit SKP ordered sets\n\nbetween compliance patterns.\n\nSTICKY."]
pub type CsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDE` reader - Compliance De- Emphasis \\[CDE\\]\n\nThis bit sets the de-emphasis level\n\n(for 5GT/s operation) or the\n\nTransmitter Preset level (for 8 GT/s\n\noperation) when the LTSSM enters\n\nthe Polling Compliance state\n\nbecause of software\n\nsetting the Enter Compliance bit in\n\nthis register. It is used only when\n\nthe link is running at 5 GT/s or 8\n\nGT/s. At 5 GT/s, the only valid\n\nsetting are 0 (-6dB) and 1 (-3.5 dB).\n\nSTICKY."]
pub type CdeR = crate::FieldReader;
#[doc = "Field `CDE` writer - Compliance De- Emphasis \\[CDE\\]\n\nThis bit sets the de-emphasis level\n\n(for 5GT/s operation) or the\n\nTransmitter Preset level (for 8 GT/s\n\noperation) when the LTSSM enters\n\nthe Polling Compliance state\n\nbecause of software\n\nsetting the Enter Compliance bit in\n\nthis register. It is used only when\n\nthe link is running at 5 GT/s or 8\n\nGT/s. At 5 GT/s, the only valid\n\nsetting are 0 (-6dB) and 1 (-3.5 dB).\n\nSTICKY."]
pub type CdeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CDEL` reader - Current De- Emphasis Level \\[CDEL\\]\n\nThis status bit indicates the current\n\noperating de- emphasis level of the\n\ntransmitter (0 = -6 dB, 1 = -3.5\n\ndB).This field is undefined when link\n\nis not at Gen2 speed."]
pub type CdelR = crate::BitReader;
#[doc = "Field `R20` reader - Reserved \\[R20\\]\n\nReserved"]
pub type R20R = crate::FieldReader;
#[doc = "Field `R19` reader - Reserved \\[R19\\]\n\nReserved"]
pub type R19R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Target Link Speed \\[TLS\\]\n\nFor an upstream component, this\n\nfield sets an upper limit on Link\n\noperational speed during\n\nreconfiguration. Additionally for both\n\nupstream and downstream\n\ncomponents, this field sets the\n\ntarget speed when the software\n\nforces the link into Compliance\n\nmode by setting the Enter\n\nCompliance bit in this register (0001\n\n= 2.5 GT/ s, 0010 = 5 GT/s, 0100 =\n\n8 GT/s). The default value of this\n\nfield is 0001 (2.5 GT/s) when the\n\nPCIE_GENERATION_SEL strap pins\n\nof the core are set to 0, 0010 (5\n\nGT/s) when the strap is set to 1.\n\nThese bits are STICKY."]
    #[inline(always)]
    pub fn tls(&self) -> TlsR {
        TlsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Enter Compliance \\[EC\\]\n\ninto the Compliance mode. The\n\ntarget speed for the Compliance\n\nmode is determined by the Target\n\nLink Speed field of this register.\n\nSTICKY."]
    #[inline(always)]
    pub fn ec(&self) -> EcR {
        EcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hardware Autonomous Speed Disable \\[HASD\\]\n\nWhen this bit is set, the LTSSM is\n\nprevented from changing the\n\noperating speed of the link, other\n\nthan reducing the speed to correct\n\nunreliable operation of the link.\n\nSTICKY"]
    #[inline(always)]
    pub fn hasd(&self) -> HasdR {
        HasdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selectable De- emphasis \\[SDE\\]\n\nThis bit selects the de-emphasis\n\nlevel when the core is operating at 5\n\nGT/s (0 = -6 dB, 1 = -3.5 dB). This\n\nis reserved for Endpoints."]
    #[inline(always)]
    pub fn sde(&self) -> SdeR {
        SdeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:9 - Transmit Margin \\[TM\\]\n\nThis field is intended for debug and\n\ncompliance testing purposes only. It\n\ncontrols the non-de- emphasized\n\nvoltage level at the transmitter\n\noutputs. Its encodings are: 000:\n\nNormal operating range. 001: 800 -\n\n1200 mV for full swing and 400 -\n\n700 mV for half swing. 010 - 111:\n\nSee PCI Express Base Specification\n\n2.0. This field is reset to 0 when the\n\nLTSSM enters the Polling\n\nConfiguration substate during link\n\ntraining. STICKY."]
    #[inline(always)]
    pub fn tm(&self) -> TmR {
        TmR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - Enter Modified Compliance \\[EMC\\]\n\nThis field is intended for debug and\n\ncompliance testing purposes only. If\n\nthis bit is set to 1, the device will\n\ntransmit the Modified Compliance\n\nPattern when the LTSSM enters the\n\nPolling. Compliance substate.\n\nSTICKY."]
    #[inline(always)]
    pub fn emc(&self) -> EmcR {
        EmcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Compliance SOS \\[CS\\]\n\nWhen this bit is set to 1, the device\n\nwill transmit SKP ordered sets\n\nbetween compliance patterns.\n\nSTICKY."]
    #[inline(always)]
    pub fn cs(&self) -> CsR {
        CsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Compliance De- Emphasis \\[CDE\\]\n\nThis bit sets the de-emphasis level\n\n(for 5GT/s operation) or the\n\nTransmitter Preset level (for 8 GT/s\n\noperation) when the LTSSM enters\n\nthe Polling Compliance state\n\nbecause of software\n\nsetting the Enter Compliance bit in\n\nthis register. It is used only when\n\nthe link is running at 5 GT/s or 8\n\nGT/s. At 5 GT/s, the only valid\n\nsetting are 0 (-6dB) and 1 (-3.5 dB).\n\nSTICKY."]
    #[inline(always)]
    pub fn cde(&self) -> CdeR {
        CdeR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Current De- Emphasis Level \\[CDEL\\]\n\nThis status bit indicates the current\n\noperating de- emphasis level of the\n\ntransmitter (0 = -6 dB, 1 = -3.5\n\ndB).This field is undefined when link\n\nis not at Gen2 speed."]
    #[inline(always)]
    pub fn cdel(&self) -> CdelR {
        CdelR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - Reserved \\[R20\\]\n\nReserved"]
    #[inline(always)]
    pub fn r20(&self) -> R20R {
        R20R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:31 - Reserved \\[R19\\]\n\nReserved"]
    #[inline(always)]
    pub fn r19(&self) -> R19R {
        R19R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Target Link Speed \\[TLS\\]\n\nFor an upstream component, this\n\nfield sets an upper limit on Link\n\noperational speed during\n\nreconfiguration. Additionally for both\n\nupstream and downstream\n\ncomponents, this field sets the\n\ntarget speed when the software\n\nforces the link into Compliance\n\nmode by setting the Enter\n\nCompliance bit in this register (0001\n\n= 2.5 GT/ s, 0010 = 5 GT/s, 0100 =\n\n8 GT/s). The default value of this\n\nfield is 0001 (2.5 GT/s) when the\n\nPCIE_GENERATION_SEL strap pins\n\nof the core are set to 0, 0010 (5\n\nGT/s) when the strap is set to 1.\n\nThese bits are STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn tls(&mut self) -> TlsW<PciePfLinkControlAndStatus2Spec> {
        TlsW::new(self, 0)
    }
    #[doc = "Bit 4 - Enter Compliance \\[EC\\]\n\ninto the Compliance mode. The\n\ntarget speed for the Compliance\n\nmode is determined by the Target\n\nLink Speed field of this register.\n\nSTICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ec(&mut self) -> EcW<PciePfLinkControlAndStatus2Spec> {
        EcW::new(self, 4)
    }
    #[doc = "Bit 5 - Hardware Autonomous Speed Disable \\[HASD\\]\n\nWhen this bit is set, the LTSSM is\n\nprevented from changing the\n\noperating speed of the link, other\n\nthan reducing the speed to correct\n\nunreliable operation of the link.\n\nSTICKY"]
    #[inline(always)]
    #[must_use]
    pub fn hasd(&mut self) -> HasdW<PciePfLinkControlAndStatus2Spec> {
        HasdW::new(self, 5)
    }
    #[doc = "Bits 7:9 - Transmit Margin \\[TM\\]\n\nThis field is intended for debug and\n\ncompliance testing purposes only. It\n\ncontrols the non-de- emphasized\n\nvoltage level at the transmitter\n\noutputs. Its encodings are: 000:\n\nNormal operating range. 001: 800 -\n\n1200 mV for full swing and 400 -\n\n700 mV for half swing. 010 - 111:\n\nSee PCI Express Base Specification\n\n2.0. This field is reset to 0 when the\n\nLTSSM enters the Polling\n\nConfiguration substate during link\n\ntraining. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn tm(&mut self) -> TmW<PciePfLinkControlAndStatus2Spec> {
        TmW::new(self, 7)
    }
    #[doc = "Bit 10 - Enter Modified Compliance \\[EMC\\]\n\nThis field is intended for debug and\n\ncompliance testing purposes only. If\n\nthis bit is set to 1, the device will\n\ntransmit the Modified Compliance\n\nPattern when the LTSSM enters the\n\nPolling. Compliance substate.\n\nSTICKY."]
    #[inline(always)]
    #[must_use]
    pub fn emc(&mut self) -> EmcW<PciePfLinkControlAndStatus2Spec> {
        EmcW::new(self, 10)
    }
    #[doc = "Bit 11 - Compliance SOS \\[CS\\]\n\nWhen this bit is set to 1, the device\n\nwill transmit SKP ordered sets\n\nbetween compliance patterns.\n\nSTICKY."]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CsW<PciePfLinkControlAndStatus2Spec> {
        CsW::new(self, 11)
    }
    #[doc = "Bits 12:15 - Compliance De- Emphasis \\[CDE\\]\n\nThis bit sets the de-emphasis level\n\n(for 5GT/s operation) or the\n\nTransmitter Preset level (for 8 GT/s\n\noperation) when the LTSSM enters\n\nthe Polling Compliance state\n\nbecause of software\n\nsetting the Enter Compliance bit in\n\nthis register. It is used only when\n\nthe link is running at 5 GT/s or 8\n\nGT/s. At 5 GT/s, the only valid\n\nsetting are 0 (-6dB) and 1 (-3.5 dB).\n\nSTICKY."]
    #[inline(always)]
    #[must_use]
    pub fn cde(&mut self) -> CdeW<PciePfLinkControlAndStatus2Spec> {
        CdeW::new(self, 12)
    }
}
#[doc = "Link Control and Status Register 2\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_link_control_and_status_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_link_control_and_status_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfLinkControlAndStatus2Spec;
impl crate::RegisterSpec for PciePfLinkControlAndStatus2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_link_control_and_status_2::R`](R) reader structure"]
impl crate::Readable for PciePfLinkControlAndStatus2Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_link_control_and_status_2::W`](W) writer structure"]
impl crate::Writable for PciePfLinkControlAndStatus2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_PF_LINK_CONTROL_AND_STATUS_2 to value 0x02"]
impl crate::Resettable for PciePfLinkControlAndStatus2Spec {
    const RESET_VALUE: u32 = 0x02;
}
