#[doc = "Register `PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS` reader"]
pub type R = crate::R<PciExpressDeviceControlAndStatusSpec>;
#[doc = "Register `PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS` writer"]
pub type W = crate::W<PciExpressDeviceControlAndStatusSpec>;
#[doc = "Field `ECER` reader - Enable Correctable Error Reporting \\[ECER\\]
Reserved"]
pub type EcerR = crate::BitReader;
#[doc = "Field `ENFER` reader - Enable Non- Fatal Error Reporting \\[ENFER\\]
Reserved"]
pub type EnferR = crate::BitReader;
#[doc = "Field `EFER` reader - Enable Fatal Error Reporting \\[EFER\\]
Reserved"]
pub type EferR = crate::BitReader;
#[doc = "Field `EURR` reader - Enable Unsupported Request Reporting \\[EURR\\]
Reserved"]
pub type EurrR = crate::BitReader;
#[doc = "Field `ERO` reader - Enable Relaxed Ordering \\[ERO\\]
Reserved"]
pub type EroR = crate::BitReader;
#[doc = "Field `MPS` reader - Max Payload Size \\[MPS\\]
Reserved"]
pub type MpsR = crate::FieldReader;
#[doc = "Field `ETFE` reader - Extended Tag Field Enable \\[ETFE\\]
Reserved"]
pub type EtfeR = crate::BitReader;
#[doc = "Field `EPF` reader - Enable Phantom Functions \\[EPF\\]
Reserved"]
pub type EpfR = crate::BitReader;
#[doc = "Field `EAP` reader - Enable Aux Power \\[EAP\\]
Reserved"]
pub type EapR = crate::BitReader;
#[doc = "Field `EBS` reader - Enable No Snoop \\[EBS\\]
Reserved"]
pub type EbsR = crate::BitReader;
#[doc = "Field `MRRS` reader - Max Read Request Size \\[MRRS\\]
Reserved"]
pub type MrrsR = crate::FieldReader;
#[doc = "Field `FLR` reader - Function- Level Reset \\[FLR\\]
Writing a 1 into this bit position generated a Function-Level Reset for the selected VF. This bit reads as 0."]
pub type FlrR = crate::BitReader;
#[doc = "Field `FLR` writer - Function- Level Reset \\[FLR\\]
Writing a 1 into this bit position generated a Function-Level Reset for the selected VF. This bit reads as 0."]
pub type FlrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CED` reader - Correctable Error Detected \\[CED\\]
Set to 1 by the core when it detects a correctable error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked"]
pub type CedR = crate::BitReader;
#[doc = "Field `CED` writer - Correctable Error Detected \\[CED\\]
Set to 1 by the core when it detects a correctable error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked"]
pub type CedW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `NFER` reader - Non-Fatal Error Detected \\[NFER\\]
Set to 1 by the core when it detects a non-fatal error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked."]
pub type NferR = crate::BitReader;
#[doc = "Field `NFER` writer - Non-Fatal Error Detected \\[NFER\\]
Set to 1 by the core when it detects a non-fatal error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked."]
pub type NferW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FED` reader - Fatal Error Detected \\[FED\\]
Set to 1 by the core when it detects a fatal error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked."]
pub type FedR = crate::BitReader;
#[doc = "Field `FED` writer - Fatal Error Detected \\[FED\\]
Set to 1 by the core when it detects a fatal error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked."]
pub type FedW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `URD` reader - Unsupported Request Detected \\[URD\\]
Set to 1 by the core when it receives an unsupported request, regardless of whether its reporting is enabled or not."]
pub type UrdR = crate::BitReader;
#[doc = "Field `URD` writer - Unsupported Request Detected \\[URD\\]
Set to 1 by the core when it receives an unsupported request, regardless of whether its reporting is enabled or not."]
pub type UrdW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `APD` reader - Aux Power Detected \\[APD\\]
Reserved"]
pub type ApdR = crate::BitReader;
#[doc = "Field `TP` reader - Transaction Pending \\[TP\\]
Indicates if any of the Non-Posted requests issued by the VF are still pending."]
pub type TpR = crate::BitReader;
#[doc = "Field `R4` reader - Reserved \\[R4\\]
Reserved"]
pub type R4R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Enable Correctable Error Reporting \\[ECER\\]
Reserved"]
    #[inline(always)]
    pub fn ecer(&self) -> EcerR {
        EcerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Non- Fatal Error Reporting \\[ENFER\\]
Reserved"]
    #[inline(always)]
    pub fn enfer(&self) -> EnferR {
        EnferR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Fatal Error Reporting \\[EFER\\]
Reserved"]
    #[inline(always)]
    pub fn efer(&self) -> EferR {
        EferR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Unsupported Request Reporting \\[EURR\\]
Reserved"]
    #[inline(always)]
    pub fn eurr(&self) -> EurrR {
        EurrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Relaxed Ordering \\[ERO\\]
Reserved"]
    #[inline(always)]
    pub fn ero(&self) -> EroR {
        EroR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Max Payload Size \\[MPS\\]
Reserved"]
    #[inline(always)]
    pub fn mps(&self) -> MpsR {
        MpsR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Extended Tag Field Enable \\[ETFE\\]
Reserved"]
    #[inline(always)]
    pub fn etfe(&self) -> EtfeR {
        EtfeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Phantom Functions \\[EPF\\]
Reserved"]
    #[inline(always)]
    pub fn epf(&self) -> EpfR {
        EpfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Aux Power \\[EAP\\]
Reserved"]
    #[inline(always)]
    pub fn eap(&self) -> EapR {
        EapR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable No Snoop \\[EBS\\]
Reserved"]
    #[inline(always)]
    pub fn ebs(&self) -> EbsR {
        EbsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Max Read Request Size \\[MRRS\\]
Reserved"]
    #[inline(always)]
    pub fn mrrs(&self) -> MrrsR {
        MrrsR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Function- Level Reset \\[FLR\\]
Writing a 1 into this bit position generated a Function-Level Reset for the selected VF. This bit reads as 0."]
    #[inline(always)]
    pub fn flr(&self) -> FlrR {
        FlrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Correctable Error Detected \\[CED\\]
Set to 1 by the core when it detects a correctable error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked"]
    #[inline(always)]
    pub fn ced(&self) -> CedR {
        CedR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Non-Fatal Error Detected \\[NFER\\]
Set to 1 by the core when it detects a non-fatal error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked."]
    #[inline(always)]
    pub fn nfer(&self) -> NferR {
        NferR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fatal Error Detected \\[FED\\]
Set to 1 by the core when it detects a fatal error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked."]
    #[inline(always)]
    pub fn fed(&self) -> FedR {
        FedR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Unsupported Request Detected \\[URD\\]
Set to 1 by the core when it receives an unsupported request, regardless of whether its reporting is enabled or not."]
    #[inline(always)]
    pub fn urd(&self) -> UrdR {
        UrdR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Aux Power Detected \\[APD\\]
Reserved"]
    #[inline(always)]
    pub fn apd(&self) -> ApdR {
        ApdR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transaction Pending \\[TP\\]
Indicates if any of the Non-Posted requests issued by the VF are still pending."]
    #[inline(always)]
    pub fn tp(&self) -> TpR {
        TpR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:31 - Reserved \\[R4\\]
Reserved"]
    #[inline(always)]
    pub fn r4(&self) -> R4R {
        R4R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - Function- Level Reset \\[FLR\\]
Writing a 1 into this bit position generated a Function-Level Reset for the selected VF. This bit reads as 0."]
    #[inline(always)]
    #[must_use]
    pub fn flr(&mut self) -> FlrW<PciExpressDeviceControlAndStatusSpec> {
        FlrW::new(self, 15)
    }
    #[doc = "Bit 16 - Correctable Error Detected \\[CED\\]
Set to 1 by the core when it detects a correctable error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked"]
    #[inline(always)]
    #[must_use]
    pub fn ced(&mut self) -> CedW<PciExpressDeviceControlAndStatusSpec> {
        CedW::new(self, 16)
    }
    #[doc = "Bit 17 - Non-Fatal Error Detected \\[NFER\\]
Set to 1 by the core when it detects a non-fatal error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked."]
    #[inline(always)]
    #[must_use]
    pub fn nfer(&mut self) -> NferW<PciExpressDeviceControlAndStatusSpec> {
        NferW::new(self, 17)
    }
    #[doc = "Bit 18 - Fatal Error Detected \\[FED\\]
Set to 1 by the core when it detects a fatal error, regardless of whether error reporting is enabled or not, and regardless of whether the error is masked."]
    #[inline(always)]
    #[must_use]
    pub fn fed(&mut self) -> FedW<PciExpressDeviceControlAndStatusSpec> {
        FedW::new(self, 18)
    }
    #[doc = "Bit 19 - Unsupported Request Detected \\[URD\\]
Set to 1 by the core when it receives an unsupported request, regardless of whether its reporting is enabled or not."]
    #[inline(always)]
    #[must_use]
    pub fn urd(&mut self) -> UrdW<PciExpressDeviceControlAndStatusSpec> {
        UrdW::new(self, 19)
    }
}
#[doc = "PCI Express Device Control and Status Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pci_express_device_control_and_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pci_express_device_control_and_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciExpressDeviceControlAndStatusSpec;
impl crate::RegisterSpec for PciExpressDeviceControlAndStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pci_express_device_control_and_status::R`](R) reader structure"]
impl crate::Readable for PciExpressDeviceControlAndStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pci_express_device_control_and_status::W`](W) writer structure"]
impl crate::Writable for PciExpressDeviceControlAndStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x000f_0000;
}
#[doc = "`reset()` method sets PCI_EXPRESS_DEVICE_CONTROL_AND_STATUS to value 0"]
impl crate::Resettable for PciExpressDeviceControlAndStatusSpec {
    const RESET_VALUE: u32 = 0;
}
