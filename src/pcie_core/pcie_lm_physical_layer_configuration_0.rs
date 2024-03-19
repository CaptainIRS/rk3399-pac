#[doc = "Register `PCIE_LM_PHYSICAL_LAYER_CONFIGURATION_0` reader"]
pub type R = crate::R<PcieLmPhysicalLayerConfiguration0Spec>;
#[doc = "Register `PCIE_LM_PHYSICAL_LAYER_CONFIGURATION_0` writer"]
pub type W = crate::W<PcieLmPhysicalLayerConfiguration0Spec>;
#[doc = "Field `LS` reader - Link Status \\[LS\\]\n\nCurrent state of link (1 = link\n\ntraining complete, 0 = link training\n\nnot complete)."]
pub type LsR = crate::BitReader;
#[doc = "Field `NLC` reader - Negotiated Lane Count \\[NLC\\]\n\nLane count negotiated with other\n\nside during link training (00 = x1, 01\n\n= x2, 10 = x4, 11 = x8)."]
pub type NlcR = crate::FieldReader;
#[doc = "Field `NS` reader - Negotiated Speed \\[NS\\]\n\nCurrent operating speed of link (00\n\n= 2.5G, 01 = 5G, 10 = 8G)."]
pub type NsR = crate::FieldReader;
#[doc = "Field `LTD` reader - Link Training Direction \\[LTD\\]\n\nThe state of this bit indicates\n\nwhether the core completed link\n\ntraining as an upstream port or a\n\ndownstream port (0 = upstream, 1\n\n= downstream). Default value\n\ndepends on CORE_TYPE strap pin."]
pub type LtdR = crate::BitReader;
#[doc = "Field `APER` reader - Phy Error Reporting \\[APER\\]\n\nIf set to 0, the core will only report\n\nthose errors that caused a TLP or\n\nDLLP to be dropped because of a\n\ndetected phy error, TLP framing\n\nerror or DLLP framing error. When\n\nset to 1, the core will report all\n\ndetected phy errors regardless of\n\nwhether a TLP or DLLP was dropped\n\nbut does not include TLP nor DLLP\n\nframing errors. Detected phy errors\n\ninclude:- received errors indicated\n\non PIPE RxStatus interface, and TLP\n\nor DLLP framing errors depending on\n\nthe programmed value of this bit."]
pub type AperR = crate::BitReader;
#[doc = "Field `APER` writer - Phy Error Reporting \\[APER\\]\n\nIf set to 0, the core will only report\n\nthose errors that caused a TLP or\n\nDLLP to be dropped because of a\n\ndetected phy error, TLP framing\n\nerror or DLLP framing error. When\n\nset to 1, the core will report all\n\ndetected phy errors regardless of\n\nwhether a TLP or DLLP was dropped\n\nbut does not include TLP nor DLLP\n\nframing errors. Detected phy errors\n\ninclude:- received errors indicated\n\non PIPE RxStatus interface, and TLP\n\nor DLLP framing errors depending on\n\nthe programmed value of this bit."]
pub type AperW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSS` reader - Tx Swing Setting \\[TSS\\]\n\nThis bit drives the PIPE_TX_SWING\n\noutput of the core."]
pub type TssR = crate::BitReader;
#[doc = "Field `TSS` writer - Tx Swing Setting \\[TSS\\]\n\nThis bit drives the PIPE_TX_SWING\n\noutput of the core."]
pub type TssW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFC` reader - Received FTS Count for 2.5 GT/s speed \\[RFC\\]\n\nFTS count received from the other\n\nside during link training for use at\n\nthe 2.5 GT/s link speed. The core\n\ntransmits this many FTS sequences\n\nwhile exiting the L0S state, when\n\noperating at the 2.5 GT/s speed."]
pub type RfcR = crate::FieldReader;
#[doc = "Field `RLID` reader - Received Link ID \\[RLID\\]\n\nLink ID received from other side\n\nduring link training."]
pub type RlidR = crate::FieldReader;
#[doc = "Field `LTSSM` reader - LTSSM State \\[LTSSM\\]\n\nCurrent state of the LTSSM. The\n\nencoding of the states is given in\n\nAppendix C."]
pub type LtssmR = crate::FieldReader;
#[doc = "Field `R0` reader - Remote Linkwidth Upconfigure Capability Status \\[R0\\]\n\nA 1 in this field indicates that the\n\nremote node advertised Link Width\n\nUp configure Capability in the\n\ntraining sequences in the\n\nConfiguration. Complete state when\n\nthe link came up. A 0 indicates that\n\nthe remote node did not set the Link\n\nUp configure bit."]
pub type R0R = crate::BitReader;
#[doc = "Field `MLE` reader - Master Loopback Enable \\[MLE\\]\n\nWhen the core is operating as a Root\n\nPort, setting this to 1 causes the\n\nLTSSM to initiate a loopback and\n\nbecome the loopback master. This\n\nbit is not used in the EndPoint Mode."]
pub type MleR = crate::BitReader;
#[doc = "Field `MLE` writer - Master Loopback Enable \\[MLE\\]\n\nWhen the core is operating as a Root\n\nPort, setting this to 1 causes the\n\nLTSSM to initiate a loopback and\n\nbecome the loopback master. This\n\nbit is not used in the EndPoint Mode."]
pub type MleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Link Status \\[LS\\]\n\nCurrent state of link (1 = link\n\ntraining complete, 0 = link training\n\nnot complete)."]
    #[inline(always)]
    pub fn ls(&self) -> LsR {
        LsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Negotiated Lane Count \\[NLC\\]\n\nLane count negotiated with other\n\nside during link training (00 = x1, 01\n\n= x2, 10 = x4, 11 = x8)."]
    #[inline(always)]
    pub fn nlc(&self) -> NlcR {
        NlcR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Negotiated Speed \\[NS\\]\n\nCurrent operating speed of link (00\n\n= 2.5G, 01 = 5G, 10 = 8G)."]
    #[inline(always)]
    pub fn ns(&self) -> NsR {
        NsR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Link Training Direction \\[LTD\\]\n\nThe state of this bit indicates\n\nwhether the core completed link\n\ntraining as an upstream port or a\n\ndownstream port (0 = upstream, 1\n\n= downstream). Default value\n\ndepends on CORE_TYPE strap pin."]
    #[inline(always)]
    pub fn ltd(&self) -> LtdR {
        LtdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Phy Error Reporting \\[APER\\]\n\nIf set to 0, the core will only report\n\nthose errors that caused a TLP or\n\nDLLP to be dropped because of a\n\ndetected phy error, TLP framing\n\nerror or DLLP framing error. When\n\nset to 1, the core will report all\n\ndetected phy errors regardless of\n\nwhether a TLP or DLLP was dropped\n\nbut does not include TLP nor DLLP\n\nframing errors. Detected phy errors\n\ninclude:- received errors indicated\n\non PIPE RxStatus interface, and TLP\n\nor DLLP framing errors depending on\n\nthe programmed value of this bit."]
    #[inline(always)]
    pub fn aper(&self) -> AperR {
        AperR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tx Swing Setting \\[TSS\\]\n\nThis bit drives the PIPE_TX_SWING\n\noutput of the core."]
    #[inline(always)]
    pub fn tss(&self) -> TssR {
        TssR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Received FTS Count for 2.5 GT/s speed \\[RFC\\]\n\nFTS count received from the other\n\nside during link training for use at\n\nthe 2.5 GT/s link speed. The core\n\ntransmits this many FTS sequences\n\nwhile exiting the L0S state, when\n\noperating at the 2.5 GT/s speed."]
    #[inline(always)]
    pub fn rfc(&self) -> RfcR {
        RfcR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Received Link ID \\[RLID\\]\n\nLink ID received from other side\n\nduring link training."]
    #[inline(always)]
    pub fn rlid(&self) -> RlidR {
        RlidR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:29 - LTSSM State \\[LTSSM\\]\n\nCurrent state of the LTSSM. The\n\nencoding of the states is given in\n\nAppendix C."]
    #[inline(always)]
    pub fn ltssm(&self) -> LtssmR {
        LtssmR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Remote Linkwidth Upconfigure Capability Status \\[R0\\]\n\nA 1 in this field indicates that the\n\nremote node advertised Link Width\n\nUp configure Capability in the\n\ntraining sequences in the\n\nConfiguration. Complete state when\n\nthe link came up. A 0 indicates that\n\nthe remote node did not set the Link\n\nUp configure bit."]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Master Loopback Enable \\[MLE\\]\n\nWhen the core is operating as a Root\n\nPort, setting this to 1 causes the\n\nLTSSM to initiate a loopback and\n\nbecome the loopback master. This\n\nbit is not used in the EndPoint Mode."]
    #[inline(always)]
    pub fn mle(&self) -> MleR {
        MleR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Phy Error Reporting \\[APER\\]\n\nIf set to 0, the core will only report\n\nthose errors that caused a TLP or\n\nDLLP to be dropped because of a\n\ndetected phy error, TLP framing\n\nerror or DLLP framing error. When\n\nset to 1, the core will report all\n\ndetected phy errors regardless of\n\nwhether a TLP or DLLP was dropped\n\nbut does not include TLP nor DLLP\n\nframing errors. Detected phy errors\n\ninclude:- received errors indicated\n\non PIPE RxStatus interface, and TLP\n\nor DLLP framing errors depending on\n\nthe programmed value of this bit."]
    #[inline(always)]
    #[must_use]
    pub fn aper(&mut self) -> AperW<PcieLmPhysicalLayerConfiguration0Spec> {
        AperW::new(self, 6)
    }
    #[doc = "Bit 7 - Tx Swing Setting \\[TSS\\]\n\nThis bit drives the PIPE_TX_SWING\n\noutput of the core."]
    #[inline(always)]
    #[must_use]
    pub fn tss(&mut self) -> TssW<PcieLmPhysicalLayerConfiguration0Spec> {
        TssW::new(self, 7)
    }
    #[doc = "Bit 31 - Master Loopback Enable \\[MLE\\]\n\nWhen the core is operating as a Root\n\nPort, setting this to 1 causes the\n\nLTSSM to initiate a loopback and\n\nbecome the loopback master. This\n\nbit is not used in the EndPoint Mode."]
    #[inline(always)]
    #[must_use]
    pub fn mle(&mut self) -> MleW<PcieLmPhysicalLayerConfiguration0Spec> {
        MleW::new(self, 31)
    }
}
#[doc = "Physical Layer Configuration Register 0\n\nWhen the core is operating as a Root\n\nPort, setting this to 1 causes the\n\nLTSSM to initiate a loopback and\n\nbecome the loopback master. This\n\nbit is not used in the EndPoint Mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_physical_layer_configuration_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_physical_layer_configuration_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmPhysicalLayerConfiguration0Spec;
impl crate::RegisterSpec for PcieLmPhysicalLayerConfiguration0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_physical_layer_configuration_0::R`](R) reader structure"]
impl crate::Readable for PcieLmPhysicalLayerConfiguration0Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_physical_layer_configuration_0::W`](W) writer structure"]
impl crate::Writable for PcieLmPhysicalLayerConfiguration0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_LM_PHYSICAL_LAYER_CONFIGURATION_0 to value 0x24"]
impl crate::Resettable for PcieLmPhysicalLayerConfiguration0Spec {
    const RESET_VALUE: u32 = 0x24;
}
