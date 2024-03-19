#[doc = "Register `PCIE_LM_LINKWIDTH_CONTROL` reader"]
pub type R = crate::R<PcieLmLinkwidthControlSpec>;
#[doc = "Register `PCIE_LM_LINKWIDTH_CONTROL` writer"]
pub type W = crate::W<PcieLmLinkwidthControlSpec>;
#[doc = "Field `TLM` reader - Target Lane Map \\[TLM\\]\n\nThis field contains the bitmap of the\n\nlanes to be included in forming the\n\nlink during the re-training. If the\n\ntarget lane map includes lanes that\n\nwere inactive when retraining is\n\ninitiated, then both the core and its\n\nlink partner must support the\n\nLinkWidth Upconfigure Capability to\n\nbe able to activate those lanes. The\n\nuser can check if the remote node\n\nhas this capability by reading the\n\nRemote Link Upconfigure Capability\n\nStatus bit in Physical Layer\n\nConfiguration Register 0 after the\n\nlink first came up."]
pub type TlmR = crate::FieldReader;
#[doc = "Field `TLM` writer - Target Lane Map \\[TLM\\]\n\nThis field contains the bitmap of the\n\nlanes to be included in forming the\n\nlink during the re-training. If the\n\ntarget lane map includes lanes that\n\nwere inactive when retraining is\n\ninitiated, then both the core and its\n\nlink partner must support the\n\nLinkWidth Upconfigure Capability to\n\nbe able to activate those lanes. The\n\nuser can check if the remote node\n\nhas this capability by reading the\n\nRemote Link Upconfigure Capability\n\nStatus bit in Physical Layer\n\nConfiguration Register 0 after the\n\nlink first came up."]
pub type TlmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `R0` reader - Reserved \\[R0\\]\n\nReserved"]
pub type R0R = crate::FieldReader<u16>;
#[doc = "Field `RL` reader - Retrain Link \\[RL\\]\n\nWriting a 1 into this field results in\n\nthe core re- training the link to\n\nchange its width. When setting this\n\nbit to 1, the software must also set\n\nthe target lane-map field to indicate\n\nthe lanes it desires to be part of the\n\nlink. The core will attempt to form a\n\nlink with this set of lanes. The link\n\nformed at the end of the retraining\n\nmay include all of these lanes (if\n\nboth nodes agree on them during re-\n\ntraining), or the largest subset that\n\nboth sides were able to activate. This\n\nbit is cleared by the internal logic of\n\nthe core after the re-training has\n\nbeen completed and link has reached\n\nthe L0 state. Software must wait for\n\nthe bit to be clear before setting it\n\nagain to change the link width."]
pub type RlR = crate::BitReader;
#[doc = "Field `RL` writer - Retrain Link \\[RL\\]\n\nWriting a 1 into this field results in\n\nthe core re- training the link to\n\nchange its width. When setting this\n\nbit to 1, the software must also set\n\nthe target lane-map field to indicate\n\nthe lanes it desires to be part of the\n\nlink. The core will attempt to form a\n\nlink with this set of lanes. The link\n\nformed at the end of the retraining\n\nmay include all of these lanes (if\n\nboth nodes agree on them during re-\n\ntraining), or the largest subset that\n\nboth sides were able to activate. This\n\nbit is cleared by the internal logic of\n\nthe core after the re-training has\n\nbeen completed and link has reached\n\nthe L0 state. Software must wait for\n\nthe bit to be clear before setting it\n\nagain to change the link width."]
pub type RlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R1` reader - Reserved \\[R1\\]\n\nReserved"]
pub type R1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Target Lane Map \\[TLM\\]\n\nThis field contains the bitmap of the\n\nlanes to be included in forming the\n\nlink during the re-training. If the\n\ntarget lane map includes lanes that\n\nwere inactive when retraining is\n\ninitiated, then both the core and its\n\nlink partner must support the\n\nLinkWidth Upconfigure Capability to\n\nbe able to activate those lanes. The\n\nuser can check if the remote node\n\nhas this capability by reading the\n\nRemote Link Upconfigure Capability\n\nStatus bit in Physical Layer\n\nConfiguration Register 0 after the\n\nlink first came up."]
    #[inline(always)]
    pub fn tlm(&self) -> TlmR {
        TlmR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Reserved \\[R0\\]\n\nReserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - Retrain Link \\[RL\\]\n\nWriting a 1 into this field results in\n\nthe core re- training the link to\n\nchange its width. When setting this\n\nbit to 1, the software must also set\n\nthe target lane-map field to indicate\n\nthe lanes it desires to be part of the\n\nlink. The core will attempt to form a\n\nlink with this set of lanes. The link\n\nformed at the end of the retraining\n\nmay include all of these lanes (if\n\nboth nodes agree on them during re-\n\ntraining), or the largest subset that\n\nboth sides were able to activate. This\n\nbit is cleared by the internal logic of\n\nthe core after the re-training has\n\nbeen completed and link has reached\n\nthe L0 state. Software must wait for\n\nthe bit to be clear before setting it\n\nagain to change the link width."]
    #[inline(always)]
    pub fn rl(&self) -> RlR {
        RlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - Reserved \\[R1\\]\n\nReserved"]
    #[inline(always)]
    pub fn r1(&self) -> R1R {
        R1R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Target Lane Map \\[TLM\\]\n\nThis field contains the bitmap of the\n\nlanes to be included in forming the\n\nlink during the re-training. If the\n\ntarget lane map includes lanes that\n\nwere inactive when retraining is\n\ninitiated, then both the core and its\n\nlink partner must support the\n\nLinkWidth Upconfigure Capability to\n\nbe able to activate those lanes. The\n\nuser can check if the remote node\n\nhas this capability by reading the\n\nRemote Link Upconfigure Capability\n\nStatus bit in Physical Layer\n\nConfiguration Register 0 after the\n\nlink first came up."]
    #[inline(always)]
    #[must_use]
    pub fn tlm(&mut self) -> TlmW<PcieLmLinkwidthControlSpec> {
        TlmW::new(self, 0)
    }
    #[doc = "Bit 16 - Retrain Link \\[RL\\]\n\nWriting a 1 into this field results in\n\nthe core re- training the link to\n\nchange its width. When setting this\n\nbit to 1, the software must also set\n\nthe target lane-map field to indicate\n\nthe lanes it desires to be part of the\n\nlink. The core will attempt to form a\n\nlink with this set of lanes. The link\n\nformed at the end of the retraining\n\nmay include all of these lanes (if\n\nboth nodes agree on them during re-\n\ntraining), or the largest subset that\n\nboth sides were able to activate. This\n\nbit is cleared by the internal logic of\n\nthe core after the re-training has\n\nbeen completed and link has reached\n\nthe L0 state. Software must wait for\n\nthe bit to be clear before setting it\n\nagain to change the link width."]
    #[inline(always)]
    #[must_use]
    pub fn rl(&mut self) -> RlW<PcieLmLinkwidthControlSpec> {
        RlW::new(self, 16)
    }
}
#[doc = "Linkwidth Control Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_linkwidth_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_linkwidth_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmLinkwidthControlSpec;
impl crate::RegisterSpec for PcieLmLinkwidthControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_linkwidth_control::R`](R) reader structure"]
impl crate::Readable for PcieLmLinkwidthControlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_linkwidth_control::W`](W) writer structure"]
impl crate::Writable for PcieLmLinkwidthControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_LM_LINKWIDTH_CONTROL to value 0x0f"]
impl crate::Resettable for PcieLmLinkwidthControlSpec {
    const RESET_VALUE: u32 = 0x0f;
}
