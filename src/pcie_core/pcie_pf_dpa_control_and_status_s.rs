#[doc = "Register `PCIE_PF_DPA_CONTROL_AND_STATUS_S` reader"]
pub type R = crate::R<PciePfDpaControlAndStatusSSpec>;
#[doc = "Register `PCIE_PF_DPA_CONTROL_AND_STATUS_S` writer"]
pub type W = crate::W<PciePfDpaControlAndStatusSSpec>;
#[doc = "Field `SS` reader - Substate Status \\[SS\\]\n\nThis field provides the current DPA\n\nsubstate of this Function. This field\n\nis writable from the local\n\nmanagement bus, and must be\n\nupdated by the local software\n\nrunning on the EndPoint upon\n\ncompletion of a DPA transition to a\n\nnew substate."]
pub type SsR = crate::FieldReader;
#[doc = "Field `R3` reader - Reserved \\[R3\\]\n\nReserved"]
pub type R3R = crate::FieldReader;
#[doc = "Field `SCE` reader - Substate Control Enabled \\[SCE\\]\n\nThis bit enables the Substate Control\n\nfield. This bit is initialized to 1 by the\n\nhardware on a power- on reset or a\n\nFunction-Level Reset. Software may\n\nclear this bit by writing a 1 to this bit\n\nposition, but cannot set this bit\n\ndirectly through a configuration\n\nwrite. Clearing this bit disables the\n\nSubstate Control field, thus\n\npreventing further substate\n\ntransitions for this Function. This bit\n\ncan be set to 0 or 1 through the local\n\nmanagement bus, by writing a 0 or\n\n1, respectively."]
pub type SceR = crate::BitReader;
#[doc = "Field `SCE` writer - Substate Control Enabled \\[SCE\\]\n\nThis bit enables the Substate Control\n\nfield. This bit is initialized to 1 by the\n\nhardware on a power- on reset or a\n\nFunction-Level Reset. Software may\n\nclear this bit by writing a 1 to this bit\n\nposition, but cannot set this bit\n\ndirectly through a configuration\n\nwrite. Clearing this bit disables the\n\nSubstate Control field, thus\n\npreventing further substate\n\ntransitions for this Function. This bit\n\ncan be set to 0 or 1 through the local\n\nmanagement bus, by writing a 0 or\n\n1, respectively."]
pub type SceW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R4` reader - Reserved \\[R4\\]\n\nReserved"]
pub type R4R = crate::FieldReader;
#[doc = "Field `SC` reader - Substate Control \\[SC\\]\n\nThis field is used to initiate a\n\ntransition of the Function's DPA to a\n\nnew substate. To initiate the\n\ntransition, software must write the\n\ndesired substate value into this field\n\nand wait for the transition latency of\n\nthe substate for the Function to\n\ncomplete the transition. This field\n\ncan also be written from the local\n\nmanagement bus. All substate\n\ntransitions are disabled when the\n\nSubstate Control Enabled bit is 0.\n\nThe core generates a one-cycle pulse\n\non the output DPA_INTERRUPT when\n\nthe value if this field is changed (bit\n\n0 is for PF 0 and so on) This\n\ninterrupt informs the client of the\n\nrequest from software to change the\n\nDPA substate. In response, the client\n\nmust read the Substate Control field\n\nfrom this register to determine the\n\nnew substate, and perform the\n\nactions necessary to effect the\n\nsubstate change. On completion of\n\nthe substate change, the client must\n\nupdate the Substate Status field to\n\nreflect the new substate the function\n\nis in."]
pub type ScR = crate::FieldReader;
#[doc = "Field `SC` writer - Substate Control \\[SC\\]\n\nThis field is used to initiate a\n\ntransition of the Function's DPA to a\n\nnew substate. To initiate the\n\ntransition, software must write the\n\ndesired substate value into this field\n\nand wait for the transition latency of\n\nthe substate for the Function to\n\ncomplete the transition. This field\n\ncan also be written from the local\n\nmanagement bus. All substate\n\ntransitions are disabled when the\n\nSubstate Control Enabled bit is 0.\n\nThe core generates a one-cycle pulse\n\non the output DPA_INTERRUPT when\n\nthe value if this field is changed (bit\n\n0 is for PF 0 and so on) This\n\ninterrupt informs the client of the\n\nrequest from software to change the\n\nDPA substate. In response, the client\n\nmust read the Substate Control field\n\nfrom this register to determine the\n\nnew substate, and perform the\n\nactions necessary to effect the\n\nsubstate change. On completion of\n\nthe substate change, the client must\n\nupdate the Substate Status field to\n\nreflect the new substate the function\n\nis in."]
pub type ScW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `R5` reader - Reserved \\[R5\\]\n\nReserved"]
pub type R5R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:4 - Substate Status \\[SS\\]\n\nThis field provides the current DPA\n\nsubstate of this Function. This field\n\nis writable from the local\n\nmanagement bus, and must be\n\nupdated by the local software\n\nrunning on the EndPoint upon\n\ncompletion of a DPA transition to a\n\nnew substate."]
    #[inline(always)]
    pub fn ss(&self) -> SsR {
        SsR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Reserved \\[R3\\]\n\nReserved"]
    #[inline(always)]
    pub fn r3(&self) -> R3R {
        R3R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Substate Control Enabled \\[SCE\\]\n\nThis bit enables the Substate Control\n\nfield. This bit is initialized to 1 by the\n\nhardware on a power- on reset or a\n\nFunction-Level Reset. Software may\n\nclear this bit by writing a 1 to this bit\n\nposition, but cannot set this bit\n\ndirectly through a configuration\n\nwrite. Clearing this bit disables the\n\nSubstate Control field, thus\n\npreventing further substate\n\ntransitions for this Function. This bit\n\ncan be set to 0 or 1 through the local\n\nmanagement bus, by writing a 0 or\n\n1, respectively."]
    #[inline(always)]
    pub fn sce(&self) -> SceR {
        SceR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Reserved \\[R4\\]\n\nReserved"]
    #[inline(always)]
    pub fn r4(&self) -> R4R {
        R4R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:20 - Substate Control \\[SC\\]\n\nThis field is used to initiate a\n\ntransition of the Function's DPA to a\n\nnew substate. To initiate the\n\ntransition, software must write the\n\ndesired substate value into this field\n\nand wait for the transition latency of\n\nthe substate for the Function to\n\ncomplete the transition. This field\n\ncan also be written from the local\n\nmanagement bus. All substate\n\ntransitions are disabled when the\n\nSubstate Control Enabled bit is 0.\n\nThe core generates a one-cycle pulse\n\non the output DPA_INTERRUPT when\n\nthe value if this field is changed (bit\n\n0 is for PF 0 and so on) This\n\ninterrupt informs the client of the\n\nrequest from software to change the\n\nDPA substate. In response, the client\n\nmust read the Substate Control field\n\nfrom this register to determine the\n\nnew substate, and perform the\n\nactions necessary to effect the\n\nsubstate change. On completion of\n\nthe substate change, the client must\n\nupdate the Substate Status field to\n\nreflect the new substate the function\n\nis in."]
    #[inline(always)]
    pub fn sc(&self) -> ScR {
        ScR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:31 - Reserved \\[R5\\]\n\nReserved"]
    #[inline(always)]
    pub fn r5(&self) -> R5R {
        R5R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 8 - Substate Control Enabled \\[SCE\\]\n\nThis bit enables the Substate Control\n\nfield. This bit is initialized to 1 by the\n\nhardware on a power- on reset or a\n\nFunction-Level Reset. Software may\n\nclear this bit by writing a 1 to this bit\n\nposition, but cannot set this bit\n\ndirectly through a configuration\n\nwrite. Clearing this bit disables the\n\nSubstate Control field, thus\n\npreventing further substate\n\ntransitions for this Function. This bit\n\ncan be set to 0 or 1 through the local\n\nmanagement bus, by writing a 0 or\n\n1, respectively."]
    #[inline(always)]
    #[must_use]
    pub fn sce(&mut self) -> SceW<PciePfDpaControlAndStatusSSpec> {
        SceW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Substate Control \\[SC\\]\n\nThis field is used to initiate a\n\ntransition of the Function's DPA to a\n\nnew substate. To initiate the\n\ntransition, software must write the\n\ndesired substate value into this field\n\nand wait for the transition latency of\n\nthe substate for the Function to\n\ncomplete the transition. This field\n\ncan also be written from the local\n\nmanagement bus. All substate\n\ntransitions are disabled when the\n\nSubstate Control Enabled bit is 0.\n\nThe core generates a one-cycle pulse\n\non the output DPA_INTERRUPT when\n\nthe value if this field is changed (bit\n\n0 is for PF 0 and so on) This\n\ninterrupt informs the client of the\n\nrequest from software to change the\n\nDPA substate. In response, the client\n\nmust read the Substate Control field\n\nfrom this register to determine the\n\nnew substate, and perform the\n\nactions necessary to effect the\n\nsubstate change. On completion of\n\nthe substate change, the client must\n\nupdate the Substate Status field to\n\nreflect the new substate the function\n\nis in."]
    #[inline(always)]
    #[must_use]
    pub fn sc(&mut self) -> ScW<PciePfDpaControlAndStatusSSpec> {
        ScW::new(self, 16)
    }
}
#[doc = "DPA Control and Status Registers\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_dpa_control_and_status_s::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_dpa_control_and_status_s::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfDpaControlAndStatusSSpec;
impl crate::RegisterSpec for PciePfDpaControlAndStatusSSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_dpa_control_and_status_s::R`](R) reader structure"]
impl crate::Readable for PciePfDpaControlAndStatusSSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_dpa_control_and_status_s::W`](W) writer structure"]
impl crate::Writable for PciePfDpaControlAndStatusSSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0100;
}
#[doc = "`reset()` method sets PCIE_PF_DPA_CONTROL_AND_STATUS_S to value 0x0100"]
impl crate::Resettable for PciePfDpaControlAndStatusSSpec {
    const RESET_VALUE: u32 = 0x0100;
}
