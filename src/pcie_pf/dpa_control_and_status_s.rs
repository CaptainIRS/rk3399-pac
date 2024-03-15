#[doc = "Register `DPA_CONTROL_AND_STATUS_S` reader"]
pub type R = crate::R<DpaControlAndStatusSSpec>;
#[doc = "Register `DPA_CONTROL_AND_STATUS_S` writer"]
pub type W = crate::W<DpaControlAndStatusSSpec>;
#[doc = "Field `SS` reader - Substate Status \\[SS\\]
This field provides the current DPA substate of this Function. This field is writable from the local management bus, and must be updated by the local software running on the EndPoint upon completion of a DPA transition to a new substate."]
pub type SsR = crate::FieldReader;
#[doc = "Field `R3` reader - Reserved \\[R3\\]
Reserved"]
pub type R3R = crate::FieldReader;
#[doc = "Field `SCE` reader - Substate Control Enabled \\[SCE\\]
This bit enables the Substate Control field. This bit is initialized to 1 by the hardware on a power- on reset or a Function-Level Reset. Software may clear this bit by writing a 1 to this bit position, but cannot set this bit directly through a configuration write. Clearing this bit disables the Substate Control field, thus preventing further substate transitions for this Function. This bit can be set to 0 or 1 through the local management bus, by writing a 0 or 1, respectively."]
pub type SceR = crate::BitReader;
#[doc = "Field `SCE` writer - Substate Control Enabled \\[SCE\\]
This bit enables the Substate Control field. This bit is initialized to 1 by the hardware on a power- on reset or a Function-Level Reset. Software may clear this bit by writing a 1 to this bit position, but cannot set this bit directly through a configuration write. Clearing this bit disables the Substate Control field, thus preventing further substate transitions for this Function. This bit can be set to 0 or 1 through the local management bus, by writing a 0 or 1, respectively."]
pub type SceW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R4` reader - Reserved \\[R4\\]
Reserved"]
pub type R4R = crate::FieldReader;
#[doc = "Field `SC` reader - Substate Control \\[SC\\]
This field is used to initiate a transition of the Function's DPA to a new substate. To initiate the transition, software must write the desired substate value into this field and wait for the transition latency of the substate for the Function to complete the transition. This field can also be written from the local management bus. All substate transitions are disabled when the Substate Control Enabled bit is 0. The core generates a one-cycle pulse on the output DPA_INTERRUPT when the value if this field is changed (bit 0 is for PF 0 and so on) This interrupt informs the client of the request from software to change the DPA substate. In response, the client must read the Substate Control field from this register to determine the new substate, and perform the actions necessary to effect the substate change. On completion of the substate change, the client must update the Substate Status field to reflect the new substate the function is in."]
pub type ScR = crate::FieldReader;
#[doc = "Field `SC` writer - Substate Control \\[SC\\]
This field is used to initiate a transition of the Function's DPA to a new substate. To initiate the transition, software must write the desired substate value into this field and wait for the transition latency of the substate for the Function to complete the transition. This field can also be written from the local management bus. All substate transitions are disabled when the Substate Control Enabled bit is 0. The core generates a one-cycle pulse on the output DPA_INTERRUPT when the value if this field is changed (bit 0 is for PF 0 and so on) This interrupt informs the client of the request from software to change the DPA substate. In response, the client must read the Substate Control field from this register to determine the new substate, and perform the actions necessary to effect the substate change. On completion of the substate change, the client must update the Substate Status field to reflect the new substate the function is in."]
pub type ScW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `R5` reader - Reserved \\[R5\\]
Reserved"]
pub type R5R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:4 - Substate Status \\[SS\\]
This field provides the current DPA substate of this Function. This field is writable from the local management bus, and must be updated by the local software running on the EndPoint upon completion of a DPA transition to a new substate."]
    #[inline(always)]
    pub fn ss(&self) -> SsR {
        SsR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Reserved \\[R3\\]
Reserved"]
    #[inline(always)]
    pub fn r3(&self) -> R3R {
        R3R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Substate Control Enabled \\[SCE\\]
This bit enables the Substate Control field. This bit is initialized to 1 by the hardware on a power- on reset or a Function-Level Reset. Software may clear this bit by writing a 1 to this bit position, but cannot set this bit directly through a configuration write. Clearing this bit disables the Substate Control field, thus preventing further substate transitions for this Function. This bit can be set to 0 or 1 through the local management bus, by writing a 0 or 1, respectively."]
    #[inline(always)]
    pub fn sce(&self) -> SceR {
        SceR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Reserved \\[R4\\]
Reserved"]
    #[inline(always)]
    pub fn r4(&self) -> R4R {
        R4R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:20 - Substate Control \\[SC\\]
This field is used to initiate a transition of the Function's DPA to a new substate. To initiate the transition, software must write the desired substate value into this field and wait for the transition latency of the substate for the Function to complete the transition. This field can also be written from the local management bus. All substate transitions are disabled when the Substate Control Enabled bit is 0. The core generates a one-cycle pulse on the output DPA_INTERRUPT when the value if this field is changed (bit 0 is for PF 0 and so on) This interrupt informs the client of the request from software to change the DPA substate. In response, the client must read the Substate Control field from this register to determine the new substate, and perform the actions necessary to effect the substate change. On completion of the substate change, the client must update the Substate Status field to reflect the new substate the function is in."]
    #[inline(always)]
    pub fn sc(&self) -> ScR {
        ScR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:31 - Reserved \\[R5\\]
Reserved"]
    #[inline(always)]
    pub fn r5(&self) -> R5R {
        R5R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 8 - Substate Control Enabled \\[SCE\\]
This bit enables the Substate Control field. This bit is initialized to 1 by the hardware on a power- on reset or a Function-Level Reset. Software may clear this bit by writing a 1 to this bit position, but cannot set this bit directly through a configuration write. Clearing this bit disables the Substate Control field, thus preventing further substate transitions for this Function. This bit can be set to 0 or 1 through the local management bus, by writing a 0 or 1, respectively."]
    #[inline(always)]
    #[must_use]
    pub fn sce(&mut self) -> SceW<DpaControlAndStatusSSpec> {
        SceW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Substate Control \\[SC\\]
This field is used to initiate a transition of the Function's DPA to a new substate. To initiate the transition, software must write the desired substate value into this field and wait for the transition latency of the substate for the Function to complete the transition. This field can also be written from the local management bus. All substate transitions are disabled when the Substate Control Enabled bit is 0. The core generates a one-cycle pulse on the output DPA_INTERRUPT when the value if this field is changed (bit 0 is for PF 0 and so on) This interrupt informs the client of the request from software to change the DPA substate. In response, the client must read the Substate Control field from this register to determine the new substate, and perform the actions necessary to effect the substate change. On completion of the substate change, the client must update the Substate Status field to reflect the new substate the function is in."]
    #[inline(always)]
    #[must_use]
    pub fn sc(&mut self) -> ScW<DpaControlAndStatusSSpec> {
        ScW::new(self, 16)
    }
}
#[doc = "DPA Control and Status Registers Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpa_control_and_status_s::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpa_control_and_status_s::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpaControlAndStatusSSpec;
impl crate::RegisterSpec for DpaControlAndStatusSSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpa_control_and_status_s::R`](R) reader structure"]
impl crate::Readable for DpaControlAndStatusSSpec {}
#[doc = "`write(|w| ..)` method takes [`dpa_control_and_status_s::W`](W) writer structure"]
impl crate::Writable for DpaControlAndStatusSSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0100;
}
#[doc = "`reset()` method sets DPA_CONTROL_AND_STATUS_S to value 0x0100"]
impl crate::Resettable for DpaControlAndStatusSSpec {
    const RESET_VALUE: u32 = 0x0100;
}
