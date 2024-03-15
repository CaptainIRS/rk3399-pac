#[doc = "Register `GMAC_VLAN_TAG` reader"]
pub type R = crate::R<GmacVlanTagSpec>;
#[doc = "Register `GMAC_VLAN_TAG` writer"]
pub type W = crate::W<GmacVlanTagSpec>;
#[doc = "Field `VL` reader - VLAN Tag Identifier for Receive Frames This contains the 802.1Q VLAN tag to identify VLAN frames, and is compared to the fifteenth and sixteenth bytes of the frames being received for VLAN frames. Bits\\[15:13\\]
are the User Priority, Bit\\[12\\]
is the Canonical Format Indicator (CFI) and bits\\[11:0\\]
are the VLAN tag's VLAN Identifier (VID) field. When the ETV bit is set, only the VID (Bits\\[11:0\\]) is used for comparison. If VL (VL\\[11:0\\]
if ETV is set) is all zeros, the GMAC does not check the fifteenth and sixteenth bytes for VLAN tag comparison, and declares all frames with a Type field value of 0x8100 to be VLAN frames."]
pub type VlR = crate::FieldReader<u16>;
#[doc = "Field `VL` writer - VLAN Tag Identifier for Receive Frames This contains the 802.1Q VLAN tag to identify VLAN frames, and is compared to the fifteenth and sixteenth bytes of the frames being received for VLAN frames. Bits\\[15:13\\]
are the User Priority, Bit\\[12\\]
is the Canonical Format Indicator (CFI) and bits\\[11:0\\]
are the VLAN tag's VLAN Identifier (VID) field. When the ETV bit is set, only the VID (Bits\\[11:0\\]) is used for comparison. If VL (VL\\[11:0\\]
if ETV is set) is all zeros, the GMAC does not check the fifteenth and sixteenth bytes for VLAN tag comparison, and declares all frames with a Type field value of 0x8100 to be VLAN frames."]
pub type VlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ETV` reader - Enable 12-Bit VLAN Tag Comparison When this bit is set, a 12-bit VLAN identifier, rather than the complete 16-bit VLAN tag, is used for comparison and filtering. Bits\\[11:0\\]
of the VLAN tag are compared with the corresponding field in the received VLAN-tagged frame. When this bit is reset, all 16 bits of the received VLAN frame's fifteenth and sixteenth bytes are used for comparison."]
pub type EtvR = crate::BitReader;
#[doc = "Field `ETV` writer - Enable 12-Bit VLAN Tag Comparison When this bit is set, a 12-bit VLAN identifier, rather than the complete 16-bit VLAN tag, is used for comparison and filtering. Bits\\[11:0\\]
of the VLAN tag are compared with the corresponding field in the received VLAN-tagged frame. When this bit is reset, all 16 bits of the received VLAN frame's fifteenth and sixteenth bytes are used for comparison."]
pub type EtvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Frames This contains the 802.1Q VLAN tag to identify VLAN frames, and is compared to the fifteenth and sixteenth bytes of the frames being received for VLAN frames. Bits\\[15:13\\]
are the User Priority, Bit\\[12\\]
is the Canonical Format Indicator (CFI) and bits\\[11:0\\]
are the VLAN tag's VLAN Identifier (VID) field. When the ETV bit is set, only the VID (Bits\\[11:0\\]) is used for comparison. If VL (VL\\[11:0\\]
if ETV is set) is all zeros, the GMAC does not check the fifteenth and sixteenth bytes for VLAN tag comparison, and declares all frames with a Type field value of 0x8100 to be VLAN frames."]
    #[inline(always)]
    pub fn vl(&self) -> VlR {
        VlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison When this bit is set, a 12-bit VLAN identifier, rather than the complete 16-bit VLAN tag, is used for comparison and filtering. Bits\\[11:0\\]
of the VLAN tag are compared with the corresponding field in the received VLAN-tagged frame. When this bit is reset, all 16 bits of the received VLAN frame's fifteenth and sixteenth bytes are used for comparison."]
    #[inline(always)]
    pub fn etv(&self) -> EtvR {
        EtvR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Tag Identifier for Receive Frames This contains the 802.1Q VLAN tag to identify VLAN frames, and is compared to the fifteenth and sixteenth bytes of the frames being received for VLAN frames. Bits\\[15:13\\]
are the User Priority, Bit\\[12\\]
is the Canonical Format Indicator (CFI) and bits\\[11:0\\]
are the VLAN tag's VLAN Identifier (VID) field. When the ETV bit is set, only the VID (Bits\\[11:0\\]) is used for comparison. If VL (VL\\[11:0\\]
if ETV is set) is all zeros, the GMAC does not check the fifteenth and sixteenth bytes for VLAN tag comparison, and declares all frames with a Type field value of 0x8100 to be VLAN frames."]
    #[inline(always)]
    #[must_use]
    pub fn vl(&mut self) -> VlW<GmacVlanTagSpec> {
        VlW::new(self, 0)
    }
    #[doc = "Bit 16 - Enable 12-Bit VLAN Tag Comparison When this bit is set, a 12-bit VLAN identifier, rather than the complete 16-bit VLAN tag, is used for comparison and filtering. Bits\\[11:0\\]
of the VLAN tag are compared with the corresponding field in the received VLAN-tagged frame. When this bit is reset, all 16 bits of the received VLAN frame's fifteenth and sixteenth bytes are used for comparison."]
    #[inline(always)]
    #[must_use]
    pub fn etv(&mut self) -> EtvW<GmacVlanTagSpec> {
        EtvW::new(self, 16)
    }
}
#[doc = "VLAN Tag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_vlan_tag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_vlan_tag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacVlanTagSpec;
impl crate::RegisterSpec for GmacVlanTagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_vlan_tag::R`](R) reader structure"]
impl crate::Readable for GmacVlanTagSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_vlan_tag::W`](W) writer structure"]
impl crate::Writable for GmacVlanTagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_VLAN_TAG to value 0"]
impl crate::Resettable for GmacVlanTagSpec {
    const RESET_VALUE: u32 = 0;
}
