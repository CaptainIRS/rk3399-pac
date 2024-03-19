#[doc = "Register `PCIE_PF_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS_2` reader"]
pub type R = crate::R<PciePfPciExpressDeviceControlAndStatus2Spec>;
#[doc = "Register `PCIE_PF_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS_2` writer"]
pub type W = crate::W<PciePfPciExpressDeviceControlAndStatus2Spec>;
#[doc = "Field `CTV` reader - Completion Timeout Value \\[CTV\\]
Specifies the Completion Timeout value for the device. Allowable values are 0101 (sub-range 1) and 0110 (sub-range 2). The corresponding timeout values are stored in the local management registers Completion Timeout Interval Registers 0 and 1, respectively."]
pub type CtvR = crate::FieldReader;
#[doc = "Field `CTV` writer - Completion Timeout Value \\[CTV\\]
Specifies the Completion Timeout value for the device. Allowable values are 0101 (sub-range 1) and 0110 (sub-range 2). The corresponding timeout values are stored in the local management registers Completion Timeout Interval Registers 0 and 1, respectively."]
pub type CtvW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CTD` reader - Completion Timeout Disable \\[CTD\\]
Setting this bit disables Completion Timeout in the device. This bit can also be written from the local management bus."]
pub type CtdR = crate::BitReader;
#[doc = "Field `CTD` writer - Completion Timeout Disable \\[CTD\\]
Setting this bit disables Completion Timeout in the device. This bit can also be written from the local management bus."]
pub type CtdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFE` reader - ARI forwarding enable \\[AFE\\]
ARI forwarding enable"]
pub type AfeR = crate::BitReader;
#[doc = "Field `AORE` reader - Atomic Op Requester Enable \\[AORE\\]
This bit must be set to enable the generation of Atomic Op Requests from the Function. If the client logic attempts to send an Atomic Op for a Function for which this bit is not set, logic in the core will nullify the TLP on its way to the link. This bit can also be written from the local management bus."]
pub type AoreR = crate::BitReader;
#[doc = "Field `R16` reader - Reserved \\[R16\\]
Reserved"]
pub type R16R = crate::BitReader;
#[doc = "Field `IDORE` reader - IDO Request Enable \\[IDORE\\]
When this bit is 1, the Function is allowed to set the ID-based Ordering (IDO) Attribute bit in the requests it generates."]
pub type IdoreR = crate::BitReader;
#[doc = "Field `IDOCE` reader - IDO Completion Enable \\[IDOCE\\]
When this bit is 1, the Function is allowed to set the ID-based Ordering (IDO) Attribute bit in the Completions it generates."]
pub type IdoceR = crate::BitReader;
#[doc = "Field `LTRME` reader - LTR Mechanism Enable \\[LTRME\\]
This must be set to 1 to enable the Latency Tolerance Reporting Mechanism. This bit is implemented only in PF 0. Its default value is 1, but can be modified from the local management bus. This bit is read- only in PF 1."]
pub type LtrmeR = crate::BitReader;
#[doc = "Field `LTRME` writer - LTR Mechanism Enable \\[LTRME\\]
This must be set to 1 to enable the Latency Tolerance Reporting Mechanism. This bit is implemented only in PF 0. Its default value is 1, but can be modified from the local management bus. This bit is read- only in PF 1."]
pub type LtrmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R17` reader - Reserved \\[R17\\]
Reserved"]
pub type R17R = crate::FieldReader;
#[doc = "Field `OBFFE` reader - OBFF Enable \\[OBFFE\\]
Enables the Optimized Buffer Flush/Fill (OBFF) capability in the device. This field is implemented only in PF 0. Valid settings are 00 (disabled), 01 (Variation A) and 10 (Variation B). This field can also be written from the local management bus. RW if OBFF capability is supported, RO otherwise."]
pub type ObffeR = crate::FieldReader;
#[doc = "Field `OBFFE` writer - OBFF Enable \\[OBFFE\\]
Enables the Optimized Buffer Flush/Fill (OBFF) capability in the device. This field is implemented only in PF 0. Valid settings are 00 (disabled), 01 (Variation A) and 10 (Variation B). This field can also be written from the local management bus. RW if OBFF capability is supported, RO otherwise."]
pub type ObffeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `R18` reader - Reserved \\[R18\\]
Reserved"]
pub type R18R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - Completion Timeout Value \\[CTV\\]
Specifies the Completion Timeout value for the device. Allowable values are 0101 (sub-range 1) and 0110 (sub-range 2). The corresponding timeout values are stored in the local management registers Completion Timeout Interval Registers 0 and 1, respectively."]
    #[inline(always)]
    pub fn ctv(&self) -> CtvR {
        CtvR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Completion Timeout Disable \\[CTD\\]
Setting this bit disables Completion Timeout in the device. This bit can also be written from the local management bus."]
    #[inline(always)]
    pub fn ctd(&self) -> CtdR {
        CtdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ARI forwarding enable \\[AFE\\]
ARI forwarding enable"]
    #[inline(always)]
    pub fn afe(&self) -> AfeR {
        AfeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Atomic Op Requester Enable \\[AORE\\]
This bit must be set to enable the generation of Atomic Op Requests from the Function. If the client logic attempts to send an Atomic Op for a Function for which this bit is not set, logic in the core will nullify the TLP on its way to the link. This bit can also be written from the local management bus."]
    #[inline(always)]
    pub fn aore(&self) -> AoreR {
        AoreR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved \\[R16\\]
Reserved"]
    #[inline(always)]
    pub fn r16(&self) -> R16R {
        R16R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IDO Request Enable \\[IDORE\\]
When this bit is 1, the Function is allowed to set the ID-based Ordering (IDO) Attribute bit in the requests it generates."]
    #[inline(always)]
    pub fn idore(&self) -> IdoreR {
        IdoreR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IDO Completion Enable \\[IDOCE\\]
When this bit is 1, the Function is allowed to set the ID-based Ordering (IDO) Attribute bit in the Completions it generates."]
    #[inline(always)]
    pub fn idoce(&self) -> IdoceR {
        IdoceR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LTR Mechanism Enable \\[LTRME\\]
This must be set to 1 to enable the Latency Tolerance Reporting Mechanism. This bit is implemented only in PF 0. Its default value is 1, but can be modified from the local management bus. This bit is read- only in PF 1."]
    #[inline(always)]
    pub fn ltrme(&self) -> LtrmeR {
        LtrmeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Reserved \\[R17\\]
Reserved"]
    #[inline(always)]
    pub fn r17(&self) -> R17R {
        R17R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - OBFF Enable \\[OBFFE\\]
Enables the Optimized Buffer Flush/Fill (OBFF) capability in the device. This field is implemented only in PF 0. Valid settings are 00 (disabled), 01 (Variation A) and 10 (Variation B). This field can also be written from the local management bus. RW if OBFF capability is supported, RO otherwise."]
    #[inline(always)]
    pub fn obffe(&self) -> ObffeR {
        ObffeR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:31 - Reserved \\[R18\\]
Reserved"]
    #[inline(always)]
    pub fn r18(&self) -> R18R {
        R18R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - Completion Timeout Value \\[CTV\\]
Specifies the Completion Timeout value for the device. Allowable values are 0101 (sub-range 1) and 0110 (sub-range 2). The corresponding timeout values are stored in the local management registers Completion Timeout Interval Registers 0 and 1, respectively."]
    #[inline(always)]
    #[must_use]
    pub fn ctv(&mut self) -> CtvW<PciePfPciExpressDeviceControlAndStatus2Spec> {
        CtvW::new(self, 0)
    }
    #[doc = "Bit 4 - Completion Timeout Disable \\[CTD\\]
Setting this bit disables Completion Timeout in the device. This bit can also be written from the local management bus."]
    #[inline(always)]
    #[must_use]
    pub fn ctd(&mut self) -> CtdW<PciePfPciExpressDeviceControlAndStatus2Spec> {
        CtdW::new(self, 4)
    }
    #[doc = "Bit 10 - LTR Mechanism Enable \\[LTRME\\]
This must be set to 1 to enable the Latency Tolerance Reporting Mechanism. This bit is implemented only in PF 0. Its default value is 1, but can be modified from the local management bus. This bit is read- only in PF 1."]
    #[inline(always)]
    #[must_use]
    pub fn ltrme(&mut self) -> LtrmeW<PciePfPciExpressDeviceControlAndStatus2Spec> {
        LtrmeW::new(self, 10)
    }
    #[doc = "Bits 13:14 - OBFF Enable \\[OBFFE\\]
Enables the Optimized Buffer Flush/Fill (OBFF) capability in the device. This field is implemented only in PF 0. Valid settings are 00 (disabled), 01 (Variation A) and 10 (Variation B). This field can also be written from the local management bus. RW if OBFF capability is supported, RO otherwise."]
    #[inline(always)]
    #[must_use]
    pub fn obffe(&mut self) -> ObffeW<PciePfPciExpressDeviceControlAndStatus2Spec> {
        ObffeW::new(self, 13)
    }
}
#[doc = "PCI Express Device Control and Status Register 2 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_pci_express_device_control_and_status_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_pci_express_device_control_and_status_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfPciExpressDeviceControlAndStatus2Spec;
impl crate::RegisterSpec for PciePfPciExpressDeviceControlAndStatus2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_pci_express_device_control_and_status_2::R`](R) reader structure"]
impl crate::Readable for PciePfPciExpressDeviceControlAndStatus2Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_pci_express_device_control_and_status_2::W`](W) writer structure"]
impl crate::Writable for PciePfPciExpressDeviceControlAndStatus2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_PF_PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS_2 to value 0"]
impl crate::Resettable for PciePfPciExpressDeviceControlAndStatus2Spec {
    const RESET_VALUE: u32 = 0;
}
