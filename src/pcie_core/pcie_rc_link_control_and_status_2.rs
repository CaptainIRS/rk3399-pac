#[doc = "Register `PCIE_RC_LINK_CONTROL_AND_STATUS_2` reader"]
pub type R = crate::R<PcieRcLinkControlAndStatus2Spec>;
#[doc = "Register `PCIE_RC_LINK_CONTROL_AND_STATUS_2` writer"]
pub type W = crate::W<PcieRcLinkControlAndStatus2Spec>;
#[doc = "Field `TLS` reader - Target Link Speed \\[TLS\\]\n\nThis field sets the target speed when\n\nthe software forces the link into\n\nCompliance mode by setting the\n\nEnter Compliance bit in this register\n\n(0001= 2.5 GT/s, 0010 = 5 GT/s,\n\n0100 = 8 GT/s). The default value\n\nof this field is 0001 (2.5 GT/s) when\n\nthe PCIE_GENERATION_SEL\\[1:0\\]\n\nstrap pins of the core are set to 0,\n\n0010 (5 GT/s) when the strap is set\n\nto 1. STICKY."]
pub type TlsR = crate::FieldReader;
#[doc = "Field `TLS` writer - Target Link Speed \\[TLS\\]\n\nThis field sets the target speed when\n\nthe software forces the link into\n\nCompliance mode by setting the\n\nEnter Compliance bit in this register\n\n(0001= 2.5 GT/s, 0010 = 5 GT/s,\n\n0100 = 8 GT/s). The default value\n\nof this field is 0001 (2.5 GT/s) when\n\nthe PCIE_GENERATION_SEL\\[1:0\\]\n\nstrap pins of the core are set to 0,\n\n0010 (5 GT/s) when the strap is set\n\nto 1. STICKY."]
pub type TlsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EC` reader - Enter Compliance \\[EC\\]\n\nThis bit is used to force the Endpoint\n\ndevice to enter the Compliance\n\nmode. Software sets this bit to 1\n\nand initiates a hot reset to force the\n\ndevice into the Compliance mode.\n\nThe target speed for the Compliance\n\nmode is determined by the Target\n\nLink Speed field of this register.\n\nSTICKY."]
pub type EcR = crate::BitReader;
#[doc = "Field `EC` writer - Enter Compliance \\[EC\\]\n\nThis bit is used to force the Endpoint\n\ndevice to enter the Compliance\n\nmode. Software sets this bit to 1\n\nand initiates a hot reset to force the\n\ndevice into the Compliance mode.\n\nThe target speed for the Compliance\n\nmode is determined by the Target\n\nLink Speed field of this register.\n\nSTICKY."]
pub type EcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASD` reader - Hardware Autonomous Speed Disable \\[HASD\\]\n\nWhen this bit is set, the LTSSM is\n\nprevented from changing the\n\noperating speed of the link, other\n\nthan reducing the speed to correct\n\nunreliable operation of the link.\n\nSTICKY"]
pub type HasdR = crate::BitReader;
#[doc = "Field `HASD` writer - Hardware Autonomous Speed Disable \\[HASD\\]\n\nWhen this bit is set, the LTSSM is\n\nprevented from changing the\n\noperating speed of the link, other\n\nthan reducing the speed to correct\n\nunreliable operation of the link.\n\nSTICKY"]
pub type HasdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD` reader - Selectable De- Emphasis \\[SD\\]\n\nThis bit selects the de-emphasis\n\nlevel when the core is operating at 5\n\nGT/s (0 = -6 dB, 1 = -3.5 dB)."]
pub type SdR = crate::BitReader;
#[doc = "Field `SD` writer - Selectable De- Emphasis \\[SD\\]\n\nThis bit selects the de-emphasis\n\nlevel when the core is operating at 5\n\nGT/s (0 = -6 dB, 1 = -3.5 dB)."]
pub type SdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM` reader - Transmit Margin \\[TM\\]\n\nThis field is intended for debug and\n\ncompliance testing purposes only. It\n\ncontrols the non- deemphasized\n\nvoltage level at the transmitter\n\noutputs. Its encodings are: 000 =\n\nNormal operating range, 001 = 800\n\n- 1200 mV for full swing and 400 -\n\n700 mV for half swing, 010 - 111 =\n\nSee PCI Express Base Specification\n\n2.0. This field is reset to 0 when th\n\nLTSSM enters the\n\nPolling.Configuration substate during\n\nlink training. STICKY."]
pub type TmR = crate::FieldReader;
#[doc = "Field `TM` writer - Transmit Margin \\[TM\\]\n\nThis field is intended for debug and\n\ncompliance testing purposes only. It\n\ncontrols the non- deemphasized\n\nvoltage level at the transmitter\n\noutputs. Its encodings are: 000 =\n\nNormal operating range, 001 = 800\n\n- 1200 mV for full swing and 400 -\n\n700 mV for half swing, 010 - 111 =\n\nSee PCI Express Base Specification\n\n2.0. This field is reset to 0 when th\n\nLTSSM enters the\n\nPolling.Configuration substate during\n\nlink training. STICKY."]
pub type TmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EMC` reader - Enter Modified Compliance \\[EMC\\]\n\nThis field is intended for debug and\n\ncompliance testing purposes only. If\n\nthis bit is set to 1, the device will\n\ntransmit the Modified Compliance\n\nPattern when the LTSSM enters the\n\nPolling.Compliance substate. STICKY"]
pub type EmcR = crate::BitReader;
#[doc = "Field `EMC` writer - Enter Modified Compliance \\[EMC\\]\n\nThis field is intended for debug and\n\ncompliance testing purposes only. If\n\nthis bit is set to 1, the device will\n\ntransmit the Modified Compliance\n\nPattern when the LTSSM enters the\n\nPolling.Compliance substate. STICKY"]
pub type EmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS` reader - Compliance SOS \\[CS\\]\n\nWhen this bit is set to 1, the device\n\nwill transmit SKP ordered sets\n\nbetween compliance patterns.\n\nSTICKY"]
pub type CsR = crate::BitReader;
#[doc = "Field `CS` writer - Compliance SOS \\[CS\\]\n\nWhen this bit is set to 1, the device\n\nwill transmit SKP ordered sets\n\nbetween compliance patterns.\n\nSTICKY"]
pub type CsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CD` reader - Compliance De- Emphasis \\[CD\\]\n\nThis bit sets the de-emphasis level\n\n(for 5 GT/s operation) or the\n\nTransmitter Preset level (for 8 GT/s\n\noperation) when the LTSSM enters\n\nthe Polling.Compliance state\n\nbecause of software setting the\n\nEnter Compliance bit in this register.\n\nIt is used only when the link is\n\nrunning at 5 GT/s or 8 GT/s. At 5\n\nGT/s, the only valid setting are 0 (-\n\n6dB) and 1 (-3.5 dB). STICKY"]
pub type CdR = crate::FieldReader;
#[doc = "Field `CD` writer - Compliance De- Emphasis \\[CD\\]\n\nThis bit sets the de-emphasis level\n\n(for 5 GT/s operation) or the\n\nTransmitter Preset level (for 8 GT/s\n\noperation) when the LTSSM enters\n\nthe Polling.Compliance state\n\nbecause of software setting the\n\nEnter Compliance bit in this register.\n\nIt is used only when the link is\n\nrunning at 5 GT/s or 8 GT/s. At 5\n\nGT/s, the only valid setting are 0 (-\n\n6dB) and 1 (-3.5 dB). STICKY"]
pub type CdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CDEL` reader - Current De- Emphasis Level \\[CDEL\\]\n\nThis status bit indicates the current\n\noperating de- emphasis level of the\n\ntransmitter (0 = -6dB, 1 = -3.5dB)."]
pub type CdelR = crate::BitReader;
#[doc = "Field `R20` reader - Reserved \\[R20\\]\n\nReserved"]
pub type R20R = crate::FieldReader;
#[doc = "Field `R19` reader - Reserved \\[R19\\]\n\nReserved"]
pub type R19R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Target Link Speed \\[TLS\\]\n\nThis field sets the target speed when\n\nthe software forces the link into\n\nCompliance mode by setting the\n\nEnter Compliance bit in this register\n\n(0001= 2.5 GT/s, 0010 = 5 GT/s,\n\n0100 = 8 GT/s). The default value\n\nof this field is 0001 (2.5 GT/s) when\n\nthe PCIE_GENERATION_SEL\\[1:0\\]\n\nstrap pins of the core are set to 0,\n\n0010 (5 GT/s) when the strap is set\n\nto 1. STICKY."]
    #[inline(always)]
    pub fn tls(&self) -> TlsR {
        TlsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Enter Compliance \\[EC\\]\n\nThis bit is used to force the Endpoint\n\ndevice to enter the Compliance\n\nmode. Software sets this bit to 1\n\nand initiates a hot reset to force the\n\ndevice into the Compliance mode.\n\nThe target speed for the Compliance\n\nmode is determined by the Target\n\nLink Speed field of this register.\n\nSTICKY."]
    #[inline(always)]
    pub fn ec(&self) -> EcR {
        EcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hardware Autonomous Speed Disable \\[HASD\\]\n\nWhen this bit is set, the LTSSM is\n\nprevented from changing the\n\noperating speed of the link, other\n\nthan reducing the speed to correct\n\nunreliable operation of the link.\n\nSTICKY"]
    #[inline(always)]
    pub fn hasd(&self) -> HasdR {
        HasdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selectable De- Emphasis \\[SD\\]\n\nThis bit selects the de-emphasis\n\nlevel when the core is operating at 5\n\nGT/s (0 = -6 dB, 1 = -3.5 dB)."]
    #[inline(always)]
    pub fn sd(&self) -> SdR {
        SdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:9 - Transmit Margin \\[TM\\]\n\nThis field is intended for debug and\n\ncompliance testing purposes only. It\n\ncontrols the non- deemphasized\n\nvoltage level at the transmitter\n\noutputs. Its encodings are: 000 =\n\nNormal operating range, 001 = 800\n\n- 1200 mV for full swing and 400 -\n\n700 mV for half swing, 010 - 111 =\n\nSee PCI Express Base Specification\n\n2.0. This field is reset to 0 when th\n\nLTSSM enters the\n\nPolling.Configuration substate during\n\nlink training. STICKY."]
    #[inline(always)]
    pub fn tm(&self) -> TmR {
        TmR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - Enter Modified Compliance \\[EMC\\]\n\nThis field is intended for debug and\n\ncompliance testing purposes only. If\n\nthis bit is set to 1, the device will\n\ntransmit the Modified Compliance\n\nPattern when the LTSSM enters the\n\nPolling.Compliance substate. STICKY"]
    #[inline(always)]
    pub fn emc(&self) -> EmcR {
        EmcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Compliance SOS \\[CS\\]\n\nWhen this bit is set to 1, the device\n\nwill transmit SKP ordered sets\n\nbetween compliance patterns.\n\nSTICKY"]
    #[inline(always)]
    pub fn cs(&self) -> CsR {
        CsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Compliance De- Emphasis \\[CD\\]\n\nThis bit sets the de-emphasis level\n\n(for 5 GT/s operation) or the\n\nTransmitter Preset level (for 8 GT/s\n\noperation) when the LTSSM enters\n\nthe Polling.Compliance state\n\nbecause of software setting the\n\nEnter Compliance bit in this register.\n\nIt is used only when the link is\n\nrunning at 5 GT/s or 8 GT/s. At 5\n\nGT/s, the only valid setting are 0 (-\n\n6dB) and 1 (-3.5 dB). STICKY"]
    #[inline(always)]
    pub fn cd(&self) -> CdR {
        CdR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Current De- Emphasis Level \\[CDEL\\]\n\nThis status bit indicates the current\n\noperating de- emphasis level of the\n\ntransmitter (0 = -6dB, 1 = -3.5dB)."]
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
    #[doc = "Bits 0:3 - Target Link Speed \\[TLS\\]\n\nThis field sets the target speed when\n\nthe software forces the link into\n\nCompliance mode by setting the\n\nEnter Compliance bit in this register\n\n(0001= 2.5 GT/s, 0010 = 5 GT/s,\n\n0100 = 8 GT/s). The default value\n\nof this field is 0001 (2.5 GT/s) when\n\nthe PCIE_GENERATION_SEL\\[1:0\\]\n\nstrap pins of the core are set to 0,\n\n0010 (5 GT/s) when the strap is set\n\nto 1. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn tls(&mut self) -> TlsW<PcieRcLinkControlAndStatus2Spec> {
        TlsW::new(self, 0)
    }
    #[doc = "Bit 4 - Enter Compliance \\[EC\\]\n\nThis bit is used to force the Endpoint\n\ndevice to enter the Compliance\n\nmode. Software sets this bit to 1\n\nand initiates a hot reset to force the\n\ndevice into the Compliance mode.\n\nThe target speed for the Compliance\n\nmode is determined by the Target\n\nLink Speed field of this register.\n\nSTICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ec(&mut self) -> EcW<PcieRcLinkControlAndStatus2Spec> {
        EcW::new(self, 4)
    }
    #[doc = "Bit 5 - Hardware Autonomous Speed Disable \\[HASD\\]\n\nWhen this bit is set, the LTSSM is\n\nprevented from changing the\n\noperating speed of the link, other\n\nthan reducing the speed to correct\n\nunreliable operation of the link.\n\nSTICKY"]
    #[inline(always)]
    #[must_use]
    pub fn hasd(&mut self) -> HasdW<PcieRcLinkControlAndStatus2Spec> {
        HasdW::new(self, 5)
    }
    #[doc = "Bit 6 - Selectable De- Emphasis \\[SD\\]\n\nThis bit selects the de-emphasis\n\nlevel when the core is operating at 5\n\nGT/s (0 = -6 dB, 1 = -3.5 dB)."]
    #[inline(always)]
    #[must_use]
    pub fn sd(&mut self) -> SdW<PcieRcLinkControlAndStatus2Spec> {
        SdW::new(self, 6)
    }
    #[doc = "Bits 7:9 - Transmit Margin \\[TM\\]\n\nThis field is intended for debug and\n\ncompliance testing purposes only. It\n\ncontrols the non- deemphasized\n\nvoltage level at the transmitter\n\noutputs. Its encodings are: 000 =\n\nNormal operating range, 001 = 800\n\n- 1200 mV for full swing and 400 -\n\n700 mV for half swing, 010 - 111 =\n\nSee PCI Express Base Specification\n\n2.0. This field is reset to 0 when th\n\nLTSSM enters the\n\nPolling.Configuration substate during\n\nlink training. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn tm(&mut self) -> TmW<PcieRcLinkControlAndStatus2Spec> {
        TmW::new(self, 7)
    }
    #[doc = "Bit 10 - Enter Modified Compliance \\[EMC\\]\n\nThis field is intended for debug and\n\ncompliance testing purposes only. If\n\nthis bit is set to 1, the device will\n\ntransmit the Modified Compliance\n\nPattern when the LTSSM enters the\n\nPolling.Compliance substate. STICKY"]
    #[inline(always)]
    #[must_use]
    pub fn emc(&mut self) -> EmcW<PcieRcLinkControlAndStatus2Spec> {
        EmcW::new(self, 10)
    }
    #[doc = "Bit 11 - Compliance SOS \\[CS\\]\n\nWhen this bit is set to 1, the device\n\nwill transmit SKP ordered sets\n\nbetween compliance patterns.\n\nSTICKY"]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CsW<PcieRcLinkControlAndStatus2Spec> {
        CsW::new(self, 11)
    }
    #[doc = "Bits 12:15 - Compliance De- Emphasis \\[CD\\]\n\nThis bit sets the de-emphasis level\n\n(for 5 GT/s operation) or the\n\nTransmitter Preset level (for 8 GT/s\n\noperation) when the LTSSM enters\n\nthe Polling.Compliance state\n\nbecause of software setting the\n\nEnter Compliance bit in this register.\n\nIt is used only when the link is\n\nrunning at 5 GT/s or 8 GT/s. At 5\n\nGT/s, the only valid setting are 0 (-\n\n6dB) and 1 (-3.5 dB). STICKY"]
    #[inline(always)]
    #[must_use]
    pub fn cd(&mut self) -> CdW<PcieRcLinkControlAndStatus2Spec> {
        CdW::new(self, 12)
    }
}
#[doc = "Link Control and Status 2 Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_link_control_and_status_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_link_control_and_status_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcLinkControlAndStatus2Spec;
impl crate::RegisterSpec for PcieRcLinkControlAndStatus2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_link_control_and_status_2::R`](R) reader structure"]
impl crate::Readable for PcieRcLinkControlAndStatus2Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_rc_link_control_and_status_2::W`](W) writer structure"]
impl crate::Writable for PcieRcLinkControlAndStatus2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_RC_LINK_CONTROL_AND_STATUS_2 to value 0x0001_0000"]
impl crate::Resettable for PcieRcLinkControlAndStatus2Spec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
