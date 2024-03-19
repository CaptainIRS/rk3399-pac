#[doc = "Register `PCIE_LM_LINKWIDTH_CONTROL` reader"]
pub type R = crate::R<PcieLmLinkwidthControlSpec>;
#[doc = "Register `PCIE_LM_LINKWIDTH_CONTROL` writer"]
pub type W = crate::W<PcieLmLinkwidthControlSpec>;
#[doc = "Field `TLM` reader - Target Lane Map \\[TLM\\]
This field contains the bitmap of the lanes to be included in forming the link during the re-training. If the target lane map includes lanes that were inactive when retraining is initiated, then both the core and its link partner must support the LinkWidth Upconfigure Capability to be able to activate those lanes. The user can check if the remote node has this capability by reading the Remote Link Upconfigure Capability Status bit in Physical Layer Configuration Register 0 after the link first came up."]
pub type TlmR = crate::FieldReader;
#[doc = "Field `TLM` writer - Target Lane Map \\[TLM\\]
This field contains the bitmap of the lanes to be included in forming the link during the re-training. If the target lane map includes lanes that were inactive when retraining is initiated, then both the core and its link partner must support the LinkWidth Upconfigure Capability to be able to activate those lanes. The user can check if the remote node has this capability by reading the Remote Link Upconfigure Capability Status bit in Physical Layer Configuration Register 0 after the link first came up."]
pub type TlmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `R0` reader - Reserved \\[R0\\]
Reserved"]
pub type R0R = crate::FieldReader<u16>;
#[doc = "Field `RL` reader - Retrain Link \\[RL\\]
Writing a 1 into this field results in the core re- training the link to change its width. When setting this bit to 1, the software must also set the target lane-map field to indicate the lanes it desires to be part of the link. The core will attempt to form a link with this set of lanes. The link formed at the end of the retraining may include all of these lanes (if both nodes agree on them during re- training), or the largest subset that both sides were able to activate. This bit is cleared by the internal logic of the core after the re-training has been completed and link has reached the L0 state. Software must wait for the bit to be clear before setting it again to change the link width."]
pub type RlR = crate::BitReader;
#[doc = "Field `RL` writer - Retrain Link \\[RL\\]
Writing a 1 into this field results in the core re- training the link to change its width. When setting this bit to 1, the software must also set the target lane-map field to indicate the lanes it desires to be part of the link. The core will attempt to form a link with this set of lanes. The link formed at the end of the retraining may include all of these lanes (if both nodes agree on them during re- training), or the largest subset that both sides were able to activate. This bit is cleared by the internal logic of the core after the re-training has been completed and link has reached the L0 state. Software must wait for the bit to be clear before setting it again to change the link width."]
pub type RlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R1` reader - Reserved \\[R1\\]
Reserved"]
pub type R1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Target Lane Map \\[TLM\\]
This field contains the bitmap of the lanes to be included in forming the link during the re-training. If the target lane map includes lanes that were inactive when retraining is initiated, then both the core and its link partner must support the LinkWidth Upconfigure Capability to be able to activate those lanes. The user can check if the remote node has this capability by reading the Remote Link Upconfigure Capability Status bit in Physical Layer Configuration Register 0 after the link first came up."]
    #[inline(always)]
    pub fn tlm(&self) -> TlmR {
        TlmR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Reserved \\[R0\\]
Reserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - Retrain Link \\[RL\\]
Writing a 1 into this field results in the core re- training the link to change its width. When setting this bit to 1, the software must also set the target lane-map field to indicate the lanes it desires to be part of the link. The core will attempt to form a link with this set of lanes. The link formed at the end of the retraining may include all of these lanes (if both nodes agree on them during re- training), or the largest subset that both sides were able to activate. This bit is cleared by the internal logic of the core after the re-training has been completed and link has reached the L0 state. Software must wait for the bit to be clear before setting it again to change the link width."]
    #[inline(always)]
    pub fn rl(&self) -> RlR {
        RlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - Reserved \\[R1\\]
Reserved"]
    #[inline(always)]
    pub fn r1(&self) -> R1R {
        R1R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Target Lane Map \\[TLM\\]
This field contains the bitmap of the lanes to be included in forming the link during the re-training. If the target lane map includes lanes that were inactive when retraining is initiated, then both the core and its link partner must support the LinkWidth Upconfigure Capability to be able to activate those lanes. The user can check if the remote node has this capability by reading the Remote Link Upconfigure Capability Status bit in Physical Layer Configuration Register 0 after the link first came up."]
    #[inline(always)]
    #[must_use]
    pub fn tlm(&mut self) -> TlmW<PcieLmLinkwidthControlSpec> {
        TlmW::new(self, 0)
    }
    #[doc = "Bit 16 - Retrain Link \\[RL\\]
Writing a 1 into this field results in the core re- training the link to change its width. When setting this bit to 1, the software must also set the target lane-map field to indicate the lanes it desires to be part of the link. The core will attempt to form a link with this set of lanes. The link formed at the end of the retraining may include all of these lanes (if both nodes agree on them during re- training), or the largest subset that both sides were able to activate. This bit is cleared by the internal logic of the core after the re-training has been completed and link has reached the L0 state. Software must wait for the bit to be clear before setting it again to change the link width."]
    #[inline(always)]
    #[must_use]
    pub fn rl(&mut self) -> RlW<PcieLmLinkwidthControlSpec> {
        RlW::new(self, 16)
    }
}
#[doc = "Linkwidth Control Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_linkwidth_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_linkwidth_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
